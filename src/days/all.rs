use super::*;
use crate::{
    common::{AocError, AocResult, AocSolution, IntoAocResult, Solver},
    program::{ProgramArgs, SolutionPart},
};
use std::{
    fs,
    time::{Duration, Instant},
};

const SOLVERS: [[Solver; 2]; 5] = [
    [Solver::Int(day01::solve_a), Solver::Int(day01::solve_b)],
    [Solver::Int(day02::solve_a), Solver::Int(day02::solve_b)],
    [Solver::Int(day03::solve_a), Solver::Int(day03::solve_b)],
    [Solver::Int(day04::solve_a), Solver::Int(day04::solve_b)],
    [Solver::Str(day05::solve_a), Solver::Str(day05::solve_b)],
];

fn get_solver(args: &ProgramArgs) -> AocResult<Solver> {
    if args.day() as usize > SOLVERS.len() {
        return Err(AocError::new("day not implemented"));
    }
    let part_index = match args.part() {
        SolutionPart::A => 0,
        SolutionPart::B => 1,
    };
    Ok(SOLVERS[(args.day() - 1) as usize][part_index].clone())
}

pub struct Solution {
    pub solution: AocSolution,
    pub time: Duration,
}

impl Solution {
    pub fn new(solution: AocSolution, time: Duration) -> Self {
        Solution { solution, time }
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
    let solution = solver.run(&input)?;
    let then = now.elapsed();
    Ok(Solution::new(solution, then))
}
