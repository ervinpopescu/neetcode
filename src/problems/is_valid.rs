use std::{path::PathBuf, str::FromStr, collections::HashMap};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub generic: GenericSolution<String, bool>,
    pub problem: String,
}

impl Solve<String, bool> for Solution {
    fn new() -> Self {
        Self {
            generic: GenericSolution {
                input: String::new(),
                output: true,
            },
            problem: "is_valid".to_string(),
        }
    }

    fn solve(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let opening = HashMap::from([(']', '['), (')', '('), ('}', '{')]);

        for c in s.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                _ => {
                    if stack.iter().last() == opening.get(&c) {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
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
                let ans: bool = Solution::solve(test);
                let ans_format = format!("=> {ans:?}");
                println!("{}", ans_format);
                println!();
                println!("{}", "-".repeat(ans_format.len() + 2));
                println!();
            }
        }
    }
}
