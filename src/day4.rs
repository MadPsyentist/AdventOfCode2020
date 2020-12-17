pub fn day(input: &str) {
    part1(input);
    part2(input);
}

pub fn part1(input: &str) {
    input.lines().enumerate().map(|(pos, ele)| println!("{}: {}", pos, ele)).count();
}

pub fn part2(input: &str) {
    println!("Part 2");
}
