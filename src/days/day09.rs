use std::collections::HashSet;

use crate::common::{AocError, AocResult, IntoAocResult};

#[repr(u8)]
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl TryFrom<&str> for Direction {
    type Error = AocError;
    fn try_from(s: &str) -> AocResult<Self> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => Err(AocError::new(&format!("invalid direction: {}", s))),
        }
    }
}

struct Motion {
    pub direction: Direction,
    pub steps: i64,
}

impl Motion {
    pub fn new(direction: Direction, steps: i64) -> Self {
        Self { direction, steps }
    }
}

fn read_motions(input: &str) -> AocResult<Vec<Motion>> {
    input
        .lines()
        .map(|line| match line.split_once(' ') {
            None => Err(AocError::new("missing space")),
            Some((first, second)) => Ok(Motion::new(
                Direction::try_from(first)?,
                second.parse().into_aoc_result()?,
            )),
        })
        .collect()
}

type Position = (i64, i64);

fn difference(a: Position, b: Position) -> Position {
    (a.0 - b.0, a.1 - b.1)
}

fn touching(a: Position, b: Position) -> bool {
    let diff = difference(a, b);
    diff.0 >= -1 && diff.0 <= 1 && diff.1 >= -1 && diff.1 <= 1
}

fn tail_visited(start: Position, segments: usize, motions: Vec<Motion>) -> HashSet<Position> {
    let mut rope = vec![start; segments];
    let mut visited = HashSet::from([*rope.last().unwrap()]);
    for Motion { direction, steps } in motions {
        for _ in 0..steps {
            // Change the head position.
            match direction {
                Direction::Up => rope[0].1 += 1,
                Direction::Down => rope[0].1 -= 1,
                Direction::Right => rope[0].0 += 1,
                Direction::Left => rope[0].0 -= 1,
            };

            for i in 1..rope.len() {
                let leader = rope[i - 1];
                let follower = &mut rope[i];

                if touching(leader, *follower) {
                    // Already touching, so no segments after this one move either.
                    break;
                }

                // Apply the difference, at most one step in both directions.
                let diff = difference(leader, *follower);
                if diff.0 >= 1 || diff.0 <= -1 {
                    follower.0 += diff.0.clamp(-1, 1);
                }
                if diff.1 >= 1 || diff.1 <= -1 {
                    follower.1 += diff.1.clamp(-1, 1);
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    visited
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    Ok(tail_visited((0, 0), 2, read_motions(input)?).len() as u64)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    return Ok(tail_visited((0, 0), 10, read_motions(input)?).len() as u64);
}
