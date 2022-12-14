use std::{fmt::Debug, marker::PhantomData, str::FromStr};

use num::Integer;

pub struct IntegerParsingIterator<'a, T>
where
    T: Integer,
{
    src: &'a str,
    radix: u32,
    i: usize,
    phantom: PhantomData<T>,
}

impl<'a, T> IntegerParsingIterator<'a, T>
where
    T: Integer,
{
    pub fn new(src: &'a str, radix: u32) -> Self {
        Self {
            src,
            radix,
            i: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IntegerParsingIterator<'a, T>
where
    T: Integer + FromStr,
    <T as FromStr>::Err: Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.src.len() {
            None
        } else if let Some(mut next) = self.src[self.i..].find(|c: char| c.is_digit(self.radix)) {
            next = next + self.i;
            let end = match self.src[next..].find(|c: char| !c.is_digit(self.radix)) {
                Some(end) => next + end,
                None => self.src.len(),
            };
            self.i = end;
            Some(self.src[next..end].parse::<T>().unwrap())
        } else {
            None
        }
    }
}

pub trait ParseIntegers {
    fn parse_integers<'a, T>(&'a self, radix: u32) -> IntegerParsingIterator<'a, T>
    where
        T: Integer;
}

impl ParseIntegers for &str {
    fn parse_integers<'a, T>(&'a self, radix: u32) -> IntegerParsingIterator<'a, T>
    where
        T: Integer,
    {
        IntegerParsingIterator::new(self, radix)
    }
}
