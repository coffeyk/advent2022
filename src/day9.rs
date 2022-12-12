use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::iter;
use std::str;

const DAY: i32 = 9;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

enum Direction {
    U,
    D,
    L,
    R,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}",
            match self {
                Direction::U => "U",
                Direction::D => "D",
                Direction::L => "L",
                Direction::R => "R",
            }
        )
    }
}

struct Input<'a> {
    lines: str::Lines<'a>,
}

impl<'a> Iterator for Input<'a> {
    type Item = (Direction, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        if let Some(text) = line {
            // print!("{text}");
            let (r_dir, r_count) = text.split_once(' ').unwrap();
            Some((
                match r_dir {
                    "U" => Direction::U,
                    "D" => Direction::D,
                    "L" => Direction::L,
                    "R" => Direction::R,
                    _ => panic!(),
                },
                r_count.parse().unwrap(),
            ))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn step(&self, d: &Direction) -> Point {
        let mut x = self.x;
        let mut y = self.y;
        match d {
            Direction::U => y += 1,
            Direction::D => y -= 1,
            Direction::L => x -= 1,
            Direction::R => x += 1,
        }
        Point { x, y }
    }
}
struct State {
    // h: Point,
    // t: Point,
    points: Vec<Point>,
    seen: HashSet<Point>,
}

impl State {
    fn new(size: usize) -> State {
        let origin = Point { x: 0, y: 0 };

        State {
            points: iter::repeat(origin).take(size).collect(),
            seen: HashSet::from([origin]),
        }
    }
    fn step(&mut self, direction: &Direction, step: u32) {
        for _ in 0..step {
            self.points[0] = self.points[0].step(direction);
            let h = self.points[0];
            // println!("0 -> {h:?}");
            for i in 1..self.points.len() {
                let f = self.fix(self.points[i - 1], self.points[i]);
                self.points[i] = f;
                let p = self.points[i];
                // println!("{i} -> {p:?} -> {f:?}");
            }
            self.seen.insert(*self.points.last().unwrap());
            // println!("");
            // self.draw();
        }
    }

    fn fix(&self, h: Point, t: Point) -> Point {
        // println!("{h:?} {t:?}");
        let dx = h.x - t.x;
        let dy = h.y - t.y;

        if dx == 0 {
            if dy.abs() >= 2 {
                Point {
                    x: t.x,
                    y: t.y + dy / dy.abs(),
                }
            } else {
                t
            }
        } else if dy == 0 {
            if dx.abs() >= 2 {
                Point {
                    x: t.x + dx / dx.abs(),
                    y: t.y,
                }
            } else {
                t
            }
        } else if dx.abs() >= 2 || dy.abs() >= 2 {
            Point {
                x: t.x + dx / dx.abs(),
                y: t.y + dy / dy.abs(),
            }
        } else {
            t
        }
    }

    fn draw(&self) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for p in &self.points {
            min_x = cmp::min(min_x, p.x);
            min_y = cmp::min(min_y, p.y);
            max_x = cmp::max(max_x, p.x);
            max_y = cmp::max(max_y, p.y);
        }
        for y in (min_y..=max_y).rev() {
            let l: String = (min_x..=max_x)
                .map(
                    |x| match self.points.iter().position(|&p| p == Point { x, y }) {
                        Some(i) => format!("{i}"),
                        None => String::from("."),
                    },
                )
                .collect();
            println!("{l}");
            // for y in min_y..=max_y {}
        }
    }
}

pub fn part_a() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let mut state = State::new(2);
    for (d, s) in it {
        state.step(&d, s);
    }
    let results = state.seen.len();

    // Solution 6314
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let mut state = State::new(10);
    for (d, s) in it {
        println!("== {d} {s} ==");
        state.step(&d, s);
        // state.draw();
    }
    let results = state.seen.len();

    // Solution 2504
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
