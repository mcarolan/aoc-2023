use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, newline, space1},
    combinator::{map, value},
    multi::many1,
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
    Two,
    Joker,
}

impl Card {
    fn parse(is_part_2: bool) -> impl Fn(&str) -> IResult<&str, Card> {
        move |input| {
            alt((
                value(Card::Ace, complete::char('A')),
                value(Card::King, complete::char('K')),
                value(Card::Queen, complete::char('Q')),
                value(
                    if is_part_2 { Card::Joker } else { Card::Jack },
                    complete::char('J'),
                ),
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
        let mut groups = cards.into_iter().counts();
        let jokers = *groups.get(&Card::Joker).unwrap_or(&0);
        groups.remove(&Card::Joker);

        let count_counts = groups.values().into_iter().counts();
        let largest_count = **count_counts.keys().max().unwrap_or(&&0);

        let next_largest_count = if count_counts.get(&largest_count) == Some(&1) {
            **count_counts
                .keys()
                .filter(|k| ***k != largest_count)
                .max()
                .unwrap_or(&&0)
        } else {
            largest_count
        };

        if largest_count + jokers >= 5 {
            HandType::FiveOfKind
        } else if largest_count + jokers >= 4 {
            HandType::FourOfKind
        } else if largest_count + jokers >= 3
            && next_largest_count + (jokers - (3 - largest_count)) >= 2
        {
            HandType::FullHouse
        } else if largest_count + jokers >= 3 {
            HandType::ThreeOfKind
        } else if largest_count + jokers >= 2
            && next_largest_count + (jokers - (2 - largest_count)) >= 2
        {
            HandType::TwoPair
        } else if largest_count + jokers >= 2 {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }

    fn parse(is_part_two: bool) -> impl Fn(&str) -> IResult<&str, Hand> {
        move |input| {
            let cards = many1(Card::parse(is_part_two));
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
}

fn solve(input: &str, is_part_two: bool) -> u32 {
    let (_, mut input) = many1(Hand::parse(is_part_two))(input).unwrap();

    input.sort_by(|a, b| {
        let hand_type = a.hand_type.cmp(&b.hand_type);

        if hand_type != Ordering::Equal {
            hand_type
        } else {
            a.cards.cmp(&b.cards)
        }
    });

    let ranked = Vec::from_iter(
        input
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, value)| (i as u32 + 1, value)),
    );

    ranked.into_iter().map(|(rank, hand)| rank * hand.bid).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
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
        assert_eq!(result, Some(5905));
    }
}
