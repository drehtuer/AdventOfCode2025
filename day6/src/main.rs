use utils;

const INPUT_PATH: &str = "day6/resources/input.txt";

fn parse_input(path: &str) -> Vec<Vec<String>> {
    let lines: Vec<String> = utils::read(path)
        .expect("Failed to read input file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    let mut columns: Vec<Vec<String>> = Vec::new();
    for line in lines {
        columns.push(
            line.split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect(),
        );
    }
    columns
}

fn calc(input: &Vec<Vec<String>>) -> Vec<u64> {
    let mut results: Vec<u64> = Vec::new();
    let columns: usize = input.len();
    let rows: usize = input[0].len();

    for row in 0..rows {
        let mut result: u64 = 0;
        println!("{}", input[columns - 1][row].as_str());
        match input[columns - 1][row].as_str().trim() {
            "*" => {
                for column in 0..columns - 1 {
                    let val = input[column][row].parse::<u64>().unwrap();
                    if result == 0 {
                        result = val;
                    } else {
                        result *= val;
                    }
                }
            }
            "+" => {
                for column in 0..columns - 1 {
                    let val = input[column][row].parse::<u64>().unwrap();
                    if result == 0 {
                        result = val;
                    } else {
                        result += val;
                    }
                }
            }
            _ => {
                panic!("Unknown operation: {}", input[columns - 1][row]);
            }
        }
        results.push(result);
    }
    results
}

fn main() {
    let worksheet = parse_input(INPUT_PATH);
    let results = calc(&worksheet);
    println!("Results: {}", results.iter().sum::<u64>());
}
