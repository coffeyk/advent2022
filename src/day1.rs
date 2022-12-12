use std::cmp;
use std::collections::BinaryHeap;
use std::fs;

pub fn part_a() {
    let file_path = "src/input1.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut current = 0;
    let mut best = 0;
    for line in contents.lines() {
        if line.is_empty() {
            best = cmp::max(best, current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    // 70764
    println!("Day 1a best:\n{best}");
}

pub fn part_b() {
    let file_path = "src/input1.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let count = 3;
    let mut current = 0;
    let mut heap = BinaryHeap::new();
    for line in contents.lines() {
        heap.push(-current);
        if line.is_empty() {
            if heap.len() > count {
                heap.pop();
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    let results: i32 = heap.iter().sum();
    let actual_results = -results;
    // 203905
    println!("Day 1b best:\n{actual_results}");
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
