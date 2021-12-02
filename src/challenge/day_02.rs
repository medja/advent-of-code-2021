use anyhow::Context;
use std::str::FromStr;

pub async fn part_01() -> anyhow::Result<u32> {
    let input = crate::http::get("https://adventofcode.com/2021/day/2/input").await?;
    let mut position = Position::default();

    for line in input.lines() {
        position.apply(&line.parse()?)
    }

    Ok(position.x * position.y)
}

#[derive(Default)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn apply(&mut self, command: &Command) {
        match command {
            Command::Forward(amount) => self.x += amount,
            Command::Down(amount) => self.y += amount,
            Command::Up(amount) => self.y -= amount,
        }
    }
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = string
            .split_once(' ')
            .with_context(|| format!("Command `{}` did not contain a space", string))?;

        let amount: u32 = amount.parse()?;

        let command = match direction {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => return Err(anyhow::anyhow!("{} is not a valid direction", direction)),
        };

        Ok(command)
    }
}
