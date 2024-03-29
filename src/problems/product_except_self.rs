use std::{path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub problem: String,
    pub generic: GenericSolution<Vec<i32>, Vec<i32>>,
}

impl Solve<Vec<i32>, Vec<i32>> for Solution {
    fn new() -> Self {
        Self {
            problem: "product_except_self".to_string(),
            generic: GenericSolution {
                input: Vec::new(),
                output: Vec::new(),
            },
        }
    }
    fn solve(mut nums: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<i32> = vec![1; nums.len()];
        for i in 0..nums.len() {
            out[i] = nums[i - 1] * out[i - 1];
        }
        let mut right = 1;
        for n in out.iter_mut().rev() {
            *n *= right;
            right *= nums.pop().unwrap();
        }
        out
    }
    fn run_tests(self) {
        let cwd = std::env::current_dir().unwrap();
        let test_file = PathBuf::from_str(&format!("tests/{}.txt", self.problem)).unwrap();
        let file_path = cwd.join(test_file);
        let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
        let lines: Vec<&str> = file_content.lines().collect();

        for (index, test) in lines.iter().enumerate() {
            if !test.trim().is_empty() {
                let test = serde_json::from_str(test).unwrap();
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
