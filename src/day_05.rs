#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::single_range_in_vec_init)]

use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
    ops,
    ops::Range,
    path::PathBuf,
};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;

pub struct Day05;

#[derive(Debug)]
struct MappingRange {
    range: Range<i64>,
    offset: i64,
}

impl MappingRange {
    fn new(dest: i64, src: i64, len: i64) -> Self {
        Self {
            range: src..(src + len),
            offset: dest - src,
        }
    }
}

impl ops::Shr<&MappingRange> for i64 {
    type Output = Option<i64>;

    fn shr(self, rhs: &MappingRange) -> Self::Output {
        rhs.range.contains(&self).then_some(self + rhs.offset)
    }
}

impl ops::Shr<&MappingRange> for Range<i64> {
    type Output = Option<(Range<i64>, Vec<Range<i64>>)>;

    // Applies a MappingRange to a range, producing one of:
    //   * None - when the ranges are exclusive
    //   * Some(mapped_range, empty_vec) - when the range is fully contained inside a MappingRange
    //   * Some(mapped_range, vec_of_unmapped_ranges) - when the range is partially contained inside a MappingRange
    fn shr(self, rhs: &MappingRange) -> Self::Output {
        let min = rhs.range.start;
        let max = rhs.range.end;

        let self_min = self.start;
        let self_max = self.end;

        // Check if the ranges are exclusive
        // -1 is added when comapring max with min because the ranges are exclusive,
        // it can be skipped most of the time since it does not matter when comparing max to max
        if self_min < min && (self_max - 1) < min || self_min > (max - 1) && self_max > max {
            return None;
        }

        // Calculate overlaps (start/min is always smaller than end/max)
        match (self_min.cmp(&min), self_max.cmp(&max)) {
            // Left overflow
            // R  [-----]   OR [-----]
            // MR   [-----]      [---]
            (Less, Less) | (Less, Equal) => Some((
                (min + rhs.offset)..(self_max + rhs.offset),
                vec![self_min..min],
            )),
            // Right overflow
            // R  [-----] OR   [-----]
            // MR [---]      [-----]
            (Equal, Greater) | (Greater, Greater) => Some((
                (self_min + rhs.offset)..(max + rhs.offset),
                vec![max..self_max],
            )),
            // Left & right overflow
            // R  [--------]
            // MR   [----]
            (Less, Greater) => Some((
                (min + rhs.offset)..(max + rhs.offset),
                vec![self_min..min, max..self_max],
            )),
            // Full overlap
            // R  [---]   OR    [---] OR [----] OR   [---]
            // MR [-----]     [-----]    [----]    [-------]
            (Equal, Less) | (Greater, Equal) | (Equal, Equal) | (Greater, Less) => {
                Some(((self_min + rhs.offset)..(self_max + rhs.offset), vec![]))
            }
        }
    }
}

impl TryFrom<String> for MappingRange {
    type Error = BoxedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (dest, src, len) = value
            .split_whitespace()
            .map(str::parse::<i64>)
            .process_results(|iter| {
                iter.collect_tuple()
                    .ok_or(format!("Invalid mapping: {value}"))
            })??;

        Ok(Self::new(dest, src, len))
    }
}

#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<MappingRange>,
}

impl TryFrom<Vec<String>> for Mapping {
    type Error = BoxedError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut iter = value.into_iter();
        let header = iter
            .next()
            .ok_or("Missing Mapping header")?
            .replace(" map:", "");

        let (from, to) = header
            .split_once("-to-")
            .ok_or(format!("Invalid Mapping header: {header}"))?;

        let ranges = iter
            .map(MappingRange::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            from: from.into(),
            to: to.into(),
            ranges,
        })
    }
}

#[derive(Debug)]
struct Almanac {
    start_ids: Vec<i64>,
    maps: HashMap<String, Mapping>,
    first_field: String,
    last_field: String,
}

impl<'src> TryFrom<AocStringIter<'src>> for Almanac {
    type Error = BoxedError;

