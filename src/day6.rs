use std::fs;
// use std::str;
use std::collections::HashSet;

const DAY: i32 = 6;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

#[derive(Debug)]
struct Buffer<T> {
    memory: Vec<T>,
    capacity: usize,
    position: usize,
    end: usize,
}
impl<T: Clone> Buffer<T> {
    pub fn with_capacity(capacity: usize, default: T) -> Buffer<T> {
        let mut v = Vec::with_capacity(capacity);
        v.extend(std::iter::repeat(default).take(capacity));
        Buffer {
            memory: v,
            capacity,
            position: 0,
            end: 0,
        }
    }
    pub fn push(&mut self, val: T) {
        self.memory[self.position] = val;
        self.position += 1;
        if self.position >= self.capacity {
            self.position = 0;
        }
    }
}

pub fn part_a() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let mut results: usize = 0;
    let mut buf = Buffer::with_capacity(4, ' ');
    for (i, c) in contents.chars().enumerate() {
        buf.push(c);
        // Not enough data
        if i < 4 {
            continue;
        }
        let set: HashSet<char> = HashSet::from_iter(buf.memory.clone());
        let m = &buf.memory;
        println!("{m:?}");
        if set.len() == 4 {
            // Need 1 based indexing
            results = i + 1;
            break;
        }
    }

    // Solution 1343
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let mut results: usize = 0;
    let mut buf = Buffer::with_capacity(14, ' ');
    for (i, c) in contents.chars().enumerate() {
        buf.push(c);
        // Not enough data
        if i < 4 {
            continue;
        }
        let set: HashSet<char> = HashSet::from_iter(buf.memory.clone());
        let m = &buf.memory;
        println!("{m:?}");
        if set.len() == buf.capacity {
            // Need 1 based indexing
            results = i + 1;
            break;
        }
    }

    // Solution 2193
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
