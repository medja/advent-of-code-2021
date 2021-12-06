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
        let line = line.parse::<Line>()?;

        if line.is_straight() || diagonals {
            map.mark_line(&line);
        }
    }

    Ok(map
        .points()
        .filter(|point| matches!(point, Point::Multi))
        .count())
}

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

impl Line {
    fn is_straight(&self) -> bool {
        self.0.x() == self.1.x() || self.0.y() == self.1.y()
    }

    fn coordinates(&self) -> impl Iterator<Item = Coordinate> {
        let (sx, dx) = match self.0.x().cmp(&self.1.x()) {
            Ordering::Less => (1i32, self.1.x() - self.0.x()),
            Ordering::Equal => (0i32, 0),
            Ordering::Greater => (-1i32, self.0.x() - self.1.x()),
        };

        let (sy, dy) = match self.0.y().cmp(&self.1.y()) {
            Ordering::Less => (1i32, self.1.y() - self.0.y()),
            Ordering::Equal => (0i32, 0),
            Ordering::Greater => (-1i32, self.0.y() - self.1.y()),
        };

        let x = self.0.x() as i32;
        let y = self.0.y() as i32;

        let delta = dx.max(dy) as i32;

        (0i32..=delta).map(move |i| Coordinate((x + i * sx) as u16, (y + i * sy) as u16))
    }
}

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
        for coordinate in line.coordinates() {
            let point = &mut self.0[coordinate.0 as usize + coordinate.1 as usize * MAP_SIZE];

            match point {
                Point::Empty => *point = Point::Single,
                Point::Single => *point = Point::Multi,
                Point::Multi => {}
            }
        }
    }

    fn points(&self) -> impl Iterator<Item = &Point> {
        self.0.iter()
    }
}
