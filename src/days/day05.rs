use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

use crate::solver::Solver;

pub struct Day05;
impl Solver for Day05 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let data = parse_sections(input);
        data.seeds
            .iter()
            .map(|seed| {
                let mut current = *seed;
                for map in &data.maps {
                    current = next_category(current, map);
                }
                (current, *seed)
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .map(|(current, seed)| current.into())
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let data = parse_sections(input);
        data.seeds
            .chunks(2)
            .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
            .par_bridge()
            .map(|seed| {
                let mut current = seed;
                for map in &data.maps {
                    current = next_category(current, map);
                }
                (current, seed)
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .map(|(current, seed)| current.into())
    }
}

#[derive(Debug)]
struct ParsedData {
    pub seeds: Vec<i64>,
    pub maps: Vec<Vec<(i64, i64, i64)>>,
}

fn parse_sections(input: &str) -> ParsedData {
    let seeds: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let maps = input.split("\n\n").into_iter().map(parse_block).skip(1).collect();
    ParsedData { seeds, maps }
}

fn parse_block(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .split("\n")
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut i = s.split_whitespace();
            (
                i.next().unwrap().parse().unwrap(),
                i.next().unwrap().parse().unwrap(),
                i.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn next_category(value: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
    map.iter()
        .filter(|m| value >= m.1 && value < (m.1 + m.2))
        .next()
        .map(|m| (value - m.1) + m.0)
        .unwrap_or(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part1() {
        let output = Day05.solve1(EXAMPLE);
        assert_eq!(output, Some(35));
    }

    #[test]
    fn get_next_category() {
        assert_eq!(next_category(99, &vec![(50, 98, 2)]), 51);
        assert_eq!(next_category(79, &vec![(50, 98, 2), (52, 50, 48)]), 81);
        assert_eq!(next_category(47, &vec![(47, 1, 46), (1, 48, 50)]), 47);
    }

    #[test]
    fn part2() {
        let output = Day05.solve2(EXAMPLE);
        assert_eq!(output, Some(46));
    }
}
