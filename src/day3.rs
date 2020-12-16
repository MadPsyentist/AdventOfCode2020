pub fn day(input: &str) {
    part1(input);
    part2(input);
}

pub fn part1(input: &str) {
    let trees: u32 = input.lines().enumerate().filter(|pos, line| line.chars().nth((3*pos)%31).unwrap().eq('#')).count();
    println!("part 1: you encounterd {} trees", trees);
}

pub fn part2(input: &str) {
    println!("");
}
