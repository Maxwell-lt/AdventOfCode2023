use itertools::Itertools;

use crate::solver::Solver;

pub struct Day12;
impl Solver for Day12 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        Some(input.split("\n")
            .filter(|s| !s.is_empty())
            .map(Record::from)
            .map(|r| r.permutations() as i64)
            .sum())
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        //Some(input.split("\n")
        //    .filter(|s| !s.is_empty())
        //    .map(Record::from)
        //    .map(|r| r.unfold())
        //    .map(|r| r.permutations() as i64)
        //    .sum())
        None
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Spring::Damaged,
            '?' => Self::Unknown,
            _ => unimplemented!("Got char {}!", value),
        }
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    broken: Vec<u32>,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        let springs = parts.next().unwrap().chars().map(Spring::from).collect_vec();
        let broken = parts.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect_vec();
        Self { springs, broken }
    }
}

impl Record {
    //fn unfold(self) -> Self {
    //    let mut unfolded_springs = Vec::with_capacity(5 * self.springs.len() + 4);
    //    let mut unfolded_broken = Vec::with_capacity(5 * self.broken.len());
    //    for i in 0..5 {
    //        unfolded_springs.extend(self.springs.iter());
    //        if i != 4 {
    //            unfolded_springs.push(Spring::Unknown);
    //        }
    //        unfolded_broken.extend(self.broken.iter());
    //    }

    //    Self { springs: unfolded_springs, broken: unfolded_broken }
    //}

    fn unknown_count(&self) -> usize {
        self.springs.iter().filter(|s| **s == Spring::Unknown).count()
    }

    fn permutations(&self) -> u32 {
        let unknowns = self.unknown_count();
        let mut valid = 0;
        // iterate over possible resolutions of Unknowns
        for bit_pattern in 0..(2u128.pow(unknowns as u32)) {
            let mut block_index = 0;
            let mut broken_seen = 0;
            let mut bit_index = 0;
            for spring in &self.springs {
                match spring {
                    Spring::Operational => {
                        if broken_seen > 0 {
                            if let Some(count) = self.broken.get(block_index) {
                                if *count == broken_seen {
                                    // Current count does match, reset for the next block
                                    block_index += 1;
                                    broken_seen = 0;
                                } else {
                                    // Current count does not match, check next pattern
                                    break;
                                }
                            } else {
                                // We have finished a contiguous block of broken springs that is
                                // not in the block list, because the current index is larger
                                break;
                            }
                        }
                    },
                    Spring::Damaged => {
                        broken_seen += 1;
                    },
                    Spring::Unknown => {
                        if ((bit_pattern >> bit_index) & 1) != 0 {
                            bit_index += 1;
                            broken_seen += 1;
                        } else {
                            bit_index += 1;
                            // Copied from Spring::Operational block
                            if broken_seen > 0 {
                                if let Some(count) = self.broken.get(block_index) {
                                    if *count == broken_seen {
                                        // Current count does match, reset for the next block
                                        block_index += 1;
                                        broken_seen = 0;
                                    } else {
                                        // Current count does not match, check next pattern
                                        break;
                                    }
                                } else {
                                    // We have finished a contiguous block of broken springs that is
                                    // not in the block list, because the current index is larger
                                    break;
                                }
                            }
                        }
                    },
                }
            }
            if broken_seen == 0 && block_index == self.broken.len() {
                valid += 1;
            } else {
                if let Some(count) = self.broken.get(block_index) {
                    if *count == broken_seen && block_index == self.broken.len() - 1 {
                        valid += 1;
                    }
                }
            }
        }
        valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;
        let output = Day12.solve1(input);
        assert_eq!(output, Some(21));
    }

    #[test]
    fn test_permutations() {
        assert_eq!(Record::from("???.### 1,1,3").permutations(), 1);
        assert_eq!(Record::from("?#?#?#?#?#?#?#? 1,3,1,6").permutations(), 1);
        assert_eq!(Record::from("?###???????? 3,2,1").permutations(), 10);
    }

//    #[test]
//    fn part2() {
//        let input = r#"???.### 1,1,3
//.??..??...?##. 1,1,3
//?#?#?#?#?#?#?#? 1,3,1,6
//????.#...#... 4,1,1
//????.######..#####. 1,6,5
//?###???????? 3,2,1
//"#;
//        let output = Day12.solve2(input);
//        assert_eq!(output, Some(525152));
//    }
}
