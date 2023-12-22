use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(21);

fn neighbours(
    pos: (i64, i64),
    bound_row: i64,
    bound_col: i64,
    step: u64,
    rocks: &HashSet<(i64, i64)>,
) -> Vec<((i64, i64), u64)> {
    vec![
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ]
    .into_iter()
    .filter_map(|p| {
        if p.0 >= 0 && p.0 < bound_row && p.1 >= 0 && p.1 < bound_col && !rocks.contains(&p) {
            Some((p, step + 1))
        }
        else {
            None
        }
    })
    .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut row = 0;
    let mut col = 0;

    let mut start: (i64, i64) = (0, 0);
    let mut rocks: HashSet<(i64, i64)> = HashSet::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        col = 0;
        for c in line.chars() {
            if c == 'S' {
                start = (row, col);
            } else if c == '#' {
                rocks.insert((row, col));
            }
            col += 1;
        }
        row += 1;
    }

    let target = 64;
    let mut reached: HashSet<(i64, i64)> = HashSet::new();
    let mut q: Vec<((i64, i64), u64)> = vec![(start, 0)];

    let mut visited: HashSet<((i64, i64), u64)> = HashSet::new();

    while let Some((pos, steps)) = q.pop() {
        if steps == target {
            reached.insert(pos);
        }
        else if visited.insert((pos, steps)) {
            q.extend(neighbours(pos, row, col, steps, &rocks));
        }
    } 

    Some(reached.len() as u32)
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
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
