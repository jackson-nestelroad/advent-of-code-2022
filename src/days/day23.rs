use std::{ops::Add, str::FromStr};

use crate::common::{AocError, AocResult};
use lazy_static::lazy_static;
use num::ToPrimitive;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Clone, Copy, ToPrimitive)]
#[repr(u8)]
enum Direction {
    North = 0b0001,
    South = 0b0010,
    West = 0b0100,
    East = 0b1000,
    NorthWest = 0b0101,
    NorthEast = 0b1001,
    SouthWest = 0b0110,
    SouthEast = 0b1010,
}

impl Direction {
    pub fn has_north_component(&self) -> bool {
        self.to_u8().unwrap() & Self::North.to_u8().unwrap() != 0
    }

    pub fn has_south_component(&self) -> bool {
        self.to_u8().unwrap() & Self::South.to_u8().unwrap() != 0
    }

    pub fn has_west_component(&self) -> bool {
        self.to_u8().unwrap() & Self::West.to_u8().unwrap() != 0
    }

    pub fn has_east_component(&self) -> bool {
        self.to_u8().unwrap() & Self::East.to_u8().unwrap() != 0
    }

    pub fn delta(&self) -> Point {
        Point::new(
            if self.has_west_component() {
                -1
            } else if self.has_east_component() {
                1
            } else {
                0
            },
            if self.has_north_component() {
                -1
            } else if self.has_south_component() {
                1
            } else {
                0
            },
        )
    }

    pub fn index(&self) -> usize {
        match self {
            Self::North => 0,
            Self::South => 1,
            Self::West => 2,
            Self::East => 3,
            Self::NorthWest => 4,
            Self::NorthEast => 5,
            Self::SouthWest => 6,
            Self::SouthEast => 7,
        }
    }

    pub fn bit(&self) -> u8 {
        1 << self.index()
    }
}

struct Grove {
    elves: FxHashSet<Point>,
}

impl FromStr for Grove {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        Ok(Self {
            elves: s
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.char_indices().filter_map(move |(x, c)| match c {
                        '#' => Some(Ok(Point::new(x as i64, y as i64))),
                        '.' => None,
                        _ => Some(Err(AocError::new("invalid character"))),
                    })
                })
                .collect::<AocResult<_>>()?,
        })
    }
}

impl Grove {
    pub fn do_rounds(&mut self, max: u64) -> u64 {
        for round in 0..max {
            if self.do_round(round) {
                return round + 1;
            }
        }
        return u64::MAX;
    }

    fn proposals() -> &'static [(Direction, u8); 4] {
        lazy_static! {
            static ref PROPOSALS: [(Direction, u8); 4] = [
                (
                    Direction::North,
                    Direction::North.bit()
                        | Direction::NorthEast.bit()
                        | Direction::NorthWest.bit()
                ),
                (
                    Direction::South,
                    Direction::South.bit()
                        | Direction::SouthEast.bit()
                        | Direction::SouthWest.bit()
                ),
                (
                    Direction::West,
                    Direction::West.bit() | Direction::NorthWest.bit() | Direction::SouthWest.bit()
                ),
                (
                    Direction::East,
                    Direction::East.bit() | Direction::NorthEast.bit() | Direction::SouthEast.bit()
                )
            ];
        }
        &PROPOSALS
    }

    fn neighbors(&self, point: &Point) -> u8 {
        lazy_static! {
            static ref ALL_DIRECTIONS: [Direction; 8] = [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::NorthWest,
                Direction::NorthEast,
                Direction::SouthWest,
                Direction::SouthEast
            ];
        }
        let mut neighbors = 0;
        for direction in *ALL_DIRECTIONS {
            if self.elves.contains(&(*point + direction.delta())) {
                neighbors |= direction.bit();
            }
        }
        neighbors
    }

    fn get_proposal(&self, point: &Point, round: u64) -> Option<Direction> {
        match self.neighbors(point) {
            0 => None,
            neighbors @ _ => (0..Self::proposals().len())
                .map(|i| Self::proposals()[(i + round as usize) % Self::proposals().len()])
                .find_map(|(direction, bits)| (neighbors & bits == 0).then_some(direction)),
        }
    }

    fn do_round(&mut self, round: u64) -> bool {
        let mut new_elves =
            FxHashSet::with_capacity_and_hasher(self.elves.capacity(), Default::default());
        let mut finished = true;
        for elf in &self.elves {
            match self.get_proposal(elf, round) {
                None => {
                    new_elves.insert(*elf);
                }
                Some(proposal) => {
                    finished = false;
                    let move_to = *elf + proposal.delta();
                    if !new_elves.insert(move_to) {
                        // This position has already been proposed by another elf.
                        //
                        // Conflicts must come from opposite directions, and there can only be one
                        // conflict for one space:
                        //
                        // If there are more than two elves one step away from a single position,
                        // then at least one of those elves is directly diagonal to another, which
                        // means this position cannot be proposed by either of those elves, which is
                        // a contradiction.
                        new_elves.remove(&move_to);
                        // Push the elf back that moved to this position.
                        new_elves.insert(move_to + proposal.delta());
                        new_elves.insert(*elf);
                    }
                }
            }
        }

        self.elves = new_elves;
        finished
    }

    pub fn bounding_rectangle_area(&self) -> u64 {
        let min_x = self.elves.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let max_x = self.elves.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let min_y = self.elves.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        let max_y = self.elves.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        ((max_x - min_x + 1) * (max_y - min_y + 1)) as u64
    }

    pub fn num_elves(&self) -> u64 {
        self.elves.len() as u64
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let mut grove = Grove::from_str(input)?;
    grove.do_rounds(10);
    Ok(grove.bounding_rectangle_area() - grove.num_elves())
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    let mut grove = Grove::from_str(input)?;
    Ok(grove.do_rounds(u64::MAX))
}
