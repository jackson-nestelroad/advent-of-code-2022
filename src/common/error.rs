use std::fmt::{Display, Formatter, Result as DisplayResult};

pub struct AocError {
    message: String,
}

impl AocError {
    pub fn new<S: Into<String>>(message: S) -> AocError {
        AocError {
            message: message.into(),
        }
    }
}

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter) -> DisplayResult {
        write!(f, "Error: {}", self.message)
    }
}

pub type AocResult<T> = Result<T, AocError>;

pub trait IntoAocResult<T> {
    fn into_aoc_result(self) -> AocResult<T>;
    fn into_aoc_result_msg(self, message: &str) -> Result<T, AocError>;
}

impl<T, E: ToString> IntoAocResult<T> for Result<T, E> {
    fn into_aoc_result(self) -> AocResult<T> {
        match self {
            Err(err) => Err(AocError::new(err.to_string())),
            Ok(res) => Ok(res),
        }
    }

    fn into_aoc_result_msg(self, message: &str) -> AocResult<T> {
        match self {
            Err(_) => Err(AocError::new(message)),
            Ok(res) => Ok(res),
        }
    }
}

impl<T> IntoAocResult<T> for Option<T> {
    fn into_aoc_result(self) -> AocResult<T> {
        match self {
            None => Err(AocError::new("option contained no value")),
            Some(res) => Ok(res),
        }
    }

    fn into_aoc_result_msg(self, message: &str) -> AocResult<T> {
        match self {
            None => Err(AocError::new(message)),
            Some(res) => Ok(res),
        }
    }
}
