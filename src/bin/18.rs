use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(PartialEq, Copy, Clone, Debug, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self, pos: (i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }
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

fn draw(grid: &HashSet<(i64, i64)>) {
    let rows_min = *grid.into_iter().map(|(r, _)| r).min().unwrap();
    let cols_min = *grid.into_iter().map(|(_, c)| c).min().unwrap();

    let rows_max = *grid.into_iter().map(|(r, _)| r).max().unwrap();
    let cols_max = *grid.into_iter().map(|(_, c)| c).max().unwrap();

    for r in rows_min..=rows_max {
        for c in cols_min..=cols_max {
            if grid.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn neighbours(pos: (i64, i64)) -> Vec<(i64, i64)> {
    vec![
        Direction::Up.offset(pos),
        Direction::Down.offset(pos),
        Direction::Left.offset(pos),
        Direction::Right.offset(pos),
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let mut grid: HashSet<(i64, i64)> = HashSet::new();

    let mut current_pos: (i64, i64) = (0, 0);

    for (direction, distance) in input.iter() {
        for _ in 0..*distance {
            current_pos = direction.offset(current_pos);
            grid.insert(current_pos);
        }
    }
    let mut q: Vec<(i64, i64)> = Vec::new();

    current_pos = (0, 0);
    for (direction, distance) in input.iter() {
        let right_of = match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };

        for _ in 0..*distance {
            current_pos = direction.offset(current_pos);
            q.push(right_of.offset(current_pos));
        }
    }

    while let Some(pos) = q.pop() {
        if grid.contains(&pos) {
            continue;
        }

        grid.insert(pos);
        for neighbour in neighbours(pos) {
            q.push(neighbour);
        }
    }
    
    Some(grid.len() as u32)
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
