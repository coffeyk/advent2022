use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::rc::Rc;

const DAY: i32 = 12;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct PriorityQueueEntry<T: Ord + Copy> {
    priority: i32,
    count: i32,
    task: RefCell<Option<T>>,
}

struct PriorityQueue<T: Hash + Ord + Copy> {
    pq: BinaryHeap<Rc<Reverse<PriorityQueueEntry<T>>>>,
    entry_finder: HashMap<T, Rc<Reverse<PriorityQueueEntry<T>>>>,
    counter: i32,
}

impl<T: Hash + Ord + Copy> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue {
            pq: BinaryHeap::new(),
            entry_finder: HashMap::new(),
            counter: 0,
        }
    }
    fn push(&mut self, task: T, priority: i32) {
        if self.entry_finder.contains_key(&task) {
            self.remove_task(task);
        }
        self.counter += 1;
        let entry = Rc::new(Reverse(PriorityQueueEntry {
            priority,
            count: self.counter,
            task: RefCell::new(Some(task)),
        }));
        self.entry_finder.insert(task, Rc::clone(&entry));
        self.pq.push(Rc::clone(&entry));
    }
    fn remove_task(&mut self, task: T) {
        self.entry_finder.entry(task).and_modify(|e| {
            e.0.task.replace(None);
            // e.task.replace(None);
        });
    }
    fn pop(&mut self) -> Option<T> {
        while !self.pq.is_empty() {
            let entry = self.pq.pop();
            if let Some(e) = entry {
                let task = e.0.task.borrow_mut().take();

                if let Some(t) = task {
                    self.entry_finder.remove(&t);
                    return task;
                }
            }
        }
        None
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Ord, PartialOrd)]
struct DataPoint {
    // val: i32,
    x: usize,
    y: usize,
}

fn neighbors(grid: &Vec<Vec<u8>>, p: DataPoint) -> Vec<DataPoint> {
    let mut results: Vec<DataPoint> = Vec::new();
    let highest_neighbor = grid[p.y][p.x] + 1;

    if p.x > 0 && grid[p.y][p.x - 1] <= highest_neighbor {
        results.push(DataPoint { x: p.x - 1, y: p.y });
    }
    if p.x + 1 < grid[p.y].len() && grid[p.y][p.x + 1] <= highest_neighbor {
        results.push(DataPoint { x: p.x + 1, y: p.y });
    }
    if p.y > 0 && grid[p.y - 1][p.x] <= highest_neighbor {
        results.push(DataPoint { x: p.x, y: p.y - 1 });
    }
    if p.y + 1 < grid.len() && grid[p.y + 1][p.x] <= highest_neighbor {
        results.push(DataPoint { x: p.x, y: p.y + 1 });
    }
    results
}

fn walk(grid: &Vec<Vec<u8>>, start: DataPoint, end: DataPoint) -> i32 {
    let mut visited: HashSet<DataPoint> = HashSet::new();
    let mut distance: HashMap<DataPoint, i32> = HashMap::new();
    let mut pq: PriorityQueue<DataPoint> = PriorityQueue::new();

    distance.insert(start, 0);
    let start_distance = distance.get(&start).unwrap();
    pq.push(start, *start_distance);

    loop {
        // let current_node = pq.pop();
        if let Some(current_node) = pq.pop() {
            let &current_distance = distance.get(&current_node).unwrap();
            if current_node == end {
                // println!("pass");
                return current_distance;
            }
            for neighbor in neighbors(grid, current_node) {
                if !visited.contains(&neighbor) {
                    if let Some(&neighbor_distance) = distance.get(&neighbor) {
                        if neighbor_distance < current_distance + 1 {
                            // There was a shorter path to neighbor already
                            continue;
                        }
                    } // else neighbor not visited yet

                    // Current path is closer or first visit
                    pq.remove_task(neighbor);
                    let new_distance = current_distance + 1;
                    distance.insert(neighbor, new_distance);
                    pq.push(neighbor, new_distance);
                }
            }
            visited.insert(current_node);
        } else {
            return std::i32::MAX;
        }
    }

    // return 0;
}

pub fn part_a() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let grid: Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();

    // A terrible way to get start and end markers
    let mut start_end: Vec<(&char, usize, usize)> = grid
        .iter()
        .enumerate()
        .filter_map(|(row_idx, line)| {
            let exists: Vec<(&char, usize, usize)> = line
                .iter()
                .enumerate()
                .filter_map(|(col_idx, c)| match c {
                    'S' | 'E' => Some((c, row_idx, col_idx)),
                    _ => None,
                })
                .collect();
            if exists.is_empty() {
                None
            } else {
                Some(exists)
            }
        })
        .flatten()
        .take(2)
        .collect();
    start_end.sort();

    let ((_, start_y, start_x), (_, end_y, end_x)) = (start_end[1], start_end[0]);

    // A terrible way to convert letter to magnitude, with special handling for S & E markers being low and high respectively
    let grid_values: Vec<Vec<u8>> = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    _ => *c,
                })
                .map(|c| c.to_string().as_bytes()[0])
                .map(|b| match b {
                    b'a'..=b'z' => b - b'a',
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let start_point = DataPoint {
        x: start_x,
        y: start_y,
    };
    let end_point = DataPoint { x: end_x, y: end_y };

    println!("start_point: {start_point:?}\nend_point: {end_point:?}");

    let results = walk(&grid_values, start_point, end_point);

    // Solution 394

    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let grid: Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();

    // A terrible way to get start and end markers
    let mut start_end: Vec<(&char, usize, usize)> = grid
        .iter()
        .enumerate()
        .filter_map(|(row_idx, line)| {
            let exists: Vec<(&char, usize, usize)> = line
                .iter()
                .enumerate()
                .filter_map(|(col_idx, c)| match c {
                    'S' | 'E' => Some((c, row_idx, col_idx)),
                    _ => None,
                })
                .collect();
            if exists.is_empty() {
                None
            } else {
                Some(exists)
            }
        })
        .flatten()
        .take(2)
        .collect();
    start_end.sort();

    let ((_, _, _), (_, end_y, end_x)) = (start_end[1], start_end[0]);

    // A terrible way to convert letter to magnitude, with special handling for S & E markers being low and high respectively
    let grid_values: Vec<Vec<u8>> = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    _ => *c,
                })
                .map(|c| c.to_string().as_bytes()[0])
                .map(|b| match b {
                    b'a'..=b'z' => b - b'a',
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    // Ignore the official start and try all the lowest points
    let end_point = DataPoint { x: end_x, y: end_y };

    let mut starts: Vec<(usize, usize)> = grid_values
        .iter()
        .enumerate()
        .filter_map(|(row_idx, line)| {
            let exists: Vec<(usize, usize)> = line
                .iter()
                .enumerate()
                .filter_map(|(col_idx, &c)| match c {
                    0 => Some((row_idx, col_idx)),
                    _ => None,
                })
                .collect();
            if exists.is_empty() {
                None
            } else {
                Some(exists)
            }
        })
        .flatten()
        .collect();
    // println!("{starts:?}");
    let results = starts
        .iter()
        .map(|(start_y, start_x)| {
            walk(
                &grid_values,
                DataPoint {
                    x: *start_x,
                    y: *start_y,
                },
                end_point,
            )
        })
        .min()
        .unwrap();
    // Solution 388
    println!("Day {DAY}b best:\n{results}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_part_a() {
        part_a();
    }

    #[test]
    fn do_part_b() {
        part_b();
    }
}
