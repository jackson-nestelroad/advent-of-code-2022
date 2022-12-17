use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::{Display, Formatter, Result as DisplayResult},
    slice,
    str::FromStr,
};

use crate::common::{AocError, AocResult, IntoAocResult, NewlineBlocks};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u64),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(left), Self::Integer(right)) => left.cmp(right),
            (Self::List(left), Self::List(right)) => left.cmp(right),
            (left @ Self::Integer(_), Self::List(right)) => {
                slice::from_ref(left).cmp(right.as_slice())
            }
            (Self::List(left), right @ Self::Integer(_)) => {
                left.as_slice().cmp(slice::from_ref(right))
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Self::Integer(n) => write!(f, "{n}"),
            Self::List(list) => {
                write!(f, "[")?;
                let mut iter = list.iter().peekable();
                while let Some(packet) = iter.next() {
                    write!(f, "{}", packet)?;
                    if iter.peek().is_some() {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl FromStr for Packet {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let mut chars = s.chars();
        let mut stack = VecDeque::new();
        let mut list = Vec::new();
        let mut number = None;

        while let Some(c) = chars.next() {
            match c {
                c if c.is_digit(10) => {
                    number = Some(number.unwrap_or(0) * 10 + (c.to_digit(10).unwrap() as u64))
                }
                ',' => {
                    if let Some(number) = number.take() {
                        list.push(Packet::Integer(number));
                    }
                }
                '[' => {
                    stack.push_back((list, number));
                    list = Vec::new();
                    number = None;
                }
                ']' => {
                    if let Some(number) = number.take() {
                        list.push(Packet::Integer(number));
                    }

                    let packet = Packet::List(list);
                    (list, number) = stack
                        .pop_back()
                        .into_aoc_result_msg("unexpected closing bracket")?;
                    list.push(packet);
                }
                _ => return Err(AocError::new(&format!("nexpected char: {c}"))),
            }
        }

        if !stack.is_empty() {
            Err(AocError::new("missing closing bracket(s)"))
        } else {
            Ok(list.remove(0))
        }
    }
}

fn parse_packet_pairs(input: &str) -> AocResult<Vec<(Packet, Packet)>> {
    input
        .newline_blocks(2)
        .map(|block| {
            let mut lines = block.lines();
            Ok((
                Packet::from_str(lines.next().into_aoc_result_msg("missing first packet")?)?,
                Packet::from_str(lines.next().into_aoc_result_msg("missing second packet")?)?,
            ))
        })
        .collect()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    Ok(parse_packet_pairs(input)?
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            if left < right {
                Some((i + 1) as u64)
            } else {
                None
            }
        })
        .sum())
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    let dividers = vec![
        Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]),
    ];
    let mut packets = parse_packet_pairs(input)?
        .into_iter()
        .map(|(left, right)| [left, right])
        .flatten()
        .collect::<Vec<_>>();
    packets.extend(dividers.clone().into_iter());
    packets.sort();
    Ok(dividers
        .into_iter()
        .map(|divider| {
            packets
                .binary_search(&divider)
                .into_aoc_result_msg(&format!("failed to find divider {divider:?}"))
                .and_then(|i| Ok(i + 1))
        })
        .collect::<AocResult<Vec<_>>>()?
        .into_iter()
        .product::<usize>() as u64)
}
