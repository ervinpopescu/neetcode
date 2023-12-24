use std::{cmp::Ordering, path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub problem: String,
    pub generic: GenericSolution<(Vec<i32>, i32), i32>,
}

impl Solve<(Vec<i32>, i32), Vec<i32>> for Solution {
    fn new() -> Self {
        Self {
            problem: "two_sum_ii".to_string(),
            generic: GenericSolution {
                input: (Vec::new(), 0),
                output: 0,
            },
        }
    }
    fn solve(nums_and_target: (Vec<i32>, i32)) -> Vec<i32> {
        let numbers = nums_and_target.0;
        let target = nums_and_target.1;
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let current_sum = numbers[left] + numbers[right];

            match current_sum.cmp(&target) {
                Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
            }
        }

        unreachable!()
    }
    fn run_tests(self) {
        let cwd = std::env::current_dir().unwrap();
        let test_file = PathBuf::from_str(&format!("tests/{}.txt", self.problem)).unwrap();
        let file_path = cwd.join(test_file);
        let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
        let lines: Vec<&str> = file_content.lines().collect();

        for (index, test) in lines.chunks(2).enumerate() {
            let test = test.to_vec();
            if !test.iter().all(|&a| a.trim().is_empty()) {
                let test = test.to_vec();
                let numbers: Vec<i32> = serde_json::from_str(test[0]).unwrap();
                let target: i32 = serde_json::from_str(test[1]).unwrap();
                println!(
                    "Test {}:\n\n=> numbers = {:?}, target = {}",
                    index + 1,
                    numbers,
                    target
                );
                let ans = Solution::solve((numbers, target));
                let ans_format = format!("=> {ans:?}");
                println!("{}", ans_format);
                println!();
                println!("{}", "-".repeat(ans_format.len() + 2));
                println!();
            }
        }
    }
}
