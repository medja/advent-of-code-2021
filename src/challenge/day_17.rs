use anyhow::Context;
use std::ops::RangeInclusive;

const INVALID_TARGET: &str = "Invalid target string";

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let max_vy = (i16::MAX as f32).sqrt() as i16;
    let target = parse_target(input[0])?;

    let mut best_vy = 0;

    for vx in 1..target.max_x() {
        for vy in 1..max_vy {
            match simulate(vx, vy, &target) {
                Result::Hit if vy > best_vy => best_vy = vy,
                Result::OverX => break,
                _ => {}
            }
        }
    }

    Ok(best_vy * (best_vy + 1) / 2)
}

fn parse_target(input: &str) -> anyhow::Result<Target> {
    let input = input.split_once('=').context(INVALID_TARGET)?.1;
    let (x1, input) = input.split_once('.').context(INVALID_TARGET)?;
    let (x2, input) = input[1..].split_once(',').context(INVALID_TARGET)?;
    let input = input.split_once('=').context(INVALID_TARGET)?.1;
    let (y1, input) = input.split_once('.').context(INVALID_TARGET)?;
    let y2 = &input[1..];

    Ok(Target::new(
        x1.parse()?..=x2.parse()?,
        y1.parse()?..=y2.parse()?,
    ))
}

fn simulate(mut vx: i16, mut vy: i16, target: &Target) -> Result {
    let mut x = 0;
    let mut y = 0;

    loop {
        x += vx;
        y += vy;

        if let Some(result) = target.check(x, y) {
            break result;
        }

        if vx > 0 {
            vx -= 1;
        }

        vy -= 1;
    }
}

enum Result {
    Hit,
    OverX,
    OverY,
}

struct Target(RangeInclusive<i16>, RangeInclusive<i16>);

impl Target {
    fn new(x: RangeInclusive<i16>, y: RangeInclusive<i16>) -> Self {
        Target(x, y)
    }

    fn max_x(&self) -> i16 {
        *self.0.end()
    }

    fn check(&self, x: i16, y: i16) -> Option<Result> {
        if self.0.contains(&x) && self.1.contains(&y) {
            Some(Result::Hit)
        } else if x > *self.0.end() {
            Some(Result::OverX)
        } else if y < *self.1.end() {
            Some(Result::OverY)
        } else {
            None
        }
    }
}
