use crate::common::{AocError, AocResult, IntoAocResult};
use std::{
    fmt::{Display, Formatter, Result as DisplayResult},
    str::FromStr,
};

#[derive(Copy, Clone)]
pub enum SolutionPart {
    A,
    B,
}

impl FromStr for SolutionPart {
    type Err = AocError;

    fn from_str(string: &str) -> AocResult<Self> {
        match string {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(AocError::new("part must be either A or B")),
        }
    }
}

impl Display for SolutionPart {
    fn fmt(&self, f: &mut Formatter) -> DisplayResult {
        let string = match self {
            Self::A => "A",
            Self::B => "B",
        };
        write!(f, "{}", string)
    }
}

pub struct ProgramArgs {
    day: u8,
    part: SolutionPart,
    filename: Option<String>,
}

impl ProgramArgs {
    pub fn new(day: u8, part: SolutionPart, filename: Option<String>) -> Self {
        ProgramArgs {
            day,
            part,
            filename,
        }
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn part(&self) -> SolutionPart {
        self.part
    }

    pub fn filename(&self) -> &Option<String> {
        &self.filename
    }

    fn get_next_string_optional(args: &mut impl Iterator<Item = String>) -> Option<String> {
        args.next()
    }

    fn get_next_string(args: &mut impl Iterator<Item = String>, name: &str) -> AocResult<String> {
        match Self::get_next_string_optional(args) {
            None => Err(AocError::new(format!("missing {}", name))),
            Some(parsed) => Ok(parsed),
        }
    }

    fn get_next_integer(args: &mut impl Iterator<Item = String>, name: &str) -> AocResult<u8> {
        Self::get_next_string(args, name)?
            .parse::<u8>()
            .into_aoc_result()
    }

    pub fn parse_from_args(mut args: impl Iterator<Item = String>) -> AocResult<Self> {
        let day = Self::get_next_integer(&mut args, "day")?;
        if day <= 0 || day > 31 {
            return Err(AocError::new("day must be between 1 and 31"));
        }
        let part = SolutionPart::from_str(&Self::get_next_string(&mut args, "part")?)?;
        let filename = Self::get_next_string_optional(&mut args);
        Ok(ProgramArgs::new(day, part, filename))
    }

    pub fn usage(program_name: &str) -> String {
        format!("{} [1-31] [A|B]", program_name)
    }
}
