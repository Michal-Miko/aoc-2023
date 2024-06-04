use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use ndarray::{s, Array1, Array2};

pub struct Day14;

struct Platform {
    data: Array2<char>,
}

enum SettleDir {
    North,
    East,
    South,
    West,
}

impl Platform {
    fn new(input: AocStringIter) -> Self {
        let data: Vec<String> = input.collect();
        let width = data.first().map_or(0, String::len);
        let height = data.len();

        let array = Array2::from_shape_vec(
            (width, height),
            data.into_iter().fold(vec![], |mut acc, s| {
                acc.extend(s.chars());
                acc
            }),
        )
        .expect("Input should be valid");

        Self { data: array }
    }

    fn settle(&mut self, dir: &SettleDir) {
        let slice = match dir {
            SettleDir::North => |idx| s![0.., idx],
            SettleDir::East => |idx| s![idx, 0..;-1],
            SettleDir::South => |idx| s![0..;-1, idx],
            SettleDir::West => |idx| s![idx, 0..],
        };

        for idx in 0..self.data.dim().0 {
            let mut col = self.data.slice_mut(slice(idx));
            let mut free_spot = 0;
            for y in 0..col.dim() {
                match col[y] {
                    '.' => {}
                    'O' if y != free_spot => {
                        col.swap(y, free_spot);
                        free_spot += 1
                    }
                    '#' => free_spot = y + 1,
                    _ => free_spot += 1,
                }
            }
        }
    }

    fn spin_cycles(&mut self, cycles: usize) {
        let mut dirs = [
            SettleDir::North,
            SettleDir::West,
            SettleDir::South,
            SettleDir::East,
        ]
        .iter()
        .cycle()
        .take(4 * cycles);

        let mut rotation = 0;
        let mut history = Vec::new();
        let mut min_rotations_required = 0;
        history.push(self.data.clone());
        for dir in dirs.by_ref() {
            self.settle(dir);
            rotation += 1;
            let new_state = self.data.clone();
            if history.contains(&new_state) {
                let cycle_start = history.iter().position(|h| h == new_state).unwrap();
                let cycle_lenght = rotation - cycle_start;
                let rotations_left = 4 * cycles - rotation;
                min_rotations_required = rotations_left % cycle_lenght;
                break;
            }
            history.push(new_state);
        }

        dirs.take(min_rotations_required)
            .for_each(|dir| self.settle(dir));
    }

    fn load(&self) -> usize {
        let mask = self.data.map(|el| (el == &'O') as usize);
        let weights = Array1::from_iter((1..=mask.dim().0).rev());
        (weights * mask.reversed_axes()).sum()
    }
}

impl AocTask for Day14 {
    fn directory(&self) -> PathBuf {
        "tasks/day_14".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let mut platform = Platform::new(input);
        match phase {
            1 => platform.settle(&SettleDir::North),
            2 => platform.spin_cycles(10_usize.pow(9)),
            _ => unimplemented!(),
        }
        platform.load().solved()
    }
}
