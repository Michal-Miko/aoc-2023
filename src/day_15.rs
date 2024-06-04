use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;

pub struct Day15;

fn hash(input: &str) -> usize {
    let mut hash = 0;
    for c in input.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn focusing_power(lenses: Vec<[&str; 3]>) -> usize {
    let mut boxmap: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for lens in lenses {
        match lens {
            [label, "-", _] => boxmap[hash(label)].retain(|e| e.0 != label),
            [label, "=", num] => {
                let parsed_num = num.parse().expect("Focal lenght should be valid");
                let bucket = &mut boxmap[hash(label)];
                if let Some(idx) = bucket.iter().position(|e| e.0 == label) {
                    bucket[idx] = (label, parsed_num);
                } else {
                    bucket.push((label, parsed_num));
                }
            }
            _ => {}
        }
    }

    boxmap
        .iter()
        .enumerate()
        .map(|(box_idx, box_contents)| {
            box_contents
                .iter()
                .enumerate()
                .map(|(slot_idx, lens)| (box_idx + 1) * (slot_idx + 1) * lens.1)
                .sum::<usize>()
        })
        .sum()
}

impl AocTask for Day15 {
    fn directory(&self) -> PathBuf {
        "tasks/day_15".into()
    }

    fn solution(&self, mut input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let data = input.next().unwrap_or_default();
        let lense_pat = regex::Regex::new(r#"(\w+)([-=])(\d?)"#)?;
        let lenses = data
            .split(',')
            .filter_map(|string| {
                let (_, lens) = lense_pat.captures(string)?.extract::<3>();
                Some(lens)
            })
            .collect_vec();

        match phase {
            1 => lenses
                .iter()
                .map(|lens| hash(&lens.join("")))
                .sum::<usize>(),
            2 => focusing_power(lenses),
            _ => unimplemented!(),
        }
        .solved()
    }
}
