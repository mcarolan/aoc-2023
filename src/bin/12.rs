use std::collections::HashMap;

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
    runs: Vec<u64>,
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
        separated_list1(complete::char(','), complete::u64),
    );
    map(parts, |(assignments, runs)| Row { assignments, runs })(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, rows) = separated_list1(newline, parse_row)(input).unwrap();
    let mut cache: HashMap<(Vec<Option<bool>>, Option<u64>, Vec<u64>), u64> = HashMap::new();

    Some(
        rows.iter()
            .map(|r| solutions(&mut cache, &r.assignments, None, &r.runs))
            .sum(),
    )
}

fn solutions(cache: &mut HashMap<(Vec<Option<bool>>, Option<u64>, Vec<u64>), u64>, assignment: &Vec<Option<bool>>, run: Option<u64>, remain: &Vec<u64>) -> u64 {
    if let Some(res) = cache.get(&(assignment.clone(), run, remain.clone())) {
        return *res;
    }
    
    if assignment.is_empty() {
        if run == None && remain.len() == 0 {
            return 1;
        }

        if remain.len() == 1 && run.is_some_and(|n| n == *remain.first().unwrap()) {
            return 1;
        }

        return 0;
    }

    let possible_more = assignment
        .iter()
        .filter(|a| **a == None || **a == Some(true))
        .count() as u64;

    if run.is_some_and(|n| n + possible_more < remain.iter().sum()) {
        return 0;
    }

    if run.is_none() && possible_more < remain.iter().sum() {
        return 0;
    }

    if run.is_some() && remain.is_empty() {
        return 0;
    }

    let head = *assignment.first().unwrap();
    let mut res = 0;

    if head == Some(false) && run.is_some_and(|n| n != *remain.first().unwrap()) {
        return 0;
    }

    if head == Some(false) && run.is_some() {
        res += solutions(cache, &assignment[1..].to_vec(), None, &remain[1..].to_vec())
    }

    if head == None && run.is_some_and(|n| n == *remain.first().unwrap()) {
        res += solutions(cache, &assignment[1..].to_vec(), None, &remain[1..].to_vec())
    }

    if (head == None || head == Some(true)) && run.is_some() {
        res += solutions(cache,&assignment[1..].to_vec(), run.map(|n| n + 1), remain);
    }

    if (head == None || head == Some(true)) && run.is_none() {
        res += solutions(cache, &assignment[1..].to_vec(), Some(1), remain)
    }

    if (head == None || head == Some(false)) && run.is_none() {
        res += solutions(cache,&assignment[1..].to_vec(), None, remain)
    }

    cache.insert((assignment.clone(), run, remain.clone()), res);
    res
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, rows) = separated_list1(newline, parse_row)(input).unwrap();

    let expanded_rows: Vec<Row> = rows
        .iter()
        .map(|r| {
            let mut assignments: Vec<Option<bool>> = Vec::new();
            let mut is_first = true;

            for _ in 0..5 {
                if !is_first {
                    assignments.push(None);
                } else {
                    is_first = false;
                }

                assignments.extend(r.assignments.iter());
            }

            Row {
                assignments,
                runs: r.runs.repeat(5),
            }
        })
        .collect();

    let mut cache: HashMap<(Vec<Option<bool>>, Option<u64>, Vec<u64>), u64> = HashMap::new();

    Some(
        expanded_rows.iter()
            .map(|r| solutions(&mut cache, &r.assignments, None, &r.runs))
            .sum(),
    )
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
        assert_eq!(result, Some(525152));
    }
}
