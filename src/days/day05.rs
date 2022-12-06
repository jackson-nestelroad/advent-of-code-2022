use std::{cmp::max, str::FromStr};

use crate::common::{AocError, AocResult, IntoAocResult, NewlineBlocks};
use itertools::Itertools;

#[derive(Debug)]
struct Stack {
    pub crates: Vec<char>,
}

impl Stack {
    pub fn new() -> Self {
        Self { crates: Vec::new() }
    }

    pub fn push(&mut self, c: char) {
        self.crates.push(c)
    }

    pub fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    pub fn top(&self) -> Option<&char> {
        self.crates.last()
    }
}

fn read_stacks(input: &str) -> AocResult<Vec<Stack>> {
    let mut stacks = Vec::new();
    let mut lines = input.lines().rev();
    for _ in &lines
        .next()
        .into_aoc_result_msg("no lines in initial configuration")?
        .chars()
        .chunks(4)
    {
        stacks.push(Stack::new())
    }

    for line in lines {
        for (stack, mut chunk) in stacks.iter_mut().zip(&line.chars().chunks(4)) {
            match chunk.nth(1) {
                None => return Err(AocError::new("missing block id")),
                Some(' ') => (),
                Some(c) => stack.push(c),
            }
        }
    }

    Ok(stacks)
}

#[derive(Debug)]
struct Move {
    pub number_of_blocks: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Move {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let mut nums = s.split(' ').skip(1).step_by(2);
        Ok(Move {
            number_of_blocks: nums
                .next()
                .into_aoc_result_msg("missing number of blocks to move")?
                .parse()
                .into_aoc_result()?,
            from: nums
                .next()
                .into_aoc_result_msg("missing stack to move from")?
                .parse()
                .into_aoc_result()?,
            to: nums
                .next()
                .into_aoc_result_msg("missing stack to move to ")?
                .parse()
                .into_aoc_result()?,
        })
    }
}

trait CanMakeMove {
    fn make_move(&mut self, m: Move) -> AocResult<()>;
}

struct CraneMover {
    stacks: Vec<Stack>,
}

impl CraneMover {
    fn get_stacks(&mut self, m: &Move) -> AocResult<(&mut Stack, &mut Stack)> {
        let max = max(m.to, m.from);
        if self.stacks.len() < max {
            return Err(AocError::new(&format!(
                "index {max} overflows number of stacks ({})",
                self.stacks.len()
            )));
        }
        let (slice1, slice2) = self.stacks.split_at_mut(max - 1);
        if max == m.from {
            Ok((&mut slice2[0], &mut slice1[m.to - 1]))
        } else {
            Ok((&mut slice1[m.from - 1], &mut slice2[0]))
        }
    }

    pub fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| {
                stack
                    .top()
                    .and_then(|c| Some(c.clone()))
                    .filter(|c| *c != ' ')
            })
            .collect()
    }
}

struct CraneMover9000(pub CraneMover);

impl CraneMover9000 {
    pub fn new(stacks: Vec<Stack>) -> Self {
        Self(CraneMover { stacks })
    }
}

impl CanMakeMove for CraneMover9000 {
    fn make_move(&mut self, m: Move) -> AocResult<()> {
        let (from, to) = self.0.get_stacks(&m)?;
        for i in 1..=m.number_of_blocks {
            to.push(from.pop().into_aoc_result_msg(&format!(
                "from stack does not have a block to move for move {}",
                i
            ))?);
        }
        Ok(())
    }
}

struct CraneMover9000v2(pub CraneMover);

impl CraneMover9000v2 {
    #[allow(dead_code)]
    pub fn new(stacks: Vec<Stack>) -> Self {
        Self(CraneMover { stacks })
    }
}

impl CanMakeMove for CraneMover9000v2 {
    fn make_move(&mut self, m: Move) -> AocResult<()> {
        let (from, to) = self.0.get_stacks(&m)?;
        if from.crates.len() < m.number_of_blocks {
            return Err(AocError::new("from stack does not enough blocks to move"));
        }
        let moved = from
            .crates
            .split_off(from.crates.len() - m.number_of_blocks);
        to.crates.extend(moved.into_iter().rev());
        Ok(())
    }
}

pub fn solve_a(input: &str) -> AocResult<String> {
    let mut blocks = input.newline_blocks(2);
    let mut mover = CraneMover9000::new(read_stacks(
        blocks
            .next()
            .into_aoc_result_msg("input is missing initial configuration")?,
    )?);
    let moves = blocks
        .next()
        .into_aoc_result_msg("input is missing moves")?
        .lines()
        .map(|line| Move::from_str(line))
        .collect::<AocResult<Vec<Move>>>()?;

    for m in moves {
        mover.make_move(m)?;
    }

    Ok(mover.0.top_crates())
}

struct CraneMover9001(pub CraneMover);

impl CraneMover9001 {
    pub fn new(stacks: Vec<Stack>) -> Self {
        Self(CraneMover { stacks })
    }
}

impl CanMakeMove for CraneMover9001 {
    fn make_move(&mut self, m: Move) -> AocResult<()> {
        let (from, to) = self.0.get_stacks(&m)?;
        if from.crates.len() < m.number_of_blocks {
            return Err(AocError::new("from stack does not enough blocks to move"));
        }
        let mut moved = from
            .crates
            .split_off(from.crates.len() - m.number_of_blocks);
        to.crates.append(&mut moved);
        Ok(())
    }
}

pub fn solve_b(input: &str) -> AocResult<String> {
    let mut blocks = input.newline_blocks(2);
    let mut mover = CraneMover9001::new(read_stacks(
        blocks
            .next()
            .into_aoc_result_msg("input is missing initial configuration")?,
    )?);
    let moves = blocks
        .next()
        .into_aoc_result_msg("input is missing moves")?
        .lines()
        .map(|line| Move::from_str(line))
        .collect::<AocResult<Vec<Move>>>()?;

    for m in moves {
        mover.make_move(m)?;
    }

    Ok(mover.0.top_crates())
}
