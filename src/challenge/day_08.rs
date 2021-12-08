use anyhow::Context;
use std::ops::{BitAnd, BitOr, Not};
use std::str::FromStr;

const SEGMENT_MASK: Segments = Segments(0b1111111);

const INPUT_LENGTH: usize = 10;
const OUTPUT_LENGTH: usize = 4;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .flat_map(|line| line.rsplit_once('|').unwrap().1.split_ascii_whitespace())
        .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
        .count())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .map(|line| line.parse::<Problem>().unwrap().solve())
        .sum::<usize>())
}

#[derive(Default)]
struct Problem {
    one: Segments,
    four: Segments,
    seven: Segments,
    sixes: [Segments; 3],
    outputs: [Segments; OUTPUT_LENGTH],
}

impl Problem {
    fn solve(&self) -> usize {
        let (zero, six, nine) = self.solve_sixes();

        let segment_c = !six;
        let segment_d = !zero;
        let segment_e = !nine;
        let segment_f = self.one & !segment_c;
        let segment_b = self.four & !self.one & !segment_d;

        let patterns = [
            zero,
            self.one,
            SEGMENT_MASK & !segment_b & !segment_f,
            nine & !segment_b,
            self.four,
            six & !segment_e,
            six,
            self.seven,
            SEGMENT_MASK,
            nine,
        ];

        self.outputs.iter().fold(0usize, |sum, &output| {
            sum * 10 + patterns.iter().position(|&x| x == output).unwrap()
        })
    }

    fn solve_sixes(&self) -> (Segments, Segments, Segments) {
        let mut zero = Segments::default();
        let mut six = Segments::default();
        let mut nine = Segments::default();

        for segments in self.sixes {
            if segments & self.four == self.four {
                nine = segments;
            } else if segments & self.one == self.one {
                zero = segments;
            } else {
                six = segments;
            }
        }

        (zero, six, nine)
    }
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(mut input: &str) -> Result<Self, Self::Err> {
        let mut problem = Problem::default();
        let mut sixes_index = 0;

        for _ in 0..INPUT_LENGTH {
            let (value, rest) = input.split_once(' ').context("Unexpected end of line")?;
            input = rest;
            let segments = Segments::parse(value);

            match segments.count() {
                2 => problem.one = segments,
                3 => problem.seven = segments,
                4 => problem.four = segments,
                6 => {
                    problem.sixes[sixes_index] = segments;
                    sixes_index += 1;
                }
                _ => {}
            }
        }

        input = &input[2..];

        for segments in &mut problem.outputs[..OUTPUT_LENGTH - 1] {
            let (value, rest) = input.split_once(' ').context("Unexpected end of line")?;
            input = rest;
            *segments = Segments::parse(value);
        }

        problem.outputs[OUTPUT_LENGTH - 1] = Segments::parse(input);

        Ok(problem)
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
struct Segments(u8);

impl Segments {
    fn parse(value: &str) -> Self {
        let value = value
            .bytes()
            .fold(0u8, |acc, char| acc | 1 << (char - b'a'));

        Segments(value)
    }

    fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

impl BitOr for Segments {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Segments(self.0 | rhs.0)
    }
}

impl BitAnd for Segments {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Segments(self.0 & rhs.0)
    }
}

impl Not for Segments {
    type Output = Self;

    fn not(self) -> Self::Output {
        Segments(!self.0 & SEGMENT_MASK.0)
    }
}
