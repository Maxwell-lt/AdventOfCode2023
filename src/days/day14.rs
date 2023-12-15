use std::time::Instant;

use itertools::Itertools;

use crate::solver::Solver;

pub struct Day14;
impl Solver for Day14 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let platform = parse_platform(input);
        let height = platform.len();
        let mut load = 0;
        for column in 0..platform[0].len() {
            // Mark last bottom edge found, so we know where the rocks will roll to.
            let mut last_empty = 0;
            // Number of rolling found in this gap.
            let mut rolling_found = 0;
            for row in 0..height {
                if platform[row][column] == Space::Rolling {
                    rolling_found += 1;
                }
                if row == height - 1 || platform[row][column] == Space::Rock {
                    let additional_load = (rolling_found * (height - last_empty))
                        - ((rolling_found * (rolling_found + 1) / 2) - rolling_found);
                    load += additional_load;
                    last_empty = row + 1;
                    rolling_found = 0;
                }
            }
        }
        Some(load as i64)
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let mut platform = parse_platform(input);
        let start = Instant::now();
        for i in 0..1_000_000_000 {
            perform_roll(&mut platform);
        }
        let height = platform.len();
        let mut load = 0;
        for row in 0..platform.len() {
            for column in 0..platform[0].len() {
                if platform[row][column] == Space::Rolling {
                    load += height - row;
                }
            }
        }
        Some(load as i64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Rolling,
    Rock,
    Empty,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Rolling,
            '#' => Self::Rock,
            '.' => Self::Empty,
            _ => unimplemented!("Got char {}!", value),
        }
    }
}

fn parse_platform(input: &str) -> Vec<Vec<Space>> {
    input
        .split_whitespace()
        .map(|s| s.chars().map(Space::from).collect_vec())
        .collect_vec()
}

fn perform_roll(platform: &mut Vec<Vec<Space>>) {
    // Northward
    for column in 0..platform[0].len() {
        let height = platform.len();
        let mut last_empty = 0;
        let mut rolling_found = 0;
        for row in 0..height {
            if platform[row][column] == Space::Rolling {
                rolling_found += 1;
            }
            if row == height - 1 || platform[row][column] == Space::Rock {
                let mut placed = 0;
                for edit_row in last_empty..row {
                    if placed < rolling_found {
                        platform[edit_row][column] = Space::Rolling;
                        placed += 1;
                    } else {
                        platform[edit_row][column] = Space::Empty;
                    }
                }
                last_empty = row + 1;
            }
        }
    }
    // Westward
    for row in 0..platform.len() {
        let width = platform[0].len();
        let mut last_empty = 0;
        let mut rolling_found = 0;
        for column in 0..width {
            if platform[row][column] == Space::Rolling {
                rolling_found += 1;
            }
            if column == width - 1 || platform[row][column] == Space::Rock {
                let mut placed = 0;
                for edit_column in last_empty..column {
                    if placed < rolling_found {
                        platform[row][edit_column] = Space::Rolling;
                        placed += 1;
                    } else {
                        platform[row][edit_column] = Space::Empty;
                    }
                }
                last_empty = column + 1;
            }
        }
    }
    // Southward
    for column in 0..platform[0].len() {
        let height = platform.len();
        let mut last_empty = height - 1;
        let mut rolling_found = 0;
        for row in (0..height).rev() {
            if platform[row][column] == Space::Rolling {
                rolling_found += 1;
            }
            if row == 0 || platform[row][column] == Space::Rock {
                let mut placed = 0;
                for edit_row in last_empty..row {
                    if placed < rolling_found {
                        platform[edit_row][column] = Space::Rolling;
                        placed += 1;
                    } else {
                        platform[edit_row][column] = Space::Empty;
                    }
                }
                if row != 0 {
                    last_empty = row - 1;
                }
            }
        }
    }
    // Eastward
    for row in 0..platform.len() {
        let width = platform[0].len();
        let mut last_empty = 0;
        let mut rolling_found = 0;
        for column in (0..width).rev() {
            if platform[column][row] == Space::Rolling {
                rolling_found += 1;
            }
            if column == 0 || platform[column][row] == Space::Rock {
                let mut placed = 0;
                for edit_column in last_empty..column {
                    if placed < rolling_found {
                        platform[row][edit_column] = Space::Rolling;
                        placed += 1;
                    } else {
                        platform[row][edit_column] = Space::Empty;
                    }
                }
                if column != 0 {
                    last_empty = column - 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

    #[test]
    fn part1() {
        let output = Day14.solve1(EXAMPLE);
        assert_eq!(output, Some(136));
    }

    // Commented out because it will be painfully slow
    //#[test]
    //fn part2() {
    //    let output = Day14.solve2(EXAMPLE);
    //    assert_eq!(output, Some(64));
    //}
}
