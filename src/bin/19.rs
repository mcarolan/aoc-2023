use std::{collections::HashMap, thread::current};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
advent_of_code::solution!(19);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Gt,
    Lt,
}

#[derive(Debug, Clone)]
struct Comparison {
    variable: char,
    operator: Operator,
    value: u64,
    goto: String,
}

#[derive(Debug)]
enum WorkflowStep {
    Comparison(Comparison),
    StraightTo(String),
}

#[derive(Debug)]
struct Workflow {
    name: String,
    steps: Vec<WorkflowStep>,
}

fn parse_comparison(input: &str) -> IResult<&str, Comparison> {
    let _variable = complete::anychar;
    let _operator = alt((
        value(Operator::Lt, complete::char('<')),
        value(Operator::Gt, complete::char('>')),
    ));
    let _value = complete::u64;

    let goto = preceded(complete::char(':'), alphanumeric1);

    let parts = tuple((_variable, _operator, _value, goto))(input);

    match parts {
        Ok((rest, (variable, operator, value, goto))) => Ok((
            rest,
            Comparison {
                variable,
                operator,
                value,
                goto: goto.to_string(),
            },
        )),
        Err(e) => Err(e),
    }
}

fn parse_workflow_step(input: &str) -> IResult<&str, WorkflowStep> {
    alt((
        map(parse_comparison, |compare| {
            WorkflowStep::Comparison(compare)
        }),
        map(alphanumeric1, |goto: &str| {
            WorkflowStep::StraightTo(goto.to_string())
        }),
    ))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let _name = terminated(complete::alphanumeric1, complete::char('{'));
    let _steps = terminated(
        separated_list1(complete::char(','), parse_workflow_step),
        complete::char('}'),
    );

    map(tuple((_name, _steps)), |(name, steps)| Workflow {
        name: name.to_string(),
        steps,
    })(input)
}

fn parse_part(input: &str) -> IResult<&str, HashMap<char, u64>> {
    let variable = tuple((
        terminated(complete::anychar, complete::char('=')),
        complete::u64,
    ));
    let variables = separated_list1(complete::char(','), variable);
    let part = preceded(
        complete::char('{'),
        terminated(variables, complete::char('}')),
    );

    map(part, |vars| HashMap::from_iter(vars))(input)
}

fn is_accepted(part: &HashMap<char, u64>, workflows: &HashMap<String, &Workflow>) -> bool {
    let mut current_workflow: String = "in".to_string();

    while current_workflow != "A" && current_workflow != "R" {
        let workflow = workflows.get(&current_workflow).unwrap();
        for step in workflow.steps.iter() {
            match step {
                WorkflowStep::Comparison(c) => {
                    let v = part.get(&c.variable).unwrap();

                    if (c.operator == Operator::Gt && *v > c.value) || (c.operator == Operator::Lt && *v < c.value) {
                        current_workflow = c.goto.clone();
                        break;
                    }
                },
                WorkflowStep::StraightTo(to) => {
                    current_workflow = to.clone();
                    break;
                },
            }
        }
    }

    current_workflow == "A"
}

pub fn part_one(input: &str) -> Option<u32> {
    let chunks = input.split("\n\n").collect_vec();

    let (_, workflows) = separated_list1(newline, parse_workflow)(chunks[0]).unwrap();
    let workflow_map: HashMap<String, &Workflow> =
        workflows.iter().map(|w| (w.name.clone(), w)).collect();
    let (_, parts) = separated_list1(newline, parse_part)(chunks[1]).unwrap();

    let res: i64 = parts.iter().map(|p| {
        if is_accepted(p, &workflow_map) {
            p.values().map(|n| *n as i64).sum()
        } else {
            0
        }
    }).sum();

    Some(res as u32)
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
