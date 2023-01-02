use std::{collections::VecDeque, ops::Add, str::FromStr};

use crate::common::{AocError, AocResult, IntoAocResult};
use itertools::Itertools;
use lazy_static::lazy_static;
use num::Integer;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

struct Neighbors<'a> {
    location: &'a Point,
    i: usize,
}

impl<'a> Neighbors<'a> {
    pub fn new(location: &'a Point) -> Self {
        Self { location, i: 0 }
    }

    fn moves() -> &'static [Point; 4] {
        lazy_static! {
            static ref MOVES: [Point; 4] = [
                Point::new(-1, 0),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(0, 1)
            ];
        }
        &MOVES
    }

    fn next_output(&self) -> Option<Point> {
        Self::moves()
            .get(self.i)
            .and_then(|delta| Some(*self.location + *delta))
    }

    fn advance_state(&mut self) {
        self.i += 1;
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let output = self.next_output()?;
        self.advance_state();
        Some(output)
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

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    negate: bool,
    start: i64,
}

impl Blizzard {
    pub fn position_at(&self, time: i64, bound: i64) -> i64 {
        (if self.negate {
            self.start - time
        } else {
            self.start + time
        })
        .rem_euclid(bound)
    }
}

// One potential speedup is to save all blizzard positions before starting
// pathfinding, so that all map operations are O(1). This has a huge memory cost
// (the same map is saved lcm(width x height) times).
#[derive(Debug)]
struct Valley {
    start: Point,
    end: Point,
    size: Point,
    // Maps x coordinates to blizzards on that column.
    x_blizzards: Vec<Vec<Blizzard>>,
    // Maps y coordinates to blizzards on that row.
    y_blizzards: Vec<Vec<Blizzard>>,
}

impl FromStr for Valley {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let lines = s.lines().collect_vec();
        if lines.len() < 3 {
            return Err(AocError::new("valley must have at least 3 lines"));
        }

        // This code assumes that the input is enclosed by a wall on all sides.
        let begin = lines[0]
            .find('.')
            .into_aoc_result_msg("missing valley opening")?
            - 1;
        let end = lines[lines.len() - 1]
            .find('.')
            .into_aoc_result_msg("missing valley exit")?
            - 1;
        let mut valley = Self {
            start: Point::new(begin as i64, -1),
            end: Point::new(end as i64, lines.len() as i64 - 2),
            size: Point::new(lines[0].len() as i64 - 2, lines.len() as i64 - 2),
            x_blizzards: vec![Vec::new(); lines[0].len() - 2],
            y_blizzards: vec![Vec::new(); lines.len() - 2],
        };
        for (y, line) in lines[1..(lines.len() - 1)].iter().enumerate() {
            for (x, c) in line[1..(line.len() - 1)].char_indices() {
                match c {
                    '>' => valley.y_blizzards[y].push(Blizzard {
                        negate: false,
                        start: x as i64,
                    }),
                    '<' => valley.y_blizzards[y].push(Blizzard {
                        negate: true,
                        start: x as i64,
                    }),
                    '^' => valley.x_blizzards[x].push(Blizzard {
                        negate: true,
                        start: y as i64,
                    }),
                    'v' => valley.x_blizzards[x].push(Blizzard {
                        negate: false,
                        start: y as i64,
                    }),
                    '.' => (),
                    _ => return Err(AocError::new("invalid character")),
                }
            }
        }

        Ok(valley)
    }
}

impl Valley {
    pub fn in_valley(&self, point: &Point) -> bool {
        (0 <= point.x && point.x < self.size.x && 0 <= point.y && point.y < self.size.y)
            || point == &self.start
            || point == &self.end
    }

    pub fn open_at(&self, point: &Point, time: i64) -> bool {
        point == &self.start
            || point == &self.end
            || (self.x_blizzards[point.x as usize]
                .iter()
                .all(|blizzard| blizzard.position_at(time, self.size.y) != point.y)
                && self.y_blizzards[point.y as usize]
                    .iter()
                    .all(|blizzard| blizzard.position_at(time, self.size.x) != point.x))
    }

    fn bfs(&self, start_state: (Point, i64), target: Point) -> AocResult<i64> {
        let mut queue = VecDeque::from([start_state]);
        let mut seen = FxHashSet::default();
        let blizzard_cycles_at = self.end.x.lcm(&self.end.y);
        while let Some((position, time)) = queue.pop_front() {
            if position == target {
                return Ok(time);
            }

            let blizzard_state = time % blizzard_cycles_at;
            if !seen.insert((position, blizzard_state)) {
                continue;
            }

            let next_time = time + 1;
            let next_blizzard_state = next_time % blizzard_cycles_at;
            for neighbor in position.explore_neighbors() {
                if self.in_valley(&neighbor) && self.open_at(&neighbor, next_blizzard_state) {
                    queue.push_back((neighbor, next_time));
                }
            }

            if self.open_at(&position, next_time) {
                queue.push_back((position, next_time));
            }
        }
        Err(AocError::new(&format!("failed to reach end: {target:?}")))
    }

    pub fn travel_to_end(&self, time_start: i64) -> AocResult<i64> {
        self.bfs((self.start, time_start), self.end)
    }

    pub fn travel_to_start(&self, time_start: i64) -> AocResult<i64> {
        self.bfs((self.end, time_start), self.start)
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let valley = Valley::from_str(input)?;
    valley.travel_to_end(0).map(|n| n as u64)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    let valley = Valley::from_str(input)?;
    let first = valley.travel_to_end(0)?;
    let second = valley.travel_to_start(first)?;
    valley.travel_to_end(second).map(|n| n as u64)
}
