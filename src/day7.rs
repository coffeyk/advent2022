use std::collections::HashMap;
use std::fs;
use std::iter;
use std::str;

const DAY: i32 = 7;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

#[derive(Debug)]
enum CommandName {
    CD,
    LS,
}

#[derive(Debug)]
struct Command {
    name: CommandName,
    args: Vec<String>,
    output: Vec<String>,
}

struct Input<'a> {
    lines: iter::Peekable<str::Lines<'a>>,
}

impl<'a> Iterator for Input<'a> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next().unwrap();
            if line.is_empty() {
                return None;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();

            let cmd_name = match parts[1] {
                "cd" => CommandName::CD,
                "ls" => CommandName::LS,
                _ => panic!("unknown command name"),
            };
            // Clear the first two elements ("$", "cmd") the rest are args
            let args = parts.iter().skip(2).map(|s| String::from(*s)).collect();

            let mut output: Vec<String> = Vec::new();

            loop {
                let next_line = self.lines.peek().unwrap();
                println!("{next_line}");
                if next_line.starts_with('$') || next_line.is_empty() {
                    // Found the start of the next command / the end of the file
                    return Some(Command {
                        name: cmd_name,
                        args,
                        output,
                    });
                } else {
                    // Otherwise we have output
                    output.push(String::from(self.lines.next().unwrap()));
                }
            }
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

#[derive(Debug)]
enum FileType {
    Directory,
    File,
}
#[derive(Debug)]
struct FileSystem {
    file_type: FileType,
    size: u32,
    contents: HashMap<String, FileSystem>,
}

impl FileSystem {
    pub fn new_directory() -> FileSystem {
        FileSystem {
            file_type: FileType::Directory,
            size: 0,
            contents: HashMap::new(),
        }
    }
    pub fn new_file(size: u32) -> FileSystem {
        FileSystem {
            file_type: FileType::File,
            size,
            contents: HashMap::new(),
        }
    }

    fn get_or_create(&mut self, path: Vec<String>) -> &mut Self {
        let mut node = self;
        for p in &path {
            node = node
                .contents
                .entry(p.clone())
                .or_insert_with(FileSystem::new_directory);
        }
        node
    }
    fn add_file(&mut self, name: &str, size: u32) {
        self.contents
            .insert(String::from(name), FileSystem::new_file(size));
    }
    fn add_directory(&mut self, name: &str) {
        self.contents
            .insert(String::from(name), FileSystem::new_directory());
    }

    fn my_size(&self) -> u32 {
        match self.file_type {
            FileType::File => self.size,
            FileType::Directory => self.contents.values().map(|c| c.my_size()).sum(),
        }
    }

    fn directory_sizes(&self) -> Vec<u32> {
        let mut results = Vec::new();
        // results.push(self.my_size());
        match self.file_type {
            FileType::File => {}
            FileType::Directory => {
                results.push(self.my_size());
                for c in self.contents.values() {
                    results.append(&mut c.directory_sizes());
                }
            }
        }
        results
    }
}

pub fn part_a() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines().peekable(),
    };

    let mut path: Vec<String> = Vec::new();
    let mut filesystem: FileSystem = FileSystem::new_directory();

    let mut current = &mut filesystem;
    for cmd in it {
        // let mut current_fs = fs_path.last_mut().unwrap().clone();
        match cmd.name {
            CommandName::CD => {
                let dir_name = cmd.args[0].clone();
                match dir_name.as_str() {
                    "/" => {
                        path.clear();
                    }
                    ".." => {
                        path.pop();
                        // TODO: empty pops
                    }
                    _ => {
                        path.push(dir_name);

                        current = filesystem.get_or_create(path.clone());
                    }
                };
            }
            CommandName::LS => {
                for file in cmd.output {
                    if let [size_type, name] =
                        file.split_whitespace().take(2).collect::<Vec<&str>>()[..]
                    {
                        match size_type.parse::<u32>().ok() {
                            Some(size) => current.add_file(name, size),
                            None => current.add_directory(name),
                        }
                    }
                }
            }
        }
    }
    print!("{filesystem:?}");

    let results: u32 = filesystem
        .directory_sizes()
        .iter()
        .filter(|s| **s <= 100000)
        .sum();
    // Solution 1367870
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines().peekable(),
    };

    let mut path: Vec<String> = Vec::new();
    let mut filesystem: FileSystem = FileSystem::new_directory();

    let mut current = &mut filesystem;
    for cmd in it {
        // let mut current_fs = fs_path.last_mut().unwrap().clone();
        match cmd.name {
            CommandName::CD => {
                let dir_name = cmd.args[0].clone();
                match dir_name.as_str() {
                    "/" => {
                        path.clear();
                    }
                    ".." => {
                        path.pop();
                        // TODO: empty pops
                    }
                    _ => {
                        path.push(dir_name);

                        current = filesystem.get_or_create(path.clone());
                    }
                };
            }
            CommandName::LS => {
                for file in cmd.output {
                    if let [size_type, name] =
                        file.split_whitespace().take(2).collect::<Vec<&str>>()[..]
                    {
                        match size_type.parse::<u32>().ok() {
                            Some(size) => current.add_file(name, size),
                            None => current.add_directory(name),
                        }
                    }
                }
            }
        }
    }
    print!("{filesystem:?}");

    let total_used = filesystem.my_size();
    let space_needed = total_used - 40000000;

    let results: u32 = *filesystem
        .directory_sizes()
        .iter()
        .filter(|s| **s >= space_needed)
        .min()
        .unwrap();
    // Results 549173
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
