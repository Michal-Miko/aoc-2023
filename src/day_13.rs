use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;

pub struct Day13;

trait SmudgedCmp {
    fn differences(&self, rhs: &Self) -> usize;
}

impl SmudgedCmp for String {
    fn differences(&self, rhs: &Self) -> usize {
        self.chars()
            .zip(rhs.chars())
            .filter(|(l, r)| l != r)
            .count()
    }
}

#[derive(Debug)]
struct Pattern {
    data: Vec<String>,
    width: usize,
}

impl Pattern {
    fn new(data: Vec<String>) -> Self {
        let width = data.first().map(|row| row.len()).unwrap_or_default();
        Self { data, width }
    }

    fn rows(&self) -> Vec<String> {
        self.data.clone()
    }

    fn cols(&self) -> Vec<String> {
        (0..self.width)
            .map(|col| {
                self.data
                    .iter()
                    .flat_map(|row| row.chars().nth(col))
                    .collect()
            })
            .collect()
    }

    fn reflection_value(&self, expected_differences: usize) -> usize {
        for (lines, multiplier) in [(self.rows(), 100), (self.cols(), 1)] {
            for i in 1..lines.len() {
                let before = &lines[0..i];
                let after = &lines[i..lines.len()];
                let differences = before
                    .iter()
                    .rev()
                    .zip(after.iter())
                    .map(|(b, a)| b.differences(a))
                    .sum::<usize>();
                if differences == expected_differences {
                    return i * multiplier;
                }
            }
        }
        0
    }
}

struct Map {
    patterns: Vec<Pattern>,
}

impl Map {
    fn new(input: AocStringIter) -> Self {
        let patterns = input
            .group_by(|line| !line.is_empty())
            .into_iter()
            .filter_map(|(not_empty, group)| not_empty.then(|| Pattern::new(group.collect())))
            .collect::<Vec<_>>();
        Self { patterns }
    }
}

impl AocTask for Day13 {
    fn directory(&self) -> PathBuf {
        "tasks/day_13".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let expected_differences = match phase {
            1 => 0,
            2 => 1,
            _ => unimplemented!(),
        };
        let map = Map::new(input);
        map.patterns
            .iter()
            .map(|pat| pat.reflection_value(expected_differences))
            .sum::<usize>()
            .solved()
    }
}
