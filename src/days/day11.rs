use crate::common::{AocError, AocResult, IntoAocResult, NewlineBlocks, ParseIntegers};
use itertools::Itertools;
use num::Integer;
use std::cell::RefCell;

struct Monkey {
    pub worry_levels: Vec<u64>,
    pub operation: Box<dyn Fn(u64) -> u64>,
    pub divisible_test: u64,
    pub if_true: usize,
    pub if_false: usize,
    pub inspect_count: u64,
}

struct KeepAway {
    pub monkeys: Vec<RefCell<Monkey>>,
    maximum_worry_level: u64,
}

impl KeepAway {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        let maximum_worry_level = monkeys
            .iter()
            .map(|m| m.divisible_test)
            .fold(1, |acc, n| acc.lcm(&n));
        Self {
            monkeys: monkeys
                .into_iter()
                .map(|monkey| RefCell::new(monkey))
                .collect(),
            maximum_worry_level,
        }
    }

    pub fn do_round(&mut self, with_relief: bool) {
        for i in 0..self.monkeys.len() {
            self.take_turn(i, with_relief);
        }
    }

    fn take_turn(&self, id: usize, with_relief: bool) {
        let mut monkey = self.monkeys[id].borrow_mut();
        while let Some(mut item) = monkey.worry_levels.pop() {
            monkey.inspect_count += 1;
            item = (monkey.operation)(item);

            if with_relief {
                item /= 3;
            } else if item > self.maximum_worry_level {
                item = item.mod_floor(&self.maximum_worry_level);
            }

            if item.is_multiple_of(&monkey.divisible_test) {
                self.monkeys[monkey.if_true]
                    .borrow_mut()
                    .worry_levels
                    .push(item);
            } else {
                self.monkeys[monkey.if_false]
                    .borrow_mut()
                    .worry_levels
                    .push(item);
            }
        }
    }

    pub fn monkey_business(&self) -> u64 {
        self.monkeys
            .iter()
            .map(|m| m.borrow().inspect_count)
            .sorted_by(|a, b| Ord::cmp(b, a))
            .take(2)
            .product()
    }
}

fn read_monkeys(input: &str) -> AocResult<Vec<Monkey>> {
    input
        .newline_blocks(2)
        .map(|block| {
            let lines = block.lines().map(|line| line.trim()).collect::<Vec<_>>();
            if lines.len() != 6 {
                return Err(AocError::new(&format!(
                    "invalid input, found {} lines, expected 6",
                    lines.len()
                )));
            }

            let starting_levels = lines[1].parse_integers(10).collect();
            let operation: Box<dyn Fn(u64) -> u64> = match lines[2].split_once(':') {
                Some(("Operation", operation)) => {
                    match operation.trim().split(' ').collect::<Vec<_>>().as_slice() {
                        ["new", "=", "old", "*", "old"] => Box::new(|old| old * old),
                        ["new", "=", "old", op, num] => {
                            let n = num
                                .parse::<u64>()
                                .into_aoc_result_msg("invalid right operand")?;
                            match *op {
                                "+" => Box::new(move |old| old + n),
                                "*" => Box::new(move |old| old * n),
                                _ => return Err(AocError::new("unexpected operator")),
                            }
                        }
                        _ => return Err(AocError::new("unexpected operation form")),
                    }
                }
                _ => return Err(AocError::new("invalid operation")),
            };
            let divisible_test = lines[3]
                .parse_integers(10)
                .next()
                .into_aoc_result_msg("missing divisible test number")?;
            let if_true = lines[4]
                .parse_integers(10)
                .next()
                .into_aoc_result_msg("missing if true number")?;
            let if_false = lines[5]
                .parse_integers(10)
                .next()
                .into_aoc_result_msg("missing if false number")?;

            Ok(Monkey {
                worry_levels: starting_levels,
                operation: Box::new(operation),
                divisible_test,
                if_true,
                if_false,
                inspect_count: 0,
            })
        })
        .collect()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    const ROUNDS: u64 = 20;
    let mut game = KeepAway::new(read_monkeys(input)?);
    for _ in 0..ROUNDS {
        game.do_round(true);
    }
    Ok(game.monkey_business())
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    const ROUNDS: u64 = 10000;
    let mut game = KeepAway::new(read_monkeys(input)?);
    for _ in 0..ROUNDS {
        game.do_round(false);
    }
    Ok(game.monkey_business())
}
