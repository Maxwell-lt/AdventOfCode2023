use itertools::Itertools;

use crate::solver::Solver;

pub struct Day13;
impl Solver for Day13 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let sections = parse_input(input);
        Some(
            sections
                .into_iter()
                .enumerate()
                .map(|(i, s)| {
                    find_vertical_rfln(&s, 0)
                        .or_else(|| find_horizontal_rfln(&s, 0).map(|n| n * 100))
                        .expect(&format!("{}: {:?}", i, s))
                })
                .sum(),
        )
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let sections = parse_input(input);
        Some(
            sections
                .into_iter()
                .enumerate()
                .map(|(i, s)| {
                    find_vertical_rfln(&s, 1)
                        .or_else(|| find_horizontal_rfln(&s, 1).map(|n| n * 100))
                        .expect(&format!("{}: {:?}", i, s))
                })
                .sum(),
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Ground {
    Ash,
    Rock,
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unimplemented!("Got char {}!", value),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Vec<Ground>>> {
    input.split("\n\n").map(parse_section).collect_vec()
}

fn parse_section(input: &str) -> Vec<Vec<Ground>> {
    input
        .split_whitespace()
        .map(|s| s.chars().map(Ground::from).collect_vec())
        .collect_vec()
}

fn find_vertical_rfln(section: &Vec<Vec<Ground>>, smudge_count: u32) -> Option<i64> {
    // Iterate over each non-edge position
    for column in 1..(section[0].len()) {
        let mut fixed_smudges = 0;
        let mut row = 0;
        'row: loop {
            if row == section.len() {
                if fixed_smudges == smudge_count {
                    return Some(column as i64);
                }
                break;
            }
            let mut steps = 0i64;
            loop {
                let left = column as i64 - steps - 1;
                let right = column as i64 + steps;
                if left < 0 || right == section[0].len() as i64 {
                    row += 1;
                    break;
                }
                if section[row][left as usize] != section[row][right as usize] {
                    if fixed_smudges == smudge_count {
                        break 'row;
                    }
                    fixed_smudges += 1;
                }
                steps += 1;
            }
        }
    }
    None
}

fn find_horizontal_rfln(section: &Vec<Vec<Ground>>, smudge_count: u32) -> Option<i64> {
    // Iterate over each non-edge position
    for row in 1..(section.len()) {
        let mut fixed_smudges = 0;
        let mut column = 0;
        'column: loop {
            if column == section[0].len() {
                if fixed_smudges == smudge_count {
                    return Some(row as i64);
                }
                break;
            }
            let mut steps = 0i64;
            loop {
                let up = row as i64 - steps - 1;
                let down = row as i64 + steps;
                if up < 0 || down == section.len() as i64 {
                    column += 1;
                    break;
                }
                if section[up as usize][column] != section[down as usize][column] {
                    if fixed_smudges == smudge_count {
                        break 'column;
                    }
                    fixed_smudges += 1;
                }
                steps += 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn part1() {
        let output = Day13.solve1(EXAMPLE);
        assert_eq!(output, Some(405));
    }

    #[test]
    fn edge_case() {
        let input = r#"..#...##.
..###..##
...##.#..
###.#.##.
..##..##.
###..###.
..#.#.#..
####..#..
...#####.
##.####..
###.#..##
..#######
..#######
###.#...#
##.####..
...#####.
####..#..
"#;
        let output = Day13.solve1(input);
        assert_eq!(output, Some(1))
    }

    #[test]
    fn edge_case2() {
        let input = r#"###...#...#.#..
.#.##.#.....#..
..###..#..#....
..###..#..#....
.#.##.#........
###...#...#.#..
###..#..##.#.##
"#;
        let output = Day13.solve1(input);
        assert_eq!(output, Some(14));
    }

    #[test]
    fn part2() {
        let output = Day13.solve2(EXAMPLE);
        assert_eq!(output, Some(400));
    }
}
