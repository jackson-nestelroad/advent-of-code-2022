mod error;
mod lines;
mod solver;

pub use error::{AocError, AocResult, IntoAocResult};
pub use lines::{NewlineBlocks, NewlineBlocksIterator};
pub use solver::SolverFn;
