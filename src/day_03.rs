use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::{repeat_n, Itertools};

pub struct Day03;

#[derive(Debug, Clone)]
enum EntityType {
    Number(usize),
    PartNumber(usize),
    Symbol(char),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Entity {
    pos: Pos,
    len: usize,
    r#type: EntityType,
}

impl<'entity> Entity {
    fn new(end_x: usize, y: usize, len: usize, r#type: EntityType) -> Self {
        Self {
            pos: Pos {
                x: (end_x + 1 - len) as i32,
                y: y as i32,
            },
            len,
            r#type,
        }
    }

    fn occupied_positions(&'entity self) -> impl Iterator<Item = Pos> + 'entity {
        (0..self.len).map(|offset| Pos {
            x: self.pos.x + offset as i32,
            y: self.pos.y,
        })
    }

    fn adjecent_positions(&'entity self) -> impl Iterator<Item = Pos> + 'entity {
        repeat_n(-1..=1, 2)
            .multi_cartesian_product()
            .filter(|pos| pos != &vec![0, 0])
            .map(|vec| Pos {
                x: vec[0] + self.pos.x,
                y: vec[1] + self.pos.y,
            })
    }
}

#[derive(Debug)]
struct Schematic {
    numbers_by_pos: HashMap<Pos, Rc<RefCell<Entity>>>,
    numbers: Vec<Rc<RefCell<Entity>>>,
    symbols: Vec<Entity>,
}

impl<'src> TryFrom<AocStringIter<'src>> for Schematic {
    type Error = BoxedError;

    fn try_from(value: AocStringIter) -> Result<Self, Self::Error> {
        let mut schematic = Self::new();

        // Parse all entities
        for (y, line) in value.into_iter().enumerate() {
            let mut number = "".to_string();
            for (x, chr) in line.chars().enumerate() {
                match chr {
                    // Number
                    '0'..='9' if x < line.len() - 1 => number.push(chr),
                    // Last number on the line
                    '0'..='9' => {
                        number.push(chr);
                        schematic.add_number(x, y, std::mem::take(&mut number))?;
                    }
                    // Symbols/empty spaces
                    symbol => {
                        if !number.is_empty() {
                            schematic.add_number(x - 1, y, std::mem::take(&mut number))?;
                        }
                        if symbol != '.' {
                            let entity = Entity::new(x, y, 1, EntityType::Symbol(chr));
                            schematic.symbols.push(entity);
                        }
                    }
                }
            }
        }

        // Find and mark all PartNumber entities
        schematic.mark_part_numbers();
        Ok(schematic)
    }
}

impl Schematic {
    fn new() -> Self {
        Self {
            numbers_by_pos: HashMap::new(),
            numbers: vec![],
            symbols: vec![],
        }
    }

    fn add_number(&mut self, x: usize, y: usize, number: String) -> Result<(), BoxedError> {
        // Store the number
        let cell = Rc::new(RefCell::new(Entity::new(
            x,
            y,
            number.len(),
            EntityType::Number(number.parse()?),
        )));
        self.numbers.push(cell.clone());

        // Mark positions occupied by this number with references
        cell.borrow().occupied_positions().for_each(|pos| {
            self.numbers_by_pos.insert(pos, cell.clone());
        });

        Ok(())
    }

    fn mark_part_numbers(&mut self) {
        for symbol in self.symbols.iter() {
            symbol
                .adjecent_positions()
                .flat_map(|pos| self.numbers_by_pos.get(&pos))
                .for_each(|entity| {
                    let current_type = &mut entity.borrow_mut().r#type;
                    if let EntityType::Number(number) = current_type {
                        *current_type = EntityType::PartNumber(*number);
                    }
                });
        }
    }

    fn part_number_checksum(&self) -> usize {
        self.numbers
            .iter()
            .filter_map(|number| match number.borrow().r#type {
                EntityType::PartNumber(value) => Some(value),
                _ => None,
            })
            .sum()
    }

    fn gear_ratio_sum(&self) -> usize {
        self.symbols
            .iter()
            // Filter out non-* Symbols, get the values of adjecent PartNumbers
            .filter_map(|symbol| match symbol.r#type {
                EntityType::Symbol('*') => Some(
                    symbol
                        .adjecent_positions()
                        .flat_map(|pos| self.numbers_by_pos.get(&pos))
                        .unique_by(|rc| rc.as_ptr())
                        .map(|entity| match entity.borrow().clone() {
                            Entity {
                                pos,
                                len,
                                r#type: EntityType::PartNumber(value),
                            } => value,
                            _ => 0,
                        }),
                ),
                _ => None,
            })
            // Filter out * Symbols with wrong number of parts
            .filter_map(|candidate_part_numbers| {
                let part_numbers: Vec<_> = candidate_part_numbers.collect();
                match part_numbers.len() {
                    2 => Some(part_numbers.iter().product::<usize>()),
                    _ => None,
                }
            })
            .sum()
    }
}

impl AocTask for Day03 {
    fn directory(&self) -> PathBuf {
        "tasks/day_03".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let schematic: Schematic = input.try_into()?;

        match phase {
            1 => schematic.part_number_checksum().solved(),
            2 => schematic.gear_ratio_sum().solved(),
            _ => unimplemented!(),
        }
    }
}
