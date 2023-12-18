#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
mod macros;
mod problems;

use problems::product_except_self::Solution;

use itertools::enumerate;
use itertools::Itertools;
use std::{path::PathBuf, str::FromStr};

fn main() {
    let solution = Solution {
        problem: "product_except_self".to_string(),
    };
    let cwd = std::env::current_dir().unwrap();
    let test_file = PathBuf::from_str(&format!("tests/{}.txt", solution.problem)).unwrap();
    let file_path = cwd.join(test_file);
    let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
    let lines: Vec<&str> = file_content.lines().collect();
    for (index, nums) in lines.iter().enumerate() {
        if !nums.trim().is_empty() {
            let nums: Vec<i32> = nums
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .split(',')
                .map(|s| s.trim().to_string().parse().unwrap())
                .collect();
            println!("Test {}:\n\ni: {:?}", index, nums);
            let solution = Solution::solve(nums);
            let ans_format = format!("o: {solution:?}");
            println!("{}", ans_format);
            println!();
            println!("{}", "-".repeat(ans_format.len() + 2));
            println!();
        }
    }
}
