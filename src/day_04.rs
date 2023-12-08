use std::{collections::VecDeque, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

use winnow::{
    ascii::{digit1, multispace0, multispace1},
    combinator::{delimited, preceded, separated, separated_pair, terminated},
    PResult, Parser,
};

pub struct Day04;

trait LotteryCard {
    fn matches(&self) -> usize;
    fn points(&self) -> u32 {
        match self.matches() {
            0 => 0,
            x => 2u32.pow(x as u32 - 1),
        }
    }
}

impl<L, R, T> LotteryCard for (L, R)
where
    for<'a> &'a L: IntoIterator<Item = &'a T>,
    for<'a> &'a R: IntoIterator<Item = &'a T>,
    T: PartialEq,
{
    fn matches(&self) -> usize {
        let (winning, card) = self;
        winning
            .into_iter()
            .filter(|&number| card.into_iter().any(|item| item == number))
            .count()
    }
}

fn parse_numbers(input: &mut &str) -> PResult<Vec<i32>> {
    separated(1.., digit1.parse_to::<i32>(), multispace1).parse_next(input)
}

fn parse_card(input: &mut &str) -> PResult<(Vec<i32>, Vec<i32>)> {
    preceded(
        ("Card", multispace1, digit1, terminated(':', multispace0)),
        separated_pair(
            parse_numbers,
            delimited(multispace0, '|', multispace0),
            parse_numbers,
        ),
    )
    .parse_next(input)
}

impl AocTask for Day04 {
    fn directory(&self) -> PathBuf {
        "tasks/day_04".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        // Parse cards
        let cards = input
            .map(|line| parse_card.parse(&line).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        match phase {
            1 => cards
                .into_iter()
                .map(|card| card.points())
                .sum::<u32>()
                .solved(),
            2 => cards
                .into_iter()
                .fold(
                    (0, VecDeque::new()),
                    |(total_cards, mut extra_copies), card| {
                        let won_extras = extra_copies.pop_front().unwrap_or(0);
                        let matches = card.matches();
                        let missing_extras = matches - matches.min(extra_copies.len());

                        (0..missing_extras).for_each(|_| extra_copies.push_back(won_extras + 1));

                        extra_copies
                            .iter_mut()
                            .take(matches - missing_extras)
                            .for_each(|extra| *extra += won_extras + 1);

                        (total_cards + 1 + won_extras, extra_copies)
                    },
                )
                .0
                .solved(),
            _ => unimplemented!(),
        }
    }
}
