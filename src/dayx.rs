use std::cmp;
use std::collections::BinaryHeap;
use std::fs;
use std::str;

const DAY: i32 = 3;

struct Input<'a> {
    lines: str::Lines<'a>,
}
impl<'a> Iterator for Input<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        match line {
            Some(text) => {
                let mut split = text.split_whitespace();
                let l = split.next().unwrap();
                let r = split.next().unwrap();
                return Some((l, r));
            }
            None => return None,
        }
    }
}

pub fn part_a() {
    let file_path = format!("src/input{DAY}.txt");

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

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
    // Solution
    println!("Day {DAY}a best:\n{best}");
}

pub fn part_b() {
    let file_path = "src/input{DAY}.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let count = 3;
    let mut current = 0;
    let mut heap = BinaryHeap::new();
    for line in contents.lines() {
        if line.is_empty() {
            heap.push(current * -1);
            if heap.len() > count {
                heap.pop();
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    let results: i32 = heap.iter().sum();
    let actual_results = results * -1;
    // Solution
    println!("Day {DAY}b best:\n{actual_results}");
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
