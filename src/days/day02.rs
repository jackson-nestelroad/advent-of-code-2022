use crate::common::{AocError, AocResult, IntoAocResult};

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl TryFrom<char> for Outcome {
    type Error = AocError;
    fn try_from(ch: char) -> AocResult<Self> {
        match ch {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(AocError::new("invalid outcome char")),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn try_from_opponent(ch: char) -> AocResult<Self> {
        match ch {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err(AocError::new("invalid opponent char")),
        }
    }

    pub fn try_from_yours(ch: char) -> AocResult<Self> {
        match ch {
            'X' => Ok(Self::Rock),
            'Y' => Ok(Self::Paper),
            'Z' => Ok(Self::Scissors),
            _ => Err(AocError::new("invalid yours char")),
        }
    }

    pub fn beats(&self, other: &Self) -> Outcome {
        use Hand::*;
        use Outcome::*;
        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    pub fn needed_for_outcome(&self, outcome: &Outcome) -> Self {
        use Hand::*;
        use Outcome::*;
        match (self, outcome) {
            (Rock, Lose) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
            (Paper, Lose) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Lose) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock,
        }
    }
}

trait Scored {
    fn score(&self) -> u64;
}

impl Scored for Hand {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Scored for Outcome {
    fn score(&self) -> u64 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl Scored for (Hand, Hand) {
    fn score(&self) -> u64 {
        self.1.score() + self.1.beats(&self.0).score()
    }
}

impl Scored for (Hand, Outcome) {
    fn score(&self) -> u64 {
        self.1.score() + self.0.needed_for_outcome(&self.1).score()
    }
}

fn line_to_hands(line: &str) -> AocResult<(Hand, Hand)> {
    let mut chars = line.chars();
    let lhs = chars
        .next()
        .into_aoc_result_msg("missing first character")?;
    if chars.next() != Some(' ') {
        return Err(AocError::new("expected space after first character"));
    }
    let rhs = chars
        .next()
        .into_aoc_result_msg("missing character after space")?;
    Ok((Hand::try_from_opponent(lhs)?, Hand::try_from_yours(rhs)?))
}

fn line_to_outcome(line: &str) -> AocResult<(Hand, Outcome)> {
    let mut chars = line.chars();
    let opponent = chars
        .next()
        .into_aoc_result_msg("missing first character")?;
    if chars.next() != Some(' ') {
        return Err(AocError::new("expected space after first character"))?;
    }
    let outcome = chars
        .next()
        .into_aoc_result_msg("missing character after space")?;
    Ok((
        Hand::try_from_opponent(opponent)?,
        Outcome::try_from(outcome)?,
    ))
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    input
        .lines()
        .map(|line| line_to_hands(line).and_then(|round| Ok(round.score())))
        .sum()
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    input
        .lines()
        .map(|line| line_to_outcome(line).and_then(|round| Ok(round.score())))
        .sum()
}
