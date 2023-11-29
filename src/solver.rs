pub trait Solver {
    fn solve1(&self, input: &str) -> Option<i64>;
    fn solve2(&self, input: &str) -> Option<i64>;
}
