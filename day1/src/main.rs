use utils;

const INPUT_PATH: &str = "day1/resources/input.txt";
const DIAL_INIT: i32 = 50;
const DIAL_MIN: i32 = 0;
const DIAL_MAX: i32 = 99;

fn parse_input(path: &str) -> Vec<String> {
    let content = utils::read(path).expect("Failed to read input file");
    utils::split(&content, '\n')
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn parse_entry(entry: &str) -> (char, u32) {
    let first = entry.chars().next().expect("Entry is empty");
    let second = entry[1..]
        .trim()
        .parse::<u32>()
        .expect("Failed to parse number");
    (first, second)
}

fn apply_command(dial: &mut i32, command: (char, u32)) -> i32 {
    let mut steps: i32 = i32::try_from(command.1).expect("Failed to convert steps to i32");

    let mut skips: i32 = steps / (DIAL_MAX + 1);
    steps = steps - skips * (DIAL_MAX + 1);

    if command.0 == 'L' {
        if (*dial != 0) && ((*dial - steps) < DIAL_MIN) {
            skips += 1;
        }
        *dial -= steps;
    }
    if command.0 == 'R' {
        if (*dial != 0) && ((*dial + steps) > (DIAL_MAX + 1)) {
            skips += 1;
        }
        *dial += steps;
    }

    if *dial > DIAL_MAX {
        *dial -= DIAL_MAX + 1;
    } else if *dial < DIAL_MIN {
        *dial += DIAL_MAX + 1;
    }

    skips
}

fn main() {
    let commands = parse_input(INPUT_PATH);
    let mut dial: i32 = DIAL_INIT;
    let mut password: u32 = 0;
    println!("The dial starts by pointing at {dial}.");
    for entry in &commands {
        if entry.is_empty() {
            continue;
        }
        let (direction, value) = parse_entry(entry);
        let skips: i32 = apply_command(&mut dial, (direction, value));
        if dial == 0 {
            password += 1;
        }
        print!("The dial is rotated {entry} to point at {dial}");
        if skips > 0 {
            password += skips as u32;
            println!("; during this rotation, it points at 0 {skips} time(s).");
        } else {
            println!(".");
        }
    }
    println!("The final password is: {}", password);
}
