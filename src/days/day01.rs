use crate::common::{AocResult, IntoAocResult, NewlineBlocks};
use itertools::Itertools;

fn read_groups(input: &str) -> AocResult<Vec<Vec<u64>>> {
    input
        .newline_blocks(2)
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()
        })
        .collect::<Result<Vec<Vec<u64>>, _>>()
        .into_aoc_result()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    read_groups(input)?
        .into_iter()
        .map(|group| group.into_iter().sum())
        .max()
        .into_aoc_result()
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    Ok(read_groups(input)?
        .into_iter()
        .map(|group| group.into_iter().sum::<u64>())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum())
}
