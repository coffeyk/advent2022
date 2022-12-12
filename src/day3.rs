use std::collections::HashSet;
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
        line.map(|text| text.split_at(text.len() / 2))
    }
}

struct Input2<'a> {
    lines: str::Lines<'a>,
}
impl<'a> Iterator for Input2<'a> {
    type Item = (&'a str, &'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        match line {
            Some(text) => Some((text, self.lines.next().unwrap(), self.lines.next().unwrap())),
            None => None,
        }
    }
}

fn byte_rank(b: &u8) -> u8 {
    match b {
        b'A'..=b'Z' => b - b'A' + 27,
        b'a'..=b'z' => b - b'a' + 1,
        _ => panic!("unknown!"),
    }
}

pub fn part_a() {
    let file_path = format!("src/input{DAY}.txt");

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let mut results: u32 = 0;
    for (pack_1, pack_2) in it {
        let p1_set: HashSet<&u8> = HashSet::from_iter(pack_1.as_bytes());
        let p2_set: HashSet<&u8> = HashSet::from_iter(pack_2.as_bytes());

        let duplicate = *p1_set.intersection(&p2_set).next().unwrap();

        results += byte_rank(duplicate) as u32;
    }
    // Solution 7446
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let file_path = format!("src/input{DAY}.txt");

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read {DAY} the file");

    let it = Input2 {
        lines: contents.lines(),
    };

    let mut results: u32 = 0;
    for (pack_1, pack_2, pack_3) in it {
        let p1_set: HashSet<&u8> = HashSet::from_iter(pack_1.as_bytes());
        let p2_set: HashSet<&u8> = HashSet::from_iter(pack_2.as_bytes());
        let p3_set: HashSet<&u8> = HashSet::from_iter(pack_3.as_bytes());

        let duplicate = *p1_set
            .intersection(&p2_set)
            .find(|x| p3_set.contains(**x))
            .unwrap();

        results += byte_rank(duplicate) as u32;
    }
    // Solution 2646
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
