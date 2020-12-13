use std::env;
use std::fs;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    day1::part1(&read_input("day1input.txt"));
    day1::part2(&read_input("day1input.txt"));
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Error reading input file.")
}
