use std::{collections::LinkedList, path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub generic: GenericSolution<Vec<String>, i32>,
    pub problem: String,
}

impl Solve<Vec<String>, i32> for Solution {
    fn new() -> Self {
        Self {
            generic: GenericSolution {
                input: Vec::new(),
                output: 0,
            },
            problem: "eval_rpn".to_string(),
        }
    }

    fn solve(tokens: Vec<String>) -> i32 {
        let mut stack: LinkedList<i32> = LinkedList::new();

        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop_back().expect("Invalid RPN expression");
                    let a = stack.pop_back().expect("Invalid RPN expression");

                    let result = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => panic!("Invalid operator"),
                    };

                    stack.push_back(result);
                }
                _ => {
                    let num: i32 = token.parse().expect("Invalid number in RPN expression");
                    stack.push_back(num);
                }
            }
        }

        stack.pop_back().expect("Invalid RPN expression")
    }

    fn run_tests(self) {
        let cwd = std::env::current_dir().unwrap();
        let test_file = PathBuf::from_str(&format!("tests/{}.txt", self.problem)).unwrap();
        let file_path = cwd.join(test_file);
        let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
        let lines: Vec<&str> = file_content.lines().collect();

        for (index, test) in lines.iter().enumerate() {
            if !test.trim().is_empty() {
                let test: Vec<String> = serde_json::from_str(test).unwrap();
                println!("Test {}:\n\n=> {:?}", index + 1, test);
                let ans = Solution::solve(test);
                let ans_format = format!("=> {ans:?}");
                println!("{}", ans_format);
                println!();
                println!("{}", "-".repeat(ans_format.len() + 2));
                println!();
            }
        }
    }
}
