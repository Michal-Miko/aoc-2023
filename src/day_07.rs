use core::cmp::Ordering::Equal;
use std::{
    cmp::Ordering::{self},
    iter::repeat as repeat_iter,
    path::PathBuf,
};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;
use winnow::{
    ascii::{digit1, multispace1},
    combinator::{fail, repeat, separated_pair, success},
    dispatch,
    token::any,
    PResult, Parser,
};

pub struct Day07;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Joker,
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Threes,
    FullHouse,
    Fours,
    Fives,
}

#[derive(Debug)]
struct Bid {
    hand: Hand,
    bid: u32,
}

#[derive(Debug, Clone)]
struct Hand(Vec<Card>);

impl Hand {
    fn eval(&self) -> HandType {
        let counts_by_card = self.0.iter().counts();
        let mut counts = counts_by_card.values().sorted_by(|a, b| b.cmp(a));
        match (counts.next(), counts.next()) {
            (Some(1), _) => HandType::HighCard,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), _) => HandType::Pair,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), _) => HandType::Threes,
            (Some(4), _) => HandType::Fours,
            (Some(5), None) => HandType::Fives,
            _ => unreachable!("{self:?}"),
        }
    }

    fn with_joker(&self) -> Hand {
        let mut non_jokers: Vec<Card> = self
            .0
            .iter()
            .filter(|card| **card != Card::Joker)
            .copied()
            .collect();

        let most_common_card = non_jokers
            .iter()
            .counts()
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(card, count)| *card)
            // If the hand has no regular cards, replace it with 5 aces
            .unwrap_or(Card::Ace);

        // Extend non_jokers with the most common card to build the strongest hand
        non_jokers.extend(repeat_iter(most_common_card).take(5 - non_jokers.len()));
        Hand(non_jokers)
    }

    fn eval_with_joker(&self) -> HandType {
        self.with_joker().eval()
    }

    fn cmp_with_joker(&self, other: &Self) -> Ordering {
        match self.eval_with_joker().partial_cmp(&other.eval_with_joker()) {
            Some(Equal) => self.0.cmp(&other.0),
            ord => ord.expect("Invalid ordering"),
        }
    }

    fn jacks_as_jokers(&self) -> Self {
        let joker_hand = self
            .0
            .iter()
            .map(|card| {
                if *card == Card::Jack {
                    Card::Joker
                } else {
                    *card
                }
            })
            .collect();
        Hand(joker_hand)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.eval().cmp(&other.eval()))
    }
}

impl Eq for Hand {}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.eval().cmp(&other.eval()) {
            Equal => self.0.cmp(&other.0),
            ord => ord,
        }
    }
}

fn parse_card(input: &mut &str) -> PResult<Card> {
    dispatch!(any;
        '1' => success(Card::One),
        '2' => success(Card::Two),
        '3' => success(Card::Three),
        '4' => success(Card::Four),
        '5' => success(Card::Five),
        '6' => success(Card::Six),
        '7' => success(Card::Seven),
        '8' => success(Card::Eight),
        '9' => success(Card::Nine),
        'T' => success(Card::Ten),
        'J' => success(Card::Jack),
        'Q' => success(Card::Queen),
        'K' => success(Card::King),
        'A' => success(Card::Ace),
        _ => fail,
    )
    .parse_next(input)
}

fn parse_hand(input: &mut &str) -> PResult<Hand> {
    Ok(Hand(repeat(5, parse_card).parse_next(input)?))
}

fn parse_bid(input: &mut &str) -> PResult<Bid> {
    let (hand, bid) =
        separated_pair(parse_hand, multispace1, digit1.parse_to::<u32>()).parse_next(input)?;
    Ok(Bid { hand, bid })
}

impl AocTask for Day07 {
    fn directory(&self) -> PathBuf {
        "tasks/day_07".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let mut bids = input
            .map(|line| parse_bid.parse(&line).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        match phase {
            1 => bids.iter_mut().sorted_by(|a, b| a.hand.cmp(&b.hand)),
            2 => bids
                .iter_mut()
                .map(|bid| {
                    bid.hand = bid.hand.jacks_as_jokers();
                    bid
                })
                .sorted_by(|a, b| a.hand.cmp_with_joker(&b.hand)),
            _ => unimplemented!(),
        }
        .enumerate()
        .map(|(i, bid)| bid.bid * (i as u32 + 1))
        .sum::<u32>()
        .solved()
    }
}
