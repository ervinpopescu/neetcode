use std::{collections::HashMap, path::PathBuf, str::FromStr};

use super::generic_solution::{GenericSolution, Solve};

pub struct Solution {
    pub generic: GenericSolution<Vec<String>, Vec<Vec<String>>>,
    pub problem: String,
}

impl Solve<Vec<String>, Vec<Vec<String>>> for Solution {
    fn new() -> Self {
        Self {
            problem: "group_anagrams".to_string(),
            generic: GenericSolution {
                input: Vec::new(),
                output: Vec::new(),
            },
        }
    }
    fn solve(input: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in input {
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
