use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use rangemap::RangeInclusiveMap;

advent_of_code::solution!(22);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
struct Brick {
    min: Point3D,
    max: Point3D,
}

impl Brick {
    fn new(start: &Point3D, end: &Point3D) -> Brick {
        Brick {
            min: Point3D {
                x: start.x.min(end.x),
                y: start.y.min(end.y),
                z: start.z.min(end.z),
            },
            max: Point3D {
                x: start.x.max(end.x),
                y: start.y.max(end.y),
                z: start.z.max(end.z),
            },
        }
    }
    fn overlaps(&self, other: &Brick) -> bool {
        let x_overlap = self.min.x <= other.max.x && self.max.x >= other.min.x;
        let y_overlap = self.min.y <= other.max.y && self.max.y >= other.min.y;
        let z_overlap = self.min.z <= other.max.z && self.max.z >= other.min.z;
        x_overlap && y_overlap && z_overlap
    }
    fn shift_down(&self) -> Brick {
        let min = Point3D {
            z: self.min.z - 1,
            ..self.min
        };
        let max = Point3D {
            z: self.max.z - 1,
            ..self.max
        };
        Brick::new(&min, &max)
    }
}

fn parse_point(str: &str) -> IResult<&str, Point3D> {
    let parts = tuple((
        terminated(complete::i32, complete::char(',')),
        terminated(complete::i32, complete::char(',')),
        complete::i32,
    ));

    map(parts, |(x, y, z)| Point3D { x, y, z })(str)
}

fn parse_brick(str: &str) -> IResult<&str, Brick> {
    let parts = tuple((terminated(parse_point, complete::char('~')), parse_point));

    map(parts, |(start, end)| Brick::new(&start, &end))(str)
}

fn plot(bricks: &Vec<(usize, Brick)>) -> HashMap<Point3D, usize> {
    HashMap::from_iter(bricks.iter().flat_map(|(id, brick)| {
        let mut points: Vec<(Point3D, usize)> = Vec::new();
        for x in brick.min.x..=brick.max.x {
            for y in brick.min.y..=brick.max.y {
                for z in brick.min.z..=brick.max.z {
                    points.push((Point3D { x, y, z }, *id));
                }
            }
        }
        points
    }))
}

fn translate_to_letter(number: usize) -> char {
    // Assuming the range is from 0 to 25
    (b'A' + number as u8).into()
}

fn viz(plot: &HashMap<Point3D, usize>, is_x: bool) {
    let max_c = plot
        .keys()
        .map(|p| if is_x { p.x } else { p.y })
        .max()
        .unwrap();
    let max_z = plot.keys().map(|p| p.z).max().unwrap();

    if is_x {
        println!("x");
    } else {
        println!("y");
    }

    for c in 0..=max_c {
        print!("{}", c);
    }
    println!();

    for z in (0..=max_z).rev() {
        for c in 0..=max_c {
            let visible = plot
                .iter()
                .filter_map(|(p, id)| {
                    let component_matches = (if is_x { p.x } else { p.y }) == c;
                    if component_matches && p.z == z {
                        Some(id)
                    } else {
                        None
                    }
                })
                .dedup()
                .collect_vec();

            if visible.len() > 1 {
                print!("?");
            } else {
                if let Some(visible) = visible.first() {
                    let letter = translate_to_letter(**visible);
                    print!("{}", letter);
                } else {
                    print!(".");
                }
            }
        }
        if z == max_z / 2 {
            println!(" {} z", z);
        } else {
            println!(" {}", z);
        }
    }
}

fn fall(bricks: &Vec<(usize, Brick)>) -> u32 {
    let mut fall_order: Vec<(usize, &Brick)> = Vec::new();

    for (id, brick) in bricks {
        fall_order.push((*id, brick));
    }

    fall_order.sort_by_key(|(_, b)| b.min.z);

    let mut res: Vec<(usize, Brick)> = Vec::new();
    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supported: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (id, brick) in fall_order {
        let mut shifted = brick.clone();

        while shifted.min.z >= 2 {
            let test = shifted.shift_down();
            let overlap: HashSet<usize> = res
                .iter()
                .filter_map(|(id, b)| {
                    if test.overlaps(b) {
                        Some(*id)
                    } else {
                        None
                    }
                })
                .collect();
            
            if !overlap.is_empty() {
                for support in overlap {
                    let mut s: HashSet<usize> = HashSet::new();
                    if let Some(current) = supports.get(&support) {
                        s.extend(current);
                    }
                    s.insert(id);
                    supports.insert(support, s);

                    let mut s: HashSet<usize> = HashSet::new();
                    if let Some(current) = supported.get(&id) {
                        s.extend(current);
                    }
                    s.insert(support);
                    supported.insert(id, s);
                }
                break;
            }

            shifted = test;
        }
        res.push((id, shifted));
    }
    let mut res = 0;

    for (id, brick) in bricks {
        if let Some(above) = supports.get(id) {
            let mut can_be_disintegrated = true;
            for b in above.iter() {
                if let Some(supporters) = supported.get(&b) {
                    if supporters.len() == 1 {
                        can_be_disintegrated = false;
                        break;
                    }
                }
            }
            if can_be_disintegrated {
                res += 1;
            }
        } else {
            res += 1;
            continue;
        }
    }

    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, input) = separated_list1(newline, parse_brick)(input).unwrap();
    let input = input.into_iter().enumerate().collect_vec();
    Some(fall(&input))
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
