use iset::IntervalMap;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space0, space1},
    combinator::{map, value},
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use rangemap::RangeMap;

advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MapEntry {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

struct SourceDestinationMap {
    map: RangeMap<u64, MapEntry>,
}

impl SourceDestinationMap {
    fn new(entries: Vec<MapEntry>) -> SourceDestinationMap {
        let mut range_map: RangeMap<u64, MapEntry> = RangeMap::new();

        for entry in entries {
            range_map.insert(
                entry.source_range_start..(entry.source_range_start + entry.range_length),
                entry,
            );
        }

        SourceDestinationMap { map: range_map }
    }

    fn translate(&self, value: u64) -> u64 {
        let entry_opt = self.map.get(&value);

        match entry_opt {
            Some(entry) => entry.destination_range_start + (value - entry.source_range_start),
            None => value,
        }
    }
}

fn parse_spaced_number(input: &str) -> IResult<&str, u64> {
    preceded(space0, terminated(complete::u64, space0))(input)
}

fn parse_map_entry(input: &str) -> IResult<&str, MapEntry> {
    let parts = tuple((
        parse_spaced_number,
        parse_spaced_number,
        parse_spaced_number,
    ));

    map(
        parts,
        |(destination_range_start, source_range_start, range_length)| MapEntry {
            destination_range_start,
            source_range_start,
            range_length,
        },
    )(input)
}

struct Input {
    seeds: Vec<u64>,
    seed_to_soil: SourceDestinationMap,
    soil_to_fertilizer: SourceDestinationMap,
    fertilizer_to_water: SourceDestinationMap,
    water_to_light: SourceDestinationMap,
    light_to_temperature: SourceDestinationMap,
    temperature_to_humidity: SourceDestinationMap,
    humidity_to_location: SourceDestinationMap,
}

fn parse_source_destination_map(input: &str) -> IResult<&str, SourceDestinationMap> {
    let entries_parser = many1(terminated(parse_map_entry, newline));

    map(entries_parser, |entries| SourceDestinationMap::new(entries))(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let parts = tuple((
        value((), tuple((tag("seeds:"), space0))),
        many1(parse_spaced_number),
        value(
            (),
            tuple((newline, newline, tag("seed-to-soil map:"), newline)),
        ),
        parse_source_destination_map,
        value(
            (),
            tuple((newline, tag("soil-to-fertilizer map:"), newline)),
        ),
        parse_source_destination_map,
        value(
            (),
            tuple((newline, tag("fertilizer-to-water map:"), newline)),
        ),
        parse_source_destination_map,
        value((), tuple((newline, tag("water-to-light map:"), newline))),
        parse_source_destination_map,
        value(
            (),
            tuple((newline, tag("light-to-temperature map:"), newline)),
        ),
        parse_source_destination_map,
        value(
            (),
            tuple((newline, tag("temperature-to-humidity map:"), newline)),
        ),
        parse_source_destination_map,
        value(
            (),
            tuple((newline, tag("humidity-to-location map:"), newline)),
        ),
        parse_source_destination_map,
    ));
    map(
        parts,
        |(
            _1,
            seeds,
            _2,
            seed_to_soil,
            _3,
            soil_to_fertilizer,
            _4,
            fertilizer_to_water,
            _5,
            water_to_light,
            _6,
            light_to_temperature,
            _7,
            temperature_to_humidity,
            _8,
            humidity_to_location,
        )| {
            Input {
                seeds,
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            }
        },
    )(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    println!("input is length {}", input.len());
    let (_, parsed) = parse_input(input).unwrap();

    let locations = parsed.seeds.iter().map(|seed| {
        let soil = parsed.seed_to_soil.translate(*seed);
        let fertilizer = parsed.soil_to_fertilizer.translate(soil);
        let water = parsed.fertilizer_to_water.translate(fertilizer);
        let light = parsed.water_to_light.translate(water);
        let temp = parsed.light_to_temperature.translate(light);
        let humid = parsed.temperature_to_humidity.translate(temp);
        let location = parsed.humidity_to_location.translate(humid);
        location
    });

    locations.min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, parsed) = parse_input(input).unwrap();

    let mut result: u64 = u64::MAX;

    for i in (0..parsed.seeds.len()).step_by(2) {
        let start = *parsed.seeds.get(i).unwrap();
        let length = *parsed.seeds.get(i + 1).unwrap();

        for i in 0..length {
            let seed = start + i;

            let soil = parsed.seed_to_soil.translate(seed);
            let fertilizer = parsed.soil_to_fertilizer.translate(soil);
            let water = parsed.fertilizer_to_water.translate(fertilizer);
            let light = parsed.water_to_light.translate(water);
            let temp = parsed.light_to_temperature.translate(light);
            let humid = parsed.temperature_to_humidity.translate(temp);
            let location = parsed.humidity_to_location.translate(humid);
            result = result.min(location);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
