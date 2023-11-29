#! /usr/bin/env zsh
for i in {01..25}
cat <<HERE > src/days/day${i}.rs
use crate::solver::Solver;

pub struct Day$i;
impl Solver for Day$i {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        None
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        None
    }
}
HERE
