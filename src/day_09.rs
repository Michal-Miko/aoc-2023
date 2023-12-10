use std::{fmt::Display, iter::once, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;
use winnow::{
    ascii::{dec_int, multispace1},
    combinator::separated,
    PResult, Parser,
};

pub struct Day09;

#[derive(Debug)]
enum ExtrapolationKind {
    Forwards,
    Backwards,
}

#[derive(Debug, Clone)]
struct History {
    data: Vec<i32>,
    derived: Vec<Vec<i32>>,
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.data)?;
        for row in &self.derived {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

// Unwraps in History impl are safe due to the parser restrictions (at least one record per history)
impl History {
    fn new(data: Vec<i32>) -> History {
        History {
            data,
            derived: vec![],
        }
        .derived()
    }

    fn derived(mut self) -> History {
        let mut current = &self.data;
        while !current.iter().all_equal() {
            let mut next = vec![];
            for (first, second) in current.iter().tuple_windows() {
                next.push(second - first);
            }
            self.derived.push(next);
            current = &self.derived.last().expect("Empty history");
        }
        self.derived.push(vec![0; current.len() - 1]);
        self
    }

    fn extrapolated(mut self, kind: ExtrapolationKind) -> History {
        let mut rows = once(&mut self.data).chain(&mut self.derived).rev();
        rows.next().expect("Empty history").push(0);

        let mut seed = 0;
        match kind {
            ExtrapolationKind::Forwards => rows.for_each(|row| {
                let last = *row.last().expect("Empty history");
                seed += last;
                row.push(seed);
            }),
            ExtrapolationKind::Backwards => rows.for_each(|row| {
                let first = *row.first().expect("Empty history");
                seed = first - seed;
                row.insert(0usize, seed);
            }),
        };

        self
    }

    fn last(&self) -> i32 {
        *self.data.last().expect("Empty history")
    }

    fn first(&self) -> i32 {
        *self.data.first().expect("Empty history")
    }
}

fn parse_history(input: &mut &str) -> PResult<History> {
    let data = separated(1.., dec_int::<_, i32, _>, multispace1).parse_next(input)?;
    Ok(History::new(data))
}

impl AocTask for Day09 {
    fn directory(&self) -> PathBuf {
        "tasks/day_09".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let report = input.map(|line| parse_history.parse(&line).map_err(|err| err.to_string()));

        match phase {
            1 => report
                .map_ok(|hist| hist.extrapolated(ExtrapolationKind::Forwards))
                .map_ok(|hist| hist.last())
                .process_results(|iter| iter.sum::<i32>()),
            2 => report
                .map_ok(|hist| hist.extrapolated(ExtrapolationKind::Backwards))
                .map_ok(|hist| hist.first())
                .process_results(|iter| iter.sum::<i32>()),
            _ => unimplemented!(),
        }
        .try_solved()
    }
}

#[cfg(test)]
mod test {
    use super::{ExtrapolationKind, History};

    #[test]
    fn single_number_history() {
        let history = History::new(vec![5]);
        assert_eq!(history.data, vec![5]);
        assert_eq!(history.derived, vec![vec![]]);

        let forwards_extrapolated = history.clone().extrapolated(ExtrapolationKind::Forwards);
        assert_eq!(forwards_extrapolated.data, vec![5, 5]);
        assert_eq!(forwards_extrapolated.derived, vec![vec![0]]);

        let backwards_extrapolated = history.clone().extrapolated(ExtrapolationKind::Backwards);
        assert_eq!(backwards_extrapolated.data, vec![5, 5]);
        assert_eq!(backwards_extrapolated.derived, vec![vec![0]]);
    }
}
