use crate::solver::Solver;

pub struct Day06;
impl Solver for Day06 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let mut lines = input.split("\n");
        let times = parse_line(lines.next().unwrap());
        let distances = parse_line(lines.next().unwrap());

        let product = times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(time, distance)| {
                find_optimal_times(time, distance)
                    .into_iter()
                    .count()
                    .try_into()
                    .unwrap()
            })
            .fold(1, |acc, x: i64| acc * x);
        Some(product)
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let mut lines = input.split("\n");
        let time = parse_line2(lines.next().unwrap());
        let distance = parse_line2(lines.next().unwrap());
        Some(find_optimal_times(time, distance).into_iter().count().try_into().unwrap())
    }
}

fn parse_line(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn parse_line2(input: &str) -> i64 {
    input
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<i64>().unwrap()
}

fn calc_distance(press_time: i64, total_time: i64) -> i64 {
    (total_time - press_time) * press_time
}

fn find_optimal_times(total_time: i64, record: i64) -> Vec<i64> {
    (0..=total_time)
        .into_iter()
        .filter(|t| calc_distance(*t, total_time) > record)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn part1() {
        let output = Day06.solve1(EXAMPLE);
        assert_eq!(output, Some(288));
    }

    #[test]
    fn test_calc_distance() {
        assert_eq!(calc_distance(0, 7), 0);
        assert_eq!(calc_distance(7, 7), 0);
        assert_eq!(calc_distance(6, 7), 6);
        assert_eq!(calc_distance(2, 7), 10);
    }

    #[test]
    fn test_find_optimal_times() {
        assert_eq!(find_optimal_times(7, 9), vec![2, 3, 4, 5]);
    }

    #[test]
    fn part2() {
        let output = Day06.solve2(EXAMPLE);
        assert_eq!(output, Some(71503));
    }

    #[test]
    fn test_parse_line2() {
        let input = "Time:            7   15    30";
        let output = parse_line2(input);
        assert_eq!(output, 71530);
    }
}
