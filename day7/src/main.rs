use utils;

const INPUT_PATH: &str = "day7/resources/input.txt";

fn parse_input(path: &str) -> utils::Matrix<char> {
    let lines: Vec<String> = utils::read(path)
        .expect("Could not read lines from file")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    let splitted: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut matrix: utils::Matrix<char> =
        utils::Matrix::new(splitted.len(), splitted[0].len(), ' ');
    for (i, row) in splitted.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            matrix.set(i, j, ch);
        }
    }
    matrix
}

fn to_string(matrix: &utils::Matrix<char>) -> String {
    let mut result = String::new();
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            result.push(*matrix.get(i, j).unwrap());
        }
        result.push('\n');
    }
    result
}

fn find_s(matrix: &utils::Matrix<char>) -> Option<(usize, usize)> {
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            if *matrix.get(i, j).unwrap() == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn step(
    matrix: &mut utils::Matrix<char>,
    position: &(usize, usize),
) -> (Vec<(usize, usize)>, bool) {
    let mut new_lines: Vec<(usize, usize)> = vec![];
    let mut split = false;
    let (row, col) = position;
    let next_row = *row + 1;
    match *matrix.get(next_row, *col).unwrap() {
        '.' => {
            matrix.set(next_row, *col, '|');
            new_lines.push((next_row, *col));
        }
        '^' => {
            if *col > 0 {
                let left_col = *col - 1;
                if matrix.get(next_row, left_col).unwrap() == &'.' {
                    matrix.set(next_row, left_col, '|');
                    new_lines.push((next_row, left_col));
                    split = true;
                } else {
                    println!(
                        "Left split at ({}, {}) blocked by '{}'",
                        next_row,
                        left_col,
                        matrix.get(next_row, left_col).unwrap()
                    );
                }
            } else {
                println!("Left split at ({}, {}) blocked by edge", next_row, col);
            }
            let right_col = *col + 1;
            if right_col < matrix.cols() {
                if matrix.get(next_row, right_col).unwrap() == &'.' {
                    matrix.set(next_row, right_col, '|');
                    new_lines.push((next_row, right_col));
                    split = true;
                } else {
                    println!(
                        "Right split at ({}, {}) blocked by '{}'",
                        next_row,
                        right_col,
                        matrix.get(next_row, right_col).unwrap()
                    );
                }
            } else {
                println!("Right split at ({}, {}) blocked by edge", next_row, col);
            }
        }
        '|' => {
            new_lines.push((next_row, *col));
        }
        _ => {}
    }

    (new_lines, split)
}

fn main() {
    let mut matrix = parse_input(INPUT_PATH);
    let mut num_splits: usize = 0;
    let start: (usize, usize) = find_s(&matrix).expect("No starting point 'S' found in the matrix");
    let mut position: Vec<(usize, usize)> = vec![start];
    for _ in 0..(matrix.rows() - 1) {
        let mut next_position: Vec<(usize, usize)> = vec![];
        for pos in position.iter() {
            let mut result = step(&mut matrix, &pos);
            next_position.append(&mut result.0);
            if result.1 {
                num_splits += 1;
            }
        }
        position = next_position;
    }
    println!("Start position: {:?}", start);
    println!("Input:\n{}", to_string(&matrix));
    println!("Number of splits: {}", num_splits);
}