    fn try_from(mut value: AocStringIter<'src>) -> Result<Self, Self::Error> {
        let header = value
            .next()
            .ok_or("Missing Almanac header")?
            .replace("seeds: ", "");

        let start_ids = header
            .split_whitespace()
            .map(str::parse::<i64>)
            .collect::<Result<Vec<_>, _>>()?;

        // Skip empty row
        value.next();

        let mut maps = vec![];

        loop {
            // Mapping ends with newline, take_while will consume it
            let mapping: Vec<_> = value
                .by_ref()
                .take_while(|string| !string.is_empty())
                .collect();
            if mapping.is_empty() {
                break;
            } else {
                maps.push(Mapping::try_from(mapping)?);
            }
        }

        Self::new(start_ids, maps)
    }
}

impl Almanac {
    fn new(start_ids: Vec<i64>, maps: Vec<Mapping>) -> Result<Self, BoxedError> {
        let first_field = maps
            .first()
            .ok_or(format!("Mappings are empty: {maps:#?}"))?
            .from
            .to_string();
        let last_field = maps
            .last()
            .ok_or(format!("Mappings are empty: {maps:#?}"))?
            .to
            .to_string();
        let maps = maps
            .into_iter()
            .map(|mapping| (mapping.from.to_string(), mapping))
            .collect();

        Ok(Self {
            start_ids,
            maps,
            first_field,
            last_field,
        })
    }

    fn map_ids(&self) -> Result<Vec<i64>, BoxedError> {
        let mut mapped_ids = vec![];
        for id in &self.start_ids {
            let mut mapping_key = &self.first_field;
            let mut current_id = *id;
            loop {
                let mapping = self
                    .maps
                    .get(mapping_key)
                    .ok_or(format!("Couldn't find the required mapping: {mapping_key}"))?;

                // Apply the mapping if a matching ID exists, keep the same ID otherwise
                let found_id = mapping.ranges.iter().find_map(|range| current_id >> range);
                if let Some(new_id) = found_id {
                    current_id = new_id;
                }

                // Move on to the next mapping
                mapping_key = &mapping.to;
                if mapping_key == &self.last_field {
                    mapped_ids.push(current_id);
                    break;
                }
            }
        }
        Ok(mapped_ids)
    }

    fn map_ranges(&self) -> Result<Vec<Range<i64>>, BoxedError> {
        let mut mapped_ranges: Vec<Range<i64>> = vec![];
        let mut ranges_to_map: Vec<Range<i64>> = self
            .start_ids
            .iter()
            .tuples()
            .map(|(id, len)| *id..(id + len))
            .collect();

        let mut mapping_key = &self.first_field;
        loop {
            let mapping = self
                .maps
                .get(mapping_key)
                .ok_or(format!("Couldn't find the required mapping: {mapping_key}"))?;

            let current_range = match ranges_to_map.pop() {
                Some(range) => range,
                None => {
                    // Move on to the next mapping if there are no ranges to map in the current one
                    std::mem::swap(&mut ranges_to_map, &mut mapped_ranges);
                    mapping_key = &mapping.to;

                    // Finished
                    if mapping_key == &self.last_field {
                        break;
                    }

                    continue;
                }
            };

            // Apply the mapping if a matching range exists, keep the same ID otherwise
            let found_ranges = mapping
                .ranges
                .iter()
                .find_map(|range| current_range.clone() >> range);

            match found_ranges {
                Some((found, split)) => {
                    mapped_ranges.push(found);
                    ranges_to_map.extend(split);
                }
                None => mapped_ranges.push(current_range),
            }
        }
        Ok(ranges_to_map)
    }
}

impl AocTask for Day05 {
    fn directory(&self) -> PathBuf {
        "tasks/day_05".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let almanac = Almanac::try_from(input)?;

        match phase {
            1 => almanac.map_ids()?.iter().min().solved(),
            2 => almanac
                .map_ranges()?
                .iter()
                .map(|range| range.start)
                .min()
                .solved(),
            _ => unimplemented!(),
        }
    }
}
