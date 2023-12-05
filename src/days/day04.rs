use std::collections::HashMap;

use itertools::Itertools;

use crate::solver::Solver;

pub struct Day04;
impl Solver for Day04 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        Some(input.split('\n')
            .filter(|s| !s.is_empty())
            .map(split_line)
            .map(|card| get_points(count_winning_matches(&card)))
            .sum())
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let cards = input.split('\n')
            .filter(|s| !s.is_empty())
            .map(split_line)
            .collect_vec();
        let mut counts: HashMap<usize, i64> = HashMap::new();
        for i in 0..cards.len() {
            let current = &cards[i];
            let count: i64 = counts.get(&i).unwrap_or(&0) + 1;
            let points = count_winning_matches(&current);
            for j in (i + 1)..(i + points as usize + 1) {
                counts.insert(j, counts.get(&j).unwrap_or(&0) + count);
            }
        }

        Some(cards.len() as i64 + counts.values().sum::<i64>())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    pub number: i64,
    pub winning: Vec<i64>,
    pub present: Vec<i64>,
}

fn split_line(input: &str) -> Card {
    let (number, data) = {
        let mut split = input.split(": ");
        let number = split
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let data = split.next().unwrap();
        (number, data)
    };

    let (winning, present) = {
        let mut split = data.split(" | ");
        let winning = split_numbers(split.next().unwrap());
        let present = split_numbers(split.next().unwrap());
        (winning, present)
    };

    Card { number, winning, present }
}

fn split_numbers(input: &str) -> Vec<i64> {
    input
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec()
}

fn count_winning_matches(card: &Card) -> i64 {
    card.present.iter().filter(|num| card.winning.contains(*num)).count().try_into().unwrap()
}

fn get_points(matches: i64) -> i64 {
    if matches > 0 {
        1 << (matches - 1)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let output = Day04.solve1(input);
        assert_eq!(output, Some(13));
    }

    #[test]
    fn test_split_line() {
        let input = "Card 5: 12 55 32  3 | 12  3 90  1 7";
        let output = split_line(input);
        assert_eq!(output, Card { number: 5, winning: vec![12, 55, 32, 3], present: vec![12, 3, 90, 1, 7]});
    }

    #[test]
    fn test_count_matches() {
        let input = Card { number: 1, winning: vec![1, 2, 3], present: vec![1, 3, 5, 7, 9] };
        let output = count_winning_matches(&input);
        assert_eq!(output, 2);
    }

    #[test]
    fn test_get_points() {
        let input = 4;
        let output = get_points(input);
        assert_eq!(output, 8);

        let input = 0;
        let output = get_points(input);
        assert_eq!(output, 0);
        
        let input = 1;
        let output = get_points(input);
        assert_eq!(output, 1);
    }

    #[test]
    fn part2() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let output = Day04.solve2(input);
        assert_eq!(output, Some(30));
    }
}
