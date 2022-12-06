use std::str::FromStr;

use crate::common::{AocError, AocResult, IntoAocResult};

struct Range {
    pub min: u64,
    pub max: u64,
}

impl Range {
    pub fn fully_contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.min <= other.max && other.min <= self.max
    }
}

impl FromStr for Range {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let (first, second) = s
            .split_once('-')
            .into_aoc_result_msg("invalid range, no hyphen")?;
        Ok(Range {
            min: first
                .parse::<u64>()
                .into_aoc_result_msg("invalid minimum")?,
            max: second
                .parse::<u64>()
                .into_aoc_result_msg("invalid maximum")?,
        })
    }
}

fn read_assignments(input: &str) -> AocResult<Vec<(Range, Range)>> {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .into_aoc_result_msg("no comma")
                .and_then(|(first, second)| Ok((Range::from_str(first)?, Range::from_str(second)?)))
        })
        .collect()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    Ok(read_assignments(input)?
        .into_iter()
        .filter(|(first, second)| first.fully_contains(second) || second.fully_contains(first))
        .count() as u64)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    Ok(read_assignments(input)?
        .into_iter()
        .filter(|(first, second)| first.overlaps(second))
        .count() as u64)
}
