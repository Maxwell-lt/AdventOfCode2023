use std::collections::HashSet;

use itertools::Itertools;

use crate::solver::Solver;

pub struct Day10;
impl Solver for Day10 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let mut map = Grid::new(input);
        let mut steps = 0i64;
        loop {
            let stepping = map.step();
            steps += 1;
            if !stepping {
                break;
            }
        }
        Some(steps / 2)
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let mut map = Grid::new(input);
        loop {
            if !map.step() {
                break;
            }
        }

        // Calculate turning direction of the loop in the order that we followed it.
        let inside_direction = map.get_turn_direction();
        // Holds all marked positions (either a pipe in the loop or an already marked interior) for
        // fast membership tests.
        let mut marked_positions_set: HashSet<Pos> =
            HashSet::from_iter(map.in_order.iter().cloned());
        // We will insert into this marked edges of the interior, then add the gaps.
        let mut interior_positions: HashSet<Pos> = HashSet::new();

        // Number of points on the pipe loop.
        let plen = map.in_order.len();

        // Find edges of interior regions
        for i in 0..plen {
            let marked = map.in_order[i]
                .adjacent_points(
                    map.in_order[wrap_dec(i, plen)],
                    map.in_order[(i + 1) % plen],
                    map.h,
                    map.w,
                    inside_direction,
                )
                .into_iter()
                .filter(|p| !marked_positions_set.contains(p))
                .collect_vec();
            marked_positions_set.extend(marked.iter());
            interior_positions.extend(marked.iter());
        }

        // Fill gaps in interior region by extending each interior edge in one direction until a
        // previously marked position is reached. Because we should have found all edge positions
        // that make up the interior positions already, choosing any direction will work equally
        // well.
        // The arbitrarily chosen extension direction is: South
        for i in interior_positions.clone().iter() {
            // Assuming we will hit a pipe or previously marked interior block before reaching the
            // bottom of the map, so unwrap should be safe.
            let mut current_pos = i.south(plen).unwrap();
            loop {
                if marked_positions_set.contains(&current_pos) {
                    break;
                }
                interior_positions.insert(current_pos);
                marked_positions_set.insert(current_pos);
                current_pos = current_pos.south(plen).unwrap();
            }
        }
        Some(interior_positions.len().try_into().unwrap())
    }
}

