use itertools::Itertools;

advent_of_code::solution!(15);

fn hash(input: &String) -> i64 {
    let mut current_value: i64 = 0;

    for c in input.chars() {
        let ascii = c as u8;
        current_value += ascii as i64;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().join("").split(",").map(|s|s.to_string()).collect_vec()
}

pub fn part_one(input: &str) -> Option<i64> {
    let steps  = parse_input(input);

    Some(steps.iter().map(|s| hash(s)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let result = hash(&"HASH".to_string());
        assert_eq!(result, 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
