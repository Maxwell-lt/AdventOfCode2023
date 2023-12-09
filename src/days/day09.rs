use itertools::Itertools;

use crate::solver::Solver;

pub struct Day09;
impl Solver for Day09 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        Some(
            input
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_vec()
                })
                .map(predict)
                .sum(),
        )
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        Some(
            input
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .rev()
                        .collect_vec()
                })
                .map(predict)
                .sum(),
        )
    }
}

fn predict(history: Vec<i64>) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![history];
    loop {
        if diffs.last().unwrap().iter().all(|&n| n == 0) {
            break;
        }
        diffs.push(
            diffs
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect_vec(),
        );
    }
    diffs.iter().rev().map(|d| d.last().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn part1() {
        let output = Day09.solve1(EXAMPLE);
        assert_eq!(output, Some(114));
    }

    #[test]
    fn test_predict() {
        assert_eq!(predict(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict(vec![10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(predict(vec![15, 12, 9, 6, 3, 0]), -3);
    }

    #[test]
    fn part2() {
        let output = Day09.solve2(EXAMPLE);
        assert_eq!(output, Some(2));
    }
}
