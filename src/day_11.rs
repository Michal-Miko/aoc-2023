use std::{collections::HashMap, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::UnitSolved, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;

pub struct Day11;

struct Universe {
    galaxies: Vec<[usize; 2]>,
    rows: HashMap<usize, Vec<usize>>,
    cols: HashMap<usize, Vec<usize>>,
}

impl Universe {
    fn new(input: AocStringIter) -> Self {
        let mut galaxies = vec![];
        let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut cols: HashMap<usize, Vec<usize>> = HashMap::new();
        for (y, line) in input.enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr == '#' {
                    let next = galaxies.len();
                    galaxies.push([x, y]);
                    rows.entry(y).or_default().push(next);
                    cols.entry(x).or_default().push(next);
                }
            }
        }
        Self {
            galaxies,
            rows,
            cols,
        }
    }

    fn compute_real_coordinates(&mut self, distnace_factor: usize) {
        for (axis, coordinate_idx) in [(&self.cols, 0), (&self.rows, 1)] {
            let size = *axis.keys().max().expect("At least one galaxy should exist");

            let mut empty_space = 0;
            let mut offset = 0;
            for coordinate in 0..=size {
                if axis.contains_key(&coordinate) {
                    if empty_space != 0 {
                        offset += empty_space * (distnace_factor - 1);
                        empty_space = 0;
                    }
                    axis.get(&coordinate)
                        .expect("Galaxy should exist since the axis contains its key")
                        .iter()
                        .for_each(|idx| self.galaxies[*idx][coordinate_idx] += offset);
                } else {
                    empty_space += 1;
                }
            }
        }
    }
}

impl AocTask for Day11 {
    fn directory(&self) -> PathBuf {
        "tasks/day_11".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let mut universe = Universe::new(input);
        match phase {
            1 => universe.compute_real_coordinates(2),
            2 => universe.compute_real_coordinates(10usize.pow(6)),
            _ => unimplemented!(),
        }

        // Sum distances between each combination of 2 galaxies
        universe
            .galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a[0].abs_diff(b[0]) + a[1].abs_diff(b[1]))
            .sum::<usize>()
            .solved()
    }
}
