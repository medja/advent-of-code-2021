use std::ops::Sub;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut positions = input[0]
        .split(',')
        .map(|value| value.parse())
        .collect::<Result<Vec<usize>, _>>()?;

    positions.sort_unstable();
    let best_position = positions[positions.len() / 2];

    Ok(positions
        .iter()
        .map(|value| abs_diff(*value, best_position))
        .sum::<usize>())
}

fn abs_diff<N: Sub + Ord>(x: N, y: N) -> N::Output {
    if x < y {
        y - x
    } else {
        x - y
    }
}
