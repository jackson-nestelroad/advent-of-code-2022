use std::{collections::HashMap, str::FromStr};

use crate::common::{AocError, AocResult, IntoAocResult};
use itertools::Itertools;
use num::Integer;

// Operations supported by our calculator.
#[derive(Debug, Clone)]
#[repr(u8)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Operator {
    pub fn commutative(&self) -> bool {
        match self {
            Self::Plus => true,
            Self::Minus => false,
            Self::Times => true,
            Self::Divide => false,
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Plus => Self::Minus,
            Self::Minus => Self::Plus,
            Self::Times => Self::Divide,
            Self::Divide => Self::Times,
        }
    }
}

impl FromStr for Operator {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Times),
            "/" => Ok(Self::Divide),
            _ => Err(AocError::new(&format!("invalid operator: {s}"))),
        }
    }
}

// Types of operands for an operator.
#[derive(Debug, Clone)]
enum Operand {
    // A variable with a currently-unknown value.
    Variable,
    // A known value.
    Number(i64),
    // A nested operation whose value is unknown. This means there is a variable somewhere in the
    // operation stack.
    Operation(Box<(Operand, Operator, Operand)>),
}

impl Operand {
    // Solves for a single variable in this operand stack, assuming it is an
    // operation with one variable.
    pub fn solve_for_single_variable(&self, rhs: i64) -> AocResult<i64> {
        // Unwind the operation stack, starting from the top, until the variable is
        // isolated. The variable must be a leaf node, and operation must have one
        // number and one nested operation.
        let mut stack = self;
        let mut solution = rhs;
        loop {
            match stack {
                Self::Variable => break,
                Self::Number(_) => return Err(AocError::new("no variable found in operand stack")),
                Self::Operation(operation) => match operation.as_ref() {
                    (Self::Number(lhs), op, rhs @ _) => {
                        solution = if op.commutative() {
                            op.inverse().perform(solution, *lhs)
                        } else {
                            op.perform(*lhs, solution)
                        };
                        stack = rhs;
                    }
                    (lhs @ _, op, Self::Number(rhs)) => {
                        solution = op.inverse().perform(solution, *rhs);
                        stack = lhs;
                    }
                    _ => {
                        return Err(AocError::new(
                            "at least one side of each operation should be a number",
                        ))
                    }
                },
            }
        }
        Ok(solution)
    }
}

impl Operator {
    pub fn perform<I: Integer>(&self, lhs: I, rhs: I) -> I {
        match self {
            Self::Plus => lhs.add(rhs),
            Self::Minus => lhs.sub(rhs),
            Self::Times => lhs.mul(rhs),
            Self::Divide => lhs.div(rhs),
        }
    }

    pub fn perform_variable(&self, lhs: Operand, rhs: Operand) -> Operand {
        match (&lhs, &rhs) {
            // The two operands are known, so the result is known.
            (Operand::Number(lhs), Operand::Number(rhs)) => {
                Operand::Number(self.perform(*lhs, *rhs))
            }
            // At least one operand is unknown, so the result is unknown.
            _ => Operand::Operation(Box::new((lhs, self.clone(), rhs))),
        }
    }
}

#[derive(Debug, Default)]
enum MonkeyRule {
    Number(i64),
    Equation(usize, Operator, usize),
    #[default]
    Variable,
}

#[derive(Debug)]
struct MonkeyRiddle {
    monkey_name_to_id: HashMap<String, usize>,
    rules: Vec<MonkeyRule>,
}

impl MonkeyRiddle {
    fn get_id_by_name(&self, name: &str) -> AocResult<usize> {
        self.monkey_name_to_id
            .get(name)
            .copied()
            .into_aoc_result_msg(&format!("monkey {name} does not exist"))
    }

    pub fn solve(&self, name: &str) -> AocResult<i64> {
        let id = self.get_id_by_name(name)?;
        self.solve_id(id)
    }

