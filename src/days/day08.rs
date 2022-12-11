use crate::common::{AocResult, IntoAocResult};
use itertools::enumerate;

fn read_tree_map(input: &str) -> AocResult<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Ok(c.to_digit(10).into_aoc_result_msg("invalid character")? as u8))
                .collect::<AocResult<Vec<_>>>()
        })
        .collect::<AocResult<Vec<_>>>()
}

fn count_visible(trees: Vec<Vec<u8>>) -> u64 {
    // Duplicate map of the forest that marks each tree as visible.
    let mut visible = Vec::new();
    for row in &trees {
        visible.push(vec![false; row.len()]);
    }

    // All trees in top edge.
    if let Some(top) = visible.first_mut() {
        top.fill(true);
    }
    // All trees in bottom edge, if it is different from the top edge.
    if visible.len() > 1 {
        if let Some(bottom) = visible.last_mut() {
            bottom.fill(true);
        }
    }

    // Check visibility from left and right at the same time.
    for i in 1..(trees.len() - 1) {
        let row = &trees[i];
        let mut max = (-1i8, -1i8);
        for (j, (left, right)) in enumerate(row.into_iter().zip(row.into_iter().rev())) {
            if *left as i8 > max.0 {
                visible[i][j] = true;
                max.0 = *left as i8;
            }
            if *right as i8 > max.1 {
                visible[i][row.len() - 1 - j] = true;
                max.1 = *right as i8;
            }
        }
    }

    let max_row_length = trees.iter().map(|row| row.len()).max().unwrap_or(0);

    // Check visibility from top and bottom at the same time.
    for j in 1..(max_row_length - 1) {
        let mut max = (-1i8, -1i8);
        for i in 0..trees.len() {
            if let Some(top) = trees[i].get(j) {
                if *top as i8 > max.0 {
                    visible[i][j] = true;
                    max.0 = *top as i8;
                }
            }
            let bottom_index = trees.len() - 1 - i;
            if let Some(bottom) = trees[bottom_index].get(j) {
                if *bottom as i8 > max.1 {
                    visible[bottom_index][j] = true;
                    max.1 = *bottom as i8;
                }
            }
        }
    }

    visible
        .into_iter()
        .map(|row| row.into_iter().filter(|v| *v).count() as u64)
        .sum()
}

fn highest_scenic_score(trees: Vec<Vec<u8>>) -> AocResult<u64> {
    const MOVEMENT: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    trees
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(j, height)| (i, j, height))
        })
        .map(|(i, j, height)| {
            MOVEMENT
                .iter()
                .map(|(di, dj)| {
                    // How far can we get in one direction?
                    let mut distance: u64 = 0;
                    let mut max = -1i8;
                    let (mut i, mut j) = (i as isize, j as isize);
                    loop {
                        // Move our current location.
                        i += di;
                        j += dj;

                        // Bounds check before converting to usize.
                        if i < 0 || j < 0 {
                            break;
                        }
                        match trees.get(i as usize).and_then(|row| row.get(j as usize)) {
                            Some(viewed_height) => {
                                // We can see another tree.
                                distance += 1;

                                // New maximum tree height.
                                if *viewed_height as i8 > max {
                                    max = *viewed_height as i8;
                                }

                                // Same height or taller than our tree.
                                // Cannot see anything behind it.
                                if viewed_height >= height {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                    distance
                })
                .product::<u64>()
        })
        .max()
        .into_aoc_result_msg("no max scenic score found")
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    Ok(count_visible(read_tree_map(input)?))
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    highest_scenic_score(read_tree_map(input)?)
}
