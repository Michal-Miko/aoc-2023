use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;
use winnow::{
    ascii::{alpha0, digit1},
    combinator::{alt, delimited, peek, repeat, repeat_till0, terminated},
    token::{any, take},
    PResult, Parser,
};

pub struct Day01;

fn parse_digit_take_1(input: &mut &str) -> PResult<u32> {
    let digit = peek(alt((
        "one".value(1),
        "two".value(2),
        "three".value(3),
        "four".value(4),
        "five".value(5),
        "six".value(6),
        "seven".value(7),
        "eight".value(8),
        "nine".value(9),
        take(1usize).and_then(digit1).parse_to::<u32>(),
    )))
    .parse_next(input)?;
    take(1usize).parse_next(input)?;
    Ok(digit)
}

fn parse_digit_with_prefix(input: &mut &str) -> PResult<u32> {
    repeat_till0(any, parse_digit_take_1)
        .map(|(_, digit): (Vec<_>, u32)| digit)
        .parse_next(input)
}

fn parse_digits_alphanum(input: &mut &str) -> PResult<Vec<u32>> {
    terminated(repeat(1.., parse_digit_with_prefix), alpha0).parse_next(input)
}

fn parse_digits_num(input: &mut &str) -> PResult<Vec<u32>> {
    repeat(
        1..,
        delimited(
            alpha0,
            take(1usize).and_then(digit1).parse_to::<u32>(),
            alpha0,
        ),
    )
    .parse_next(input)
}

impl AocTask for Day01 {
    fn directory(&self) -> PathBuf {
        "tasks/day_01".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let mut parser = match phase {
            1 => parse_digits_num,
            2 => parse_digits_alphanum,
            _ => unimplemented!(),
        };
        input
            .map(|line| parser.parse(&line).map_err(|e| e.to_string()))
            .map_ok(|digits| {
                let first = digits.first().unwrap_or(&0);
                let last = digits.last().unwrap_or(first);
                first * 10 + last
            })
            .process_results(|iter| iter.sum::<u32>())?
            .solved()
    }
}

#[cfg(test)]
mod phase_2 {
    use winnow::Parser;

    use crate::day_01::parse_digits_alphanum;

    #[test]
    fn single_digit() {
        assert_eq!(parse_digits_alphanum.parse("6").unwrap(), vec![6])
    }

    #[test]
    fn single_digit_string() {
        assert_eq!(parse_digits_alphanum.parse("two").unwrap(), vec![2])
    }

    #[test]
    fn string_digit() {
        assert_eq!(parse_digits_alphanum.parse("two1").unwrap(), vec![2, 1])
    }

    #[test]
    fn string_string() {
        assert_eq!(parse_digits_alphanum.parse("twosix").unwrap(), vec![2, 6])
    }

    #[test]
    fn digit_duble_string() {
        assert_eq!(
            parse_digits_alphanum.parse("3twone").unwrap(),
            vec![3, 2, 1]
        )
    }

    #[test]
    fn tokenize_following_string_digits() {
        assert_eq!(
            parse_digits_alphanum.parse("sixtwone").unwrap(),
            vec![6, 2, 1]
        )
    }

    #[test]
    fn duplicate_strings_persist() {
        assert_eq!(
            parse_digits_alphanum.parse("sixsixsix").unwrap(),
            vec![6, 6, 6]
        )
    }

    #[test]
    fn extra_suffix() {
        assert_eq!(parse_digits_alphanum.parse("six2foo").unwrap(), vec![6, 2])
    }
}
