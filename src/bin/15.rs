use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete,
    combinator::map,
    sequence::{terminated, tuple},
    IResult,
};

advent_of_code::solution!(15);

enum Op {
    Remove(String),
    Set(String, i64),
}

fn hash(input: &String) -> i64 {
    let mut current_value: i64 = 0;

    for c in input.chars() {
        let ascii = c as u8;
        current_value += ascii as i64;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .join("")
        .split(",")
        .map(|s| s.to_string())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<i64> {
    let steps = parse_input(input);

    Some(steps.iter().map(|s| hash(s)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let steps = parse_input(input);
    let mut boxes: HashMap<i64, Vec<(String, i64)>> = HashMap::new();

    for i in 0..256 {
        boxes.insert(i, Vec::new());
    }

    for op in steps {
        let lbl: String = op.chars().take_while(|c| *c != '-' && *c != '=').collect();
        let box_no = hash(&lbl);
        let b = boxes.get_mut(&box_no).unwrap();

        if let Some(equal_index ) = op.find("=") {
            let value: String = op.chars().skip(equal_index + 1).collect();
            let focal_length: i64 = value.parse().unwrap();
            let existing = b.iter().find_position(|(s,_)| s == &lbl);

            match existing {
                Some((i, _)) => {
                    b[i] = (lbl, focal_length);
                },
                None => {
                    b.push((lbl, focal_length));
                }
            }
        }
        else {
            let to_remove = b.iter().find_position(|(s,_)| s == &lbl);

            match to_remove {
                Some((i, _)) => {
                    b.remove(i);
                }
                None => (),
            }
        }
    }

    let mut res: i64 = 0;

    for (box_number, b) in boxes {
        for (slot_no, (_, focal_length)) in b.iter().enumerate()  {
            res += (box_number + 1) * (slot_no as i64 + 1) * focal_length;
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let result = hash(&"HASH".to_string());
        assert_eq!(result, 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
