#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashMap;

use itertools::Itertools;

// use itertools::Itertools;

pub struct Solution {
    pub problem: String,
}

impl Solution {
    pub fn solve(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for i in nums {
            *hashmap.entry(i).or_default() += 1;
        }
        let mut tuple_vec: Vec<(i32, i32)> = hashmap
            .into_iter()
            .collect();
        tuple_vec.sort_by(|&a, &b| a.1.cmp(&b.1));
        tuple_vec
            .iter()
            .rev()
            .map(|(k, v)| *k)
            .take(k as usize)
            .collect::<Vec<i32>>()
    }
}


// use itertools::enumerate;
// use itertools::Itertools;
// use std::{path::PathBuf, str::FromStr};

// fn main() {
//     let solution = Solution {
//         problem: "product_except_self".to_string(),
//     };
//     let cwd = std::env::current_dir().unwrap();
//     let test_file = PathBuf::from_str(&format!("tests/{}.txt", solution.problem)).unwrap();
//     let file_path = cwd.join(test_file);
//     let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
//     let lines: Vec<&str> = file_content.lines().collect();
//     for (index, nums_and_k) in lines.chunks(2).enumerate() {
//         let nums_and_k = nums_and_k.to_vec();
//         let nums = nums_and_k[0];
//         let k = nums_and_k[1];
//         if !nums.trim().is_empty() {
//             let nums: Vec<i32> = nums
//                 .strip_prefix('[')
//                 .unwrap()
//                 .strip_suffix(']')
//                 .unwrap()
//                 .split(',')
//                 .map(|s| s.trim().to_string().parse().unwrap())
//                 .collect();
//             println!("Test {}:\n\ni: {:?}", index, nums);
//             let solution = Solution::solve(nums, k.parse().unwrap());
//             let ans_format = format!("o: {solution:?}");
//             println!("{}", ans_format);
//             println!();
//             println!("{}", "-".repeat(ans_format.len() + 2));
//             println!();
//         }
//     }
// }
