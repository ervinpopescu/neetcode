use std::{path::PathBuf, str::FromStr};

use itertools::Itertools;

use super::generic_solution::{GenericSolution, Solve};

type InputType = (Vec<String>, Vec<Vec<i32>>);

pub struct Solution {
    pub generic: GenericSolution<InputType, Vec<i32>>,
    pub problem: String,
}

impl Solve<(Vec<String>, Vec<Vec<i32>>), Vec<Option<i32>>> for Solution {
    fn new() -> Self {
        Self {
            generic: GenericSolution {
                input: (Vec::<String>::new(), Vec::new()),
                output: Vec::<i32>::new(),
            },
            problem: "min_stack".to_string(),
        }
    }
    fn solve(input: (Vec<String>, Vec<Vec<i32>>)) -> Vec<Option<i32>> {
        let actions = input.0.iter().skip(1).collect_vec();
        let inputs = input.1.iter().skip(1).collect_vec();
        let mut outputs = Vec::new();
        let mut min_stack = MinStack::new();
        for (action, input_for_action) in actions.iter().zip(inputs) {
            let out: Option<i32>;
            match action.as_str() {
                "push" => {
                    MinStack::push(&mut min_stack, input_for_action[0]);
                    out = None;
                }
                "pop" => {
                    MinStack::pop(&mut min_stack);
                    out = None;
                }
                "top" => {
                    out = Some(MinStack::top(&min_stack));
                }
                "getMin" => {
                    out = Some(MinStack::get_min(&min_stack));
                }
                _ => {
                    unreachable!()
                }
            };
            outputs.push(out);
        }
        outputs
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
                let actions: Vec<String> = serde_json::from_str(test[0]).unwrap();
                let inputs: Vec<Vec<i32>> = serde_json::from_str(test[1]).unwrap();
                println!(
                    "Test {}:\n\n=> actions = {:?}\n   inputs = {:?}",
                    index + 1,
                    &actions,
                    &inputs
                );
                let ans = Solution::solve((actions, inputs));
                let ans_format = format!("=> {ans:?}");
                println!("{}", ans_format);
                println!();
                println!("{}", "-".repeat(ans_format.len() + 2));
                println!();
            }
        }
    }
}

pub struct MinStack {
    pub stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) {
        self.stack.pop().unwrap();
    }

    pub fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    pub fn get_min(&self) -> i32 {
        *self.stack.iter().min().unwrap()
    }
}
