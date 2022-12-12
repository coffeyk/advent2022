use std::collections::HashMap;
use std::fs;
use std::str;

const DAY: i64 = 11;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

type Items = Vec<i64>;
type MonkeyUpdates = HashMap<usize, Items>;

#[derive(Debug)]
struct Monkey {
    items: Items,
    operation: Box<dyn Eval>,
    test: Test,
}
impl Monkey {
    fn run(&mut self, divisor: i64, modulus: i64) -> MonkeyUpdates {
        let mut results: MonkeyUpdates = HashMap::new();

        for i in &self.items {
            // println!("  Monkey inspects an item with a worry level of {i}");
            let (new_worry, next_monkey) = self.interact(*i, divisor, modulus);
            results.entry(next_monkey).or_default().push(new_worry);
        }
        self.items = Vec::new();
        results
    }
    fn interact(&self, item: i64, divisor: i64, modulus: i64) -> (i64, usize) {
        let mut new = self.operation.eval(item);
        if divisor != 0 {
            new /= divisor;
        } else {
            new %= modulus;
        }
        // println!("    Monkey gets bored with item. Worry level is divided by {divisor} to {new}");
        (new, self.test.eval(new))
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    operation: TestOperation,
    pass: usize,
    fail: usize,
}

impl Test {
    fn eval(&self, item: i64) -> usize {
        if self.operation.eval(item) {
            let pass = self.pass;
            // println!("    Item with worry level {item} is thrown to monkey {pass}");
            self.pass
        } else {
            let fail = self.fail;
            // println!("    Item with worry level {item} is thrown to monkey {fail}");
            self.fail
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TestOperation {
    divisor: i64,
}
impl TestOperation {
    fn eval(&self, val: i64) -> bool {
        let result = val % self.divisor == 0;
        let divisor = self.divisor;
        let not = if result { "" } else { "not " };
        // println!("    Current worry level is {not}divisible by {divisor}.");
        result
    }
}

#[derive(Debug)]
struct OperationArg {
    is_literal: bool,
    literal: i64,
}
impl OperationArg {
    fn new_literal(literal: i64) -> OperationArg {
        OperationArg {
            is_literal: true,
            literal,
        }
    }
    fn new_old() -> OperationArg {
        OperationArg {
            is_literal: false,
            literal: 0,
        }
    }
    fn from_str(raw: &str) -> OperationArg {
        match raw {
            "old" => OperationArg::new_old(),
            _ => OperationArg::new_literal(raw.parse().unwrap()),
        }
    }
    fn val(&self, old: i64) -> i64 {
        if self.is_literal {
            self.literal
        } else {
            old
        }
    }
}

trait Eval: std::fmt::Debug {
    fn eval(&self, old: i64) -> i64;
}

#[derive(Debug)]
struct AddOperation {
    lhs: OperationArg,
    rhs: OperationArg,
}
impl Eval for AddOperation {
    fn eval(&self, old: i64) -> i64 {
        let lhs = &self.lhs;
        let rhs = &self.rhs;
        let results = lhs.val(old) + rhs.val(old);
        // println!("    Worry level Adding {lhs:?} + {rhs:?} = {results}");
        results
    }
}

#[derive(Debug)]
struct MulOperation {
    lhs: OperationArg,
    rhs: OperationArg,
}
impl Eval for MulOperation {
    fn eval(&self, old: i64) -> i64 {
        let lhs = &self.lhs;
        let rhs = &self.rhs;
        let results = lhs.val(old) * rhs.val(old);
        // println!("    Worry level Multiplying {lhs:?} + {rhs:?} = {results}");
        results
    }
}

struct Input<'a> {
    lines: str::Lines<'a>,
}

impl<'a> Iterator for Input<'a> {
    type Item = Monkey;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(text) = self.lines.next() {
            if text.is_empty() {
                return None;
            }

            let _monkey_id: usize = text
                .strip_prefix("Monkey ")
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse()
                .unwrap();

            let starting_items_str = self.lines.next().unwrap();
            let items: Items = starting_items_str
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();

            let operation_str = self.lines.next().unwrap();
            let operation_func_parts: Vec<&str> = operation_str
                .strip_prefix("  Operation: new = ")
                .unwrap()
                .split_whitespace()
                .take(3)
                .collect();

            let lhs = OperationArg::from_str(operation_func_parts[0]);
            let rhs = OperationArg::from_str(operation_func_parts[2]);

            let operation: Box<dyn Eval> = match operation_func_parts[1] {
                "+" => Box::new(AddOperation { lhs, rhs }),
                "*" => Box::new(MulOperation { lhs, rhs }),
                _ => panic!("unknown operation function part"),
            };

            let test_str = self.lines.next().unwrap();
            let test_divisor: i64 = test_str
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();

            let test_true_str = self.lines.next().unwrap();
            let test_true_id: usize = test_true_str
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let test_false_str = self.lines.next().unwrap();
            let test_false_id: usize = test_false_str
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            let test = Test {
                operation: TestOperation {
                    divisor: test_divisor,
                },
                pass: test_true_id,
                fail: test_false_id,
            };

            // Skip a blank line
            self.lines.next();

            Some(Monkey {
                items,
                operation,
                test,
            })
        } else {
            None
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

    let it = Input {
        lines: contents.lines(),
    };

    let mut monkeys: Vec<Monkey> = it.collect();

    let mut monkey_iteraction_counts: HashMap<usize, usize> = HashMap::new();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let m = monkeys.get_mut(i).unwrap();

            // println!("Monkey {i}:");
            // Keep track of how many items this monkey is about to interact with
            monkey_iteraction_counts
                .entry(i)
                .and_modify(|c| *c += m.items.len())
                .or_insert(m.items.len());

            // Find where this monkey is throwing all its items
            let mut monkey_updates = m.run(3, 0);

            // Give the monkeys the new items
            for (monkey_id, new_items) in monkey_updates.iter_mut() {
                let m = monkeys.get_mut(*monkey_id).unwrap();
                m.items.append(new_items);
            }
        }
    }

    for i in 0..monkeys.len() {
        let interaction_count = monkey_iteraction_counts.get(&i).unwrap();
        // println!("Monkey {i} inspected items {interaction_count} times.");
    }
    let mut counts: Vec<&usize> = monkey_iteraction_counts.values().collect();
    counts.sort();
    let results: usize = counts.iter().rev().take(2).map(|e| **e).product();

    // Solution 50616
    println!("Day {DAY}a best:\n{results}");
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let it = Input {
        lines: contents.lines(),
    };

    let mut monkeys: Vec<Monkey> = it.collect();

    let mut monkey_iteraction_counts: HashMap<usize, usize> = HashMap::new();

    // All the numbers in my example are primes so no point being too clever
    let mut modulus = 1;
    for i in 0..monkeys.len() {
        let m = monkeys.get_mut(i).unwrap();

        modulus *= m.test.operation.divisor;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let m = monkeys.get_mut(i).unwrap();

            // println!("Monkey {i}:");
            // Keep track of how many items this monkey is about to interact with
            monkey_iteraction_counts
                .entry(i)
                .and_modify(|c| *c += m.items.len())
                .or_insert(m.items.len());

            // Find where this monkey is throwing all its items
            // a multiple of all tests
            // Use a multiple of all test divisions to keep the worry in bounds
            let mut monkey_updates = m.run(0, modulus);

            // Give the monkeys the new items
            for (monkey_id, new_items) in monkey_updates.iter_mut() {
                let m = monkeys.get_mut(*monkey_id).unwrap();
                m.items.append(new_items);
            }
        }
    }

    for i in 0..monkeys.len() {
        let interaction_count = monkey_iteraction_counts.get(&i).unwrap();
        // println!("Monkey {i} inspected items {interaction_count} times.");
    }
    let mut counts: Vec<&usize> = monkey_iteraction_counts.values().collect();
    counts.sort();
    let results: usize = counts.iter().rev().take(2).map(|e| **e).product();

    // Solution 11309046332
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
