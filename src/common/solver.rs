use std::fmt::{Display, Formatter, Result as DisplayResult};

use crate::common::AocResult;

#[derive(Clone)]
pub enum AocSolution {
    Int(u64),
    Str(String),
}

impl Display for AocSolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Self::Int(n) => write!(f, "{}", n),
            Self::Str(s) => write!(f, "{}", s),
        }
    }
}

pub type IntSolverFn = fn(&str) -> AocResult<u64>;
pub type StringSolverFn = fn(&str) -> AocResult<String>;

#[derive(Clone)]
pub enum Solver {
    Int(IntSolverFn),
    Str(StringSolverFn),
}

impl Solver {
    pub fn run(self, input: &str) -> AocResult<AocSolution> {
        Ok(match self {
            Self::Int(solver) => AocSolution::Int(solver(&input)?),
            Self::Str(solver) => AocSolution::Str(solver(&input)?),
        })
    }
}
