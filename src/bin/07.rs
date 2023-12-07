use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, newline, space1},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(7);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
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

impl Card {
    fn parse(input: &str) -> IResult<&str, Card> {
        alt((
            value(Card::Ace, complete::char('A')),
            value(Card::King, complete::char('K')),
            value(Card::Queen, complete::char('Q')),
            value(Card::Joker, complete::char('J')),
            value(Card::Ten, complete::char('T')),
            value(Card::Nine, complete::char('9')),
            value(Card::Eight, complete::char('8')),
            value(Card::Seven, complete::char('7')),
            value(Card::Six, complete::char('6')),
            value(Card::Five, complete::char('5')),
            value(Card::Four, complete::char('4')),
            value(Card::Three, complete::char('3')),
            value(Card::Two, complete::char('2')),
        ))(input)
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    /*
       can make full house


       no three of a kind..
           is the largest count count + number of jokers at least three?
           calc remaining jokers
           is the next biggest count count + number of remaining  jokers at  least  two?  yes
           no

    */


    fn cc_remove(key: usize, count_counts: HashMap<usize, usize>) -> HashMap<usize, usize> {
        let mut res = count_counts.clone();

        match res.get(&key) {
            Some(1) => res.remove(&key),
            Some(n) => res.insert(key, n - 1),
            None => panic!("Well well well"),
        };

        res
    }

    fn counts(map: &HashMap<&Card, usize>) -> HashMap<usize, usize> {
        let mut res: HashMap<usize, usize> = HashMap::new();
        map.values().into_iter().for_each(|item| *res.entry(*item).or_default() += 1);
        res
    }

    fn can_make_full_house(groups: &HashMap<&Card, usize>, jokers: usize) -> bool {
        let mut gc = groups.clone();
        gc.remove(&Card::Joker);
        let mut cc = Hand::counts(&gc);

        if cc.contains_key(&3) {
            cc = Hand::cc_remove(3, cc);
            let next_largest = *cc.keys().max().unwrap();
            return next_largest + jokers >= 2;
        } else {
            let largest = *cc.keys().max().unwrap();

            if largest + jokers < 3 {
                return false;
            }

            let remaining_jokers = jokers - (3 - largest);
            cc = Hand::cc_remove(largest, cc);
            let next_largest = *cc.keys().max().unwrap();

            return next_largest + remaining_jokers >= 2;
        }
    }

    fn can_make_n_of_kind(n: &usize, groups: &HashMap<&Card, usize>, jokers: usize) -> bool {
        let mut gc = groups.clone();
        gc.remove(&Card::Joker);
        let mut cc = Hand::counts(&gc);

        if cc.contains_key(n) {
            return true;
        } else {
            let largest = *cc.keys().max().unwrap_or(&0);
            return largest + jokers >= *n;
        }
    }

    fn can_make_pair(n: usize, groups: &HashMap<&Card, usize>, jokers: usize) -> bool {
        let mut gc = groups.clone();
        gc.remove(&Card::Joker);
        let mut cc = Hand::counts(&gc);

        let number_of_pairs_remaining = n - *cc.get(&2).unwrap_or(&0);

        let mut remaining_jokers = jokers;

        for _ in 0..number_of_pairs_remaining {
            let largest = *cc.keys().max().unwrap();

            if largest + remaining_jokers < 2 {
                return false;
            }

            remaining_jokers = remaining_jokers - (2 - largest);
            cc = Hand::cc_remove(largest, cc);
        }

        true
    }

    fn compute_hand_type(cards: &Vec<Card>) -> HandType {
        let groups = cards.into_iter().counts();

        let jokers = groups.get(&Card::Joker);

        let count_counts = groups.values().into_iter().counts();

        match jokers {
            Some(jokers) => {
                if Hand::can_make_n_of_kind(&5, &groups, *jokers) {
                    return HandType::FiveOfKind;
                } else if Hand::can_make_n_of_kind(&4, &groups, *jokers) {
                    return HandType::FourOfKind;
                } else if Hand::can_make_full_house(&groups, *jokers) {
                    HandType::FullHouse
                } else if Hand::can_make_n_of_kind(&3, &groups, *jokers) {
                    HandType::ThreeOfKind
                } else if Hand::can_make_pair(2, &groups, *jokers) {
                    HandType::TwoPair
                } else if Hand::can_make_pair(1, &groups, *jokers) {
                    HandType::Pair
                } else {
                    HandType::HighCard
                }
            }
            None => {
                if count_counts.contains_key(&5) {
                    return HandType::FiveOfKind;
                } else if count_counts.contains_key(&4) {
                    return HandType::FourOfKind;
                }
                if count_counts.contains_key(&3) && count_counts.contains_key(&2) {
                    HandType::FullHouse
                } else if count_counts.contains_key(&3) {
                    HandType::ThreeOfKind
                } else if count_counts.get(&2) == Some(&2) {
                    HandType::TwoPair
                } else if count_counts.get(&2) == Some(&1) || jokers.is_some_and(|n| n >= &2) {
                    HandType::Pair
                } else {
                    HandType::HighCard
                }
            }
        }
    }

    fn parse(input: &str) -> IResult<&str, Hand> {
        let cards = many1(Card::parse);
        map(
            tuple((cards, space1, complete::u32, newline)),
            |(cards, _1, bid, _)| {
                let hand_type = Hand::compute_hand_type(&cards);
                Hand {
                    cards,
                    bid,
                    hand_type,
                }
            },
        )(input)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("{}", input);
    let (_, mut input) = many1(Hand::parse)(input).unwrap();

    input.sort_by(|a, b| {
        let hand_type = a.hand_type.cmp(&b.hand_type);

        if hand_type != Ordering::Equal {
            hand_type
        } else {
            for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
                let cmp = card_a.cmp(card_b);

                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            return Ordering::Equal;
        }
    });

    let ranked: Vec<(u32, &Hand)> = input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, value)| (i as u32 + 1, value))
        .collect();

    let result: u32 = ranked.iter().map(|(rank, hand)| rank * hand.bid).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut input) = many1(Hand::parse)(input).unwrap();

    input.sort_by(|a, b| {
        let hand_type = a.hand_type.cmp(&b.hand_type);

        if hand_type != Ordering::Equal {
            hand_type
        } else {
            for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
                let cmp = card_a.cmp(card_b);

                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            return Ordering::Equal;
        }
    });

    let ranked: Vec<(u32, &Hand)> = input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, value)| (i as u32 + 1, value))
        .collect();

    let result: u32 = ranked.iter().map(|(rank, hand)| rank * hand.bid).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
