use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign},
};

use crate::common::{AocError, AocResult};
use num::Integer;

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

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

#[derive(Debug, Clone)]
struct Rock {
    pub points: Vec<Point>,
}

impl Rock {
    pub fn new(points: &[Point]) -> Self {
        Self {
            points: Vec::from(points),
        }
    }

    pub fn drift(&mut self, delta: &Point) {
        for point in &mut self.points {
            *point += *delta;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Jet {
    Left,
    Right,
    Down,
}

impl Jet {
    pub fn point(&self) -> Point {
        match self {
            Self::Left => Point::new(-1, 0),
            Self::Right => Point::new(1, 0),
            Self::Down => Point::new(0, -1),
        }
    }
}

fn parse_jet_pattern(input: &str) -> AocResult<Vec<Jet>> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Ok(Jet::Left),
            '>' => Ok(Jet::Right),
            _ => Err(AocError::new(&format!("unexpected character: {c}"))),
        })
        .collect()
}

struct VerticalChamber {
    // Each row is stored as a byte, where 7 bits (up to the width) represent if a rock is present.
    // This optimization makes pattern matching for part B easier.
    map: Vec<u8>,
    // An array that keeps track of the height in each column. Used for hashing the current state.
    height_in_column: [u64; VerticalChamber::WIDTH as usize],
    jet_pattern: Vec<Jet>,
    rocks: Vec<Rock>,
    jet_pattern_index: usize,
    rock_index: usize,
}

impl VerticalChamber {
    const WIDTH: i64 = 7;

    pub fn default_rocks() -> Vec<Rock> {
        Vec::from([
            Rock::new(&[
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ]),
            Rock::new(&[
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(1, 2),
            ]),
            Rock::new(&[
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(2, 1),
                Point::new(2, 2),
            ]),
            Rock::new(&[
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
            ]),
            Rock::new(&[
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
            ]),
        ])
    }

    pub fn new(jet_pattern: Vec<Jet>, rocks: Vec<Rock>) -> Self {
        Self {
            map: Vec::from([u8::MAX]),
            height_in_column: [0; Self::WIDTH as usize],
            jet_pattern,
            rocks,
            jet_pattern_index: 0,
            rock_index: 0,
        }
    }

    pub fn height(&self) -> usize {
        self.map.len() - 1
    }

    pub fn rock_at(&self, point: &Point) -> bool {
        point.x <= 0
            || point.x >= Self::WIDTH + 1
            || point.y < 0
            || self.map.get(point.y as usize).unwrap_or(&0) & (1 << point.x) != 0
    }

    pub fn set_rock_at(&mut self, point: &Point) {
        if point.y as usize >= self.map.len() {
            self.map.resize(point.y as usize + 1, 0);
        }
        let height_at_x = &mut self.height_in_column[point.x as usize - 1];
        if point.y as u64 > *height_at_x {
            *height_at_x = point.y as u64;
        }
        self.map[point.y as usize] |= 1 << point.x
    }

    fn next_rock(&mut self) -> &Rock {
        let out = &self.rocks[self.rock_index];
        self.rock_index += 1;
        if self.rock_index >= self.rocks.len() {
            self.rock_index = 0;
        }
        out
    }

    fn next_jet_stream(&mut self) -> Jet {
        let out = if self.jet_pattern_index & 1 == 0 {
            self.jet_pattern[self.jet_pattern_index >> 1]
        } else {
            Jet::Down
        };
        self.jet_pattern_index += 1;
        if (self.jet_pattern_index >> 1) >= self.jet_pattern.len() {
            self.jet_pattern_index = 0;
        }
        out
    }

    fn hash_current_state(&self) -> u64 {
        // The current state is a combination of:
        //  - The current index in the jet pattern.
        //  - The current index in the rock pattern.
        //  - The height in each column, relative to the lowest height.
        //
        // We use the height in each column because the next rock cannot go below any of
        // these points without passing through a resting rock.
        let mut hasher = DefaultHasher::new();
        self.jet_pattern_index.hash(&mut hasher);
        self.rock_index.hash(&mut hasher);
        let lowest_column_height = self.height_in_column.iter().min().unwrap();
        for column_height in self
            .height_in_column
            .iter()
            .map(|height| height - lowest_column_height)
        {
            column_height.hash(&mut hasher);
        }
        hasher.finish()
    }

    pub fn place_rocks(&mut self, num_rocks: usize, look_for_cycle: bool) -> usize {
        // Keep track of which states have been seen, for cycle detection.
        let mut states_seen = HashMap::new();
        // Keep track of the height at each rock placed, for the remaining rocks that
        // must be placed after the last iteration cycle.
        let mut height_at_rocks_placed = Vec::new();
        for rock in 0..num_rocks {
            let mut current_rock = self.next_rock().clone();

            // Move rock to initial point.
            current_rock.drift(&Point::new(2 + 1, self.height() as i64 + 3 + 1));

            if look_for_cycle {
                height_at_rocks_placed.push(self.height());

                let current_state = self.hash_current_state();
                match states_seen.insert(current_state, rock) {
                    None => (),
                    Some(rocks_placed_at_start_of_cycle) => {
                        let cycle_length_in_rocks = rock - rocks_placed_at_start_of_cycle;
                        let rocks_remaining_to_be_placed = num_rocks - rock;

                        let (repeats, remaining) =
                            rocks_remaining_to_be_placed.div_mod_floor(&cycle_length_in_rocks);

                        let height_at_start_of_cycle =
                            height_at_rocks_placed[rocks_placed_at_start_of_cycle];
                        let height_added_in_cycle = self.height() - height_at_start_of_cycle;
                        let height_from_cycles = repeats * height_added_in_cycle;

                        let height_after_cycle = height_at_rocks_placed
                            [rocks_placed_at_start_of_cycle + remaining]
                            - height_at_start_of_cycle;

                        return self.height() + height_from_cycles + height_after_cycle;
                    }
                }
            }

            loop {
                let direction = self.next_jet_stream();
                let delta = direction.point();
                let blocked = current_rock
                    .points
                    .iter()
                    .map(|point| point + &delta)
                    .any(|next_point| self.rock_at(&next_point));

                if !blocked {
                    current_rock.drift(&delta);
                } else if direction == Jet::Down {
                    // Rock has come to rest.
                    break;
                }
            }

            for point in current_rock.points {
                self.set_rock_at(&point);
            }
        }

        self.height()
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let jet_pattern = parse_jet_pattern(input)?;
    let mut chamber = VerticalChamber::new(jet_pattern, VerticalChamber::default_rocks());
    Ok(chamber.place_rocks(2022, true) as u64)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    let jet_pattern = parse_jet_pattern(input)?;
    let mut chamber = VerticalChamber::new(jet_pattern, VerticalChamber::default_rocks());
    Ok(chamber.place_rocks(1_000_000_000_000, true) as u64)
}
