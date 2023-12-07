use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl std::str::FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Err("Invalid card".to_string()),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Card2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl std::str::FromStr for Card2 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card2::Two),
            "3" => Ok(Card2::Three),
            "4" => Ok(Card2::Four),
            "5" => Ok(Card2::Five),
            "6" => Ok(Card2::Six),
            "7" => Ok(Card2::Seven),
            "8" => Ok(Card2::Eight),
            "9" => Ok(Card2::Nine),
            "T" => Ok(Card2::Ten),
            "J" => Ok(Card2::Joker),
            "Q" => Ok(Card2::Queen),
            "K" => Ok(Card2::King),
            "A" => Ok(Card2::Ace),
            _ => Err("Invalid card".to_string()),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Hash)]
pub enum Hand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let mut cards = cards;
        cards.sort();
        let mut map: Vec<usize> = cards
            .into_iter()
            .group_by(move |&c| c)
            .into_iter()
            .map(|(_, g)| g.collect::<Vec<_>>().len())
            .collect();
        map.sort();
        let mut map_iter = map.iter();
        match map_iter.next_back() {
            Some(1) => Hand::HighCard,
            Some(4) => Hand::FourOfAKind,
            Some(5) => Hand::FiveOfAKind,
            Some(2) => match map_iter.next_back() {
                Some(2) => Hand::TwoPair,
                Some(1) => Hand::Pair,
                _ => panic!("weird hand"),
            },
            Some(3) => match map_iter.next_back() {
                Some(2) => Hand::FullHouse,
                Some(1) => Hand::ThreeOfAKind,
                _ => panic!("weird hand"),
            },

            _ => panic!("weird hand"),
        }
    }
}

impl std::str::FromStr for Hand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().map(|c| c.to_string().parse().unwrap()).collect();
        Ok(Hand::new(cards))
    }
}

pub fn parse_input(input: &str) -> Vec<(Hand, Vec<Card>, usize)> {
    input
        .lines()
        .map(|l| {
            if let Some((cards_str, bid)) = l.split_once(' ') {
                let cards: Vec<Card> = cards_str
                    .chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect();
                let hand = Hand::new(cards.clone());
                (hand, cards, bid.parse().unwrap())
            } else {
                panic!("no space")
            }
        })
        .collect()
}

fn replace_joker(original_cards: &str) -> Hand {
    if !original_cards.contains('J') {
        return original_cards.parse().unwrap();
    }
    let to_replace: HashSet<_> = original_cards.chars().filter(|x| x != &'J').collect();
    if to_replace.is_empty() {
        return Hand::FiveOfAKind;
    }
    let mut current_max = Hand::HighCard;
    for c in to_replace.into_iter().unique() {
        let cards = original_cards.to_owned();
        let h = cards.replace('J', &c.to_string()).parse().unwrap();
        current_max = current_max.max(h);
    }
    current_max
}

pub fn parse_input_p2(input: &str) -> Vec<(Hand, Vec<Card2>, usize)> {
    input
        .lines()
        .map(|l| {
            if let Some((cards_str, bid)) = l.split_once(' ') {
                let cards: Vec<Card2> = cards_str
                    .chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect();
                let hand = replace_joker(cards_str);
                (hand, cards, bid.parse().unwrap())
            } else {
                panic!("no space")
            }
        })
        .collect()
}

pub fn part1(hands_and_bids: Vec<(Hand, Vec<Card>, usize)>) -> usize {
    let mut hands_and_bids = hands_and_bids;
    hands_and_bids.sort();
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank + 1) * bid)
        .sum()
}
pub fn part2(hands_and_bids: Vec<(Hand, Vec<Card2>, usize)>) -> usize {
    let mut hands_and_bids = hands_and_bids;
    hands_and_bids.sort();
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank + 1) * bid)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part2() {
        let input = indoc! {
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            "
        };
        let res = part2(parse_input_p2(input));
        assert_eq!(res, 5905);
    }
    #[test]
    fn test_part1() {
        let input = indoc! {
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            "
        };
        let res = part1(parse_input(input));
        assert_eq!(res, 6440);
    }
    #[test]
    fn test_parse_input() {
        use super::Card::*;
        let input = indoc! {
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            "
        };
        let parsed = parse_input(input);
        let expected = vec![
            (Hand::Pair, vec![Three, Two, Ten, Three, King], 765),
            (Hand::ThreeOfAKind, vec![Ten, Five, Five, Jack, Five], 684),
            (Hand::TwoPair, vec![King, King, Six, Seven, Seven], 28),
            (Hand::TwoPair, vec![King, Ten, Jack, Jack, Ten], 220),
            (
                Hand::ThreeOfAKind,
                vec![Queen, Queen, Queen, Jack, Ace],
                483,
            ),
        ];
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_card() {
        for (string, expected) in [("T", Card::Ten), ("A", Card::Ace), ("2", Card::Two)] {
            if let Ok(c) = string.parse::<Card>() {
                assert_eq!(c, expected);
            } else {
                panic!()
            }
        }
    }

    #[test]
    fn test_hand_high_card() {
        let cards = vec![Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five];
        let hand = Hand::new(cards);
        assert_eq!(hand, Hand::HighCard);
    }
    #[test]
    fn test_hand_pair() {
        let cards = vec![Card::Ace, Card::Two, Card::Three, Card::Ace, Card::Five];
        let hand = Hand::new(cards);
        assert_eq!(dbg!(hand), Hand::Pair);
    }

    #[test]
    fn test_hand_two_pair() {
        let cards = vec![Card::Ace, Card::Three, Card::Three, Card::Ace, Card::Five];
        let hand = Hand::new(cards);
        assert_eq!(dbg!(hand), Hand::TwoPair);
    }
    #[test]
    fn test_hand_three_of_a_kind() {
        let cards = vec![Card::Ace, Card::Ace, Card::Three, Card::Ace, Card::Five];
        let hand = Hand::new(cards);
        assert_eq!(dbg!(hand), Hand::ThreeOfAKind);
    }
    #[test]
    fn test_hand_full() {
        let cards = vec![Card::Ace, Card::Ace, Card::Three, Card::Ace, Card::Three];
        let hand = Hand::new(cards);
        assert_eq!(dbg!(hand), Hand::FullHouse);
    }
    #[test]
    fn test_hand_four() {
        let cards = vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Three];
        let hand = Hand::new(cards);
        assert_eq!(dbg!(hand), Hand::FourOfAKind);
    }
    #[test]
    fn test_hand_five() {
        let cards = vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace];
        let hand = Hand::new(cards);
        assert_eq!(dbg!(hand), Hand::FiveOfAKind);
    }
}
