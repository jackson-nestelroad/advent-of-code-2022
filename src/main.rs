#[macro_use]
extern crate num_derive;

mod common;
mod days;
mod program;

use std::env;

use days::{solve, solve_all};
use program::ProgramArgs;

fn run_all() {
    match solve_all() {
        Err(err) => eprintln!("{}", err),
        Ok(total_time) => println!(
            "All Days and Parts ran in {} seconds ({} us)",
            total_time.as_secs_f64(),
            total_time.as_micros()
        ),
    }
}

fn run_part(program_name: &str, args: &mut impl Iterator<Item = String>) {
    let args = match ProgramArgs::parse_from_args(args) {
        Err(err) => {
            eprintln!("{}", err);
            return eprintln!("{}", ProgramArgs::usage(&program_name));
        }
        Ok(args) => args,
    };
    let solution = match solve(&args) {
        Err(err) => {
            return eprintln!("{}", err);
        }
        Ok(solution) => solution,
    };
    println!("Day {}, Part {}", args.day(), args.part());
    println!(
        "Solution: {} ({} us)",
        solution.solution,
        solution.time.as_micros()
    );
}

fn main() {
    let mut args = env::args().peekable();
    let program_name = match args.next() {
        None => return eprintln!("args is empty"),
        Some(name) => name,
    };
    match args.peek().and_then(|s| Some(s.as_str())) {
        Some("all") => run_all(),
        _ => run_part(&program_name, &mut args),
    };
}
