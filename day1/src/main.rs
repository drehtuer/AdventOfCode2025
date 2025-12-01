use utils;

const INPUT_PATH: &str = "day1/resources/input.txt";
const DIAL_INIT: i32 = 50;

fn parse_input(path: &str) -> Vec<String> {
    let content = utils::read(path).expect("Failed to read input file");
    utils::split(&content, '\n').iter().map(|s| s.to_string()).collect()
}

fn parse_entry(entry: &str) -> (char, u32) {
    let first = entry.chars().next().expect("Entry is empty");
    let second = entry[1..].trim().parse::<u32>().expect("Failed to parse number");
    (first, second)
}

fn apply_command(dial: &mut i32, command: (char, u32)) {
    let steps:i32 = i32::try_from(command.1).expect("Failed to convert steps to i32");
    match command.0 {
        'L' => *dial = (*dial).wrapping_sub(steps),
        'R' => *dial = (*dial).wrapping_add(steps),
        _ => {},
        };
    *dial = *dial % 100;
    if *dial < 0 {
        *dial += 100;
    }
}

fn main() {
    let commands = parse_input(INPUT_PATH);
    let mut dial:i32 = DIAL_INIT;
    let mut password: u32 = 0;
    println!("The dial starts by pointing at {dial}.");
    for entry in &commands {
      if entry.is_empty() {
          continue;
      }
      let (direction, value) = parse_entry(entry);
      apply_command(&mut dial, (direction, value));
      if dial == 0
      {
        password += 1;
      }
      println!("The dial is rotated {entry} to point at {dial}.");
    }
    println!("The final password is: {}", password);
}
