mod blocks;
mod error;
mod integers;
mod solver;

pub use blocks::{NewlineBlocks, NewlineBlocksIterator};
pub use error::{AocError, AocResult, IntoAocResult};
pub use integers::{IntegerParsingIterator, ParseIntegers};
pub use solver::{AocSolution, IntSolverFn, Solver, StringSolverFn};
