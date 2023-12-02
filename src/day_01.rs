use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
    str::Chars,
};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use lazy_static::lazy_static;

pub struct Day01;

lazy_static! {
    static ref DIGITS: HashMap<&'static str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .copied()
    .collect();
}

pub struct DigitTokenizer<I>
where
    I: Iterator<Item = char>,
{
    iter: I,
    current_token: String,
    // Which index to start the digit search at
    token_search_index: usize,
}

impl<I: Iterator<Item = char>> DigitTokenizer<I> {
    pub fn new(iter: I) -> DigitTokenizer<I> {
        DigitTokenizer {
            iter,
            current_token: "".into(),
            token_search_index: 0,
        }
    }
}

impl<I> Iterator for DigitTokenizer<I>
where
    I: Iterator<Item = char>,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next();
            match next.map(|character| character.to_digit(10)) {
                // Consumed the entire iterator, nothing to return
                None => return None,
                // Not a digit - extend the token with chars until a digit is found
                Some(None) => {
                    self.current_token.push(next.unwrap());
                    let (idx, last_digit) = DIGITS
                        .iter()
                        // Find new string match
                        .find_map(|digit| {
                            Some((
                                self.current_token[self.token_search_index..].find(digit.0)?,
                                *digit.1,
                            ))
                        })
                        .unzip();

                    if let Some(match_index) = idx {
                        self.token_search_index += match_index + 1;
                        return last_digit;
                    }
                }
                // Digit
                Some(digit) => {
                    self.token_search_index = 0;
                    self.current_token = "".into();
                    return digit;
                }
            }
        }
    }
}

trait TokenizeDigits<I: Iterator<Item = char>> {
    fn tokenize_digits(self) -> DigitTokenizer<I>;
}

impl<'src> TokenizeDigits<Chars<'src>> for Chars<'src> {
    fn tokenize_digits(self) -> DigitTokenizer<Chars<'src>> {
        DigitTokenizer::new(self)
    }
}

fn parse_row<S>(row: S, phase: usize) -> u32
where
    S: Into<String>,
{
    let row = row.into();
    let mut numbers: VecDeque<u32> = match phase {
        1 => row
            .chars()
            .filter_map(|character| character.to_digit(10))
            .collect(),
        2 => row.chars().tokenize_digits().collect(),
        _ => unimplemented!(),
    };
    let first = numbers.pop_front().unwrap_or_default();
    let last = numbers.pop_back().unwrap_or(first);
    first * 10 + last
}

impl AocTask for Day01 {
    fn directory(&self) -> PathBuf {
        "tasks/day_01".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.map(|row| parse_row(row, phase)).sum::<u32>().solved()
    }
}

#[cfg(test)]
mod phase_2 {
    use super::{parse_row, TokenizeDigits};

    fn parse_calibration(string: &str) -> u32 {
        parse_row(string, 2)
    }

    fn parse_digits(string: &str) -> Vec<u32> {
        string.chars().tokenize_digits().collect()
    }

    #[test]
    fn single_digit() {
        assert_eq!(parse_calibration("6"), 66)
    }

    #[test]
    fn single_digit_string() {
        assert_eq!(parse_calibration("two"), 22)
    }

    #[test]
    fn string_digit() {
        assert_eq!(parse_calibration("two1"), 21)
    }

    #[test]
    fn string_string() {
        assert_eq!(parse_calibration("twosix"), 26)
    }

    #[test]
    fn digit_duble_string() {
        assert_eq!(parse_calibration("3twone"), 31)
    }

    #[test]
    fn tokenize_following_string_digits() {
        assert_eq!(parse_digits("sixtwone"), vec![6, 2, 1])
    }

    #[test]
    fn duplicate_strings_persist() {
        assert_eq!(parse_digits("sixsixsix"), vec![6, 6, 6])
    }

    #[test]
    fn extra_suffix() {
        assert_eq!(parse_digits("six2foo"), vec![6, 2])
    }
}
