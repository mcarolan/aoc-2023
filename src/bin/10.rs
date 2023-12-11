use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Heading {
    North,
    South,
    East,
    West,
}

impl Heading {
    fn offset(&self, point: &(i32, i32)) -> (i32, i32) {
        match self {
            Heading::North => (point.0, point.1 - 1),
            Heading::South => (point.0, point.1 + 1),
            Heading::East => (point.0 + 1, point.1),
            Heading::West => (point.0 - 1, point.1),
        }
    }
    fn opposite(&self) -> Heading {
        match self {
            Heading::North => Heading::South,
            Heading::South => Heading::North,
            Heading::East => Heading::West,
            Heading::West => Heading::East,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pipe {
    from: Heading,
    to: Heading,
}

#[derive(Debug)]
struct Input {
    tiles: HashMap<(i32, i32), Pipe>,
    ground: HashSet<(i32, i32)>,
    start: (i32, i32),
    width: i32,
    height: i32,
}

fn parse_pipe(input: char) -> Pipe {
    match input {
        '|' => Pipe {
            from: Heading::North,
            to: Heading::South,
        },
        '-' => Pipe {
            from: Heading::East,
            to: Heading::West,
        },
        'L' => Pipe {
            from: Heading::North,
            to: Heading::East,
        },
        'J' => Pipe {
            from: Heading::North,
            to: Heading::West,
        },
        '7' => Pipe {
            from: Heading::South,
            to: Heading::West,
        },
        'F' => Pipe {
            from: Heading::South,
            to: Heading::East,
        },
        _ => panic!(),
    }
}

fn valid_next(
    from: &(i32, i32),
    heading: Heading,
    map: &HashMap<(i32, i32), Pipe>,
) -> Option<((i32, i32), Heading)> {
    let to = heading.offset(from);
    let to_pipe = map.get(&to).unwrap();

    let new_heading = if to_pipe.from != heading && to_pipe.from != heading.opposite() {
        to_pipe.from
    } else if to_pipe.to != heading && to_pipe.to != heading.opposite() {
        to_pipe.to
    } else {
        heading
    };

    Some((to, new_heading))
}

fn furthest(start: (i32, i32), map: &HashMap<(i32, i32), Pipe>) -> Option<i32> {
    let mut counter = 0;
    let mut at = start;
    let mut heading = map.get(&start).unwrap().to;

    loop {
        match valid_next(&at, heading, map) {
            Some((to, new_heading)) => {
                if to == start {
                    break;
                }
                at = to;
                heading = new_heading;
                counter += 1;
            }
            None => break,
        }
    }

    if counter == 0 {
        None
    } else {
        Some((counter + 1) / 2)
    }
}

fn loop_path(start: (i32, i32), map: &HashMap<(i32, i32), Pipe>) -> HashMap<(i32, i32), Pipe> {
    let mut at = start;
    let mut heading = map.get(&start).unwrap().to;

    let mut result: HashMap<(i32, i32), Pipe> = HashMap::new();
    result.insert(at, *map.get(&at).unwrap());

    loop {
        match valid_next(&at, heading, map) {
            Some((to, new_heading)) => {
                if to == start {
                    break;
                }
                at = to;
                result.insert(at, *map.get(&at).unwrap());
                heading = new_heading;
            }
            None => break,
        }
    }

    result
}

fn parse_input(input: &str) -> Input {
    let mut start: Option<(i32, i32)> = None;
    let mut y = 0;
    let mut x = 0;

    let mut ground: HashSet<(i32, i32)> = HashSet::new();
    let mut map: HashMap<(i32, i32), Pipe> = HashMap::new();

    for line in input.lines() {
        x = 0;
        for char in line.chars() {
            match char {
                'S' => start = Some((x, y)),
                '.' => {
                    ground.insert((x, y));
                    ()
                }
                _ => {
                    map.insert((x, y), parse_pipe(char));
                    ()
                }
            }

            x += 1;
        }

        y += 1;
    }

    Input {
        start: start.unwrap(),
        tiles: map,
        height: y,
        width: x,
        ground,
    }
}

fn adjacent_pipes(start: (i32, i32), map: &HashMap<(i32, i32), Pipe>) -> Vec<Heading> {
    let headings = vec![Heading::North, Heading::South, Heading::East, Heading::West];

    headings
        .into_iter()
        .filter(|h| {
            map.get(&h.offset(&start)).is_some_and(|p| {
                p.from == h.opposite() || p.to == h.opposite()
            })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse_input(input);

    let mut map = input.tiles.clone();

    let adjacent_headings = adjacent_pipes(input.start, &map);
    let start_pipe = Pipe {
        from: *adjacent_headings.get(0).unwrap(),
        to: *adjacent_headings.get(1).unwrap(),
    };

    map.insert(input.start, start_pipe);

    furthest(input.start, &map)
}

fn double_tuple(tup: &(i32, i32)) -> (i32, i32) {
    (2 * tup.0, 2 * tup.1)
}

fn double_res(tiles: &HashMap<(i32, i32), Pipe>) -> HashSet<(i32, i32)> {
    let res: HashSet<(i32, i32)> = tiles
        .iter()
        .flat_map(|(pos, pipe)| {
            vec![
                pipe.from.offset(&double_tuple(pos)),
                double_tuple(pos),
                pipe.to.offset(&double_tuple(pos)),
            ]
        })
        .collect();

    res
}

fn clamp(pos: &(i32, i32), width: i32, height: i32) -> Option<(i32, i32)> {
    if pos.0 > width || pos.1 > height || pos.0 < 0 || pos.1 < 0 {
        None
    } else {
        Some(*pos)
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse_input(input);

    let mut map = input.tiles.clone();
    map.insert(
        input.start,
        Pipe {
            from: Heading::South,
            to: Heading::East,
        },
    );

    let longest_loop = loop_path(input.start, &map);
    let path_points = double_res(&longest_loop);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, i32)> = vec![];

    for x in 0..2 * input.width {
        queue.push((x, 0));
        queue.push((x, (2 * input.height) - 1));
    }
    for y in 0..2 * input.height {
        queue.push((0, y));
        queue.push(((2 * input.width) - 1, y));
    }

    while let Some(at) = queue.pop() {
        if visited.contains(&at) || path_points.contains(&at) {
            continue;
        }
        visited.insert(at);

        let neighbours: Vec<(i32, i32)> = vec![
            Heading::North.offset(&at),
            Heading::South.offset(&at),
            Heading::East.offset(&at),
            Heading::West.offset(&at),
        ]
        .iter()
        .flat_map(|p| clamp(p, 2 * input.width, 2 * input.height))
        .collect();

        for neighbour in neighbours {
            queue.push(neighbour);
        }
    }

    let mut counter = 0;

    let longest: HashSet<&(i32, i32)> = longest_loop.keys().collect();

    for x in 0..input.width {
        for y in 0..input.height {
            if !longest.contains(&(x, y)) && !visited.contains(&(2 * x, 2 * y)) {
                counter += 1;
            }
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
