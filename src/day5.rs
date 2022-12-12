use std::fs;
use std::str;

const DAY: i32 = 5;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

#[derive(Debug)]
struct Command {
    count: u8,
    from: usize,
    to: usize,
}

struct Input<'a> {
    lines: str::Lines<'a>,
}

impl<'a> Input<'a> {
    fn stacks(&mut self) -> Stacks {
        let mut first_line = true;
        let mut results = Stacks::new();

        for line in &mut self.lines {
            // Blank row before moves input
            if line.is_empty() {
                // results are all backwards
                results.iter_mut().for_each(|r| r.reverse());

                println! {"{results:?}"};
                return results;
            }
            // indexing row
            if line.starts_with(" 1 ") {
                continue;
            }

            let result_size = (line.len() + 1) / 4;
            let mut chars = line.chars();

            let mut boxes = Vec::new();
            for _ in 0..result_size {
                // 1..41..4
                // [0] [2]
                // 0123456
                //  1...5
                // entries are ~4 characters wide
                // we want the 2nd character, so offset of 1

                // This would be so much easier if we could iterate over iterators
                // skip one
                chars.next();
                let val = chars.next().unwrap();
                boxes.push(val);
                // skip two more
                chars.next();
                chars.next();
            }

            // Size out the results on the first real row
            if first_line {
                first_line = false;

                // Add one to length for missing separator after final stack
                for _ in 0..result_size {
                    results.push(Stack::new());
                }
            }
            // True row of data
            for (i, &v) in boxes.iter().enumerate() {
                if v != ' ' {
                    results[i].push(v);
                }
            }
        }
        panic!("not enough input");
    }
}

impl<'a> Iterator for Input<'a> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        match line {
            Some(text) => {
                if text.is_empty() {
                    return None;
                }

                let values: Vec<u8> = text
                    .split_whitespace()
                    .filter_map(|section| section.parse::<u8>().ok())
                    .collect();

                Some(Command {
                    count: values[0],
                    from: values[1] as usize,
                    to: values[2] as usize,
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
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let mut it = Input {
        lines: contents.lines(),
    };

    let mut stacks = it.stacks();

    println!("{stacks:?}");
    // let mut results: Vec<char> = Vec::new();
    for cmd in it {
        println!("{cmd:?}");
        // move `count` from `from` to `to`
        for _ in 0..cmd.count {
            let c = stacks[cmd.from - 1].pop().unwrap();
            stacks[cmd.to - 1].push(c);
        }
    }

    let results: String = stacks.iter().filter_map(|s| s.last()).collect();
    // Solution BWNCQRMDB
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let mut it = Input {
        lines: contents.lines(),
    };

    let mut stacks = it.stacks();

    println!("{stacks:?}");
    for cmd in it {
        println!("{cmd:?}");
        // move `count` from `from` to `to`
        // Push and pop onto a temporary stack to preserve order
        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..cmd.count {
            let c = stacks[cmd.from - 1].pop().unwrap();
            tmp.push(c);
        }
        for _ in 0..cmd.count {
            let c = tmp.pop().unwrap();
            stacks[cmd.to - 1].push(c);
        }
    }

    let results: String = stacks.iter().filter_map(|s| s.last()).collect();
    // Solution NHWZCBNBF
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
