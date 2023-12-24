pub struct GenericSolution<T, V> {
    pub input: T,
    pub output: V,
}

pub trait Solve<T, V> {
    fn new() -> Self;
    fn solve(input: T) -> V;
    fn run_tests(self);
}
