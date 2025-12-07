use utils;

const INPUT_PATH: &str = "day5/resources/input.txt";

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }

    fn contains(&self, value: &u64) -> bool {
        *value >= self.start && *value <= self.end
    }
}

struct Database {
    fresh_ingredients: Vec<Range>,
    available_ingredients: Vec<u64>,
}

impl Database {
    fn new(fresh: &Vec<String>, available: Vec<String>) -> Self {
        let fresh_ranges: Vec<Range> = fresh
            .iter()
            .map(|line| {
                let parts: Vec<&str> = utils::split(line, '-');
                let start = parts[0].parse::<u64>().unwrap();
                let end = parts[1].parse::<u64>().unwrap();
                Range::new(start, end)
            })
            .collect();
        let available_not_empty: Vec<u64> = available
            .iter()
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u64>().unwrap())
            .collect();
        Database {
            fresh_ingredients: fresh_ranges,
            available_ingredients: available_not_empty,
        }
    }
}

fn parse_input(path: &str) -> Vec<Vec<String>> {
    let content = utils::read(path).expect("Failed to read input file");
    let splitted: Vec<String> = content.split("\n\n").map(|s| s.to_string()).collect();
    splitted
        .iter()
        .map(|block| {
            utils::split(block, '\n')
                .iter()
                .map(|s| s.to_string())
                .collect()
        })
        .collect()
}

fn main() {
    let fresh_available = parse_input(INPUT_PATH);
    let database = Database::new(&fresh_available[0], fresh_available[1].clone());

    let mut fresh_counter = 0;
    for ingredient in database.available_ingredients.iter() {
        let mut is_fresh = false;
        for range in database.fresh_ingredients.iter() {
            if range.contains(ingredient) {
                is_fresh = true;
                break;
            }
        }
        if is_fresh {
            println!("Found stale ingredient: {}", ingredient);
            fresh_counter += 1;
        }
    }
    println!("Number of fresh ingredients: {}", fresh_counter);
}
