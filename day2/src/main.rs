use utils;

const INPUT_PATH: &str = "day2/resources/input.txt";

fn parse_input(path: &str) -> Vec<String> {
    let content = utils::read(path).expect("Failed to read input file");
    utils::split(&content, ',')
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn parse_entry(entry: &str) -> (u64, u64) {
    let start_end = utils::split(entry, '-');
    (
        start_end.first().unwrap().trim().parse::<u64>().unwrap(),
        start_end.last().unwrap().trim().parse::<u64>().unwrap(),
    )
}

fn is_silly_number(number: u64) -> bool {
    let number_of_digits = utils::number_of_digits(number);
    let mut is_silly = false;
    // Number must be even in length
    if number_of_digits % 2 == 0 {
        let str = number.to_string();
        let mid = number_of_digits / 2;
        let first_half: u64 = str[..mid as usize].parse().unwrap();
        let second_half: u64 = str[mid as usize..].parse().unwrap();
        if first_half == second_half {
            is_silly = true;
        }
    }

    is_silly
}

fn main() {
    let ranges = parse_input(INPUT_PATH);
    let mut answer: u64 = 0;
    for entry in &ranges {
        let (start, end) = parse_entry(entry);
        for number in start..=end {
            if is_silly_number(number) {
                println!("Silly number found: {}", number);
                answer += number;
            }
        }
    }
    println!("Sum of all invalid IDs produces {answer}");
}
