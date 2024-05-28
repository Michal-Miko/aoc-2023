use std::{collections::HashMap, iter::once, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use winnow::{
    ascii::{alphanumeric1, multispace0},
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

    fn follow_many(
        &self,
        mut path: Path,
        start_char: char,
        target_char: char,
    ) -> Option<Vec<usize>> {
        let mut current_nodes: Vec<_> = self
            .nodes
            .keys()
            .filter(|location| location.ends_with(start_char))
            .collect();

        let mut next_nodes = vec![];
        let mut cycles = vec![];
        let mut steps = 0;
        while !current_nodes.is_empty() {
            steps += 1;
            let step = path.0.next()?;
            while !current_nodes.is_empty() {
                let current_node = current_nodes.pop()?;
                let directions = self.nodes.get(current_node)?;
                let next_node = match step {
                    Instruction::Left => &directions.0,
                    Instruction::Right => &directions.1,
                };
                if next_node.ends_with(target_char) {
                    cycles.push(steps);
                } else {
                    next_nodes.push(next_node);
                }
            }
            std::mem::swap(&mut next_nodes, &mut current_nodes);
        }
        Some(cycles)
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
        alphanumeric1.output_into(),
        (multispace0, '=', multispace0),
        delimited(
            '(',
            separated_pair(
                alphanumeric1.output_into(),
                (',', multispace0),
                alphanumeric1.output_into(),
            ),
            ')',
        ),
    )
    .parse_next(input)
}

trait Lcm
where
    Self: Sized,
{
    fn prime_factors(&self) -> HashMap<Self, Self>;
    #[allow(dead_code)]
    fn lcm(&self, other: &Self) -> Self;
}

impl Lcm for usize {
    fn prime_factors(&self) -> HashMap<Self, Self> {
        assert_ne!(self, &0usize);

        let mut factors = HashMap::new();
        let mut number = *self;

        for factor in once(2).chain((3..).step_by(2)) {
            while number % factor == 0 {
                factors
                    .entry(factor)
                    .and_modify(|power| *power *= factor)
                    .or_insert(factor);
                number /= factor;
            }

            if number == 1 {
                break;
            }
        }

        factors
    }

    fn lcm(&self, other: &Self) -> Self {
        if self == &0usize && self == other {
            0
        } else {
            [*self, *other].lcm()
        }
    }
}

trait LcmMany<L>
where
    L: Lcm,
{
    fn lcm(&self) -> L;
}

impl<I> LcmMany<usize> for I
where
    for<'a> &'a I: IntoIterator<Item = &'a usize>,
{
    fn lcm(&self) -> usize {
        let highest_power_factors = self
            .into_iter()
            .flat_map(|number| number.prime_factors())
            .fold(
                HashMap::<usize, usize>::new(),
                |mut acc, (factor, power)| {
                    acc.entry(factor)
                        .and_modify(|max_power| *max_power = power.max(*max_power))
                        .or_insert(power);
                    acc
                },
            );
        highest_power_factors.values().product()
    }
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

        match phase {
            1 | 2 => map
                .follow_many(path, 'A', 'Z')
                .ok_or("Invalid path")?
                .lcm()
                .solved(),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod phase_2 {
    use winnow::Parser;

    use super::{parse_node, parse_path, LcmMany, Map};

    #[test]
    fn lcm() {
        assert_eq!(vec![8, 9, 21].lcm(), 504)
    }

    #[test]
    fn phase_specific_example() {
        let header = "LR";
        let nodes = "11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let path = parse_path.parse(header).unwrap();
        let map = Map::new(
            nodes
                .split('\n')
                .skip_while(|line| line.is_empty())
                .map(|line| parse_node.parse(line).unwrap())
                .collect::<Vec<_>>(),
        );
        let paths_to_cycles = map.follow_many(path, 'A', 'Z').unwrap();
        assert_eq!(paths_to_cycles, vec![2, 3]);

        let result = paths_to_cycles.lcm();
        assert_eq!(result, 6);
    }
}
