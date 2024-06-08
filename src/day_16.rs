use std::{
    collections::{HashMap, VecDeque},
    iter::repeat,
    path::PathBuf,
};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct Day16;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

impl Direction {
    fn horizontal(&self) -> bool {
        matches!(self, East | West)
    }

    fn opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Clone)]
struct Beam {
    head: (i32, i32),
    direction: Direction,
}

impl Beam {
    fn new(x: i32, y: i32, dir: Direction) -> Self {
        Self {
            head: (x, y),
            direction: dir,
        }
    }

    fn next(mut self, tiles: &mut Tiles, hit_mirror: bool) -> Option<Self> {
        tiles.energize(&self);

        let offset = match self.direction {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        };
        let new_head = (self.head.0 + offset.0, self.head.1 + offset.1);
        if new_head.0 < 0
            || new_head.1 < 0
            || new_head.0 >= tiles.width
            || new_head.1 >= tiles.height
        {
            return None;
        }

        self.head = new_head;

        if hit_mirror {
            tiles.is_new_reflection(&self).then_some(self)
        } else {
            Some(self)
        }
    }

    fn turn(mut self, dir: Direction) -> Self {
        self.direction = dir;
        self
    }
}

struct Tiles {
    data: Vec<Vec<char>>,
    // Holds information about the directions form which the mirrors have been energized already
    energized_mirrors: HashMap<(i32, i32), Vec<Direction>>,
    energized_tiles: Vec<Vec<bool>>,
    width: i32,
    height: i32,
}

impl Tiles {
    fn new(input: AocStringIter) -> Self {
        let data: Vec<Vec<_>> = input.map(|str| str.chars().collect()).collect();
        let height = data.len();
        let width = data
            .first()
            .expect("Tiles should contain at least one row")
            .len();
        Self {
            data,
            width: width as i32,
            height: height as i32,
            energized_mirrors: HashMap::new(),
            energized_tiles: vec![vec![false; width]; height],
        }
    }

    fn reset(&mut self) {
        self.energized_mirrors.clear();
        self.energized_tiles
            .iter_mut()
            .for_each(|row| row.fill(false));
    }

    fn tile(&self, beam: &Beam) -> char {
        self.data[beam.head.1 as usize][beam.head.0 as usize]
    }

    fn energize(&mut self, beam: &Beam) {
        self.energized_tiles[beam.head.1 as usize][beam.head.0 as usize] = true
    }

    fn is_new_reflection(&mut self, beam: &Beam) -> bool {
        if let Some(ref mut dirs) = self.energized_mirrors.get_mut(&beam.head) {
            if dirs.contains(&beam.direction.opposite()) {
                false
            } else {
                dirs.push(beam.direction.opposite());
                true
            }
        } else {
            self.energized_mirrors
                .insert(beam.head, vec![beam.direction.opposite()]);
            true
        }
    }

    fn simulate_beam(&mut self, beam: Beam) -> usize {
        let mut beams_to_check: VecDeque<Beam> = VecDeque::from(vec![beam]);
        while let Some(beam) = beams_to_check.pop_front() {
            match self.tile(&beam) {
                '.' => {
                    let next = beam.next(self, false);
                    [next, None]
                }
                '|' => match beam.direction.horizontal() {
                    true => {
                        let north = beam.clone().turn(North).next(self, true);
                        let south = beam.turn(South).next(self, true);
                        [north, south]
                    }
                    false => [beam.next(self, true), None],
                },
                '-' => match beam.direction.horizontal() {
                    true => [beam.next(self, true), None],
                    false => {
                        let east = beam.clone().turn(East).next(self, true);
                        let west = beam.turn(West).next(self, true);
                        [east, west]
                    }
                },
                '/' => {
                    let reflected = match beam.direction {
                        North => beam.turn(East),
                        East => beam.turn(North),
                        South => beam.turn(West),
                        West => beam.turn(South),
                    }
                    .next(self, true);
                    [reflected, None]
                }
                '\\' => {
                    let reflected = match beam.direction {
                        North => beam.turn(West),
                        East => beam.turn(South),
                        South => beam.turn(East),
                        West => beam.turn(North),
                    }
                    .next(self, true);
                    [reflected, None]
                }
                _ => [None, None],
            }
            .into_iter()
            .flatten()
            .for_each(|new_beam| beams_to_check.push_back(new_beam));
        }

        let result = self
            .energized_tiles
            .iter()
            .map(|row| row.iter().filter(|tile| **tile).count())
            .sum();
        self.reset();
        result
    }
}

impl AocTask for Day16 {
    fn directory(&self) -> PathBuf {
        "tasks/day_16".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let mut tiles = Tiles::new(input);

        match phase {
            1 => tiles.simulate_beam(Beam::new(0, 0, East)),
            2 => {
                let left = repeat(0).zip(0..tiles.height).zip(repeat(East));
                let right = repeat(tiles.width - 1)
                    .zip(0..tiles.height)
                    .zip(repeat(West));
                let top = (0..tiles.width).zip(repeat(0)).zip(repeat(South));
                let bottom = (0..tiles.width)
                    .zip(repeat(tiles.height - 1))
                    .zip(repeat(North));
                let beams = left.chain(right).chain(top).chain(bottom);
                beams
                    .map(|((x, y), dir)| tiles.simulate_beam(Beam::new(x, y, dir)))
                    .max()
                    .unwrap_or_default()
            }
            _ => unimplemented!(),
        }
        .solved()
    }
}
