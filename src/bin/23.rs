use std::collections::{HashMap, HashSet};

use advent_of_code::solution;

advent_of_code::solution!(23);

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

fn parse_slope_direction(c: char) -> Direction {
    match c {
        '>' => Direction::Right,
        '<' => Direction::Left,
        '^' => Direction::Up,
        'v' => Direction::Down,
        _ => panic!(),
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point {
    row: i32,
    col: i32,
}

struct Map {
    paths: HashSet<Point>,
    slopes: HashMap<Point, Direction>,
    cols: i32,
    rows: i32,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}x{} map", self.cols, self.rows)?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let p = Point { row, col };
                if self.paths.contains(&p) {
                    write!(f, ".")?;
                } else if let Some(direction) = self.slopes.get(&p) {
                    write!(f, "{:?}", direction)?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    fn find_start(&self) -> Point {
        for col in 0..self.cols {
            let p = Point { row: 0, col };
            if self.paths.contains(&p) {
                return p;
            }
        }
        panic!()
    }

    fn neighbours(&self, point: Point, ignore_slope_direction: bool) -> Vec<Point> {
        let to_check = vec![
            (
                Point {
                    row: point.row + 1,
                    col: point.col,
                },
                Direction::Down,
            ),
            (
                Point {
                    row: point.row - 1,
                    col: point.col,
                },
                Direction::Up,
            ),
            (
                Point {
                    row: point.row,
                    col: point.col - 1,
                },
                Direction::Left,
            ),
            (
                Point {
                    row: point.row,
                    col: point.col + 1,
                },
                Direction::Right,
            ),
        ];

        to_check
            .into_iter()
            .filter_map(|(p, direction)| {
                if p.row < 0 || p.row >= self.rows || p.col < 0 || p.col >= self.cols {
                    None
                } else if self.paths.contains(&p) {
                    Some(p)
                } else if let Some(slope_direction) = self.slopes.get(&p) {
                    if ignore_slope_direction || *slope_direction == direction { Some(p) } else { None }
                }
                else {
                    None
                }
            })
            .collect()
    }
}

fn parse_map(input: &str) -> Map {
    let mut row = 0;
    let mut col = 0;

    let mut paths = HashSet::new();
    let mut slopes = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        col = 0;
        for c in line.chars() {
            if c == '.' {
                paths.insert(Point { row, col });
            } else if c != '#' {
                slopes.insert(Point { row, col }, parse_slope_direction(c));
            }
            col += 1;
        }

        row += 1;
    }

    Map {
        paths,
        slopes,
        cols: col,
        rows: row,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);
    let start = map.find_start();

    let mut q: Vec<(Point, HashSet<Point>)> = vec![(start.clone(), HashSet::from_iter(vec![start]))];

    let mut longest_walk = 0;

    while let Some((point, visited)) = q.pop() {
        if point.row == map.rows - 1 {
            longest_walk = longest_walk.max(visited.len() as u32 - 1);
            continue;
        }

        for to_visit in map.neighbours(point, false) {
            if !visited.contains(&to_visit) {
                let mut new_visited = visited.clone();
                new_visited.insert(to_visit.clone());
                q.push((to_visit, new_visited))
            }
        }
    }
    
    Some(longest_walk)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);
    let start = map.find_start();

    let mut q: Vec<(Point, HashSet<Point>)> = vec![(start.clone(), HashSet::from_iter(vec![start]))];

    let mut longest_walk = 0;

    while let Some((point, visited)) = q.pop() {
        if point.row == map.rows - 1 {
            longest_walk = longest_walk.max(visited.len() as u32 - 1);
            continue;
        }

        for to_visit in map.neighbours(point, true) {
            if !visited.contains(&to_visit) {
                let mut new_visited = visited.clone();
                new_visited.insert(to_visit.clone());
                q.push((to_visit, new_visited))
            }
        }
    }
    
    Some(longest_walk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
