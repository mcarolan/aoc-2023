use itertools::Itertools;
use nom::{
    character::complete::{self, newline, space0},
    combinator::map,
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(9);

#[derive(Debug)]
struct Input {
    entries: Vec<Vec<i32>>,
}

fn parse_spaced_number(input: &str) -> IResult<&str, i32> {
    preceded(space0, terminated(complete::i32, space0))(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let parse_entry = terminated(many1(parse_spaced_number), newline);
    let entries = many1(parse_entry);
    map(entries, |entries| Input { entries })(input)
}

fn diffs(input: &Vec<i32>) -> Vec<i32> {
    Vec::from_iter(input.into_iter().tuple_windows().map(|(a, b)| b - a))
}

fn build_diffs(readings: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![readings.clone()];

    loop {
        let last = res.last().unwrap();
        let next = diffs(last);

        if next.iter().all(|n| *n == 0) {
            break;
        }
        res.push(next.clone());
    }
    res
}

fn predict(diffs: Vec<Vec<i32>>) -> i32 {
    let mut prev_prediction = 0;

    for d in diffs.iter().rev() {
        prev_prediction += d.last().unwrap();
    }

    prev_prediction
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, input) = parse_input(input).unwrap();
    let res = Vec::from_iter(input.entries.into_iter().map(|e| predict(build_diffs(&e))));

    Some(res.iter().sum())
}

fn predict_part_2(diffs: Vec<Vec<i32>>) -> i32 {
    let mut prev_prediction = 0;

    for d in diffs.iter().rev() {
        prev_prediction = d.first().unwrap() - prev_prediction;
    }

    prev_prediction
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, input) = parse_input(input).unwrap();

    predict_part_2(build_diffs(input.entries.get(2).unwrap()));

    let res = Vec::from_iter(
        input
            .entries
            .into_iter()
            .map(|e| predict_part_2(build_diffs(&e))),
    );

    Some(res.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
