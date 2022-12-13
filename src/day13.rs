use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str;
use std::vec;

const DAY: i64 = 13;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

/////

type PacketList = Vec<PacketElement>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PacketElement {
    is_int: bool,
    i: u8,
    l: PacketList,
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_int && other.is_int {
            self.i.cmp(&other.i)
        } else if !self.is_int && !other.is_int {
            let answer_in_list =
                self.l
                    .iter()
                    .zip(other.l.iter())
                    .find_map(|(l, r)| match l.cmp(r) {
                        Ordering::Equal => None,
                        other => Some(other),
                    });
            if let Some(answer) = answer_in_list {
                // Answer found in list
                answer
            } else {
                // self should be shorter than other
                self.l.len().cmp(&other.l.len())
            }
        } else {
            // Mixed int and list
            if self.is_int {
                PacketElement::new_list(self.clone()).cmp(other)
            } else {
                self.cmp(&PacketElement::new_list(other.clone()))
            }
        }
    }
}

impl PacketElement {
    fn new_list(p: PacketElement) -> PacketElement {
        PacketElement {
            is_int: false,
            i: 0,
            l: vec![p],
        }
    }
    //                                   V
    // [[3,[[1,2,5,7,7],[9,8,5,8,7],7],[10,9],0],[10,[],5,[6,[],[0,6,6,4,4],[6,1,1,6,0],[]],[4]],[[7,[4,10,0],9,[9]],[[0]]]]
    fn from_str(s: &str) -> PacketElement {
        let mut stack: Vec<Box<PacketList>> = Vec::new();

        stack.push(Box::new(Vec::new()));

        let mut int: Option<u8> = None;
        for c in s.chars() {
            match c {
                '[' => {
                    stack.push(Box::new(Vec::new()));
                }
                ']' => {
                    if let Some(val) = int {
                        let finished_int = PacketElement {
                            is_int: true,
                            i: val,
                            l: Vec::new(),
                        };
                        stack.last_mut().unwrap().push(finished_int);
                        int = None;
                    }

                    let list_contents = stack.pop().unwrap();
                    // TODO: Can we do this without to_vec?
                    let finished_list = PacketElement {
                        is_int: false,
                        i: 0,
                        l: list_contents.to_vec(),
                    };
                    stack.last_mut().unwrap().push(finished_list);
                }
                ',' => {
                    if let Some(val) = int {
                        let finished_int = PacketElement {
                            is_int: true,
                            i: val,
                            l: Vec::new(),
                        };
                        stack.last_mut().unwrap().push(finished_int);
                        int = None;
                    }
                }
                '0'..='9' => {
                    let d = c.to_digit(10).unwrap() as u8;
                    match int {
                        Some(old) => int = Some(old * 10 + d),
                        None => int = Some(d),
                    }
                }
                _ => panic!(),
            }
        }
        let mut final_list = stack.pop().unwrap();
        final_list.pop().unwrap()
    }
}

/////////

struct Input<'a> {
    lines: str::Lines<'a>,
}

impl<'a> Iterator for Input<'a> {
    type Item = (PacketElement, PacketElement);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(text) = self.lines.next() {
            if text.is_empty() {
                return None;
            }

            let line_1 = PacketElement::from_str(text);
            let line_2 = PacketElement::from_str(self.lines.next().unwrap());
            self.lines.next();

            Some((line_1, line_2))
        } else {
            None
        }
    }
}

pub fn part_a() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let results: usize = it
        .enumerate()
        .filter_map(|(idx, (l1, l2))| if l1 < l2 { Some(idx + 1) } else { None })
        .sum();

    // Solution 6076
    // let results = "";
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let divider_packets = vec![
        PacketElement::from_str("[[2]]"),
        PacketElement::from_str("[[6]]"),
    ];
    let mut packets: PacketList = it
        .flat_map(|(l1, l2)| vec![l1, l2])
        .chain(divider_packets.clone())
        .collect();

    packets.sort();

    let results: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(idx, pe)| {
            if divider_packets[0].eq(pe) || divider_packets[1].eq(pe) {
                println!("Sentinel: {idx}");
                Some(idx + 1)
            } else {
                None
            }
        })
        .product();

    // Solution 24805
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
