use std::str;

pub struct NewlineBlocksIterator<'a> {
    newlines: u32,
    next_group_begin_index: usize,
    next_index: usize,
    bytes: &'a [u8],
}

impl<'a> NewlineBlocksIterator<'a> {
    fn new(input: &'a str, newlines: u32) -> Self {
        Self {
            newlines,
            next_group_begin_index: 0,
            next_index: 0,
            bytes: input.as_bytes(),
        }
    }
}

impl<'a> NewlineBlocksIterator<'a> {
    fn find_next_newline(&mut self) -> Option<usize> {
        for i in self.next_index..self.bytes.len() {
            if self.bytes[i] == b'\n' {
                self.next_index = i + 1;
                return if i > self.next_group_begin_index && self.bytes[i - 1] == b'\r' {
                    Some(i - 1)
                } else {
                    Some(i)
                };
            }
        }
        self.next_index = self.bytes.len();
        None
    }

    fn get_next_byte(&mut self) -> Option<u8> {
        if self.next_index >= self.bytes.len() {
            None
        } else {
            let byte = self.bytes[self.next_index];
            self.next_index += 1;
            Some(byte)
        }
    }

    fn try_finish_group(&mut self) -> bool {
        for _ in 1..self.newlines {
            match self.get_next_byte() {
                None => return false,
                Some(b'\n') => (),
                Some(b'\r') => match self.get_next_byte() {
                    None => return false,
                    Some(b'\n') => (),
                    _ => return false,
                },
                _ => return false,
            }
        }
        true
    }
}

impl<'a> Iterator for NewlineBlocksIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.bytes.len() {
            None
        } else {
            unsafe {
                while let Some(index) = self.find_next_newline() {
                    if self.try_finish_group() {
                        let result = &self.bytes[self.next_group_begin_index..index];
                        self.next_group_begin_index = self.next_index;
                        return Some(str::from_utf8_unchecked(result));
                    }
                }
                let last_group =
                    str::from_utf8_unchecked(&self.bytes[self.next_group_begin_index..]);
                self.next_index = self.bytes.len();
                Some(last_group)
            }
        }
    }
}

pub trait NewlineBlocks {
    fn newline_blocks<'a>(&'a self, newlines: u32) -> NewlineBlocksIterator<'a>;
}

impl NewlineBlocks for &str {
    fn newline_blocks<'a>(&'a self, newlines: u32) -> NewlineBlocksIterator<'a> {
        NewlineBlocksIterator::new(self, newlines)
    }
}
