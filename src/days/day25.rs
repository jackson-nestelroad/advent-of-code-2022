use crate::common::{AocError, AocResult};
use num::Integer;

fn parse_snafu(digits: &str) -> AocResult<u64> {
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

fn to_snafu(mut num: u64) -> AocResult<String> {
    let mut digits = Vec::new();
    let mut i = 0;
    while num != 0 {
        let (div, rem) = num.div_rem(&5);
        num = div;
        match digits.get_mut(i) {
            None => digits.push(rem as i64),
            Some(digit) => *digit += rem as i64,
        };
        if digits[i] > 2 {
            digits[i] -= 5;
            digits.push(1);
        }
        i += 1;
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

pub fn solve_a(input: &str) -> AocResult<String> {
    let sum = input
        .lines()
        .map(|line| parse_snafu(line))
        .sum::<AocResult<_>>()?;
    to_snafu(sum)
}

pub fn solve_b(_: &str) -> AocResult<String> {
    Ok("Start The Blender".to_owned())
}
