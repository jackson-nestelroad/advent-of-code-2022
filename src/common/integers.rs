use std::{marker::PhantomData, str::FromStr};

use num::{Integer, Signed, Unsigned};

pub struct IntegerParsingIterator<'a, T, const SIGNED: bool> {
    src: &'a str,
    radix: u32,
    i: usize,
    phantom: PhantomData<T>,
}

impl<'a, T, const SIGNED: bool> IntegerParsingIterator<'a, T, SIGNED>
where
    T: Integer + FromStr,
{
    pub fn new(src: &'a str, radix: u32) -> Self {
        Self {
            src,
            radix,
            i: 0,
            phantom: PhantomData,
        }
    }

    fn get_next(&mut self) -> Option<(bool, T)> {
        if self.i >= self.src.len() {
            None
        } else if let Some(mut next) = self.src[self.i..].find(|c: char| c.is_digit(self.radix)) {
            next = next + self.i;
            let negative = SIGNED && next > 0 && &self.src[(next - 1)..next] == "-";
            let end = match self.src[next..].find(|c: char| !c.is_digit(self.radix)) {
                Some(end) => next + end,
                None => self.src.len(),
            };
            self.i = end;
            unsafe {
                Some((
                    negative,
                    self.src[next..end].parse::<T>().unwrap_unchecked(),
                ))
            }
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for IntegerParsingIterator<'a, T, false>
where
    T: Integer + Unsigned + FromStr,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next().and_then(|(_, n)| Some(n))
    }
}

impl<'a, T> Iterator for IntegerParsingIterator<'a, T, true>
where
    T: Integer + Signed + FromStr,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next()
            .and_then(|(neg, n)| Some(if neg { n.neg() } else { n }))
    }
}

pub trait ParseIntegers {
    fn parse_integers<'a, T>(&'a self, radix: u32) -> IntegerParsingIterator<'a, T, false>
    where
        T: Integer + Unsigned + FromStr;

    fn parse_signed_integers<'a, T>(&'a self, radix: u32) -> IntegerParsingIterator<'a, T, true>
    where
        T: Integer + Signed + FromStr;
}

impl ParseIntegers for &str {
    fn parse_integers<'a, T>(&'a self, radix: u32) -> IntegerParsingIterator<'a, T, false>
    where
        T: Integer + Unsigned + FromStr,
    {
        IntegerParsingIterator::new(self, radix)
    }

    fn parse_signed_integers<'a, T>(&'a self, radix: u32) -> IntegerParsingIterator<'a, T, true>
    where
        T: Integer + Signed + FromStr,
    {
        IntegerParsingIterator::new(self, radix)
    }
}
