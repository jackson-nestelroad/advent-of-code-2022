use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::common::{AocError, AocResult, IntoAocResult};
use itertools::Itertools;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Tile {
    Rock,
    Sand,
}

type Point = (u64, u64);
type Delta = (i64, i64);

trait Transform<T>
where
    Self: Sized,
{
    fn transform(&self, delta: &T) -> Option<Self>;
}

impl Transform<Delta> for Point {
    fn transform(&self, delta: &Delta) -> Option<Self> {
        Some((
            u64::try_from((self.0 as i64).checked_add(delta.0)?).ok()?,
            u64::try_from((self.1 as i64).checked_add(delta.1)?).ok()?,
        ))
    }
}

struct CaveMap {
    map: HashMap<Point, Tile>,
    deepest: u64,
    floor: bool,
}

impl FromStr for CaveMap {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let mut map = HashMap::new();
        for line in s.lines() {
            let coords = line
                .split("->")
                .map(|coord| {
                    coord
                        .trim()
                        .split_once(',')
                        .into_aoc_result_msg("invalid coordinates")
                        .and_then(|(x, y)| {
                            Ok((
                                x.parse::<u64>()
                                    .into_aoc_result_msg("invalid x coordinate")?,
                                y.parse::<u64>()
                                    .into_aoc_result_msg("invalid y coordinate")?,
                            ))
                        })
                })
                .collect::<AocResult<Vec<_>>>()?;
            for (from, to) in coords.iter().tuple_windows() {
                match (from, to) {
                    ((x1, y1), (x2, y2)) if x1 == x2 => {
                        for y in *y1.min(y2)..=(*y1.max(y2)) {
                            map.insert((*x1, y), Tile::Rock);
                        }
                    }
                    ((x1, y1), (x2, y2)) if y1 == y2 => {
                        for x in *x1.min(x2)..=(*x1.max(x2)) {
                            map.insert((x, *y1), Tile::Rock);
                        }
                    }
                    _ => return Err(AocError::new("cannot draw diagonal wall")),
                }
            }
        }
        Self::from_map(map)
    }
}

impl CaveMap {
    pub fn from_map(map: HashMap<Point, Tile>) -> AocResult<Self> {
        let deepest = map
            .iter()
            .max_by_key(|((_, y), _)| y)
            .into_aoc_result_msg("failed to find deepest height in cave")?
            .0
             .1;
        Ok(Self {
            map,
            deepest,
            floor: false,
        })
    }

    pub fn add_floor(&mut self) {
        self.floor = true;
    }

    pub fn get(&self, point: &Point) -> Option<Tile> {
        if self.floor && point.1 == self.deepest + 2 {
            Some(Tile::Rock)
        } else {
            self.map.get(point).copied()
        }
    }

    pub fn set(&mut self, point: &Point, tile: Tile) {
        self.map.insert(*point, tile);
    }

    const SAND_MOVES: [Delta; 3] = [(0, 1), (-1, 1), (1, 1)];

    fn pour_sand(&mut self, source: Point) -> AocResult<u64> {
        let mut sand_count = 0;
        // We keep a stack of the current path. Once a single piece of sand has come to
        // rest, the next piece immediately starts at the previous position.
        let mut path = VecDeque::from([source]);
        'outer: loop {
            let resting_position;
            'inner: loop {
                // Current sand position.
                let sand_position = path
                    .back()
                    .into_aoc_result_msg("missing last sand position")?;

                if !self.floor && sand_position.1 > self.deepest {
                    // This piece of sand will begin falling infinitely.
                    break 'outer;
                }

                // Find the first move that puts us in an open space.
                match Self::SAND_MOVES
                    .iter()
                    .map(|delta| sand_position.transform(delta))
                    .find(|pos| match pos {
                        None => false,
                        Some(pos) => self.get(pos).is_none(),
                    }) {
                    // Found a new position to move to.
                    Some(Some(pos)) => path.push_back(pos),
                    // Failed to find a new position; this sand is at rest.
                    _ => {
                        // Unwrap is safe here because we checked that the back exists at the
                        // beginning of this loop iteration.
                        resting_position = path.pop_back().unwrap();
                        break 'inner;
                    }
                }
            }

            sand_count += 1;
            self.set(&resting_position, Tile::Sand);

            if resting_position == source {
                // This piece of sand did not move, so the source is covered.
                break 'outer;
            }
        }
        Ok(sand_count)
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    const SAND_SOURCE: Point = (500, 0);
    let mut cave = CaveMap::from_str(input)?;
    cave.pour_sand(SAND_SOURCE)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    const SAND_SOURCE: Point = (500, 0);
    let mut cave = CaveMap::from_str(input)?;
    cave.add_floor();
    cave.pour_sand(SAND_SOURCE)
}
