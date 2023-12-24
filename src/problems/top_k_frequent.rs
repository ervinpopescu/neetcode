use std::{collections::HashMap, path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub problem: String,
    pub generic: GenericSolution<(Vec<i32>, i32), Vec<i32>>,
}

impl Solve<(Vec<i32>, i32), Vec<i32>> for Solution {
    fn new() -> Self {
        Self {
            problem: "top_k_frequent".to_string(),
            generic: GenericSolution {
                input: (Vec::new(), 0),
                output: Vec::new(),
            },
        }
    }
    fn solve(nums_and_k: (Vec<i32>, i32)) -> Vec<i32> {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        let nums = nums_and_k.0;
        let k = nums_and_k.1;
        for i in nums {
            *hashmap.entry(i).or_default() += 1;
        }
        let mut tuple_vec: Vec<(i32, i32)> = hashmap.into_iter().collect();
        tuple_vec.sort_by(|&a, &b| a.1.cmp(&b.1));
        tuple_vec
            .iter()
            .rev()
            .map(|(k, _v)| *k)
            .take(k as usize)
            .collect::<Vec<i32>>()
    }
    fn run_tests(self) {
        let cwd = std::env::current_dir().unwrap();
        let test_file = PathBuf::from_str(&format!("tests/{}.txt", self.problem)).unwrap();
        let file_path = cwd.join(test_file);
        let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
        let lines: Vec<&str> = file_content.lines().collect();

        for (index, test) in lines.chunks(2).enumerate() {
            if !test.iter().all(|&a| a.trim().is_empty()) {
                let test = test.to_vec();
                let nums: Vec<i32> = serde_json::from_str(test[0]).unwrap();
                let k: i32 = serde_json::from_str(test[1]).unwrap();
                println!("Test {}:\n\n=> nums = {:?}, k = {}", index + 1, &nums, k);
                let ans = Solution::solve((nums, k));
                let ans_format = format!("=> {ans:?}");
                println!("{}", ans_format);
                println!();
                println!("{}", "-".repeat(ans_format.len() + 2));
                println!();
            }
        }
    }
}
