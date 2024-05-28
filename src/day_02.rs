use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;
use winnow::{
    ascii::{alpha1, digit1, multispace0, multispace1},
    combinator::{dispatch, empty, fail, preceded, separated, separated_pair, terminated},
    PResult, Parser,
};

pub struct Day02;

#[derive(Clone)]
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

struct CubeSet(Vec<Cubes>);

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

fn parse_cubes(input: &mut &str) -> PResult<Cubes> {
    dispatch!(
        separated_pair(digit1.parse_to::<usize>(), multispace1, alpha1);
        (count, "red") => empty.value(Cubes::Red(count)),
        (count, "green") => empty.value(Cubes::Green(count)),
        (count, "blue") => empty.value(Cubes::Blue(count)),
        _ => fail,
    )
    .parse_next(input)
}

fn parse_cubeset(input: &mut &str) -> PResult<CubeSet> {
    let cube_set = separated(1.., parse_cubes, terminated(',', multispace0)).parse_next(input)?;
    Ok(CubeSet(cube_set))
}

fn parse_game(input: &mut &str) -> PResult<Game> {
    let (id, cube_sets) = preceded(
        ("Game", multispace1),
        separated_pair(
            digit1.parse_to::<usize>(),
            terminated(':', multispace0),
            separated(1.., parse_cubeset, terminated(';', multispace0)),
        ),
    )
    .parse_next(input)?;

    Ok(Game { id, cube_sets })
}

impl AocTask for Day02 {
    fn directory(&self) -> PathBuf {
        "tasks/day_02".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let games = input.map(|line| parse_game.parse(&line).map_err(|e| e.to_string()));
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
