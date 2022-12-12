mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = if args.len() == 1 { "2a" } else { &args[1] };

    type VoidFunc = fn();
    let days: HashMap<&str, VoidFunc> = HashMap::from([
        ("1a", day1::part_a as VoidFunc),
        ("1b", day1::part_b as VoidFunc),
        ("2a", day2::part_a as VoidFunc),
        ("2b", day2::part_b as VoidFunc),
        ("3a", day3::part_a as VoidFunc),
        ("3b", day3::part_b as VoidFunc),
        ("4a", day4::part_a as VoidFunc),
        ("4b", day4::part_b as VoidFunc),
        ("5a", day5::part_a as VoidFunc),
        ("5b", day5::part_b as VoidFunc),
        ("6a", day6::part_a as VoidFunc),
        ("6b", day6::part_b as VoidFunc),
        ("7a", day7::part_a as VoidFunc),
        ("7b", day7::part_b as VoidFunc),
        ("8a", day8::part_a as VoidFunc),
        ("8b", day8::part_b as VoidFunc),
        ("9a", day9::part_a as VoidFunc),
        ("9b", day9::part_b as VoidFunc),
        ("10a", day10::part_a as VoidFunc),
        ("10b", day10::part_b as VoidFunc),
        ("11a", day11::part_a as VoidFunc),
        ("11b", day11::part_b as VoidFunc),
    ]);
    let func = days
        .get(cmd)
        .expect("Should have had a function for day {cmd}");
    func();
}
