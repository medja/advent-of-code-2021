use std::num::ParseIntError;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let bit_count = input[0].as_bytes().len();
    let majority_count = input.len() / 2;

    let max = 2u32.pow(bit_count as u32) - 1;
    let mut counters = vec![0; bit_count];

    for line in input {
        let bytes = line.as_bytes();

        for i in 0..bit_count {
            if bytes[i] == b'1' {
                counters[i] += 1
            }
        }
    }

    let gamma = counters.iter().fold(0u32, |acc, count| {
        if *count < majority_count {
            acc << 1
        } else {
            acc << 1 | 1
        }
    });

    let epsilon = max & !gamma;

    Ok(gamma * epsilon)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let oxygen_generator = find_rating(input, 0, Rating::OxygenGenerator)?;
    let co2_scrubber = find_rating(input, 0, Rating::Co2Scrubber)?;

    Ok(oxygen_generator * co2_scrubber)
}

enum Rating {
    OxygenGenerator,
    Co2Scrubber,
}

impl Rating {
    fn majority(&self) -> u8 {
        match self {
            Rating::OxygenGenerator => b'1',
            Rating::Co2Scrubber => b'0',
        }
    }

    fn minority(&self) -> u8 {
        match self {
            Rating::OxygenGenerator => b'0',
            Rating::Co2Scrubber => b'1',
        }
    }
}

fn find_rating(input: &[&str], index: usize, rating: Rating) -> Result<usize, ParseIntError> {
    let length = input.len();

    if length < 2 || index >= input[0].len() {
        usize::from_str_radix(input[0], 2)
    } else {
        let count = input
            .iter()
            .filter(|line| line.as_bytes()[index] == b'1')
            .count();

        let bit = if count >= length - count {
            rating.majority()
        } else {
            rating.minority()
        };

        let filtered = input
            .iter()
            .filter(|line| line.as_bytes()[index] == bit)
            .cloned()
            .collect::<Vec<_>>();

        find_rating(&filtered, index + 1, rating)
    }
}
