use anyhow::Context;
use std::str::FromStr;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut x = 0i32;
    let mut y = 0i32;

    for line in input {
        match line.parse::<Command>()? {
            Command::Forward(amount) => x += amount,
            Command::Down(amount) => y += amount,
            Command::Up(amount) => y -= amount,
        }
    }

    Ok(x * y)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut a = 0i32;

    for line in input {
        match line.parse::<Command>()? {
            Command::Forward(amount) => {
                x += amount;
                y += a * amount;
            }
            Command::Down(amount) => a += amount,
            Command::Up(amount) => a -= amount,
        }
    }

    Ok(x * y)
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = string
            .split_once(' ')
            .with_context(|| format!("Command `{}` did not contain a space", string))?;

        let amount = amount.parse()?;

        let command = match direction {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => {
                return Err(anyhow::anyhow!(
                    "{} is not a valid direction, expecting forward, down or up",
                    direction
                ))
            }
        };

        Ok(command)
    }
}
