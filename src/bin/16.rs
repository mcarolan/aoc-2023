use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(16);

fn parse_input(input: &str) -> (HashMap<(i64, i64), char>, i64, i64) {
    let mut res: HashMap<(i64, i64), char> = HashMap::new();

    let mut row = 0;
    let mut col = 0;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        col = 0;
        for c in line.chars() {
            res.insert((row, col), c);
            col += 1;
        }

        row += 1;
    }

    (res, row, col)
}

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
    fn forward_slash_mirror(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn back_slash_mirror(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

enum StepOutcome {
    OutOfBounds,
    Moved,
    NewBeams(Vec<Beam>),
}

#[derive(Clone)]
struct Beam {
    pos: (i64, i64),
    direction: Direction
}

impl Beam {
    fn step(&mut self, grid: &HashMap<(i64, i64), char>) -> StepOutcome {
        match grid.get(&self.pos) {
            Some('\\') => {
                self.direction = self.direction.back_slash_mirror();
                self.pos = self.direction.offset(self.pos);
                StepOutcome::Moved
            }
            Some('/') => {
                self.direction = self.direction.forward_slash_mirror();
                self.pos = self.direction.offset(self.pos);
                StepOutcome::Moved
            }
            Some('-') if self.direction == Direction::Up || self.direction == Direction::Down => {
                let beams = vec![
                    Beam {
                        pos: Direction::Left.offset(self.pos),
                        direction: Direction::Left
                    },
                    Beam {
                        pos: Direction::Right.offset(self.pos),
                        direction: Direction::Right
                    },
                ];
                StepOutcome::NewBeams(beams)
            }
            Some('|')
                if self.direction == Direction::Left || self.direction == Direction::Right =>
            {
                let beams = vec![
                    Beam {
                        pos: Direction::Up.offset(self.pos),
                        direction: Direction::Up
                    },
                    Beam {
                        pos: Direction::Down.offset(self.pos),
                        direction: Direction::Down
                    },
                ];
                StepOutcome::NewBeams(beams)
            }
            Some('.') | Some('-') | Some('|') => {
                self.pos = self.direction.offset(self.pos);
                StepOutcome::Moved
            }
            Some(_) => panic!(),
            None => StepOutcome::OutOfBounds,
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let (grid, _rows, _cols) = parse_input(input);

    let mut beams = vec![Beam {
        pos: (0, 0),
        direction: Direction::Right
    }];
    let mut energised: HashSet<(i64, i64)> = HashSet::new();
    let mut visited: HashSet<((i64, i64), Direction)> = HashSet::new();

    while !beams.is_empty() {

        let mut hit_list: HashSet<usize> = HashSet::new();
        let mut add_list: Vec<Beam> = Vec::new();

        for (i, beam) in beams.iter_mut().enumerate() {
            if grid.get(&beam.pos).is_none() {
                hit_list.insert(i);
                continue;
            }

            energised.insert(beam.pos);

            if !visited.insert((beam.pos, beam.direction)) {
                hit_list.insert(i);
                continue;
            }

            match beam.step(&grid) {
                StepOutcome::OutOfBounds => {
                    hit_list.insert(i);
                }
                StepOutcome::Moved => (),
                StepOutcome::NewBeams(bs) => {
                    add_list.extend(bs.clone());
                    hit_list.insert(i);
                }
            }
        }

        hit_list.iter().sorted().rev().for_each(|i| {
            beams.remove(*i);
        });
        beams.extend(add_list);
    }

    Some(energised.len() as i64)
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
