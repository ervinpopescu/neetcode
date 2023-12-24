use std::{path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub problem: String,
    pub generic: GenericSolution<Vec<Vec<char>>, bool>,
}

const SIZE: usize = 9;

impl Solve<Vec<Vec<char>>, bool> for Solution {
    fn new() -> Self {
        Self {
            problem: "is_valid_sudoku".to_string(),
            generic: GenericSolution {
                input: Vec::new(),
                output: true,
            },
        }
    }
    fn solve(board: Vec<Vec<char>>) -> bool {
        for i in 0..SIZE {
            if !is_valid_row(&board, i) || !is_valid_column(&board, i) {
                return false;
            }
        }

        // Check 3x3 subgrids
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                if !is_valid_subgrid(&board, i, j) {
                    return false;
                }
            }
        }
        true
    }

    fn run_tests(self) {
        let cwd = std::env::current_dir().unwrap();
        let test_file = PathBuf::from_str(&format!("tests/{}.txt", self.problem)).unwrap();
        let file_path = cwd.join(test_file);
        let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
        let lines: Vec<&str> = file_content.lines().collect();

        for (index, test) in lines.iter().enumerate() {
            if !test.trim().is_empty() {
                let test: Vec<Vec<char>> = serde_json::from_str(test).unwrap();
                println!("Test {}:", index + 1);
                println!();
                (0..SIZE).for_each(|i| {
                    print!("|");
                    for elem in &test[i] {
                        if *elem != '.' {
                            print!("{elem}");
                        } else {
                            print!(" ");
                        }
                        if i != 9 {
                            print!("|")
                        };
                    }
                    println!();
                });
                println!();
                let ans = Solution::solve(test);
                let ans_format = format!("=> {ans:?}");
                println!("{}", ans_format);
                println!();
                println!("{}", "-".repeat(4 * SIZE + 2));
                println!();
            }
        }
    }
}

fn is_valid_row(board: &[Vec<char>], row: usize) -> bool {
    let mut set = vec![false; SIZE];
    for col in 0..SIZE {
        let digit = board[row][col];
        if !update_set(&mut set, digit) {
            println!("=> Element ({},{}) doesn't obey the rule", row + 1, col + 1);
            return false;
        }
    }
    true
}

fn is_valid_column(board: &[Vec<char>], col: usize) -> bool {
    let mut set = vec![false; SIZE];
    for (row, item) in board.iter().enumerate().take(SIZE) {
        let digit = item[col];
        if !update_set(&mut set, digit) {
            println!("=> Element ({},{}) doesn't obey the rule", row + 1, col + 1);
            return false;
        }
    }
    true
}

fn is_valid_subgrid(board: &[Vec<char>], start_row: usize, start_col: usize) -> bool {
    let mut set = vec![false; SIZE];
    for i in 0..3 {
        for j in 0..3 {
            let digit = board[start_row + i][start_col + j];
            if !update_set(&mut set, digit) {
                return false;
            }
        }
    }
    true
}

fn update_set(set: &mut [bool], digit: char) -> bool {
    if digit == '.' {
        return true; // Skip empty cells
    }

    let index = digit.to_digit(10).unwrap() as usize - 1;
    if set[index] {
        println!("=> digit {digit} already used");
        return false; // Digit already encountered in the set
    }

    set[index] = true;
    true
}
