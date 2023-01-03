use super::*;
use crate::{
    common::{AocError, AocResult, AocSolution, IntoAocResult, Solver},
    program::{ProgramArgs, SolutionPart},
};
use std::{
    fs,
    time::{Duration, Instant},
};

const SOLVERS: [[Solver; 2]; 25] = [
    [Solver::Int(day01::solve_a), Solver::Int(day01::solve_b)],
    [Solver::Int(day02::solve_a), Solver::Int(day02::solve_b)],
    [Solver::Int(day03::solve_a), Solver::Int(day03::solve_b)],
    [Solver::Int(day04::solve_a), Solver::Int(day04::solve_b)],
    [Solver::Str(day05::solve_a), Solver::Str(day05::solve_b)],
    [Solver::Int(day06::solve_a), Solver::Int(day06::solve_b)],
    [Solver::Int(day07::solve_a), Solver::Int(day07::solve_b)],
    [Solver::Int(day08::solve_a), Solver::Int(day08::solve_b)],
    [Solver::Int(day09::solve_a), Solver::Int(day09::solve_b)],
    [Solver::Int(day10::solve_a), Solver::Str(day10::solve_b)],
    [Solver::Int(day11::solve_a), Solver::Int(day11::solve_b)],
    [Solver::Int(day12::solve_a), Solver::Int(day12::solve_b)],
    [Solver::Int(day13::solve_a), Solver::Int(day13::solve_b)],
    [Solver::Int(day14::solve_a), Solver::Int(day14::solve_b)],
    [Solver::Int(day15::solve_a), Solver::Int(day15::solve_b)],
    [Solver::Int(day16::solve_a), Solver::Int(day16::solve_b)],
    [Solver::Int(day17::solve_a), Solver::Int(day17::solve_b)],
    [Solver::Int(day18::solve_a), Solver::Int(day18::solve_b)],
    [Solver::Int(day19::solve_a), Solver::Int(day19::solve_b)],
    [Solver::Int(day20::solve_a), Solver::Int(day20::solve_b)],
    [Solver::Int(day21::solve_a), Solver::Int(day21::solve_b)],
    [Solver::Int(day22::solve_a), Solver::Int(day22::solve_b)],
    [Solver::Int(day23::solve_a), Solver::Int(day23::solve_b)],
    [Solver::Int(day24::solve_a), Solver::Int(day24::solve_b)],
    [Solver::Str(day25::solve_a), Solver::Str(day25::solve_b)],
];

fn get_solver(args: &ProgramArgs) -> AocResult<&Solver> {
    if args.day() as usize > SOLVERS.len() {
        return Err(AocError::new("day not implemented"));
    }
    let part_index = match args.part() {
        SolutionPart::A => 0,
        SolutionPart::B => 1,
    };
    Ok(&SOLVERS[(args.day() - 1) as usize][part_index])
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

fn run_solver(args: &ProgramArgs, solver: &Solver) -> AocResult<Solution> {
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

pub fn solve(args: &ProgramArgs) -> AocResult<Solution> {
    run_solver(args, get_solver(args)?)
}

pub fn solve_all() -> AocResult<Duration> {
    let mut total_time = Duration::new(0, 0);
    for (day, solvers) in SOLVERS.iter().enumerate() {
        for (part, solver) in solvers.iter().enumerate() {
            let part = match part {
                0 => SolutionPart::A,
                1 => SolutionPart::B,
                _ => return Err(AocError::new(&format!("unknown part: {}", part + 1))),
            };
            let args = ProgramArgs::new(day as u8 + 1, part, None);
            match run_solver(&args, solver) {
                Err(err) => {
                    return Err(AocError::new(&format!(
                        "Day {day} Part {part} failed: {err:?}"
                    )))
                }
                Ok(result) => {
                    total_time += result.time;
                }
            }
        }
    }
    Ok(total_time)
}
