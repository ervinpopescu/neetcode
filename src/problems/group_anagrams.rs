#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;

pub struct Solution {
    pub problem: String,
}

impl Solution {
    pub fn solve(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = [0_u8; 26];

            for c in s.chars() {
                key[c as usize - 'a' as usize] += 1;
            }

            if let Some(vals) = map.get_mut(&key) {
                vals.push(s);
            } else {
                map.insert(key, vec![s]);
            }
        }
        let mut output: Vec<Vec<String>> = map.into_values().collect::<Vec<Vec<String>>>();
        output.sort_by_key(|a| a.len());
        output
    }
}

// use std::{path::PathBuf, str::FromStr};

// fn main() {
//     let cwd = std::env::current_dir().unwrap();
//     let test_file = PathBuf::from_str(&format!("tests/{}.txt", "group_anagrams")).unwrap();
//     let file_path = cwd.join(test_file);
//     let file_content = std::fs::read_to_string(file_path).expect("file does not exist");
//     let lines: Vec<&str> = file_content.lines().collect();

//     for (index, line) in lines.iter().enumerate() {
//         if !line.trim().is_empty() {
//             let line: String = line.trim().chars().filter(|&c| c != '"').collect();
//             let line: Vec<String> = line
//                 .strip_prefix('[')
//                 .unwrap()
//                 .strip_suffix(']')
//                 .unwrap()
//                 .split(',')
//                 .map(|s| s.trim().to_string())
//                 .collect();
//             println!("Test {}:\n\ni: {:?}", index, line);
//             let solution = Solution::solve(line);
//             let ans_format = format!("o: {solution:?}");
//             println!("{}", ans_format);
//             println!();
//             println!("{}", "-".repeat(ans_format.len() + 2));
//             println!();
//         }
//     }
// }
