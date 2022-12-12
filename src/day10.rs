use std::fs;
use std::str;

const DAY: i32 = 10;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

#[derive(Debug, Clone, Copy)]
enum CommandName {
    Noop,
    AddX,
}

#[derive(Debug, Clone, Copy)]
struct Command {
    name: CommandName,
    arg: i32,
}

#[derive(Debug)]
struct Input<'a> {
    lines: str::Lines<'a>,
}

impl<'a> Iterator for Input<'a> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let opt = self.lines.next();

        // Return None when opt is None
        opt?;

        let line = opt.unwrap();
        if line.is_empty() {
            return None;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        let cmd_name = match parts[0] {
            "noop" => CommandName::Noop,
            "addx" => CommandName::AddX,
            _ => panic!("unknown command name"),
        };
        // Clear the first two elements ("$", "cmd") the rest are args
        let arg = parts
            .iter()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .next()
            .unwrap_or(0);

        Some(Command {
            name: cmd_name,
            arg,
        })
    }
}

#[derive(Debug)]
struct Machine<'a> {
    cmd: Command,
    cmds: Input<'a>,
    cycle: i32,
    x: i32,
    wait: u32,
    done: bool,
}
impl<'a> Iterator for Machine<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            let cmd = self.cmds.next();
            match cmd {
                Some(c) => {
                    // print!("{c:?}");
                    self.cmd = c;
                    self.done = false;
                    match self.cmd.name {
                        CommandName::Noop => self.wait = 0,
                        CommandName::AddX => self.wait = 1,
                    }
                }
                None => return None,
            }
        }
        self.cycle += 1;

        // Save x before the command goes off.
        let result = self.x;

        if self.wait == 0 {
            self.done = true;
            match self.cmd.name {
                CommandName::Noop => {}
                CommandName::AddX => self.x += self.cmd.arg,
            }
        } else {
            self.wait -= 1;
        }
        Some((self.cycle, result))
    }
}

impl<'a> Machine<'a> {
    fn new(cmds: Input) -> Machine {
        Machine {
            cmd: Command {
                name: CommandName::Noop,
                arg: 0,
            },
            cmds,
            cycle: 0,
            x: 1,
            wait: 0,
            done: true,
        }
    }
}

pub fn part_a() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let machine = Machine::new(it);

    let mut results = 0;

    // 20th, 60th, 100th, 140th, 180th, and 220
    let mut next_cycle = 20;

    for (cycle, x) in machine {
        if cycle == next_cycle {
            results += cycle * x;
            next_cycle += 40
        }
        // println!("{cycle}, {x}");
    }
    // Solution 15220
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let machine = Machine::new(it);

    for (cycle, x) in machine {
        let current_column = cycle % 40;

        if x <= current_column && x + 2 >= current_column {
            print!("#");
        } else {
            print!(".");
        }

        if cycle % 40 == 0 {
            println!();
        }
        // println!("{cycle}, {x}");
    }

    let results = "read picture";
    // Solution RFZEKBFA
    println!("Day {DAY}a best:\n{results}");
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
