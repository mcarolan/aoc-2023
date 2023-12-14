use std::collections::{HashMap, HashSet};

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
    fn tilt_north(&self) -> Input {
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
    let north = input.tilt_north();

    Some((0..north.rows).map(|r| {
        let row = north.round_rocks.get(&r).unwrap();
        row.len() as u64 * (north.rows - r)
    }).sum())
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
