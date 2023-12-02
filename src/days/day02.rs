use regex::Regex;

use crate::solver::Solver;

pub struct Day02;
impl Solver for Day02 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        Some(input.split('\n')
            .filter(|s| !s.is_empty())
            .map(max_cubes)
            .filter(|c| c.1 <= 12 && c.2 <= 13 && c.3 <= 14)
            .map(|c| c.0)
            .sum())

    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        Some(input.split('\n')
             .filter(|s| !s.is_empty())
             .map(max_cubes)
             .map(|c| c.1 * c.2 * c.3)
             .sum())
    }
}

fn max_cubes(input: &str) -> (i64, i64, i64, i64) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let binding = Regex::new(": |, |; ").unwrap();
    let mut splits = binding.split(input);
    let game_id = splits.next().unwrap()
        .split(' ').collect::<Vec<_>>().get(1).unwrap()
        .parse::<i64>().unwrap();
    for count in Regex::new(": |, |; ").unwrap().split(input).skip(1) {
        let mut it = count.split(' ');
        let n = it.next().unwrap().parse::<i64>().unwrap();
        let c = it.next().unwrap();
        match c {
            "red" => red = red.max(n),
            "green" => green = green.max(n),
            "blue" => blue = blue.max(n),
            _ => {},
        }
    };

    (game_id, red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let output = Day02.solve1(input);
        assert_eq!(output, Some(8));
    }

    #[test]
    fn test_max_cubes() {
        let input = "Game 293: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let output = max_cubes(input);
        assert_eq!(output, (293, 4, 2, 6));
    }

    #[test]
    fn part2() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let output = Day02.solve2(input);
        assert_eq!(output, Some(2286));
    }
}
