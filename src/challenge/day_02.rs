use anyhow::Context;
use std::str::FromStr;

pub async fn part_01() -> anyhow::Result<i32> {
    let input = crate::http::get("https://adventofcode.com/2021/day/2/input").await?;

    let mut x = 0i32;
    let mut y = 0i32;

    for line in input.lines() {
        match line.parse::<Command>()? {
            Command::Forward(amount) => x += amount,
            Command::Down(amount) => y += amount,
            Command::Up(amount) => y -= amount,
        }
    }

    Ok(x * y)
}

pub async fn part_02() -> anyhow::Result<i32> {
    let input = crate::http::get("https://adventofcode.com/2021/day/2/input").await?;

    let mut x = 0i32;
    let mut y = 0i32;
    let mut a = 0i32;

    for line in input.lines() {
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
            _ => return Err(anyhow::anyhow!("{} is not a valid direction", direction)),
        };

        Ok(command)
    }
}
