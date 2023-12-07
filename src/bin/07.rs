use std::{collections::HashMap, cmp::Ordering};

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
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Card> {
        alt((
            value(Card::Ace, complete::char('A')),
            value(Card::King, complete::char('K')),
            value(Card::Queen, complete::char('Q')),
            value(Card::Jack, complete::char('J')),

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
    fn compute_hand_type(cards: &Vec<Card>) -> HandType {
        let groups = cards.into_iter().counts();

        let count_counts = groups.values().into_iter().counts();
        if count_counts.contains_key(&5) {
            HandType::FiveOfKind
        } else if count_counts.contains_key(&4) {
            HandType::FourOfKind
        } else if count_counts.contains_key(&3) && count_counts.contains_key(&2) {
            HandType::FullHouse
        } else if count_counts.contains_key(&3) {
            HandType::ThreeOfKind
        } else if count_counts.get(&2) == Some(&2) {
            HandType::TwoPair
        } else if count_counts.get(&2) == Some(&1) {
            HandType::Pair
        } else {
            HandType::HighCard
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
        }
        else {
            for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
                let cmp = card_a.cmp(card_b);

                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            return Ordering::Equal;
        }
    });

    let ranked: Vec<(u32, &Hand)> = input.iter().rev().enumerate().map(|(i, value)| (i as u32 + 1, value)).collect();

    let result: u32 = ranked.iter().map(|(rank,  hand)| {
        rank * hand.bid
    }).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
