use std::str::FromStr;

use crate::common::{AocError, AocResult, IntoAocResult};

#[derive(Clone)]
struct EncryptedFile {
    // Store each number with their original index, so that we can locate individual numbers by
    // their original index.
    pub indexed_numbers: Vec<(usize, i64)>,
}

impl EncryptedFile {
    pub fn new(indexed_numbers: Vec<(usize, i64)>) -> Self {
        Self { indexed_numbers }
    }

    pub fn get(&self, i: isize) -> i64 {
        let index = i.rem_euclid(self.indexed_numbers.len() as isize) as usize;
        self.indexed_numbers[index].1
    }

    pub fn get_current_index_by_original_index(&self, i: usize) -> Option<usize> {
        self.indexed_numbers
            .iter()
            .position(|&(original_index, _)| original_index == i)
    }

    pub fn get_index_by_value(&self, value: i64) -> Option<usize> {
        self.indexed_numbers.iter().position(|&(_, n)| n == value)
    }

    pub fn mix(&mut self, decryption_key: i64, rounds: i64) {
        // Apply decryption key before we start.
        for (_, n) in &mut self.indexed_numbers {
            *n *= decryption_key;
        }

        let length = self.indexed_numbers.len();

        for _ in 0..rounds {
            for original_index in 0..length {
                let current_index = self
                    .get_current_index_by_original_index(original_index)
                    .unwrap();

                let n = self.indexed_numbers[current_index].1;

                let new_index = current_index as i64 + n;
                // length - 1 because the start and end positions are the same.
                let new_index = new_index.rem_euclid(length as i64 - 1) as usize;

                // Shift the contents of the vector using memmove.
                let wrapped = new_index < current_index;
                if wrapped {
                    let (begin, end) = (new_index as usize, current_index as usize);
                    self.indexed_numbers.copy_within(begin..end, begin + 1);
                } else {
                    let (begin, end) = (current_index as usize, new_index as usize);
                    self.indexed_numbers.copy_within((begin + 1)..=end, begin);
                }

                self.indexed_numbers[new_index] = (original_index, n);
            }
        }
    }

    pub fn sum_grove_coordinates(&self) -> AocResult<i64> {
        let zero_index = self
            .get_index_by_value(0)
            .into_aoc_result_msg("no zero found")?;
        Ok(self.get(zero_index as isize + 1000)
            + self.get(zero_index as isize + 2000)
            + self.get(zero_index as isize + 3000))
    }
}

impl FromStr for EncryptedFile {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        Ok(Self::new(
            s.lines()
                .enumerate()
                .map(|(i, line)| line.parse().and_then(|n| Ok((i, n))))
                .collect::<Result<_, _>>()
                .into_aoc_result_msg("invalid integer in encrypted file")?,
        ))
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let mut file = EncryptedFile::from_str(input)?;
    file.mix(1, 1);
    file.sum_grove_coordinates().map(|n| n as u64)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    const DECRYPTION_KEY: i64 = 811589153;
    let mut file = EncryptedFile::from_str(input)?;
    file.mix(DECRYPTION_KEY, 10);
    file.sum_grove_coordinates().map(|n| n as u64)
}
