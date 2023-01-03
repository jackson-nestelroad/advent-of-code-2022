use std::{iter::Sum, ops::Add, str::FromStr};

use crate::common::{AocError, AocResult};
use itertools::{EitherOrBoth, Itertools};
use num::Integer;

fn snafu_to_base_10(digits: &str) -> AocResult<u64> {
    let mut final_value = 0;
    for digit in digits.as_bytes() {
        let value = match digit {
            b'=' => -2,
            b'-' => -1,
            b'0' | b'1' | b'2' => (digit - b'0') as i64,
            _ => return Err(AocError::new(&format!("invalid snafu digit: {digit}"))),
        };

        if value < 0 && final_value == 0 {
            return Err(AocError::new("invalid snafu number: failed to borrow"));
        }

        final_value = 5 * final_value + value;
    }

    Ok(final_value as u64)
}

fn base_10_to_snafu(mut num: u64) -> AocResult<String> {
    let mut digits = Vec::new();
    let mut borrow = 0;
    while num != 0 {
        let (div, rem) = num.div_rem(&5);
        num = div;
        let mut rem = rem as i64;
        rem += borrow;
        if rem > 2 {
            rem -= 5;
            borrow = 1;
        } else {
            borrow = 0;
        }
        digits.push(rem);
    }
    Ok(digits
        .into_iter()
        .rev()
        .skip_while(|d| d == &0)
        .map(|digit| match digit {
            -2 => '=',
            -1 => '-',
            0 | 1 | 2 => char::from_digit(digit as u32, 3).unwrap(),
            _ => unreachable!(),
        })
        .collect())
}

#[derive(Debug)]
struct Snafu(Vec<i64>);

impl Snafu {
    fn to_string(&self) -> AocResult<String> {
        self.0
            .iter()
            .rev()
            .map(|digit| match digit {
                -2 => Ok('='),
                -1 => Ok('-'),
                0 => Ok('0'),
                1 => Ok('1'),
                2 => Ok('2'),
                _ => Err(AocError::new("invalid snafu digit")),
            })
            .collect()
    }
}

impl FromStr for Snafu {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        Ok(Snafu(
            s.bytes()
                .rev()
                .map(|b| match b {
                    b'=' => Ok(-2),
                    b'-' => Ok(-1),
                    b'0' => Ok(0),
                    b'1' => Ok(1),
                    b'2' => Ok(2),
                    _ => Err(AocError::new("invalid character in snafu number")),
                })
                .collect::<AocResult<_>>()?,
        ))
    }
}

impl From<&str> for Snafu {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

impl Add for Snafu {
    type Output = Snafu;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();
        let mut borrow = 0;
        for pair in self.0.iter().zip_longest(rhs.0.iter()) {
            let (a, b) = match pair {
                EitherOrBoth::Both(&a, &b) => (a, b),
                EitherOrBoth::Left(&a) => (a, 0),
                EitherOrBoth::Right(&b) => (0, b),
            };
            let mut sum = a + b + borrow;

            if sum > 2 {
                sum -= 5;
                borrow = 1;
            } else if sum < -2 {
                sum += 5;
                borrow = -1;
            } else {
                borrow = 0;
            }
            result.push(sum);
        }
        if borrow == 1 {
            result.push(borrow);
        }
        Snafu(result)
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = "0".into();
        for item in iter {
            sum = sum + item;
        }
        sum
    }
}

pub fn solve_a(input: &str) -> AocResult<String> {
    let sum = input
        .lines()
        .map(|line| snafu_to_base_10(line))
        .sum::<AocResult<_>>()?;
    let conversion_result = base_10_to_snafu(sum)?;
    let sum = input
        .lines()
        .map(|line| Snafu::from_str(line))
        .sum::<AocResult<Snafu>>()?;
    let direct_result = sum.to_string()?;
    if conversion_result != direct_result {
        Err(AocError::new(
            "result from base-10 conversion and result from direct addition are not equivalent",
        ))
    } else {
        Ok(direct_result)
    }
}

pub fn solve_b(_: &str) -> AocResult<String> {
    Ok("Start The Blender".to_owned())
}