    fn solve_id(&self, id: usize) -> AocResult<i64> {
        match &self.rules[id] {
            MonkeyRule::Number(n) => Ok(*n),
            MonkeyRule::Equation(lhs, op, rhs) => {
                Ok(op.perform(self.solve_id(*lhs)?, self.solve_id(*rhs)?))
            }
            MonkeyRule::Variable => Err(AocError::new(
                "variables not supported in normal solving mode",
            )),
        }
    }

    pub fn solve_for_variable(&mut self, variable: &str, test: &str) -> AocResult<i64> {
        let variable_id = self.get_id_by_name(variable)?;
        self.rules[variable_id] = MonkeyRule::Variable;

        let test_id = self.get_id_by_name(test)?;
        match &self.rules[test_id] {
            MonkeyRule::Equation(lhs, _, rhs) => {
                // Solve left and right sides.
                let left_stack = self.solve_id_with_variables(*lhs);
                let right_stack = self.solve_id_with_variables(*rhs);

                // At this point, because there should be only one variable, one side should be
                // a number and the other should be an operation stack.
                //
                // If not, we are unable to solve this equation, because there is more than one
                // variable.
                match (&left_stack, &right_stack) {
                    (Operand::Operation(_), Operand::Number(equal)) => {
                        left_stack.solve_for_single_variable(*equal)
                    }
                    (Operand::Number(equal), Operand::Operation(_)) => {
                        right_stack.solve_for_single_variable(*equal)
                    }
                    _ => Err(AocError::new("unsupported use case")),
                }
            }
            _ => Err(AocError::new(&format!(
                "monkey {test} does not have an lhs and rhs to compare"
            ))),
        }
    }

    pub fn solve_id_with_variables(&self, id: usize) -> Operand {
        match &self.rules[id] {
            MonkeyRule::Number(n) => Operand::Number(*n),
            MonkeyRule::Equation(lhs, op, rhs) => op.perform_variable(
                self.solve_id_with_variables(*lhs),
                self.solve_id_with_variables(*rhs),
            ),
            MonkeyRule::Variable => Operand::Variable,
        }
    }
}

impl FromStr for MonkeyRiddle {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let mut riddle = Self {
            monkey_name_to_id: HashMap::new(),
            rules: Vec::new(),
        };

        // Parse each line once and check for errors early.
        let parsed_lines = s
            .lines()
            .map(|line| {
                line.split_once(':')
                    .into_aoc_result_msg("invaid input line")
            })
            .collect::<AocResult<Vec<_>>>()?;

        // First, convert all names to a numberic id.
        let mut id = 0;
        for (name, _) in &parsed_lines {
            riddle.monkey_name_to_id.insert(name.to_string(), id);
            id += 1;
        }

        // Next, parse all equations.
        riddle.rules.resize_with(id, Default::default);
        for (name, equation) in parsed_lines {
            let my_id = riddle.monkey_name_to_id[name];
            let equation = equation.trim();
            riddle.rules[my_id] = match equation.trim().split(' ').collect_tuple() {
                Some((lhs, op, rhs)) => {
                    let left_id = riddle
                        .monkey_name_to_id
                        .get(lhs)
                        .into_aoc_result_msg(&format!("monkey {lhs} does not exist"))?;
                    let right_id = riddle
                        .monkey_name_to_id
                        .get(rhs)
                        .into_aoc_result_msg(&format!("monkey {rhs} does not exist"))?;
                    let operator = Operator::from_str(op)?;
                    MonkeyRule::Equation(*left_id, operator, *right_id)
                }
                None => MonkeyRule::Number(equation.parse().into_aoc_result()?),
            }
        }

        Ok(riddle)
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    const ROOT: &str = "root";
    let riddle = MonkeyRiddle::from_str(input)?;
    riddle
        .solve(ROOT)
        .and_then(|n| n.try_into().into_aoc_result())
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    const ROOT: &str = "root";
    const HUMAN: &str = "humn";
    let mut riddle = MonkeyRiddle::from_str(input)?;
    riddle
        .solve_for_variable(HUMAN, ROOT)
        .and_then(|n| n.try_into().into_aoc_result())
}
