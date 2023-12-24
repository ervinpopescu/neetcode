use std::{path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub generic: GenericSolution<String, bool>,
    pub problem: String,
}

impl Solve<String, bool> for Solution {
    fn new() -> Self {
        Self {
            problem: "is_palindrome".to_string(),
            generic: GenericSolution {
                input: String::new(),
                output: true,
            },
        }
    }
    fn solve(input: String) -> bool {
        let out: String = input
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase();
        out == out.chars().rev().collect::<String>()
    }
    fn run_tests(self) {
        let cwd = std::env::current_dir().unwrap();
        let test_file = PathBuf::from_str(&format!("tests/{}.txt", self.problem)).unwrap();
        let file_path = cwd.join(test_file);
        let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
        let lines: Vec<&str> = file_content.lines().collect();

        for (index, test) in lines.iter().enumerate() {
            if !test.trim().is_empty() {
                let test: String = serde_json::from_str(test).unwrap();
                println!("Test {}:\n\n=> {:?}", index + 1, test);
                let ans = Solution::solve(test.clone());
                let ans_format = format!("=> {ans:?}");
                println!("{}", ans_format);
                println!();
                println!("{}", "-".repeat(test.len() + 5));
                println!();
            }
        }
    }
}
