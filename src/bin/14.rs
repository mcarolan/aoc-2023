use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Input {
    round_rocks: HashMap<u64, HashSet<u64>>,
    square_rocks: HashMap<u64, HashSet<u64>>,
    cols: u64,
    rows: u64
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.round_rocks.get(&row).unwrap().contains(&col) {
                    write!(f, "O")?;
                }
                else if self.square_rocks.get(&row).unwrap().contains(&col) {
                    write!(f, "#")?;
                }
                else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

impl Input {
    fn tilt(&self) -> Input {
        let mut round_rocks: HashMap<u64, HashSet<u64>> = HashMap::new();

        for row in 0..self.rows {
            round_rocks.insert(row, HashSet::new());
            for col in self.round_rocks.get(&row).unwrap() {
                let move_to_opt = (0..row).rev().find(|r| {
                    self.square_rocks.get(r).unwrap().contains(&col) ||
                    round_rocks.get(r).unwrap().contains(&col)
                }).map(|r| r + 1);

                round_rocks.get_mut(&move_to_opt.unwrap_or(0)).unwrap().insert(*col);
            }
        }
        
        Input { round_rocks, square_rocks: self.square_rocks.clone(), cols: self.cols, rows: self.rows }
    }

    fn rotate(&self) -> Input {
        let mut square_rocks: HashMap<u64, HashSet<u64>> = HashMap::new();
        let mut round_rocks: HashMap<u64, HashSet<u64>> = HashMap::new();

        for col in 0..self.cols {
            square_rocks.insert(col, HashSet::new());
            round_rocks.insert(col, HashSet::new());
        }

        for row in 0..self.rows {
            for col in self.round_rocks.get(&row).unwrap() {
                round_rocks.get_mut(col).unwrap().insert(&self.rows - 1 -row);
            }
            for col in self.square_rocks.get(&row).unwrap() {
                square_rocks.get_mut(col).unwrap().insert(&self.rows - 1 -row);
            }
        }

        Input { round_rocks, square_rocks, cols: self.rows, rows: self.cols }
    }

    fn cycle(&self) -> Input {
        self.tilt().rotate().tilt().rotate().tilt().rotate().tilt().rotate()
    }
}

fn parse_input(input: &str) -> Input {
    let mut row = 0;
    let mut col = 0;

    let mut round_rocks: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut square_rocks: HashMap<u64, HashSet<u64>> = HashMap::new();

    for line in input.lines() {
        col = 0;
        let line = line.trim();

        if line.len() == 0 {
            continue;
        }


        let mut line_round_rocks: HashSet<u64> = HashSet::new();
        let mut line_square_rocks: HashSet<u64> = HashSet::new();

        for c in line.chars() {
            if c == '#' {
                line_square_rocks.insert(col);
            }
            else if c == 'O' {
                line_round_rocks.insert(col);
            }
            col += 1;
        }

        square_rocks.insert(row, line_square_rocks);
        round_rocks.insert(row, line_round_rocks);

        row += 1;
    }

    Input { round_rocks, square_rocks, rows: row, cols: col }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let north = input.tilt();

    Some((0..north.rows).map(|r| {
        let row = north.round_rocks.get(&r).unwrap();
        row.len() as u64 * (north.rows - r)
    }).sum())
}

fn map_key(m: HashMap<u64, HashSet<u64>>) -> Vec<(u64, Vec<u64>)> {
    let mut res = m.into_iter().map(|(k, v)| {
        let mut v : Vec<u64> =  v.into_iter().collect();
        v.sort();
        (k, v)
    }).collect_vec();
    res.sort();
    res
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = parse_input(input);

    let mut cache: HashMap<Vec<(u64, Vec<u64>)>, i32> = HashMap::new();

    cache.insert(map_key(input.round_rocks.clone()), 0);
    let mut remaining = 0;

    for i in 1..1000000000 {
        input = input.cycle();
        let key = map_key(input.round_rocks.clone());

        if let Some(j) = cache.get(&key) {
            remaining = (1000000000 - i) % (i - j);
            break;
        }
        cache.insert(key, i);
    }

    for _ in 0..remaining {
        input = input.cycle();
    }

    Some((0..input.rows).map(|r| {
        let row = input.round_rocks.get(&r).unwrap();
        row.len() as u64 * (input.rows - r)
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
