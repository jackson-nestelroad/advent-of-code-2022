use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use num::Integer;

use crate::common::{AocError, AocResult, IntoAocResult};

type Point = (u64, u64, u64);
type Delta = (i64, i64, i64);

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
            u64::try_from((self.2 as i64).checked_add(delta.2)?).ok()?,
        ))
    }
}

struct Heightmap {
    pub flat_map: Vec<u64>,
    pub width: usize,
    pub start: Point,
    pub end: Point,
}

impl FromStr for Heightmap {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let start = RefCell::new((0, 0, 0));
        let end = RefCell::new((0, 0, 0));
        let height = s.lines().count();
        let width = s.lines().next().into_aoc_result_msg("no row")?.len();
        let mut flat_map = Vec::new();
        flat_map.reserve(height * width);
        // No iterators because of the mutable references to the start and end
        // locations.
        for (y, line) in s.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                let h = match b {
                    b'S' => {
                        let h = b'a' - b'a';
                        *start.borrow_mut() = (x as u64, y as u64, h as u64);
                        h
                    }
                    b'E' => {
                        let h = b'z' - b'a';
                        *end.borrow_mut() = (x as u64, y as u64, h as u64);
                        h
                    }
                    b'a'..=b'z' => b - b'a',
                    _ => {
                        return Err(AocError::new(&format!(
                            "invalid byte in heightmap: {}",
                            b as char
                        )))
                    }
                };
                flat_map.push(h as u64)
            }
        }
        Ok(Heightmap {
            flat_map,
            width,
            start: start.into_inner(),
            end: end.into_inner(),
        })
    }
}

const MOVES: [Delta; 4] = [(-1, 0, 0), (0, -1, 0), (1, 0, 0), (0, 1, 0)];

struct Neighbors<'a> {
    location: &'a Point,
    i: usize,
    end: usize,
}

impl<'a> Neighbors<'a> {
    pub fn new(location: &'a Point) -> Self {
        Self {
            location,
            i: 0,
            end: MOVES.len(),
        }
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.end {
            let option = MOVES
                .get(self.i)
                .and_then(|delta| self.location.transform(delta));
            self.i += 1;
            if let Some(point) = option {
                return Some(point);
            }
        }
        None
    }
}

trait ExploreNeighbors {
    fn explore_neighbors<'a>(&'a self) -> Neighbors<'a>;
}

impl ExploreNeighbors for Point {
    fn explore_neighbors<'a>(&'a self) -> Neighbors<'a> {
        Neighbors::new(self)
    }
}

impl Heightmap {
    pub fn get(&self, point: &Point) -> Option<u64> {
        match self.width.overflowing_mul(point.1 as usize) {
            (_, true) => None,
            (offset, false) => match offset.overflowing_add(point.0 as usize) {
                (_, true) => None,
                (index, false) => self.flat_map.get(index).copied(),
            },
        }
    }

    pub fn shortest_path(&self, from_any_low_point: bool) -> AocResult<u64> {
        // BFS implementation.
        let mut to_explore = VecDeque::new();
        let mut seen = HashMap::new();

        if from_any_low_point {
            for position in self
                .flat_map
                .iter()
                .enumerate()
                .filter(|(_, &h)| h == 0u64)
                .map(|(i, h)| {
                    let (y, x) = i.div_mod_floor(&self.width);
                    (x as u64, y as u64, *h)
                })
            {
                to_explore.push_back((position, 0));
            }
        } else {
            to_explore.push_back((self.start, 0));
        }

        while let Some((position, steps)) = to_explore.pop_front() {
            if position == self.end {
                // We have reached our destination.
                return Ok(steps);
            }

            if steps >= seen.get(&position).copied().unwrap_or(u64::MAX) {
                // There is some better path than this one through this position, so ignore this
                // path.
                continue;
            }

            seen.insert(position, steps);

            for mut neighbor in position.explore_neighbors() {
                if let Some(height) = self.get(&neighbor) {
                    // Update the height of the next point with what the heightmap says.
                    neighbor.2 = height;
                    if neighbor.2 <= position.2 || neighbor.2 - position.2 == 1 {
                        // We can move up or down to this point.
                        to_explore.push_back((neighbor, steps + 1));
                    }
                }
            }
        }
        Err(AocError::new("no path found"))
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let heightmap = Heightmap::from_str(input)?;
    heightmap.shortest_path(false)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    let heightmap = Heightmap::from_str(input)?;
    heightmap.shortest_path(true)
}
