mod blocks;
mod error;
mod solver;

pub use blocks::{NewlineBlocks, NewlineBlocksIterator};
pub use error::{AocError, AocResult, IntoAocResult};
pub use solver::{AocSolution, IntSolverFn, Solver, StringSolverFn};
