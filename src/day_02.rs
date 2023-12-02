use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;
use thiserror::Error;

pub struct Day02;

enum Cubes {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Cubes {
    fn under_limit(&self, [red, green, blue]: &[usize; 3]) -> bool {
        match self {
            Cubes::Red(count) => count <= red,
            Cubes::Green(count) => count <= green,
            Cubes::Blue(count) => count <= blue,
        }
    }
}

#[derive(Debug, Error)]
enum Task02ParseError {
    #[error("Failed to parse Cubes: {0}")]
    Cubes(String),

    #[error("Failed to parse CubeSet: {0}")]
    CubeSet(String),

    #[error("Failed to parse Game: {0}")]
    Game(String),
}

impl TryFrom<&str> for Cubes {
    type Error = Task02ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (count, color): (&str, &str) = value
            .split_whitespace()
            .collect_tuple()
            .ok_or(Task02ParseError::Cubes(value.into()))?;

        match (color, count.parse::<usize>()) {
            ("red", Ok(number)) => Ok(Cubes::Red(number)),
            ("green", Ok(number)) => Ok(Cubes::Green(number)),
            ("blue", Ok(number)) => Ok(Cubes::Blue(number)),
            (_, _) => Err(Task02ParseError::Cubes(value.into())),
        }
    }
}

struct CubeSet(Vec<Cubes>);

impl TryFrom<&str> for CubeSet {
    type Error = Task02ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cubes: Vec<Cubes> = value
            .split(',')
            .map(|cubes| cubes.trim().try_into())
            .collect::<Result<_, _>>()?;

        if cubes.is_empty() {
            Err(Task02ParseError::CubeSet(value.into()))
        } else {
            Ok(Self(cubes))
        }
    }
}

struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn min_cubes_required(&self) -> [usize; 3] {
        self.cube_sets.iter().flat_map(|set| set.0.iter()).fold(
            [0, 0, 0],
            |acc: [usize; 3], cubes| match cubes {
                Cubes::Red(count) if *count > acc[0] => [*count, acc[1], acc[2]],
                Cubes::Green(count) if *count > acc[1] => [acc[0], *count, acc[2]],
                Cubes::Blue(count) if *count > acc[2] => [acc[0], acc[1], *count],
                _ => acc,
            },
        )
    }

    fn cubes_under_limits(&self) -> bool {
        self.cube_sets
            .iter()
            .all(|set| set.0.iter().all(|cubes| cubes.under_limit(&[12, 13, 14])))
    }
}

impl TryFrom<String> for Game {
    type Error = Task02ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (id_string, cube_sets_string) = value
            .split(':')
            .collect_tuple()
            .ok_or(Task02ParseError::Game(value.clone()))?;

        let id: usize = id_string
            .replace("Game ", "")
            .parse()
            .map_err(|_| Task02ParseError::Game(id_string.into()))?;

        let cube_sets: Vec<CubeSet> = cube_sets_string
            .split(';')
            .map(|cube_set| cube_set.trim().try_into())
            .collect::<Result<_, _>>()?;

        if cube_sets.is_empty() {
            Err(Task02ParseError::Game(cube_sets_string.into()))
        } else {
            Ok(Self { id, cube_sets })
        }
    }
}

impl AocTask for Day02 {
    fn directory(&self) -> PathBuf {
        "tasks/day_02".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let games = input.map(Game::try_from);
        match phase {
            1 => games
                .filter_ok(Game::cubes_under_limits)
                .map_ok(|game| game.id)
                .process_results(|games| games.sum::<usize>())
                .try_solved(),
            2 => games
                .map_ok(|game| game.min_cubes_required().iter().product::<usize>())
                .process_results(|games| games.sum::<usize>())
                .try_solved(),
            _ => unimplemented!(),
        }
    }
}
