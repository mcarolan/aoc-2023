use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(PartialEq, Copy, Clone, Debug, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_direction(input: &str) -> Direction {
    match input {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> Vec<(Direction, u64)> {
    let mut res: Vec<(Direction, u64)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts = line.split_ascii_whitespace().collect_vec();
        res.push((
            parse_direction(parts.get(0).unwrap()),
            parts.get(1).unwrap().parse().unwrap(),
        ));
    }

    res
}

fn parse_input2(input: &str) -> Vec<(Direction, u64)> {
    let mut res: Vec<(Direction, u64)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts = line.split_ascii_whitespace().collect_vec();
        let hex = parts.get(2).unwrap().replace("(#", "").replace(")", "");

        let distance: u64 = u64::from_str_radix(hex.chars().take(5).collect::<String>().as_str(), 16).unwrap();

        let direction = match hex.chars().last() {
            Some('0') => Direction::Right,
            Some('1') => Direction::Down,
            Some('2') => Direction::Left,
            Some('3') => Direction::Up,
            _ => todo!(),
        };
        
        res.push((direction, distance));
    }

    res
}

fn vertices(instructions: &Vec<(Direction, u64)>) -> Vec<(i64, i64)> {
    let mut res = Vec::new();
    
    let mut current_pos: (i64, i64) = (0, 0);
    
    for (direction, distance) in instructions.iter() {
        let distance = *distance as i64;
        current_pos = match direction {
            Direction::Up => (current_pos.0 - distance, current_pos.1),
            Direction::Down => (current_pos.0 + distance, current_pos.1),
            Direction::Left => (current_pos.0, current_pos.1 - distance),
            Direction::Right => (current_pos.0, current_pos.1 + distance),
        };
        res.push(current_pos);
    }
    res
}

fn solve(instructions: &Vec<(Direction, u64)>) -> i64 {
    let vertices = vertices(&instructions);
    let vertex_count = vertices.len();

    let n: i64 = vertices.iter().enumerate().map(|(i, (row, col))| {
        let next_index = if i == vertex_count - 1 { 0 } else { i + 1 };
        let next = vertices.get(next_index).unwrap();

        (*col * next.0) - (next.1 * row) 
    }).sum();

    let perimeter: i64 = instructions.iter().map(|(_, d)| *d as i64).sum();


    (n.abs() / 2) + (perimeter / 2 + 1)
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse_input(input);
    Some(solve(&input))
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse_input2(input);
    Some(solve(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
