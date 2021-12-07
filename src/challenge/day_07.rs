use std::cmp::Ordering;
use std::num::ParseIntError;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut positions = parse_positions(input[0])?;
    let best_position = find_median(&mut positions);
    Ok(calculate_cost(&positions, best_position, abs_diff))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let positions = parse_positions(input[0])?;
    let best_position = (positions.iter().sum::<usize>() as f64) / (positions.len() as f64);
    let floor_cost = calculate_cost(&positions, best_position.floor() as usize, fuel_cost);
    let ceil_cost = calculate_cost(&positions, best_position.ceil() as usize, fuel_cost);
    Ok(floor_cost.min(ceil_cost))
}

fn parse_positions(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(',').map(|value| value.parse()).collect()
}

fn calculate_cost<F>(positions: &[usize], destination: usize, cost_fn: F) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    positions.iter().map(|pos| cost_fn(*pos, destination)).sum()
}

fn fuel_cost(x: usize, y: usize) -> usize {
    let distance = abs_diff(x, y);
    distance * (distance + 1) / 2
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn find_median(numbers: &mut [usize]) -> usize {
    let desired_pivot = numbers.len() / 2;

    let mut start = 0;
    let mut end = numbers.len() - 1;

    loop {
        let pivot = start + partition(&mut numbers[start..=end]);

        match pivot.cmp(&desired_pivot) {
            Ordering::Equal => break numbers[pivot],
            Ordering::Less => start = pivot + 1,
            Ordering::Greater => end = pivot - 1,
        }
    }
}

fn partition(numbers: &mut [usize]) -> usize {
    let last_index = numbers.len() - 1;
    let mut pivot = 0;

    for i in 0..last_index {
        if numbers[i] < numbers[last_index] {
            numbers.swap(i, pivot);
            pivot += 1;
        }
    }

    numbers.swap(pivot, last_index);
    pivot
}
