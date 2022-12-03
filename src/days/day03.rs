use std::collections::HashSet;

use crate::common::{AocError, AocResult, IntoAocResult};
use itertools::Itertools;

fn priority(letter: u8) -> AocResult<u64> {
    match letter as char {
        'a'..='z' => Ok((letter - ('a' as u8) + 1).into()),
        'A'..='Z' => Ok((letter - ('A' as u8) + 27).into()),
        _ => Err(AocError::new("unknown item code")),
    }
}

fn read_compartments(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect()
}

fn read_grouped_rupsacks(input: &str) -> Vec<(&str, &str, &str)> {
    input.lines().tuples().collect()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    read_compartments(input)
        .into_iter()
        .map(|(first, second)| {
            let set: HashSet<u8> = first.bytes().collect();
            second
                .bytes()
                .find(|c| set.contains(c))
                .into_aoc_result_msg("no common item")
        })
        .map(|c| priority(c?))
        .sum()
}

fn multi_intersection(sets: impl IntoIterator<Item = HashSet<u8>>) -> HashSet<u8> {
    let mut sets_iter = sets.into_iter();
    let intersection = sets_iter.next();
    match intersection {
        None => HashSet::new(),
        Some(mut intersection) => {
            for set in sets_iter {
                intersection.retain(|v| set.contains(v));
            }
            intersection
        }
    }
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    read_grouped_rupsacks(input)
        .into_iter()
        .map(|(a, b, c)| {
            let common = multi_intersection([
                a.bytes().collect(),
                b.bytes().collect(),
                c.bytes().collect(),
            ]);
            match common.len() {
                1 => Ok(common.into_iter().next().unwrap()),
                _ => Err(AocError::new(format!(
                    "intersection does not have a single item, contains {}",
                    common.into_iter().map(|v| v.to_string()).join(", ")
                ))),
            }
        })
        .map(|c| priority(c?))
        .sum()
}
