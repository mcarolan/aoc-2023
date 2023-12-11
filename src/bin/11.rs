use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug)]
struct Input {
    galaxies: HashSet<(i64, i64)>,
    rows: i64,
    cols: i64,
}

fn parse_input(input: &str) -> Input {
    let mut row = 0;
    let mut col = 0;
    let mut galaxies: HashSet<(i64, i64)> = HashSet::new();

    for line in input.lines() {
        col = 0;
        for c in line.chars() {
            if c == '#' {
                galaxies.insert((row, col));
            }
            col += 1;
        }
        row += 1;
    }

    Input {
        galaxies,
        rows: row,
        cols: col,
    }
}

fn manhattan_distance(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    (a.1 - b.1).abs() + (a.0 - b.0).abs()
}

fn solve(input: &str, factor: i64) -> i64 {
    let input = parse_input(input);

    let mut cols_to_expand: HashSet<i64> = (0..input.cols).collect();
    let mut rows_to_expand: HashSet<i64> = (0..input.rows).collect();

    input.galaxies.iter().for_each(|g| {
        rows_to_expand.remove(&g.0);
        cols_to_expand.remove(&g.1);
    });

    let expanded_galaxies: HashSet<(i64, i64)> = input
        .galaxies
        .iter()
        .map(|(row, col)| {
            let rows_expanded_before = rows_to_expand.iter().filter(|rn| *rn < row).count() as i64;
            let cols_expanded_before = cols_to_expand.iter().filter(|cn| *cn < col).count() as i64;

            (
                row + (rows_expanded_before * factor) - rows_expanded_before,
                col + (cols_expanded_before * factor) - cols_expanded_before,
            )
        })
        .collect();

    expanded_galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            manhattan_distance(a, b)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, 1000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
