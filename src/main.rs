mod macros;
mod problems;
use crate::problems::eval_rpn::Solution;
use crate::problems::generic_solution::Solve;

fn main() {
    let solution: Solution = Solution::new();
    println!(
        "Problem {}\n{}\n",
        solution.problem,
        "-".repeat(solution.problem.len() + 8)
    );
    Solution::run_tests(solution);
}
