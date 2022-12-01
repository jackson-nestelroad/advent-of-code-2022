use crate::common::{AocResult, IntoAocResult, MultipleLines};

fn read_groups(input: &str) -> AocResult<Vec<Vec<i64>>> {
    input
        .multiple_lines(2)
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()
        })
        .collect::<Result<Vec<Vec<i64>>, _>>()
        .into_aoc_result()
}

pub fn solve_a(input: &str) -> AocResult<i64> {
    read_groups(input)?
        .into_iter()
        .map(|group| group.into_iter().sum())
        .max()
        .into_aoc_result()
}

pub fn solve_b(input: &str) -> AocResult<i64> {
    let mut groups = read_groups(input)?
        .into_iter()
        .map(|group| group.into_iter().sum())
        .collect::<Vec<i64>>();
    groups.sort_by(|a, b| b.cmp(a));
    Ok(groups[0..3].into_iter().sum())
}
