use utils;

const INPUT_PATH: &str = "day4/resources/input.txt";

fn parse_input(path: &str) -> Vec<String> {
    let content = utils::read(path).expect("Failed to read input file");
    utils::split(&content, '\n')
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn build_matrix(line: &Vec<String>) -> utils::Matrix<char> {
    let mut matrix = utils::Matrix::new(line.len(), line[0].len(), ' ');
    for (i, row) in line.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            matrix.set(i, j, c);
        }
    }
    matrix
}

fn find_accessible_paper_rolls(matrix: &mut utils::Matrix<char>) -> u32 {
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            // paper roll at position (i, j)
            if *matrix.get(i, j).unwrap() == '@' {
                let mut other_rolls = 0;
                // check adjacent positions
                for (di, dj) in directions.iter() {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0
                        && ni < matrix.rows() as isize
                        && nj >= 0
                        && nj < matrix.cols() as isize
                    {
                        let other = *matrix.get(ni as usize, nj as usize).unwrap();
                        if (other == '@') || (other == 'x') {
                            other_rolls += 1;
                        }
                    }
                }
                if other_rolls < 4 {
                    matrix.set(i, j, 'x');
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let lines = parse_input(INPUT_PATH);
    let mut data = build_matrix(&lines);
    let count = find_accessible_paper_rolls(&mut data);

    for i in 0..data.rows() {
        for j in 0..data.cols() {
            print!("{}", data.get(i, j).unwrap());
        }
        println!();
    }

    println!("Number of accessible paper rolls: {}", count);
}
