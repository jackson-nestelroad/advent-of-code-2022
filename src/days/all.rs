use super::*;
use crate::{
    common::{AocError, AocResult, IntoAocResult, SolverFn},
    program::{ProgramArgs, SolutionPart},
};
use std::{
    fs,
    time::{Duration, Instant},
};

const SOLVERS: [[SolverFn; 2]; 2] = [
    [day01::solve_a, day01::solve_b],
    [day02::solve_a, day02::solve_b],
];

fn get_solver(args: &ProgramArgs) -> AocResult<SolverFn> {
    if args.day() as usize > SOLVERS.len() {
        return Err(AocError::new("day not implemented"));
    }
    let part_index = match args.part() {
        SolutionPart::A => 0,
        SolutionPart::B => 1,
    };
    Ok(SOLVERS[(args.day() - 1) as usize][part_index])
}

pub struct Solution {
    solution: u64,
    time: Duration,
}

impl Solution {
    pub fn new(solution: u64, time: Duration) -> Self {
        Solution { solution, time }
    }

    pub fn solution(&self) -> u64 {
        self.solution
    }

    pub fn time(&self) -> Duration {
        self.time
    }
}

pub fn solve(args: &ProgramArgs) -> AocResult<Solution> {
    let solver = get_solver(args)?;
    let filename = match args.filename() {
        None => format!("input/{}.txt", args.day()),
        Some(filename) => format!("input/{}", filename),
    };
    let input = fs::read_to_string(filename).into_aoc_result()?;
    let now = Instant::now();
    let solution = solver(&input)?;
    let then = now.elapsed();
    Ok(Solution::new(solution, then))
}
