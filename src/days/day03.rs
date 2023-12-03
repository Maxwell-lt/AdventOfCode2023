use crate::solver::Solver;

pub struct Day03;
impl Solver for Day03 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let a = to_vec(input);
        let len = a[0].len();
        let height = a.len();
        let mut sum: i64 = 0;
        let mut p = (0, 0);
        loop {
            if a[p.0][p.1].is_ascii_digit() {
                let mut end = p.1;
                loop {
                    end += 1;
                    if end >= len || !a[p.0][end].is_ascii_digit() {
                        break;
                    }
                }
                // Check for symbol
                let top = if p.0 > 0 { p.0 - 1 } else { p.0 };
                let bottom = if p.0 + 1 < height { p.0 + 1 } else { p.0 };
                let left = if p.1 > 0 { p.1 - 1 } else { p.1 };
                let right = if end < len { end + 1 } else { end };
                // println!("Searching from {} to {} and {} to {}", top, bottom, left, right);
                let mut found = false;
                'outer: for i in top..=bottom {
                    for j in left..right {
                        // println!("Checking {}, {}", i, j);
                        if !a[i][j].is_ascii_digit() && a[i][j] != '.' {
                            found = true;
                            break 'outer;
                        } else {
                            // println!("Not a match: {}", a[i][j]);
                        }
                    }
                }
                if found {
                    let p_num = to_int(&a[p.0][p.1..end]);
                    // println!("Found {} at ({}, {})", p_num, p.0, p.1);
                    sum += p_num;
                }
                if end < len {
                    p = (p.0, end);
                } else {
                    if p.0 + 1 == height {
                        break;
                    }
                    p = (p.0 + 1, 0);
                }
            }
            if p.1 + 1 < len {
                p = (p.0, p.1 + 1);
            } else {
                if p.0 + 1 == height {
                    break;
                }
                p = (p.0 + 1, 0);
            }
            // println!("({}, {})", p.0, p.1);
        }
        Some(sum)
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let a = to_vec(input);
        let y = a.len();
        let x = a[0].len();
        let mut gears = vec![];
        let mut sum = 0;
        // Get potential gear positions
        for i in 0..(a.len()) {
            for j in 0..(a[0].len()) {
                if a[i][j] == '*' {
                    gears.push((i, j));
                }
            }
        }
        for gear in gears {
            let mut part_nums: Vec<i64> = vec![];
            // Is there a part number on the left?
            if gear.1 > 0 && a[gear.0][gear.1 - 1].is_ascii_digit() {
                part_nums.push(extract_number(&a, &(gear.0, gear.1 - 1)));
            }
            // Is there a part number on the right?
            if gear.1 + 1 < x && a[gear.0][gear.1 + 1].is_ascii_digit() {
                part_nums.push(extract_number(&a, &(gear.0, gear.1 + 1)));
            }
            // directly above?
            if gear.0 > 0 && a[gear.0 - 1][gear.1].is_ascii_digit() {
                part_nums.push(extract_number(&a, &(gear.0 - 1, gear.1)));
            } else {
                // top left?
                if gear.0 > 0 && gear.1 > 0 && a[gear.0 - 1][gear.1 - 1].is_ascii_digit() {
                    part_nums.push(extract_number(&a, &(gear.0 - 1, gear.1 - 1)));
                }
                // top right?
                if gear.0 > 0 && gear.1 + 1 < y && a[gear.0 - 1][gear.1 + 1].is_ascii_digit() {
                    part_nums.push(extract_number(&a, &(gear.0 - 1, gear.1 + 1)));
                }
            }
            // directly below?
            if gear.0 + 1 < y && a[gear.0 + 1][gear.1].is_ascii_digit() {
                part_nums.push(extract_number(&a, &(gear.0 + 1, gear.1)));
            } else {
                // bottom left?
                if gear.0 + 1 < y && gear.1 > 0 && a[gear.0 + 1][gear.1 - 1].is_ascii_digit() {
                    part_nums.push(extract_number(&a, &(gear.0 + 1, gear.1 - 1)));
                }
                // bottom right?
                if gear.0 + 1 < y && gear.1 + 1 < y && a[gear.0 + 1][gear.1 + 1].is_ascii_digit() {
                    part_nums.push(extract_number(&a, &(gear.0 + 1, gear.1 + 1)));
                }
            }
            // Only a "gear" if there are exactly two adjacent numbers
            if part_nums.len() == 2 {
                sum += part_nums[0] * part_nums[1];
            }
        }
        Some(sum)
    }
}

fn to_vec(input: &str) -> Vec<Vec<char>> {
    input.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

fn to_int(slice: &[char]) -> i64 {
    slice.iter().collect::<String>().parse::<i64>().unwrap()
}

fn extract_number(a: &Vec<Vec<char>>, pos: &(usize, usize)) -> i64 {
    let mut start = pos.1;
    let mut end = pos.1;
    loop {
        if start > 0 && a[pos.0][start - 1].is_ascii_digit() {
            start -= 1;
        } else {
            break;
        }
    }
    loop {
        if end + 1 < a[pos.0].len() && a[pos.0][end + 1].is_ascii_digit() {
            end += 1;
        } else {
            break;
        }
    }
    to_int(&a[pos.0][start..=end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let output = Day03.solve1(input);
        assert_eq!(output, Some(4361));
    }

    #[test]
    fn slice_convert() {
        let i = ['1', '2', '3'];
        assert_eq!(to_int(&i), 123);
    }

    #[test]
    fn find_number() {
        let i = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '1', '2', '3', '.'],
            vec!['.', '*', '.', '.', '.']
        ];
        let p = (1, 1);
        assert_eq!(extract_number(&i, &p), 123);
    }

    #[test]
    fn part2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let output = Day03.solve2(input);
        assert_eq!(output, Some(467835));
    }
}
