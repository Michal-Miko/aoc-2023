#![allow(unused_imports, dead_code)]
use core::cmp::Ordering::Equal;
use std::{
    cmp::Ordering::{Greater, Less},
    path::PathBuf,
};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;
use winnow::{
    ascii::{alphanumeric1, digit1, multispace1},
    combinator::{repeat, separated_pair, todo},
    error::{ErrMode, ErrorKind, ParserError},
    token::take,
    PResult, Parser,
};

pub struct Day07;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Threes,
    FullHouse,
    Fours,
    Fives,
}

struct Bid {
    hand: Vec<Card>,
    bid: u32,
}

impl Bid {
    fn eval(&self) -> HandType {
        let counts_by_card = self.hand.iter().counts();
        let mut counts = counts_by_card.values().sorted();
        match (counts.next(), counts.next()) {
            (Some(1), _) => HandType::HighCard,
            (Some(2), None) => HandType::Pair,
            (Some(3), None) => HandType::Threes,
            (Some(4), None) => HandType::Fours,
            (Some(5), None) => HandType::Fives,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(2), Some(2)) => HandType::TwoPair,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Bid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.eval().cmp(&other.eval()) {
            Equal => Some(self.hand.cmp(&other.hand)),
            ord => Some(ord),
        }
    }
}

fn parse_card<'s>(input: &mut &'s str) -> PResult<Card> {
    let card = take(1usize).parse_next(input)?;
    match card {
        "1" => Ok(Card::One),
        "2" => Ok(Card::Two),
        "3" => Ok(Card::Three),
        "4" => Ok(Card::Four),
        "5" => Ok(Card::Five),
        "6" => Ok(Card::Six),
        "7" => Ok(Card::Seven),
        "8" => Ok(Card::Eight),
        "9" => Ok(Card::Nine),
        "T" => Ok(Card::Ten),
        "J" => Ok(Card::Jack),
        "Q" => Ok(Card::Queen),
        "K" => Ok(Card::King),
        "A" => Ok(Card::Ace),
        c => Err(ErrMode::from_error_kind(input, ErrorKind::Verify)),
    }
}

fn parse_hand<'s>(input: &mut &'s str) -> PResult<Vec<Card>> {
    repeat(5, parse_card).parse_next(input)
}

fn parse_bid<'s>(input: &mut &'s str) -> PResult<Bid> {
    let (hand, bid) =
        separated_pair(parse_hand, multispace1, digit1.parse_to::<u32>()).parse_next(input)?;
    Ok(Bid { hand, bid })
}

impl AocTask for Day07 {
    fn directory(&self) -> PathBuf {
        "tasks/day_07".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let bids = input
            .map(|line| parse_bid.parse(&line).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        bids.iter()
            .sorted()
            .enumerate()
            .map(|(i, bid)| bid.bid * i)
            .sum()
            .solved()
    }
}
