use std::ops::Add;

pub fn part1(input: &str) {
    let numbers = process_input(input);
    for (pos, element) in numbers.iter().enumerate() {
        match find_right_side_pair(*element, &numbers[pos+1..], 2020) {
            Some(num) => println!("found {} and {} add to 2020, multiplied = {}", element, num, element * num),
            None => (),
        }
    }
}

pub fn part2(input: &str) {
    let numbers = process_input(input);
    for (pos, element) in numbers.iter().enumerate() {
        for (pos2, element2) in (&numbers[pos+1..]).iter().enumerate() {
            match find_right_side_pair(element + element2, &numbers[pos+pos2+1..], 2020) {
                Some(num) => println!("found {} and {} and {} add to 2020, multiplied = {}", element, element2, num, element * element2 * num),
                None => (),
            }
        }
    }
}

fn find_right_side_pair(left_side: i32, potentials: &[i32], target: i32) -> Option<&i32> {
    potentials
        .iter()
        .find(|&num| left_side + num == target)
}

fn process_input(input: &str) -> Vec<i32> {
    input.lines()
        .map(|num| { num.parse::<i32>().unwrap() })
        .collect()
}