use std::cmp::{max, min};

use aoclib::AocError;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64 as u64_parser},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Copy, Clone)]
pub struct Range {
    pub start: u64,
    pub length: u64,
}

impl Range {
    pub fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }

    pub fn end(&self) -> u64 {
        self.start + self.length
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..self.start + self.length
    }

    pub fn contains(&self, value: u64) -> bool {
        self.start <= value && value < self.start + self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn is_overlapping_with(&self, other: &Self) -> bool {
        self.start < other.end() && other.start < self.end()
    }

    pub fn overlap(&self, other: &Self) -> Option<Self> {
        if self.is_overlapping_with(other) {
            let start = max(self.start, other.start);
            Some(Self {
                start,
                length: min(self.end(), other.end()) - start,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
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
    pub fn location_for_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        self.humidity_to_location.map(humidity)
    }

    pub fn location_ranges_for_seed_range(&self, range: &Range) -> Vec<Range> {
        self.seed_to_soil
            .map_range(range)
            .into_iter()
            .flat_map(|range| self.soil_to_fertilizer.map_range(&range))
            .flat_map(|range| self.fertilizer_to_water.map_range(&range))
            .flat_map(|range| self.water_to_light.map_range(&range))
            .flat_map(|range| self.light_to_temperature.map_range(&range))
            .flat_map(|range| self.temperature_to_humidity.map_range(&range))
            .flat_map(|range| self.humidity_to_location.map_range(&range))
            .collect()
    }
}

#[derive(Debug)]
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

    pub fn map_range(&self, range: &Range) -> Vec<Range> {
        let mut mapped_ranges: Vec<Range> = Vec::new();
        let mut mappings = self.mappings.iter();
        let mut current_mapping = mappings.next().unwrap();
        let mut current_range = *range;
        loop {
            if current_mapping.source.is_overlapping_with(&current_range) {
                let (pre_ol, ol, post_ol) = current_mapping.map_range(&current_range);
                if let Some(pre_ol) = pre_ol {
                    mapped_ranges.push(pre_ol);
                }
                mapped_ranges.push(ol.unwrap());
                if let Some(post_ol) = post_ol {
                    current_range = post_ol
                } else {
                    break;
                }
            } else if let Some(next_mapping) = mappings.next() {
                current_mapping = next_mapping;
            } else {
                mapped_ranges.push(current_range);
                break;
            }
        }
        mapped_ranges
    }
}

#[derive(Debug)]
pub struct Mapping {
    source: Range,
    destination: Range,
}

impl Mapping {
    fn is_mapped(&self, value: u64) -> bool {
        self.source.contains(value)
    }

    fn map(&self, value: u64) -> u64 {
        if self.is_mapped(value) {
            self.destination.start + value - self.source.start
        } else {
            value
        }
    }

    fn map_range(&self, range: &Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        let overlap = self.source.overlap(range);
        if let Some(overlap) = overlap {
            let pre_overlap = if range.start < overlap.start {
                Some(Range {
                    start: range.start,
                    length: overlap.start - range.start,
                })
            } else {
                None
            };
            let mapped_overlap = Range {
                start: self.map(overlap.start),
                length: overlap.length,
            };
            let post_overlap = if overlap.end() < range.end() {
                Some(Range {
                    start: overlap.end(),
                    length: range.end() - overlap.end(),
                })
            } else {
                None
            };
            (pre_overlap, Some(mapped_overlap), post_overlap)
        } else if range.end() <= self.source.start {
            (Some(*range), None, None)
        } else {
            (None, None, Some(*range))
        }
    }
}

fn parse_map<'a>(input: &'a str, map_name: &'static str) -> IResult<&'a str, Map> {
    let (input, _) = newline(input)?;
    let (input, _) = preceded(tag(map_name), tag(" map:"))(input)?;
    let (input, _) = newline(input)?;
    let (input, mut mappings) = separated_list1(newline, parse_mapping)(input)?;
    let (input, _) = newline(input)?;
    mappings.sort_by_key(|mapping| mapping.source.start);
    let map = Map { mappings };
    Ok((input, map))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, dst_range_start) = u64_parser(input)?;
    let (input, src_range_start) = preceded(space1, u64_parser)(input)?;
    let (input, range_length) = preceded(space1, u64_parser)(input)?;
    let mapping = Mapping {
        source: Range {
            start: src_range_start,
            length: range_length,
        },
        destination: Range {
            start: dst_range_start,
            length: range_length,
        },
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
