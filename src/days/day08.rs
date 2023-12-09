use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;

use crate::solver::Solver;

pub struct Day08;
impl Solver for Day08 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let mut parts = input.split("\n\n");
        let sequence = parts
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unimplemented!("Got direction {}!", c),
            })
            .collect_vec();
        let nodes = parts
            .next()
            .unwrap()
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(parse_node)
            .collect::<HashMap<_, _>>();
        let mut current = String::from("AAA");
        let mut steps: i64 = 0;
        loop {
            if current == "ZZZ" {
                break;
            }
            current =
                nodes.get(&current).unwrap()[sequence[steps as usize % sequence.len()]].to_owned();
            steps += 1;
        }
        Some(steps)
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let mut parts = input.split("\n\n");
        let sequence = parts
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unimplemented!("Got direction {}!", c),
            })
            .collect_vec();
        let nodes = parts
            .next()
            .unwrap()
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(parse_node)
            .collect::<HashMap<_, _>>();
        let mut current = nodes
            .keys()
            .filter(|s| s.chars().last().unwrap() == 'A')
            .collect_vec();
        let mut steps: i64 = 0;
        current.iter()
            .map(|current| {
                let mut current = current.to_owned();
                let mut steps: i64 = 0;
                loop {
                    if current.chars().last().unwrap() == 'Z' { break; }
                    current = &nodes.get(current).unwrap()[sequence[steps as usize % sequence.len()]];
                    steps += 1;
                }
                return steps;
            })
        .reduce(|a: i64, b: i64| a.lcm(&b))
    }
}

fn parse_node(input: &str) -> (String, [String; 2]) {
    let key = input[0..3].to_owned();
    let values = [input[7..10].to_owned(), input[12..15].to_owned()];
    (key, values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
        let output = Day08.solve1(input);
        assert_eq!(output, Some(2));
    }

    #[test]
    fn part2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;
        let output = Day08.solve2(input);
        assert_eq!(output, Some(6))
    }
}
