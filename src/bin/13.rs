use core::fmt;
use std::ops::Deref;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, newline},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(13);

struct Pattern {
    rows: Vec<Vec<bool>>,
}

impl Pattern {
    fn transpose(&self) -> Pattern {
        let cols = self.rows.get(0).unwrap().len();

        let mut res: Vec<Vec<bool>> = Vec::new();

        for c in 0..cols {
            let mut row: Vec<bool> = Vec::new();

            for r in self.rows.iter() {
                row.push(*r.get(c).unwrap());
            }

            res.push(row);
        }

        Pattern { rows: res }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in self.patterns.iter() {
            writeln!(f, "{}", p)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            for col in row {
                if *col {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn is_row_reflection(pattern: &Pattern, point: u32) -> bool {
    let mut l = point as i32 - 1;
    let mut r  = point;

    while l >= 0 && r < pattern.rows.len() as u32 {
        if pattern.rows.get(l as usize) != pattern.rows.get(r as usize) {
            return false
        }
        l -= 1;
        r += 1;
    }
    true
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let ash_or_rock = alt((
        value(true, complete::char('#')),
        value(false, complete::char('.')),
    ));

    map(separated_list1(newline, many1(ash_or_rock)), |rows| {
        Pattern { rows }
    })(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        separated_list1(tuple((newline, newline)), parse_pattern),
        |patterns| Input { patterns },
    )(input)
}

struct Input {
    patterns: Vec<Pattern>,
}
pub fn part_one(input: &str) -> Option<u32> {
    let (_, input) = parse_input(input).unwrap();

    let mut result: u32 = 0;

    for pattern in input.patterns.iter() {
        for p in 1..pattern.rows.len() {
            let p = p as u32;
            if is_row_reflection(&pattern, p) {
                result += p * 100;
                break;
            }
        }

        let tp = pattern.transpose();

        for p in  1..tp.rows.len() {
            let p = p as u32;
            if is_row_reflection(&tp, p) {
                result += p;
                break;
            }
        }
    }

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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
