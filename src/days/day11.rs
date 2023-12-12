use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

use crate::solver::Solver;

pub struct Day11;
impl Solver for Day11 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let space = Space::new(input);
        Some(space.distances(1).into_iter().sum())
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let space = Space::new(input);
        Some(space.distances(999_999).into_iter().sum())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Point {
    Galaxy,
    Space,
}

impl From<char> for Point {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => unimplemented!("Got character {}!", value),
        }
    }
}

struct Space {
    map: Vec<Vec<Point>>,
}


/// Added for debugging purposes
impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for point in row {
                match point {
                    Point::Space => f.write_str(".")?,
                    Point::Galaxy => f.write_str("#")?,
                }
            }
            f.write_str("\n")?;
        }
        f.write_str("\n")?;
        write!(f, "\n")
    }
}

impl Space {
    fn new(input: &str) -> Self {
        Self {
            map: input
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().map(Point::from).collect_vec())
                .collect_vec(),
        }
    }

    /// Return list of galaxy coordinates in (x,y) format
    fn list_galaxies(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, point)| **point == Point::Galaxy)
                    .map(move |(x, _)| (x, y))
            })
            .collect_vec()
    }

    // Removed due to part 2 complications
    //fn expand(&mut self, factor: usize) {
    //    let mut rows = (0..self.map.len()).collect::<HashSet<_>>();
    //    let mut columns = (0..self.map[0].len()).collect::<HashSet<_>>();
    //    let galaxy_positions = self.list_galaxies();
    //    for (x, y) in galaxy_positions.iter() {
    //        rows.remove(&y);
    //        columns.remove(&x);
    //    }
    //    // expand rows
    //    let mut expansions = 0;
    //    for row in rows.iter().sorted() {
    //        for _ in 0..factor {
    //            self.map
    //                .insert(row + expansions, vec![Point::Space; self.map[0].len()]);
    //            expansions += 1;
    //        }
    //    }
    //    // expand columns
    //    for row in 0..self.map.len() {
    //        let mut expansions = 0;
    //        for column in columns.iter().sorted() {
    //            for _ in 0..factor {
    //                self.map[row].insert(column + expansions, Point::Space);
    //                expansions += 1;
    //            }
    //        }
    //    }
    //}

    /// Find distances between every pair of galaxies.
    /// expansion_factor determines how much additional empty space is added in each row and column
    /// without galaxies. When set to 0 it will not add any additional space, when set to 999,999
    /// it make the empty space 1,000,000 times larger (as in part 2).
    fn distances(&self, expansion_factor: usize) -> Vec<i64> {
        let mut rows = (0..self.map.len()).collect::<HashSet<_>>();
        let mut columns = (0..self.map[0].len()).collect::<HashSet<_>>();
        let galaxy_positions = self.list_galaxies();
        for (x, y) in galaxy_positions.iter() {
            rows.remove(&y);
            columns.remove(&x);
        }
        let mut distances = vec![];
        for i in 0..galaxy_positions.len() {
            for j in i..galaxy_positions.len() {
                let a = galaxy_positions[i];
                let b = galaxy_positions[j];
                let base_distance = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
                let additional_x = expansion_factor * columns.iter().filter(|&c| *c < a.0.max(b.0) && *c > a.0.min(b.0)).count();
                let additional_y = expansion_factor * rows.iter().filter(|&c| *c < a.1.max(b.1) && *c > a.1.min(b.1)).count();
                distances.push((base_distance + additional_y + additional_x).try_into().unwrap());
            }
        }
        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;
    #[test]
    fn part1() {
        let output = Day11.solve1(EXAMPLE);
        assert_eq!(output, Some(374));
    }

// Removed due to part 2 complications
//    #[test]
//    fn test_expand() {
//        let expanded = r#"....#........
//.........#...
//#............
//.............
//.............
//........#....
//.#...........
//............#
//.............
//.............
//.........#...
//#....#......."#;
//        let mut space = Space::new(EXAMPLE);
//        space.expand(1);
//        assert_eq!(
//            space.map,
//            expanded
//                .split("\n")
//                .map(|s| s.chars().map(Point::from).collect_vec())
//                .collect_vec()
//        )
//    }

    #[test]
    fn test_large_expand() {
        let space = Space::new(EXAMPLE);
        assert_eq!(space.distances(99).iter().sum::<i64>(), 8410);
    }

    #[test]
    fn part2() {
        let output = Day11.solve2(EXAMPLE);
        assert_eq!(output, Some(82000210));
    }
}
