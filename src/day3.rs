pub fn day(input: &str) {
    part1(input);
    part2(input);
}

pub fn part1(input: &str) {
    let trees: u32 = input.lines()
        .enumerate()
        .filter(|(pos, line)| 
            line.chars()
            .nth((3*pos)%31)
            .unwrap()
            .eq(&'#'))
        .count() as u32;
    println!("part 1: you encounterd {} trees", trees);
    println!("part 1 using new method: you encounters {} trees", countTreeEncountersForGivenSlope(3,1,input));
}

pub fn part2(input: &str) {
    let slope1 = countTreeEncountersForGivenSlope(1, 1, input);
    let slope2 = countTreeEncountersForGivenSlope(3, 1, input);
    let slope3 = countTreeEncountersForGivenSlope(5, 1, input);
    let slope4 = countTreeEncountersForGivenSlope(7, 1, input);
    let slope5 = countTreeEncountersForGivenSlope(1, 2, input);
    println!("part 2:\nslope 1 has {} trees ", slope1);
    println!("slope 2 has {} trees ", slope2);
    println!("slope 3 has {} trees ", slope3);
    println!("slope 4 has {} trees ", slope4);
    println!("slope 5 has {} trees ", slope5);
    println!("multiplied together you get {}", slope1*slope2*slope3*slope4*slope5);
}

fn countTreeEncountersForGivenSlope(right: u32, down: u32, map: &str) -> u32 {
    map.lines()
        .enumerate()
        .step_by(down as usize)
        .filter(|(pos, line)| 
            line.chars()
            .nth((right as usize * (pos / down as usize)) % line.len())
            .unwrap()
            .eq(&'#'))
        .count() as u32
}

// slope 1 has 65 trees
// slope 2 has 237 trees
// slope 3 has 59 trees
// slope 4 has 61 trees
// slope 5 has 37 trees

// slope 1 has 65 trees
// slope 2 has 237 trees
// slope 3 has 59 trees
// slope 4 has 61 trees
// slope 5 has 33 trees