use std::{collections::BTreeMap, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 07;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Card(u8);

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            anyhow::bail!("Invalid card: {}", s);
        }
        let ch = s.chars().next().unwrap();
        Ok(Self(match ch {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => anyhow::bail!("Invalid card: {}", s),
        }))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Hand {
    t: HandType,
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            anyhow::bail!("Invalid hand: {}", s);
        }
        let mut cards: [Card; 5] = [Card(0); 5];
        for i in 0..5 {
            cards[i] = s[i..i + 1].parse()?
        }
        let set = cards.iter().fold(BTreeMap::new(), |mut set, &c| {
            *set.entry(c).or_insert(0) += 1;
            set
        });
        let t = match set.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if set.values().any(|&v| v == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            2 => {
                if set.values().any(|&v| v == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            1 => HandType::FiveOfAKind,
            _ => anyhow::bail!("Invalid hand: {}", s),
        };
        Ok(Self { t, cards })
    }
}

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|l| l.split(' ').collect_tuple().unwrap())
        .map(|(hand, bid)| (hand.parse::<Hand>().unwrap(), bid.parse::<u32>().unwrap()))
        .sorted()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum())
}

fn jokerify(mut h: Hand) -> Hand {
    const JOKER: Card = Card(11);
    let mut joker_count = 0;
    for c in h.cards.iter_mut() {
        if *c == JOKER {
            *c = Card(1);
            joker_count += 1;
        }
    }
    if joker_count == 0 {
        return h;
    }
    h.t = match h.t {
        HandType::HighCard => HandType::OnePair,
        HandType::OnePair => HandType::ThreeOfAKind,
        HandType::TwoPair => {
            if joker_count == 2 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        HandType::ThreeOfAKind => HandType::FourOfAKind,
        _ => HandType::FiveOfAKind,
    };
    h
}

fn part_2(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|l| l.split(' ').collect_tuple().unwrap())
        .map(|(hand, bid)| (hand.parse::<Hand>().unwrap(), bid.parse::<u32>().unwrap()))
        .map(|(hand, bid)| (jokerify(hand), bid))
        .sorted()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum())
}

fn main() -> Result<()> {
    aoc::solve_all(DAY, part_1, part_2)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let res = "6440";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "5905";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
