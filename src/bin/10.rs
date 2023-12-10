use std::collections::HashMap;

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
    start: (i32, i32),
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
    let to_pipe_opt = map.get(&to);
    if to_pipe_opt.is_none() {
        return None;
    }
    let to_pipe = to_pipe_opt.unwrap();

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

fn parse_input(input: &str) -> Input {
    let mut start: Option<(i32, i32)> = None;
    let mut y = 0;

    let mut map: HashMap<(i32, i32), Pipe> = HashMap::new();

    for line in input.lines() {
        let mut x = 0;
        for char in line.chars() {
            match char {
                'S' => start = Some((x, y)),
                '.' => (),
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
        tiles: map
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse_input(input);

    let mut map = input.tiles.clone();

    let possible_starts = vec![
        Pipe {
            from: Heading::North,
            to: Heading::South,
        },
        Pipe {
            from: Heading::East,
            to: Heading::West,
        },
        Pipe {
            from: Heading::North,
            to: Heading::East,
        },
        Pipe {
            from: Heading::North,
            to: Heading::West,
        },
        Pipe {
            from: Heading::South,
            to: Heading::West,
        },
        Pipe {
            from: Heading::South,
            to: Heading::East,
        },
    ];

    let mut results: Vec<Option<i32>> = Vec::new();

    for start_pipe in possible_starts {
        map.insert(input.start, start_pipe);
        let result = furthest(input.start, &map);
        results.push(result);
    }

    results.into_iter().flatten().max()
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
