use anyhow::Context;

const INVALID_TARGET: &str = "Invalid target string";

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let target = parse_target(input[0])?;
    Ok(target.min_y() * (target.min_y() + 1) / 2)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let target = parse_target(input[0])?;
    let max_vy = -target.min_y();

    let mut count = 0;

    for vx in 1..=target.max_x() {
        for vy in target.min_y()..max_vy {
            match simulate(vx, vy, &target) {
                Result::Hit => count += 1,
                Result::OverX => break,
                _ => {}
            }
        }
    }

    Ok(count)
}

fn parse_target(input: &str) -> anyhow::Result<Target> {
    let input = input.split_once('=').context(INVALID_TARGET)?.1;
    let (x1, input) = input.split_once('.').context(INVALID_TARGET)?;
    let (x2, input) = input[1..].split_once(',').context(INVALID_TARGET)?;
    let input = input.split_once('=').context(INVALID_TARGET)?.1;
    let (y1, input) = input.split_once('.').context(INVALID_TARGET)?;
    let y2 = &input[1..];

    Ok(Target::new(
        x1.parse()?,
        x2.parse()?,
        y1.parse()?,
        y2.parse()?,
    ))
}

fn simulate(mut vx: i16, mut vy: i16, target: &Target) -> Result {
    let mut x = 0;
    let mut y = 0;

    loop {
        x += vx;
        y += vy;

        let result = target.check(x, y);

        if !matches!(result, Result::Miss) {
            break result;
        }

        if vx > 0 {
            vx -= 1;
        }

        vy -= 1;
    }
}

enum Result {
    Miss,
    Hit,
    OverX,
    OverY,
}

struct Target(i16, i16, i16, i16);

impl Target {
    fn new(min_x: i16, max_x: i16, min_y: i16, max_y: i16) -> Self {
        Target(min_x, max_x, min_y, max_y)
    }

    fn max_x(&self) -> i16 {
        self.1
    }

    fn min_y(&self) -> i16 {
        self.2
    }

    fn check(&self, x: i16, y: i16) -> Result {
        if x > self.1 {
            Result::OverX
        } else if y < self.2 {
            Result::OverY
        } else if x >= self.0 && y <= self.3 {
            Result::Hit
        } else {
            Result::Miss
        }
    }
}
