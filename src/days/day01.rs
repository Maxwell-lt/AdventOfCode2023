use regex::Regex;

use crate::solver::Solver;

pub struct Day01;
impl Solver for Day01 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        Some(input
            .split('\n')
            .filter_map(get_digits)
            .sum())
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        Some(input.split('\n')
            .filter_map(get_word_digits)
            .sum())
    }
}

fn get_digits(s: &str) -> Option<i64> {
    let mut f: Option<char> = None;
    let mut l: Option<char> = None;
    for c in s.chars() {
        if c.is_ascii_digit() {
            if f.is_none() {
                f = Some(c);
                l = Some(c);
            } else {
                l = Some(c);
            }
        }
    }
    f.zip(l).map(|(f, l)| f.to_string() + &l.to_string()).and_then(|s| s.parse::<i64>().ok())
}

fn get_word_digits(s: &str) -> Option<i64> {
    let r = Regex::new("one|two|three|four|five|six|seven|eight|nine|\\d").unwrap();
    let rr = Regex::new("eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\\d").unwrap();
    let m = r.find(s).map(|m| m.as_str().to_string()).map(to_digit);
    let m2 = rr.find(&s.chars().rev().collect::<String>()).map(|m| m.as_str().chars().rev().collect::<String>()).map(to_digit);
    m.zip(m2).map(|(f, l)| f * 10 + l)
}

fn to_digit(s: String) -> i64 {
    match s.as_str() {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let output = Day01.solve1(input);
        assert_eq!(output, Some(142));
    }

    #[test]
    fn part2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let output = Day01.solve2(input);
        assert_eq!(output, Some(281));
    }
}
