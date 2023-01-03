use std::fmt::{Display, Formatter, Result as DisplayResult, Write};

use crate::common::{AocError, AocResult, IntoAocResult};

#[derive(Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl Instruction {
    pub fn cycles(&self) -> u64 {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = AocError;
    fn try_from(s: &str) -> AocResult<Self> {
        Ok(match s.split_once(' ') {
            Some(("addx", val)) => Instruction::Addx(
                val.parse()
                    .into_aoc_result_msg(&format!("invalid operand for addx: {val}"))?,
            ),
            None => match s {
                "noop" => Instruction::Noop,
                _ => return Err(AocError::new(&format!("unknown instruction: {s}"))),
            },
            _ => return Err(AocError::new(&format!("unknown instruction: {s}"))),
        })
    }
}

struct ExecutingInstruction {
    pub instruction: Instruction,
    pub cycles: u64,
}

impl ExecutingInstruction {
    pub fn new(instruction: Instruction) -> Self {
        Self {
            instruction,
            cycles: 0,
        }
    }

    pub fn tick(&mut self) {
        self.cycles += 1;
    }

    pub fn finished(&self) -> bool {
        self.cycles >= self.instruction.cycles()
    }
}

trait Clocked {
    fn tick(&mut self);
}

struct Cpu {
    x: i64,
    executing: Option<ExecutingInstruction>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            x: 1,
            executing: None,
        }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn ready_for_instruction(&self) -> bool {
        self.executing.is_none()
    }

    pub fn execute(&mut self, instruction: Instruction) {
        self.executing = Some(ExecutingInstruction::new(instruction));
    }

    fn finish_instruction(&mut self) {
        match self.executing.as_mut().unwrap().instruction {
            Instruction::Addx(val) => self.x += val,
            Instruction::Noop => (),
        }
        self.executing = None;
    }
}

impl Clocked for Cpu {
    fn tick(&mut self) {
        if let Some(instr) = &mut self.executing {
            instr.tick();
            if instr.finished() {
                self.finish_instruction()
            }
        }
    }
}

struct Crt {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
    cycle: u64,
}

impl Crt {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![false; width * height],
            cycle: 0,
        }
    }

    fn column(&self) -> u64 {
        self.cycle % (self.width as u64)
    }

    pub fn set(&mut self) {
        self.pixels[self.cycle as usize] = true;
    }
}

impl Clocked for Crt {
    fn tick(&mut self) {
        self.cycle += 1;
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        for i in 0..self.height {
            if i != 0 {
                f.write_char('\n')?;
            }
            for pixel in &self.pixels[(self.width * i)..(self.width * (i + 1))] {
                f.write_char(if *pixel { '#' } else { '.' })?;
            }
        }
        Ok(())
    }
}

fn read_instructions(input: &str) -> AocResult<Vec<Instruction>> {
    input
        .lines()
        .map(|line| Instruction::try_from(line))
        .collect()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    const OFFSET: u64 = 20;
    const PERIOD: u64 = 40;
    const CHECKS: u64 = 6;
    const MAX_CYCLE: u64 = OFFSET + PERIOD * (CHECKS - 1);

    let mut instructions = read_instructions(input)?.into_iter();
    let mut cpu = Cpu::new();
    let mut signal_strenghts = Vec::new();

    for cycle in 1..=MAX_CYCLE {
        if cpu.ready_for_instruction() {
            match instructions.next() {
                Some(instruction) => cpu.execute(instruction),
                None => (),
            }
        }

        if cycle >= OFFSET && (cycle - OFFSET) % PERIOD == 0 {
            signal_strenghts.push(cycle as i64 * cpu.x());
        }

        cpu.tick();
    }

    Ok(signal_strenghts.into_iter().sum::<i64>() as u64)
}

pub fn solve_b(input: &str) -> AocResult<String> {
    let mut instructions = read_instructions(input)?.into_iter();
    let mut cpu = Cpu::new();
    let mut crt = Crt::new(40, 6);

    loop {
        if cpu.ready_for_instruction() {
            match instructions.next() {
                Some(instruction) => cpu.execute(instruction),
                None => break,
            }
        }

        match (crt.column() as i64) - cpu.x() {
            -1 | 0 | 1 => crt.set(),
            _ => (),
        }

        cpu.tick();
        crt.tick();
    }

    println!("{}", crt);
    Ok("check stdout".to_owned())
}
