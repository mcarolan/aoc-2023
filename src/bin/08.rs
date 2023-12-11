use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{self, newline, space0},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use num_integer::Integer;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Entry {
    from: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    entries: Vec<Entry>,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Right, complete::char('R')),
        value(Direction::Left, complete::char('L')),
    ))(input)
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    terminated(preceded(space0, complete::alphanumeric1), space0)(input)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let equals = terminated(preceded(space0, complete::char('=')), space0);

    let f = terminated(parse_label, equals);
    let l = preceded(complete::char('('), parse_label);
    let sep = preceded(complete::char(','), space0);
    let r = terminated(preceded(sep, parse_label), complete::char(')'));

    map(tuple((f, l, r)), |(from, left, right)| Entry {
        from: String::from(from),
        left: String::from(left),
        right: String::from(right),
    })(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let directions = terminated(many1(parse_direction), newline);
    let entries = preceded(newline, separated_list1(newline, parse_entry));

    map(tuple((directions, entries)), |(directions, entries)| {
        Input {
            directions,
            entries,
        }
    })(input)
}

fn build_map(entries: &Vec<Entry>) -> HashMap<String, (String, String)> {
    let mut res = HashMap::new();

    for entry in entries {
        res.insert(
            entry.from.clone(),
            (entry.left.clone(), entry.right.clone()),
        );
    }

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, input) = parse_input(input).unwrap();
    let map = build_map(&input.entries);
    Some(steps(&String::from("AAA"), &input.directions, &map))
}

fn steps(
    from: &String,
    directions: &Vec<Direction>,
    map: &HashMap<String, (String, String)>,
) -> u64 {
    let mut step = 0;
    let mut direction_index = 0;

    let mut at = from;

    while !at.ends_with("Z") {
        let direction = directions.get(direction_index).unwrap();

        match direction {
            Direction::Left => at = &map.get(at).unwrap().0,
            Direction::Right => at = &map.get(at).unwrap().1,
        }

        direction_index = (direction_index + 1) % directions.len();
        step += 1;
    }

    step
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, input) = parse_input(input).unwrap();
    let map = build_map(&input.entries);

    let starting = map.keys().filter(|k| k.ends_with("A"));
    let steps = starting.map(|from| steps(from, &input.directions, &map));

    Some(steps.into_iter().fold(1, |acc, x| acc.lcm(&x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
