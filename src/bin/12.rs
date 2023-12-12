use std::{collections::VecDeque, ops::Index};

use nom::{
    branch::alt,
    character::complete::{self, newline, space1},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(12);

#[derive(Debug)]
struct Row {
    assignments: Vec<Option<bool>>,
    runs: Vec<u32>,
}

fn parse_assignment(input: &str) -> IResult<&str, Option<bool>> {
    alt((
        value(Some(true), complete::char('#')),
        value(Some(false), complete::char('.')),
        value(None, complete::char('?')),
    ))(input)
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    let parts = separated_pair(
        many1(parse_assignment),
        space1,
        separated_list1(complete::char(','), complete::u32),
    );
    map(parts, |(assignments, runs)| Row { assignments, runs })(input)
}

fn combinations(assignments: &Vec<Option<bool>>) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = Vec::new();
    let mut q: Vec<Vec<Option<bool>>> = vec![assignments.clone()];

    while let Some(assignment) = q.pop() {
        let unassigned = assignment.iter().enumerate().find(|(_, o)| o.is_none());

        match unassigned {
            Some((i, _)) => {
                let mut truey: Vec<Option<bool>> = assignment.clone();
                truey[i] = Some(true);
                q.push(truey);

                let mut falsey: Vec<Option<bool>> = assignment.clone();
                falsey[i] = Some(false);
                q.push(falsey);
            }
            None => result.push(assignment.iter().map(|o| o.unwrap()).collect()),
        }
    }

    result
}

fn is_valid(assignment: &Vec<bool>, runs: &Vec<u32>) -> bool {
    let mut runs_remaining: VecDeque<u32> = VecDeque::new();
    runs_remaining.extend(runs);
    let mut current_run: Option<u32> = None;

    for b in assignment {
        if *b && current_run.is_some() {
            current_run = current_run.map(|c| c + 1);
        } else if *b && current_run.is_none() {
            current_run = Some(1)
        } else if !*b && current_run.is_some() {
            let run_achieved = current_run.unwrap_or_default();
            current_run = None;

            let expected = runs_remaining.pop_front();
            match expected {
                Some(v) if v != run_achieved => return false,
                Some(_) => (),
                None => return false,
            }
        }
    }

    (current_run.is_none() && runs_remaining.is_empty())
        || (runs_remaining.len() == 1 && runs_remaining.pop_front() == current_run)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, rows) = separated_list1(newline, parse_row)(input).unwrap();

    Some(
        rows.iter()
            .map(|r| {
                combinations(&r.assignments)
                    .iter()
                    .filter(|a| is_valid(a, &r.runs))
                    .count() as u32
            })
            .sum(),
    )
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
