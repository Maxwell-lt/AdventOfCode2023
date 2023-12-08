use itertools::Itertools;

use crate::solver::Solver;

pub struct Day07;
impl Solver for Day07 {
    #[allow(unused)]
    fn solve1(&self, input: &str) -> Option<i64> {
        let mut hands = input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(Hand::new)
            .collect::<Vec<_>>();
        hands.sort();
        hands.reverse();
        Some(
            hands
                .iter()
                .enumerate()
                .map(|(i, hand)| (i as i64 + 1) * hand.bid)
                .sum(),
        )
    }

    #[allow(unused)]
    fn solve2(&self, input: &str) -> Option<i64> {
        let mut hands = input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(Hand2::new)
            .collect::<Vec<_>>();
        hands.sort();
        hands.reverse();
        Some(
            hands
                .iter()
                .enumerate()
                .map(|(i, hand)| (i as i64 + 1) * hand.bid)
                .sum(),
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum CardType {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for CardType {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unimplemented!("{}", value),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum CardType2 {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl From<char> for CardType2 {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            'J' => Self::Joker,
            _ => unimplemented!("{}", value),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [CardType; 5],
    hand_type: HandType,
    bid: i64,
}

impl Hand {
    fn new(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let cards: [CardType; 5] = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| CardType::from(c))
            .collect::<Vec<CardType>>()
            .try_into()
            .unwrap();
        let hand_type: HandType = cards.into();
        let bid = parts.next().unwrap().parse().unwrap();
        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand_type
            .partial_cmp(&other.hand_type)
            .map(|ord| match ord {
                std::cmp::Ordering::Equal => compare_card_values(self.cards, other.cards),
                _ => ord,
            })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn compare_card_values(a: [CardType; 5], b: [CardType; 5]) -> std::cmp::Ordering {
    if a[0] == b[0] {
        if a[1] == b[1] {
            if a[2] == b[2] {
                if a[3] == b[3] {
                    if a[4] == b[4] {
                        std::cmp::Ordering::Equal
                    } else {
                        a[4].cmp(&b[4])
                    }
                } else {
                    a[3].cmp(&b[3])
                }
            } else {
                a[2].cmp(&b[2])
            }
        } else {
            a[1].cmp(&b[1])
        }
    } else {
        a[0].cmp(&b[0])
    }
}

#[derive(Debug)]
struct Hand2 {
    cards: [CardType2; 5],
    hand_type: HandType,
    bid: i64,
}

impl Hand2 {
    fn new(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let cards: [CardType2; 5] = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| CardType2::from(c))
            .collect::<Vec<CardType2>>()
            .try_into()
            .unwrap();
        let hand_type: HandType = cards.into();
        let bid = parts.next().unwrap().parse().unwrap();
        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand2 {}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand_type
            .partial_cmp(&other.hand_type)
            .map(|ord| match ord {
                std::cmp::Ordering::Equal => compare_card_values2(self.cards, other.cards),
                _ => ord,
            })
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn compare_card_values2(a: [CardType2; 5], b: [CardType2; 5]) -> std::cmp::Ordering {
    if a[0] == b[0] {
        if a[1] == b[1] {
            if a[2] == b[2] {
                if a[3] == b[3] {
                    if a[4] == b[4] {
                        std::cmp::Ordering::Equal
                    } else {
                        a[4].cmp(&b[4])
                    }
                } else {
                    a[3].cmp(&b[3])
                }
            } else {
                a[2].cmp(&b[2])
            }
        } else {
            a[1].cmp(&b[1])
        }
    } else {
        a[0].cmp(&b[0])
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<[CardType; 5]> for HandType {
    fn from(value: [CardType; 5]) -> Self {
        let unique: Vec<_> = value.iter().unique().collect();
        if unique.len() == 1 {
            return Self::FiveOfKind;
        }
        if unique.len() == 2 {
            if value.iter().filter(|ct| *ct == unique[0]).count() == 4
                || value.iter().filter(|ct| *ct == unique[1]).count() == 4
            {
                return Self::FourOfKind;
            } else {
                return Self::FullHouse;
            }
        }
        if unique.len() == 3 {
            if value.iter().filter(|ct| *ct == unique[0]).count() == 3
                || value.iter().filter(|ct| *ct == unique[1]).count() == 3
                || value.iter().filter(|ct| *ct == unique[2]).count() == 3
            {
                return Self::ThreeOfKind;
            } else {
                return Self::TwoPair;
            }
        }
        if unique.len() == 4 {
            return Self::OnePair;
        }
        return Self::HighCard;
    }
}

impl From<[CardType2; 5]> for HandType {
    fn from(value: [CardType2; 5]) -> Self {
        let mut unique: Vec<_> = value.iter().unique().collect();
        unique.retain(|ct| **ct != CardType2::Joker);
        let jokers = value.iter().filter(|ct| **ct == CardType2::Joker).count();
        if unique.len() <= 1 {
            return Self::FiveOfKind;
        }
        if unique.len() == 2 {
            if value.iter().filter(|ct| *ct == unique[0]).count() + jokers == 4
                || value.iter().filter(|ct| *ct == unique[1]).count() + jokers == 4
            {
                return Self::FourOfKind;
            } else {
                return Self::FullHouse;
            }
        }
        if unique.len() == 3 {
            if value.iter().filter(|ct| *ct == unique[0]).count() + jokers == 3
                || value.iter().filter(|ct| *ct == unique[1]).count() + jokers == 3
                || value.iter().filter(|ct| *ct == unique[2]).count() + jokers == 3
            {
                return Self::ThreeOfKind;
            } else {
                return Self::TwoPair;
            }
        }
        if unique.len() == 4 {
            return Self::OnePair;
        }
        return Self::HighCard;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1() {
        let output = Day07.solve1(EXAMPLE);
        assert_eq!(output, Some(6440));
    }

    #[test]
    fn part2() {
        let output = Day07.solve2(EXAMPLE);
        assert_eq!(output, Some(5905));
    }

    #[test]
    fn part2_orderings() {
        assert!(Hand2::new("KTJJT 1") < Hand2::new("QQQJA 1"));
        assert!(Hand2::new("22222 1") < Hand2::new("2222J 1"));
    }
}
