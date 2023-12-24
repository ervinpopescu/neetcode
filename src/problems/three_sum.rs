use std::{path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub problem: String,
    pub generic: GenericSolution<Vec<i32>, Vec<Vec<i32>>>,
}

impl Solve<Vec<i32>, Vec<Vec<i32>>> for Solution {
    fn new() -> Self {
        Self {
            problem: "three_sum".to_string(),
            generic: GenericSolution {
                input: Vec::new(),
                output: Vec::new(),
            },
        }
    }

    fn solve(mut numbers: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        numbers.sort();

        for i in 0..numbers.len() {
            if i > 0 && numbers[i] == numbers[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, numbers.len() - 1);

            while l < r {
                match (numbers[i] + numbers[l] + numbers[r]).cmp(&0) {
                    std::cmp::Ordering::Less => l += 1,
                    std::cmp::Ordering::Greater => r -= 1,
                    std::cmp::Ordering::Equal => {
                        result.push(vec![numbers[i], numbers[l], numbers[r]]);
                        l += 1;
                        while numbers[l] == numbers[l - 1] && l < r {
                            l += 1;
                        }
                        r -= 1;
                        while numbers[r] == numbers[r + 1] && l < r {
                            r -= 1;
                        }
                    }
                }
            }
        }

        result
    }

    fn run_tests(self) {
        let cwd = std::env::current_dir().unwrap();
        let test_file = PathBuf::from_str(&format!("tests/{}.txt", self.problem)).unwrap();
        let file_path = cwd.join(test_file);
        let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
        let lines: Vec<&str> = file_content.lines().collect();

        for (index, test) in lines.iter().enumerate() {
            if !test.trim().is_empty() {
                let test: Vec<i32> = serde_json::from_str(test).unwrap();
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
