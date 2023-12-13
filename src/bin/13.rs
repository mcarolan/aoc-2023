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

fn is_row_reflection(pattern: &Pattern, point: u32, expected_diff: usize) -> bool {
    let mut l = point as i32 - 1;
    let mut r = point;

    let mut diffs = 0;

    while l >= 0 && r < pattern.rows.len() as u32 && diffs <= expected_diff {
        let r1 = pattern.rows.get(l as usize).unwrap();
        let r2 = pattern.rows.get(r as usize).unwrap();

        diffs += r1.iter().zip(r2).filter(|(a, b)| a != b).count();

        l -= 1;
        r += 1;
    }

    diffs == expected_diff
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

fn solve(input: &str, expected_diff: usize) -> Option<u32> {
    let (_, input) = parse_input(input).unwrap();

    let mut result: u32 = 0;

    for pattern in input.patterns.iter() {
        let mut done = false;

        for p in 1..pattern.rows.len() {
            let p = p as u32;
            if is_row_reflection(&pattern, p, expected_diff) {
                result += p * 100;
                done = true;
                break;
            }
        }

        if !done {
            let tp = pattern.transpose();

            for p in 1..tp.rows.len() {
                let p = p as u32;
                if is_row_reflection(&tp, p, expected_diff) {
                    result += p;
                    break;
                }
            }
        }
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 0)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 1)
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
        assert_eq!(result, Some(400));
    }
}
