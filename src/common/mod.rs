mod error;
mod lines;
mod solver;

pub use error::{AocError, AocResult, IntoAocResult};
pub use lines::{MultipleLines, MultipleLinesGroup};
pub use solver::SolverFn;