fn wrap_dec(i: usize, modulo: usize) -> usize {
    if i == 0 {
        modulo - 1
    } else {
        i - 1
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Pipe {
    Start,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Ground,
}

impl Pipe {
    fn new(c: char) -> Self {
        match c {
            'S' => Self::Start,
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            'F' => Self::SE,
            '7' => Self::SW,
            '.' => Self::Ground,
            _ => unimplemented!("Got char {}!", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn north(&self) -> Option<Pos> {
        if self.y > 0 {
            Some(Pos {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }
    fn south(&self, max: usize) -> Option<Pos> {
        if self.y < max {
            Some(Pos {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }
    fn east(&self, max: usize) -> Option<Pos> {
        if self.x < max {
            Some(Pos {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }
    fn west(&self) -> Option<Pos> {
        if self.x > 0 {
            Some(Pos {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn compare(&self, other: Pos, h: usize, w: usize) -> Direction {
        if self.north() == Some(other) {
            Direction::North
        } else if self.south(h) == Some(other) {
            Direction::South
        } else if self.east(w) == Some(other) {
            Direction::East
        } else if self.west() == Some(other) {
            Direction::West
        } else {
            Direction::NotAdjacent
        }
    }

    /// Find the adjacent points on either the left or right of the current position.
    fn adjacent_points(
        &self,
        prev: Pos,
        next: Pos,
        h: usize,
        w: usize,
        inner: TurnDirection,
    ) -> Vec<Pos> {
        let (l, r): (Vec<Option<Pos>>, Vec<Option<Pos>>) =
            match (self.compare(prev, h, w), self.compare(next, h, w)) {
                (Direction::North, Direction::South) => (vec![self.east(w)], vec![self.west()]),
                (Direction::North, Direction::East) => (
                    vec![],
                    vec![
                        self.south(h),
                        self.west(),
                        self.south(h).and_then(|p| p.west()),
                    ],
                ),
                (Direction::North, Direction::West) => (
                    vec![
                        self.south(h),
                        self.east(w),
                        self.south(h).and_then(|p| p.east(w)),
                    ],
                    vec![],
                ),
                (Direction::South, Direction::North) => (vec![self.west()], vec![self.east(w)]),
                (Direction::South, Direction::East) => (
                    vec![
                        self.west(),
                        self.north(),
                        self.north().and_then(|p| p.west()),
                    ],
                    vec![],
                ),
                (Direction::South, Direction::West) => (
                    vec![],
                    vec![
                        self.east(w),
                        self.north(),
                        self.north().and_then(|p| p.east(w)),
                    ],
                ),
                (Direction::East, Direction::North) => (
                    vec![
                        self.west(),
                        self.south(h),
                        self.south(h).and_then(|p| p.west()),
                    ],
                    vec![],
                ),
                (Direction::East, Direction::South) => (
                    vec![],
                    vec![
                        self.west(),
                        self.north(),
                        self.north().and_then(|p| p.west()),
                    ],
                ),
                (Direction::East, Direction::West) => (vec![self.south(h)], vec![self.north()]),
                (Direction::West, Direction::North) => (
                    vec![],
                    vec![
                        self.east(w),
                        self.south(h),
                        self.south(h).and_then(|p| p.east(w)),
                    ],
                ),
                (Direction::West, Direction::South) => (
                    vec![
                        self.north(),
                        self.east(w),
                        self.north().and_then(|p| p.east(w)),
                    ],
                    vec![],
                ),
                (Direction::West, Direction::East) => (vec![self.north()], vec![self.south(h)]),
                _ => panic!(
                    "Failed to find turn direction for sequence of positions {:?}, {:?}, {:?}",
                    prev, self, next
                ),
            };
        match inner {
            TurnDirection::Left => l,
            TurnDirection::Right => r,
        }
        .into_iter()
        .flatten()
        .collect_vec()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
    NotAdjacent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<Pipe>>,
    position: Pos,
    last_position: Pos,
    h: usize,
    w: usize,
    turn_direction: i32,
    in_order: Vec<Pos>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().map(Pipe::new).collect_vec())
            .collect_vec();
        let y = map
            .iter()
            .position(|line| line.iter().any(|pipe| *pipe == Pipe::Start))
            .unwrap();
        let x = map[y].iter().position(|pipe| *pipe == Pipe::Start).unwrap();
        let pos = Pos { x, y };
        Self {
            h: map.len(),
            w: map[0].len(),
            map,
            position: pos,
            last_position: pos,
            turn_direction: 0,
            in_order: vec![],
        }
    }

    fn pipe_at(&self, p: Pos) -> Pipe {
        self.map[p.y][p.x]
    }

    fn get_turn_direction(&self) -> TurnDirection {
        if self.turn_direction > 0 {
            TurnDirection::Right
        } else {
            TurnDirection::Left
        }
    }

    /// Steps along the pipe. Returns false if the start point has been reached, true otherwise.
    fn step(&mut self) -> bool {
        self.in_order.push(self.position);
        let current = self.position;
        let north = self.position.north();
        let south = self.position.south(self.h);
        let east = self.position.east(self.w);
        let west = self.position.west();
        // Should only be true if we are at the start
        if self.position == self.last_position {
            if north.is_some()
                && match self.pipe_at(self.position.north().unwrap()) {
                    Pipe::NS | Pipe::SE | Pipe::SW => true,
                    _ => false,
                }
            {
                self.position = north.unwrap();
            } else if south.is_some()
                && match self.pipe_at(south.unwrap()) {
                    Pipe::NS | Pipe::NE | Pipe::NW => true,
                    _ => false,
                }
            {
                self.position = south.unwrap();
            } else if east.is_some()
                && match self.pipe_at(east.unwrap()) {
                    Pipe::EW | Pipe::NE | Pipe::SE => true,
                    _ => false,
                }
            {
                self.position = east.unwrap();
            } else if west.is_some()
                && match self.pipe_at(west.unwrap()) {
                    Pipe::EW | Pipe::NW | Pipe::SW => true,
                    _ => false,
                }
            {
                self.position = west.unwrap();
            }
        } else {
            match self.last_position.compare(self.position, self.h, self.w) {
                Direction::North => match self.pipe_at(self.position) {
                    Pipe::NS => self.position = north.unwrap(),
                    Pipe::SE => {
                        self.position = east.unwrap();
                        self.turn_direction += 1;
                    }
                    Pipe::SW => {
                        self.position = west.unwrap();
                        self.turn_direction -= 1;
                    }
                    _ => panic!(
                        "Going north, hit {:?} at {:?}",
                        self.pipe_at(self.position),
                        self.position
                    ),
                },
                Direction::South => match self.pipe_at(self.position) {
                    Pipe::NS => self.position = south.unwrap(),
                    Pipe::NE => {
                        self.position = east.unwrap();
                        self.turn_direction -= 1;
                    }
                    Pipe::NW => {
                        self.position = west.unwrap();
                        self.turn_direction += 1;
                    }
                    _ => panic!(
                        "Going south, hit {:?} at {:?}",
                        self.pipe_at(self.position),
                        self.position
                    ),
                },
                Direction::East => match self.pipe_at(self.position) {
                    Pipe::EW => self.position = east.unwrap(),
                    Pipe::NW => {
                        self.position = north.unwrap();
                        self.turn_direction -= 1;
                    }
                    Pipe::SW => {
                        self.position = south.unwrap();
                        self.turn_direction += 1;
                    }
                    _ => panic!(
                        "Going east, hit {:?} at {:?}",
                        self.pipe_at(self.position),
                        self.position
                    ),
                },
                Direction::West => match self.pipe_at(self.position) {
                    Pipe::EW => self.position = west.unwrap(),
                    Pipe::NE => {
                        self.position = north.unwrap();
                        self.turn_direction += 1;
                    }
                    Pipe::SE => {
                        self.position = south.unwrap();
                        self.turn_direction -= 1;
                    }
                    _ => panic!(
                        "Going west, hit {:?} at {:?}",
                        self.pipe_at(self.position),
                        self.position
                    ),
                },
                Direction::NotAdjacent => panic!("Last and current position are not adjacent!!!"),
            }
        }
        self.last_position = current;
        if self.pipe_at(self.position) == Pipe::Start {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    #[test]
    fn part1() {
        let output = Day10.solve1(EXAMPLE);
        assert_eq!(output, Some(8));
    }

    #[test]
    fn part2() {
        let example1 = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
        let example2 = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
        let example3 = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;
        assert_eq!(Day10.solve2(example1), Some(4));
        assert_eq!(Day10.solve2(example2), Some(8));
        assert_eq!(Day10.solve2(example3), Some(10));

        let fill_test = r#".........
.S-----7.
.|.....|.
.|.....|.
.|.....|.
.|.....|.
.|.....|.
.L-----J.
.........
"#;
        assert_eq!(Day10.solve2(fill_test), Some(25));
    }
}
