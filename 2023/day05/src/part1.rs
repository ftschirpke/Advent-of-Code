use aoclib::AocError;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64 as u64_parser},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

pub struct Almanac {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn location_for_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        self.humidity_to_location.map(humidity)
    }
}

pub struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn map(&self, value: u64) -> u64 {
        for mapping in self.mappings.iter() {
            if mapping.is_mapped(value) {
                return mapping.map(value);
            }
        }
        value
    }
}

struct Mapping {
    src_range_start: u64,
    dst_range_start: u64,
    range_length: u64,
}

impl Mapping {
    fn is_mapped(&self, value: u64) -> bool {
        self.src_range_start <= value && value < self.src_range_start + self.range_length
    }

    fn map(&self, value: u64) -> u64 {
        if self.is_mapped(value) {
            self.dst_range_start + value - self.src_range_start
        } else {
            value
        }
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("seeds: "), separated_list1(space1, u64_parser), newline)(input)
}

fn parse_map<'a>(input: &'a str, map_name: &'static str) -> IResult<&'a str, Map> {
    let (input, _) = newline(input)?;
    let (input, _) = preceded(tag(map_name), tag(" map:"))(input)?;
    let (input, _) = newline(input)?;
    let (input, mappings) = separated_list1(newline, parse_mapping)(input)?;
    let (input, _) = newline(input)?;
    let map = Map { mappings };
    Ok((input, map))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, dst_range_start) = u64_parser(input)?;
    let (input, src_range_start) = preceded(space1, u64_parser)(input)?;
    let (input, range_length) = preceded(space1, u64_parser)(input)?;
    let mapping = Mapping {
        src_range_start,
        dst_range_start,
        range_length,
    };
    Ok((input, mapping))
}

pub fn parse_almanac(input: &str) -> Result<Almanac, AocError> {
    let (input, seed_to_soil) = parse_map(input, "seed-to-soil")?;
    let (input, soil_to_fertilizer) = parse_map(input, "soil-to-fertilizer")?;
    let (input, fertilizer_to_water) = parse_map(input, "fertilizer-to-water")?;
    let (input, water_to_light) = parse_map(input, "water-to-light")?;
    let (input, light_to_temperature) = parse_map(input, "light-to-temperature")?;
    let (input, temperature_to_humidity) = parse_map(input, "temperature-to-humidity")?;
    let (rest, humidity_to_location) = parse_map(input, "humidity-to-location")?;
    if rest.is_empty() {
        Ok(Almanac {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    } else {
        Err(AocError::ParseError(format!(
            "Parsing Almanac left over rest: {}",
            rest
        )))
    }
}

pub fn process(input: &'static str) -> Result<u64, AocError> {
    let (input, seeds) = parse_seeds(input)?;
    let almanac = parse_almanac(input)?;
    let seed_locations = seeds
        .into_iter()
        .map(|seed| almanac.location_for_seed(seed));
    match seed_locations.len() {
        0 => Err(AocError::LogicError(
            "There should be one location for each seed but found 0 locations".to_string(),
        )),
        _ => seed_locations.min().ok_or(AocError::LogicError(
            "Minimum should exist for vector of length > 0".to_string(),
        )),
    }
}
