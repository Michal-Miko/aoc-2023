use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use winnow::{
    ascii::{digit1, multispace0, multispace1},
    combinator::{alt, separated},
    error::{ErrMode, ErrorKind, FromExternalError},
    PResult, Parser,
};

pub struct Day06;

fn parse_prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt(("Time:", "Distance:")).parse_next(input)?;
    multispace0.parse_next(input)
}

fn parse_data_separated(input: &mut &str) -> PResult<Vec<i64>> {
    parse_prefix.parse_next(input)?;
    separated(1.., digit1.parse_to::<i64>(), multispace1).parse_next(input)
}

fn parse_data_single(input: &mut &str) -> PResult<Vec<i64>> {
    parse_prefix.parse_next(input)?;
    let num: String = separated(1.., digit1, multispace1).parse_next(input)?;
    match num.parse::<i64>() {
        Ok(num) => Ok(vec![num]),
        Err(e) => Err(ErrMode::from_external_error(input, ErrorKind::Verify, e)),
    }
}

impl AocTask for Day06 {
    fn directory(&self) -> PathBuf {
        "tasks/day_06".into()
    }

    fn solution(&self, mut input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let time_data = input.next().ok_or("Missing time data")?;
        let distance_data = input.next().ok_or("Missing distance data")?;

        let mut parser = match phase {
            1 => parse_data_separated,
            2 => parse_data_single,
            _ => unimplemented!(),
        };

        let time = parser.parse(&time_data).map_err(|e| e.to_string())?;
        let distance = parser.parse(&distance_data).map_err(|e| e.to_string())?;

        // x = hold time
        // t = total time
        // d = distance to beat
        // x * (t - x) = d
        // -x² + xt - d = 0
        // roots will be the min and max hold times for getting the exact
        // distance as the record, so we'll take the integers between them

        time.into_iter()
            .zip(distance)
            .map(|(t, d)| {
                let common_sqrt = ((t.pow(2) - 4 * d) as f64).sqrt();
                let mut x1 = (-t as f64 + common_sqrt) / -2.0;
                let mut x2 = (-t as f64 - common_sqrt) / -2.0;
                if x2 < x1 {
                    std::mem::swap(&mut x1, &mut x2)
                }
                // min = x1_floor + 1
                let x1_floor = x1.floor() as i64;
                // max = x2_ceil - 1
                let x2_ceil = x2.ceil() as i64;
                x2_ceil - x1_floor - 1
            })
            .inspect(|wins| println!("{wins}"))
            .product::<i64>()
            .solved()
    }
}
