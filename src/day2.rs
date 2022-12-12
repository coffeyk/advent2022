use std::fs;
use std::str;

const DAY: i32 = 2;

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
                Some((l, r))
            }
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
    let mut total = 0;
    for (l, r) in it {
        let outcome = match l {
            "A" => match r {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => panic!("unmatched"),
            },
            "B" => match r {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("unmatched"),
            },
            "C" => match r {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => panic!("unmatched"),
            },
            _ => panic!("unmatched"),
        };
        let shape = match r {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("unmatched"),
        };
        total += shape + outcome;
    }

    // // Solution 13052
    println!("Day {DAY}a best:\n{total}");
}

pub fn part_b() {
    let file_path = format!("src/input{DAY}.txt");

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };
    let mut total = 0;
    for (l, r) in it {
        // X means you need to lose = 0
        // Y means you need to end the round in a draw = 3
        // Z means you need to win = 6
        let outcome = match r {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("unmatched"),
        };
        let shape = match l {
            "A" => match r {
                "X" => 3,
                "Y" => 1,
                "Z" => 2,
                _ => panic!("unmatched"),
            },
            "B" => match r {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("unmatched"),
            },
            "C" => match r {
                "X" => 2,
                "Y" => 3,
                "Z" => 1,
                _ => panic!("unmatched"),
            },
            _ => panic!("unmatched"),
        };
        total += shape + outcome;
    }

    // // Solution 13693
    println!("Day {DAY}b best:\n{total}");
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
