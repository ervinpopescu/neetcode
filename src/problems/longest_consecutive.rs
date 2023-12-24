use std::{collections::HashSet, path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub problem: String,
    pub generic: GenericSolution<Vec<i32>, i32>,
}

impl Solve<Vec<i32>, i32> for Solution {
    fn new() -> Self {
        Self {
            problem: "longest_consecutive".to_string(),
            generic: GenericSolution {
                input: Vec::new(),
                output: 0,
            },
        }
    }
    fn solve(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums);
        let mut max_cnt = 0;
        // println!("{set:?}");
        // println!();
        for n in &set {
            // println!("Start iteration");
            // dbg!(n);
            if !set.contains(&(n - 1)) {
                // dbg!(n);
                let mut next = n + 1;
                let mut cnt = 1;
                while set.contains(&next) {
                    // dbg!(cnt);
                    // dbg!(next);
                    cnt += 1;
                    next += 1;
                }
                max_cnt = max_cnt.max(cnt);
                // dbg!(max_cnt);
            }
            // println!();
        }

        max_cnt
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
