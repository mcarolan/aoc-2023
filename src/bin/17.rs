use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

advent_of_code::solution!(17);
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
    fn rot(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn anti_rot(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn parse_input(input: &str) -> (HashMap<(i64, i64), i64>, i64, i64) {
    let mut res: HashMap<(i64, i64), i64> = HashMap::new();

    let mut row = 0;
    let mut col = 0;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        col = 0;
        for c in line.chars() {
            res.insert((row, col), c.to_digit(10).unwrap() as i64);
            col += 1;
        }

        row += 1;
    }

    (res, row, col)
}

fn neighbours(
    grid: &HashMap<(i64, i64), i64>,
    min_steps: u32,
    max_steps: u32
) -> impl FnMut(
    &((i64, i64), Option<Direction>, u32),
) -> Vec<(((i64, i64), Option<Direction>, u32), usize)>
       + '_ {
    move |(pos, dir, steps)| {
        let mut res: Vec<((i64, i64), Option<Direction>, u32)> = Vec::new();
        let pos = *pos;
        let steps = *steps;

        if let Some(dir) = *dir {
            if (steps as i64) < (min_steps as i64) - 1 {
                let straight = dir.offset(pos);
                res.push((straight, Some(dir), steps + 1));
            }
            else {
                if steps < max_steps - 1 {
                    let straight = dir.offset(pos);
                    res.push((straight, Some(dir), steps + 1));
                }
                let rot = dir.rot();
                let anti_rot = dir.anti_rot();
    
                res.push((rot.offset(pos), Some(rot), 0));
                res.push((anti_rot.offset(pos), Some(anti_rot), 0));
            }
           
        } else {
            res.push((Direction::Right.offset(pos), Some(Direction::Right), 0));
            res.push((Direction::Down.offset(pos), Some(Direction::Down), 0));
        }

        res.into_iter()
            .flat_map(|v| grid.get(&v.0).map(|cost| (v, *cost as usize)))
            .collect_vec()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input, rows, cols) = parse_input(input);

    let start: ((i64, i64), Option<Direction>, u32) = ((0, 0), None, 0);

    dijkstra(&start, neighbours(&input, 0, 3), |(pos, _, _)| {
        *pos == (rows - 1, cols - 1)
    })
    .map(|(_, c)| c as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (input, rows, cols) = parse_input(input);

    let start: ((i64, i64), Option<Direction>, u32) = ((0, 0), None, 0);

    dijkstra(&start, neighbours(&input, 4, 10), |(pos, _, steps)| {
        *pos == (rows - 1, cols - 1) && *steps >= 3
    }).map(|(_, c)| c as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_example_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(71));
    }
}
