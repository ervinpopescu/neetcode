#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashMap;
// use itertools::Itertools;

pub struct Solution {
    pub problem: String,
}

impl Solution {
    pub fn solve(mut nums: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<i32> = vec![1; nums.len()];
        for (i, &num) in nums.iter().enumerate().skip(1) {
            out[i] = nums[i - 1] * out[i - 1];
        }
        let mut right = 1;
        for n in out.iter_mut().rev() {
            *n *= right;
            right *= nums.pop().unwrap();
        }
        out
    }    
}
