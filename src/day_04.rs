use std::{collections::VecDeque, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::eyre::Context;
use itertools::Itertools;

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

impl AocTask for Day04 {
    fn directory(&self) -> PathBuf {
        "tasks/day_04".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        // Parse cards
        let cards = input
            .map(|line| {
                line.split(':')
                    .last()
                    .ok_or(format!("Invalid card: {line}").into_boxed_str())
                    .map(str::to_string)
            })
            .map_ok(|numbers| {
                numbers
                    .split_once('|')
                    .map(|(l, r)| (l.to_string(), r.to_string()))
                    .ok_or(format!("Invalid numbers: {numbers}").into_boxed_str())
            })
            .flatten()
            .map_ok(|(winning, card)| -> Result<_, BoxedError> {
                Ok((
                    winning
                        .split_whitespace()
                        .map(str::parse)
                        .collect::<Result<Vec<i32>, _>>()
                        .context(winning.to_owned())?,
                    card.split_whitespace()
                        .map(str::parse)
                        .collect::<Result<Vec<i32>, _>>()
                        .context(card.to_owned())?,
                ))
            })
            .flatten()
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
                        let missing_extras = matches as i32 - extra_copies.len() as i32;
                        if missing_extras > 0 {
                            (0..missing_extras).for_each(|_| extra_copies.push_back(0));
                        }

                        extra_copies
                            .iter_mut()
                            .take(matches)
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
