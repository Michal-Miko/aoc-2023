use std::{collections::HashMap, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use winnow::{
    ascii::{alpha1, multispace0},
    combinator::{delimited, fail, repeat, separated_pair, success},
    dispatch,
    token::any,
    PResult, Parser,
};

pub struct Day08;

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

struct Path(Box<dyn Iterator<Item = Instruction>>);

type NodeDirections = (String, String);
type Node = (String, NodeDirections);

#[derive(Debug)]
struct Map {
    nodes: HashMap<String, NodeDirections>,
}

impl Map {
    fn new(nodes: Vec<Node>) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
        }
    }

    fn follow(&self, mut path: Path, start: &str, target: &str) -> Option<usize> {
        let mut current = start;
        let mut steps = 0;
        while current != target {
            let step = path.0.next();
            let directions = self.nodes.get(current)?;
            // println!("{current} - {directions:?} - {step:?}");
            current = match step? {
                Instruction::Left => &directions.0,
                Instruction::Right => &directions.1,
            };
            steps += 1;
        }
        Some(steps)
    }
}

fn parse_path(input: &mut &str) -> PResult<Path> {
    let instructions: Vec<Instruction> = repeat(
        1..,
        dispatch!(any;
            'L' => success(Instruction::Left),
            'R' => success(Instruction::Right),
            _ => fail,
        ),
    )
    .parse_next(input)?;
    Ok(Path(Box::new(instructions.into_iter().cycle())))
}

fn parse_node(input: &mut &str) -> PResult<Node> {
    separated_pair(
        alpha1.output_into(),
        (multispace0, '=', multispace0),
        delimited(
            '(',
            separated_pair(
                alpha1.output_into(),
                (',', multispace0),
                alpha1.output_into(),
            ),
            ')',
        ),
    )
    .parse_next(input)
}

impl AocTask for Day08 {
    fn directory(&self) -> PathBuf {
        "tasks/day_08".into()
    }

    fn solution(&self, mut input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let path = input
            .next()
            .map(|line| parse_path.parse(&line).map_err(|e| e.to_string()))
            .ok_or("Missing path")??;

        let map = Map::new(
            input
                .skip_while(|line| line.is_empty())
                .map(|line| parse_node.parse(&line).map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?,
        );

        map.follow(path, "AAA", "ZZZ")
            .ok_or("Invalid path")
            .solved()
    }
}
