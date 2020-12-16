pub fn day(input: &str) {
    part1(input);
    part2(input);
}

pub fn part1(input: &str) {
    for (pos, ele) in input.lines().enumerate() {
        let pap = PasswordAndPolicyFromLine(ele);
        println!("{}: {}, min:{} max:{} char:{} pass:{}", pos, ele, pap.Min_Character_Count, pap.Max_Character_Count, pap.Character, pap.Password);
    }
    println!("{} passwords meet their policy", 
        input.lines().filter(|i| PasswordMeetsPolicy(PasswordAndPolicyFromLine(i))).count());
}

pub fn part2(input: &str) {
    println!("{} passwords meet their policy", 
        input.lines().map(|l| PasswordAndPolicyFromLine(l))
            .filter(|pap| CheckCharacterInPosition(pap.Character, pap.Min_Character_Count, &pap.Password)
                || CheckCharacterInPosition(pap.Character, pap.Max_Character_Count, &pap.Password)).count());
}

struct PasswordAndPolicy {
    Min_Character_Count: u32,
    Max_Character_Count: u32,
    Character: char,
    Password: String,
}

fn PasswordAndPolicyFromLine(line: &str) -> PasswordAndPolicy {
    let min: u32 = GetMinCharacterCountFromLine(line);
    let max: u32 = GetMaxCharacterCountFromLine(line);
    let character: char = GetPolicyCharacterFromLine(line);
    let pass: String = GetPasswordFromLine(line);

    return PasswordAndPolicy { Min_Character_Count: min, 
        Max_Character_Count: max,
        Character: character,
        Password: pass.to_string() };
}

fn GetMinCharacterCountFromLine(line: &str) -> u32 {
    line.split("-").next().unwrap().parse().unwrap()
}

fn GetMaxCharacterCountFromLine(line: &str) -> u32 {
    line.split("-").nth(1).unwrap().split(" ").next().unwrap()
        .parse().unwrap()
}

fn GetPolicyCharacterFromLine(line: &str) -> char {
    line.split(" ").nth(1).unwrap().chars().next().unwrap()
}

fn GetPasswordFromLine(line: &str) -> String {
    line.split(" ").nth(2).unwrap().to_string()
}

fn PasswordMeetsPolicy(passAndpPol: PasswordAndPolicy) -> bool {
    let count: u32 = passAndpPol.Password
        .matches(|character| character == passAndpPol.Character)
        .collect::<String>()
        .len() as u32;
    count >= passAndpPol.Min_Character_Count && count <= passAndpPol.Max_Character_Count
}

fn CheckCharacterInPosition(character: char, position: u32, password: &str) -> bool {
    password.chars().nth((position-1) as usize).unwrap_or('$').eq(&character)
}