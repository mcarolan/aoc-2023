use nom::{sequence::{preceded, terminated, tuple}, character::complete::{space0, self, newline}, IResult, bytes::complete::tag, multi::many1, combinator::map};

advent_of_code::solution!(6);

#[derive(Debug)]
struct Input {
    times: Vec<u32>,
    distances: Vec<u32>
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64
}

fn parse_spaced_number(input: &str) -> IResult<&str, u32> {
    preceded(space0, terminated(complete::u32, space0))(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let times = terminated(preceded(tag("Time:"), many1(parse_spaced_number)), newline);
    let distances = preceded(tag("Distance:"), many1(parse_spaced_number));

    map(tuple((times, distances)), |(times, distances)| {
        Input { times, distances }
    })(input)
}

fn parse_part_2_input(input: &str) -> IResult<&str, Race> {
    let time = terminated(preceded(tag("Time:"), complete::u64), newline);
    let distance = terminated(preceded(tag("Distance:"), complete::u64), newline);

    map(tuple((time, distance)), |(time, distance)| {
        Race { time, distance }
    })(input)
}

fn to_races(input: Input) -> Vec<Race> {
    Vec::from_iter(input.times.iter().zip(input.distances).map(|(time, distance)| {
        Race { time: *time as u64, distance: distance as u64 }
    }))
}

fn simulate(race: &Race, hold_for: u64) -> u64 {
    hold_for * (race.time - hold_for)
}

fn ways_to_win(race: &Race) -> u64 {
    let mut result: u64 = 0;

    for hold_for in 1..race.time {
        let travelled = simulate(race, hold_for);

        if travelled > race.distance {
            result += 1
        }
    }
    
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, input) = parse_input(input).unwrap();
    let input = to_races(input);

    let ways = input.iter().map(ways_to_win);
    
    Some(ways.product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, input) = parse_part_2_input(input.replace(" ", "").as_str()).unwrap();
    Some(ways_to_win(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
