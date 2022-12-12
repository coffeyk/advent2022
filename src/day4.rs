use std::cmp;
use std::fs;
use std::str;

const DAY: i32 = 4;

struct InputRow {
    e1_start: u32,
    e1_end: u32,
    e2_start: u32,
    e2_end: u32,
}
struct Input<'a> {
    lines: str::Lines<'a>,
}
impl<'a> Iterator for Input<'a> {
    type Item = InputRow;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        match line {
            Some(text) => {
                if text.is_empty() {
                    return None;
                }
                println!("{text}");
                let v: Vec<&str> = text.split(|c| c == ',' || c == '-').collect();
                Some(InputRow {
                    e1_start: v[0].parse::<u32>().unwrap(),
                    e1_end: v[1].parse::<u32>().unwrap(),
                    e2_start: v[2].parse::<u32>().unwrap(),
                    e2_end: v[3].parse::<u32>().unwrap(),
                })
            }
            None => None,
        }
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

pub fn part_a() {
    let file_path = format!("src/input{DAY}.txt");

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let mut results: u32 = 0;
    for row in it {
        let overlap_start = cmp::max(row.e1_start, row.e2_start);
        let overlap_end = cmp::min(row.e1_end, row.e2_end);

        if (overlap_start == row.e1_start && overlap_end == row.e1_end)
            || (overlap_start == row.e2_start && overlap_end == row.e2_end)
        {
            results += 1
        }
    }
    // Solution 651
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let file_path = format!("src/input{DAY}.txt");

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let mut results: u32 = 0;
    for row in it {
        let overlap_start = cmp::max(row.e1_start, row.e2_start);
        let overlap_end = cmp::min(row.e1_end, row.e2_end);

        if overlap_start <= overlap_end {
            results += 1
        }
    }
    // Solution 956
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
