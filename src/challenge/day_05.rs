use anyhow::Context;
use std::cmp::Ordering;
use std::str::FromStr;

const MAP_SIZE: usize = 1000;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    count_intersections(input, false)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    count_intersections(input, true)
}

fn count_intersections(input: &[&str], diagonals: bool) -> anyhow::Result<usize> {
    let mut map = Map::new();

    for line in input {
        map.mark_line(&line.parse::<Line>()?, diagonals);
    }

    Ok(map
        .points()
        .filter(|point| matches!(point, Point::Multi))
        .count())
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Coordinate(u16, u16);

impl Coordinate {
    fn x(&self) -> u16 {
        self.0
    }

    fn y(&self) -> u16 {
        self.1
    }
}

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

        let first = Coordinate(x1, y1);
        let second = Coordinate(x2, y2);

        match first.cmp(&second) {
            Ordering::Greater => Ok(Line(second, first)),
            _ => Ok(Line(first, second)),
        }
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

    fn mark_line(&mut self, line: &Line, diagonals: bool) {
        if line.0.x() == line.1.x() {
            for y in line.0.y()..=line.1.y() {
                self.mark_point(Coordinate(line.0.x(), y))
            }
        } else if line.0.y() == line.1.y() {
            for x in line.0.x()..=line.1.x() {
                self.mark_point(Coordinate(x, line.0.y()))
            }
        } else if !diagonals {
        } else if line.0.y() < line.1.y() {
            for i in 0..=(line.1.x() - line.0.x()) {
                self.mark_point(Coordinate(line.0.x() + i, line.0.y() + i))
            }
        } else {
            for i in 0..=(line.1.x() - line.0.x()) {
                self.mark_point(Coordinate(line.0.x() + i, line.0.y() - i))
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
