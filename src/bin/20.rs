use std::{
    collections::{HashMap, VecDeque},
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

advent_of_code::solution!(20);

#[derive(Debug)]
enum Module {
    FlipFlop(String, Vec<String>),
    Conjunction(String, Vec<String>),
    Broadcast(Vec<String>),
}

fn parse_flip_flop_module(input: &str) -> IResult<&str, Module> {
    let _name = terminated(preceded(tag("%"), alphanumeric1::<&str, _>), space1);
    let _dest = preceded(space0, terminated(alphanumeric1, space0));
    let _output = preceded(tag("->"), separated_list1(complete::char(','), _dest));

    map(tuple((_name, _output)), |(name, output)| {
        Module::FlipFlop(
            name.to_string(),
            output.iter().map(|s| s.to_string()).collect_vec(),
        )
    })(input)
}

fn parse_conjunction_module(input: &str) -> IResult<&str, Module> {
    let _name = terminated(preceded(tag("&"), alphanumeric1::<&str, _>), space1);
    let _dest = preceded(space0, terminated(alphanumeric1, space0));
    let _output = preceded(tag("->"), separated_list1(complete::char(','), _dest));

    map(tuple((_name, _output)), |(name, output)| {
        Module::Conjunction(
            name.to_string(),
            output.iter().map(|s| s.to_string()).collect_vec(),
        )
    })(input)
}

fn parse_broadcaster_module(input: &str) -> IResult<&str, Module> {
    let _dest = preceded(space0, terminated(alphanumeric1, space0));
    let _output = preceded(tag("broadcaster ->"), separated_list1(tag(","), _dest));

    map(_output, |output: Vec<&str>| {
        Module::Broadcast(output.iter().map(|s| s.to_string()).collect_vec())
    })(input)
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    alt((
        parse_broadcaster_module,
        parse_conjunction_module,
        parse_flip_flop_module,
    ))(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, input) = separated_list1(newline, parse_module)(input).unwrap();

    let button = "button".to_string();
    let broadcaster = "broadcaster".to_string();
    let mut flip_flop_outputs: Vec<Vec<String>> = Vec::new();
    let mut flip_flop_index: HashMap<&String, usize> = HashMap::new();
    let mut flip_flop_state: Vec<bool> = Vec::new();

    let mut conj_outputs: Vec<Vec<String>> = Vec::new();
    let mut conj_index: HashMap<&String, usize> = HashMap::new();
    let mut conj_state: Vec<HashMap<&String, bool>> = Vec::new();

    let mut broadcaster_outputs: Vec<String> = Vec::new();

    for m in input.iter() {
        match m {
            Module::FlipFlop(name, outputs) => {
                flip_flop_index.insert(name, flip_flop_outputs.len());
                flip_flop_outputs.push(outputs.clone());
                flip_flop_state.push(false);
            }
            Module::Conjunction(conj_name, outputs) => {
                conj_index.insert(conj_name, conj_outputs.len());
                conj_outputs.push(outputs.clone());

                let inputs = input.iter().filter_map(|m| {
                    match m {
                        Module::FlipFlop(name, outputs) => {
                            if outputs.contains(conj_name) {
                                Some(name)
                            }
                            else {
                                None
                            }
                        },
                        Module::Conjunction(name, outputs) => {
                            if outputs.contains(conj_name) {
                                Some(name)
                            }
                            else {
                                None
                            }
                        },
                        Module::Broadcast(outputs) => if outputs.contains(conj_name) {
                            Some(&broadcaster)
                        }
                        else {
                            None
                        },
                    }
                });

                conj_state.push(HashMap::from_iter(inputs.map(|input| (input, false))));
            }
            Module::Broadcast(outputs) => {
                broadcaster_outputs = outputs.clone();
            }
        }
    }

    let mut high_signals = 0;
    let mut low_signals = 0;

    for _ in 0..1000 {
        let mut signals: VecDeque<(&String, bool, &String)> = VecDeque::new();
        signals.push_back((&button, false, &broadcaster));

        while let Some((from, signal, destination)) = signals.pop_front() {
            // print!("{} -", from);
            // if signal {
            //     print!("high");
            // } else {
            //     print!("low");
            // }
            // println!("-> {}", destination);

            if signal {
                high_signals += 1;
            }
            else {
                low_signals += 1;
            }

            if *destination == broadcaster {
                for o in broadcaster_outputs.iter() {
                    signals.push_back((&broadcaster, signal, o));
                }
            } else if let Some(&i) = flip_flop_index.get(destination) {
                if !signal {
                    flip_flop_state[i] = !flip_flop_state[i];
                    for o in flip_flop_outputs[i].iter() {
                        signals.push_back((destination, flip_flop_state[i], o));
                    }
                }
            } else if let Some(&i) = conj_index.get(destination) {
                let mut current = conj_state[i].clone();
                current.insert(from, signal);
                conj_state[i] = current;

                let all_high = conj_state[i].values().all(|b| *b);

                for o in conj_outputs[i].iter() {
                    signals.push_back((destination, !all_high, o));
                }
            }
        }
    }

    Some(high_signals * low_signals)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(32000000));
    }


    #[test]
    fn test_part_one_example_2() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
