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

    Ok(map.count_intersections())
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
    fn start_x(&self) -> u16 {
        self.0.x()
    }

    fn end_x(&self) -> u16 {
        self.1.x()
    }

    fn start_y(&self) -> u16 {
        self.0.y()
    }

    fn end_y(&self) -> u16 {
        self.1.y()
    }

    fn is_straight(&self) -> bool {
        self.0.x() == self.1.x() || self.0.y() == self.1.y()
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

struct Map(usize, [Point; MAP_SIZE * MAP_SIZE]);

impl Map {
    fn new() -> Self {
        Map(0, [Point::Empty; MAP_SIZE * MAP_SIZE])
    }

    fn mark_line(&mut self, line: &Line) {
        let start_x = line.start_x() as usize;
        let start_y = line.start_y() as usize;
        let end_x = line.end_x() as usize;
        let end_y = line.end_y() as usize;

        let (start, offset, steps) = match start_y.cmp(&end_y) {
            Ordering::Less => {
                let start = start_x + start_y * MAP_SIZE;
                let steps = end_y - start_y;

                let offset = match start_x.cmp(&end_x) {
                    Ordering::Less => MAP_SIZE + 1,
                    Ordering::Equal => MAP_SIZE,
                    Ordering::Greater => MAP_SIZE - 1,
                };

                (start, offset, steps)
            }
            Ordering::Greater => {
                let start = end_x + end_y * MAP_SIZE;
                let steps = start_y - end_y;

                let offset = match start_x.cmp(&end_x) {
                    Ordering::Less => MAP_SIZE - 1,
                    Ordering::Equal => MAP_SIZE,
                    Ordering::Greater => MAP_SIZE + 1,
                };

                (start, offset, steps)
            }
            Ordering::Equal => match start_x.cmp(&end_x) {
                Ordering::Less => {
                    let start = start_x + start_y * MAP_SIZE;
                    let steps = end_x - start_x;
                    (start, 1, steps)
                }
                Ordering::Greater => {
                    let start = end_x + start_y * MAP_SIZE;
                    let steps = start_x - end_x;
                    (start, 1, steps)
                }
                Ordering::Equal => {
                    return;
                }
            },
        };

        let end = start + steps * offset;
        let mut index = start;

        while index <= end {
            let point = &mut self.1[index];

            match point {
                Point::Empty => *point = Point::Single,
                Point::Single => {
                    self.0 += 1;
                    *point = Point::Multi;
                }
                Point::Multi => {}
            }

            index += offset;
        }
    }

    fn count_intersections(&self) -> usize {
        self.0
    }
}
