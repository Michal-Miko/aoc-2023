use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    ops::{Add, Sub},
    path::PathBuf,
};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;

pub struct Day10;

enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = match self {
            Tile::Vertical => '┃',
            Tile::Horizontal => '━',
            Tile::NorthEast => '┗',
            Tile::NorthWest => '┛',
            Tile::SouthWest => '┓',
            Tile::SouthEast => '┏',
            Tile::Ground => '░',
            Tile::Start => '╳',
        };
        write!(f, "{chr}")
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::Start,
            _ => Self::Ground,
        }
    }
}

impl Tile {
    fn connections(&self) -> Vec<Pos> {
        match self {
            Tile::NorthWest => vec![Pos::new(-1, 0), Pos::new(0, -1)],
            Tile::SouthWest => vec![Pos::new(-1, 0), Pos::new(0, 1)],
            Tile::Horizontal => vec![Pos::new(-1, 0), Pos::new(1, 0)],
            Tile::Vertical => vec![Pos::new(0, -1), Pos::new(0, 1)],
            Tile::NorthEast => vec![Pos::new(0, -1), Pos::new(1, 0)],
            Tile::SouthEast => vec![Pos::new(0, 1), Pos::new(1, 0)],
            Tile::Start => vec![
                Pos::new(-1, 0),
                Pos::new(0, -1),
                Pos::new(0, 1),
                Pos::new(1, 0),
            ],
            _ => vec![],
        }
    }

    fn from_connections(mut connections: Vec<Pos>) -> Self {
        connections.sort();
        if connections == Tile::Vertical.connections() {
            Tile::Vertical
        } else if connections == Tile::Horizontal.connections() {
            Tile::Horizontal
        } else if connections == Tile::NorthEast.connections() {
            Tile::NorthEast
        } else if connections == Tile::NorthWest.connections() {
            Tile::NorthWest
        } else if connections == Tile::SouthWest.connections() {
            Tile::SouthWest
        } else if connections == Tile::SouthEast.connections() {
            Tile::SouthEast
        } else {
            Tile::Ground
        }
    }

    fn is_vertical_edge(&self) -> bool {
        matches!(
            self,
            Tile::Vertical | Tile::NorthEast | Tile::NorthWest | Tile::SouthWest | Tile::SouthEast
        )
    }

    fn forms_left_diagonal_with(&self, other: &Tile) -> bool {
        match (self, other) {
            //┏┛ (reverse)
            (Tile::NorthWest, Tile::SouthEast) => true,
            //┗┓ (reverse)
            (Tile::SouthWest, Tile::NorthEast) => true,
            _ => false,
        }
    }
}

struct Field {
    tiles: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
    start: Pos,
    distances: HashMap<Pos, usize>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Field {
    fn new(input: AocStringIter) -> Self {
        let mut tiles: Vec<Vec<Tile>> = vec![];
        let mut start = None;

        for (y, row) in input.enumerate() {
            for (x, chr) in row.chars().enumerate() {
                let tile: Tile = chr.into();
                if matches!(tile, Tile::Start) && start.is_none() {
                    start = Some(Pos::new(x as i32, y as i32));
                }
            }
            tiles.push(row.chars().map_into().collect());
        }

        let height = tiles.len() as i32;
        let width = tiles[0].len() as i32;

        Self {
            tiles,
            width,
            height,
            start: start.unwrap_or(Pos::new(0, 0)),
            distances: HashMap::new(),
        }
    }

    fn compute_distances(&mut self) {
        self.distances.insert(self.start, 0);

        let mut current_tiles = vec![self.start];
        let mut next_tiles = vec![];

        let mut step = 1;
        loop {
            loop {
                if current_tiles.is_empty() {
                    break;
                }

                let pos = current_tiles.pop().unwrap();
                let connected_tiles = self.connections_at(&pos);

                for tile in connected_tiles {
                    if let Entry::Vacant(e) = self.distances.entry(tile) {
                        e.insert(step);
                        next_tiles.push(tile);
                    }
                }
            }

            if next_tiles.is_empty() {
                break;
            }

            std::mem::swap(&mut current_tiles, &mut next_tiles);
            step += 1;
        }
    }

    fn guess_start_tile(&mut self) {
        let start = self.start;
        let first_tile_connections = self
            .distances
            .iter()
            .filter_map(|(pos, dist)| (*dist == 1).then_some(*pos - start))
            .collect_vec();
        let start_tile = Tile::from_connections(first_tile_connections);
        self.tiles[start.y as usize][start.x as usize] = start_tile;
    }

    fn compute_area(&mut self) -> usize {
        self.guess_start_tile();
        let mut area = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x, y);
                if self.distances.contains_key(&pos) {
                    continue;
                }
                // If a tile is inside the loop, it will be surrounded by an odd number of edges on
                // any side
                if self.edges_to_the_left(&pos) % 2 == 1 {
                    area += 1;
                }
            }
        }
        area
    }

    fn edges_to_the_left(&self, pos: &Pos) -> usize {
        let dir = Pos::new(-1, 0);
        let mut edges = 0;
        let mut new = *pos + dir;
        let mut last_tile = &Tile::Ground;
        while new.x >= 0 && new.y >= 0 && new.x < self.width && new.y < self.height {
            let tile = self.tile_at(&new);
            // If two corners form a diagonoal, ignore the 2nd corner and count them as 1 wall
            if !last_tile.forms_left_diagonal_with(tile)
                && tile.is_vertical_edge()
                && self.distances.contains_key(&new)
            {
                edges += 1;
            }
            // Ignore horizontal pipes to check if the whole segment is a diagonal above
            if !matches!(tile, Tile::Horizontal) {
                last_tile = tile;
            }
            new = new + dir;
        }
        edges
    }

    fn tile_at<'own>(&'own self, pos: &Pos) -> &'own Tile {
        &self.tiles[pos.y as usize][pos.x as usize]
    }

    fn connections_at(&self, pos: &Pos) -> Vec<Pos> {
        self.tile_at(pos)
            .connections()
            .into_iter()
            .map(|connection| *pos + connection)
            .filter(|new| new.x >= 0 && new.y >= 0 && new.x < self.width && new.y < self.height)
            .filter(|new| self.tile_at(new).connections().contains(&(*pos - *new)))
            .collect()
    }

    #[allow(dead_code)]
    fn debug(&self, color: bool) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Some(dist) = self.distances.get(&Pos::new(x as i32, y as i32)) {
                    if color {
                        print!("{}", tile.to_string().yellow());
                    } else {
                        print!("{}", dist % 10)
                    }
                } else {
                    print!("{tile}");
                }
            }
            println!();
        }
    }
}

impl AocTask for Day10 {
    fn directory(&self) -> PathBuf {
        "tasks/day_10".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let mut field = Field::new(input);
        // field.debug(true);
        field.compute_distances();
        field.guess_start_tile();
        // field.debug(true);

        match phase {
            1 => field.distances.values().max().solved(),
            // Since the Start tile in the input data doesn't touch any tiles that are not part of the loop
            // and have an open connection to it, we can assume that all keys in field.distances are part of the loop.
            // This:
            //    ┏━━┓
            //  ┏━╳  ┃
            //  ┛ ┃  ┃
            //    ┗━━┛
            // does not happen.
            2 => field.compute_area().solved(),
            _ => unimplemented!(),
        }
    }
}
