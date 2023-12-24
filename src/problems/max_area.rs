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
            problem: "max_area".to_string(),
        }
    }

    fn solve(height: Vec<i32>) -> i32 {
        let (mut max_area, mut left, mut right) = (0, 0, height.len() - 1);

        while left < right {
            let difference = (right - left) as i32;
            let height_n = height[left].min(height[right]);
            let area = difference * height_n;
            max_area = area.max(max_area);
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        max_area
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
