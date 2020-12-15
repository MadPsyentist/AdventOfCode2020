use std::env;
use std::fs;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = consume_input_command(&args);
    if args.len() > 1 {
        match args[1].to_lowercase().as_str() {
            "day1" => day1::day(&input),
            _ => print_help(),
        }
    } else {
        print_help();
    }

    //day1::part1(&read_input("day1input.txt"));
    //day1::part2(&read_input("day1input.txt"));
}

fn consume_input_command(args: &[String]) -> String {
    if args.len() > 2 {
        return read_input(&args[2]);
    } else if args.len() > 1 {
        let dayCommand: String = args[1].to_lowercase();
        return read_input(&(dayCommand + "input.txt"));
    }
    return "".to_string();
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Error reading input file.")
}

fn print_help() {
    println!("Advent of Code 2020\n");
    println!("useage: AoC2020 <command> <input file>");
    println!("vaild commands is the day of the challenge you would like to run i.e.\n
        \tday1\n
        \tday2\n
        \tday3\n
        \tetc...\n
        if no valid command is given you will get this helpfull text.\n" 
    );
    println!("valid input files can also be passed in by giving name and location of the file.\n
        There is no validation on these input files so expect the program to panic if you give it bogus input.\n
        If no input is passed then the program will use the built in data for the day you are running.\n");
}
