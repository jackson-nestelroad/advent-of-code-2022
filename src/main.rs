#[macro_use]
extern crate lazy_static;

mod common;
mod days;
mod program;

use std::env;

use days::solve;
use program::ProgramArgs;

fn main() {
    let mut args = env::args();
    let program_name = match args.next() {
        None => return eprintln!("args is empty"),
        Some(name) => name,
    };
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
        solution.solution(),
        solution.time().as_micros()
    );
}
