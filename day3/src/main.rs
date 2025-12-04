use utils;

const INPUT_PATH: &str = "day3/resources/input.txt";

fn parse_input(path: &str) -> Vec<String> {
    let content = utils::read(path).expect("Failed to read input file");
    utils::split(&content, '\n')
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn find_max_voltage(line: &str) -> u32 {
    let mut j1: u32 = 0;
    let mut j2: u32 = 0;
    for i in 0..line.len() {
        let n = line.chars().nth(i).unwrap().to_digit(10).unwrap();
        if (n > j1) && (i < line.len() - 1) {
            j1 = n;
            j2 = 0;
        } else if n > j2 {
            j2 = n;
        }
    }
    j1 * 10 + j2
}

fn main() {
    let lines = parse_input(INPUT_PATH);
    let mut password = 0;
    for entry in &lines {
        let joltage = find_max_voltage(entry);
        println!("In {} -> max joltage is {}", entry, joltage);
        password += joltage;
    }
    println!("The password is {}", password);
}
