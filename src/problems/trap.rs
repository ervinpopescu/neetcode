use std::{path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub generic: GenericSolution<Vec<i32>, i32>,
    pub problem: String,
}

impl Solve<Vec<i32>, i32> for Solution {
    fn new() -> Self {
        Self {
            generic: GenericSolution {
                input: Vec::new(),
                output: 0,
            },
            problem: "trap".to_string(),
        }
    }

    fn solve(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let (mut result, mut left, mut right) = (0, 0, height.len() - 1);
        let (mut left_max, mut right_max) = (height[left], height[right]);
        while left < right {
            if left_max < right_max {
                left += 1;
                left_max = left_max.max(height[left]);
                result += left_max - height[left];
            } else {
                right -= 1;
                right_max = right_max.max(height[right]);
                result += right_max - height[right];
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
