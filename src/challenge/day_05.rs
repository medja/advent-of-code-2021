use anyhow::Context;
use std::str::FromStr;

const MAP_SIZE: usize = 1000;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut map = Map::new();

    for line in input {
        map.mark_line(&line.parse::<Line>()?);
    }

    Ok(map
        .points()
        .filter(|point| matches!(point, Point::Multi))
        .count())
}

struct Coordinate(u16, u16);
struct Line(Coordinate, Coordinate);

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (x1, line) = line
            .split_once(',')
            .context("Cannot find first point delimiter (`,`)")?;

        let (y1, line) = line
            .split_once(' ')
            .context("Cannot find end of first point (` `)")?;

        let (_, line) = line
            .split_once(' ')
            .context("Cannot find start of second point (` `)")?;

        let (x2, y2) = line
            .split_once(',')
            .context("Cannot find first point delimiter (`,`)")?;

        let x1 = x1.parse()?;
        let y1 = y1.parse()?;
        let x2 = x2.parse()?;
        let y2 = y2.parse()?;

        Ok(Line(Coordinate(x1, y1), Coordinate(x2, y2)))
    }
}

#[derive(Copy, Clone)]
enum Point {
    Empty,
    Single,
    Multi,
}

struct Map([Point; MAP_SIZE * MAP_SIZE]);

impl Map {
    fn new() -> Self {
        Map([Point::Empty; MAP_SIZE * MAP_SIZE])
    }

    fn mark_line(&mut self, line: &Line) {
        if line.0 .0 == line.1 .0 {
            // same x
            let min = line.0 .1.min(line.1 .1);
            let max = line.0 .1.max(line.1 .1);

            for y in min..=max {
                self.mark_point(Coordinate(line.0 .0, y))
            }
        } else if line.0 .1 == line.1 .1 {
            // same y
            let min = line.0 .0.min(line.1 .0);
            let max = line.0 .0.max(line.1 .0);

            for x in min..=max {
                self.mark_point(Coordinate(x, line.0 .1))
            }
        }
    }

    fn mark_point(&mut self, coordinate: Coordinate) {
        let point = &mut self.0[coordinate.0 as usize + coordinate.1 as usize * MAP_SIZE];

        match point {
            Point::Empty => *point = Point::Single,
            Point::Single => *point = Point::Multi,
            Point::Multi => {}
        }
    }

    fn points(&self) -> impl Iterator<Item = &Point> {
        self.0.iter()
    }
}
