use anyhow::{anyhow, Context};
use std::cmp::Ordering;
use std::str::FromStr;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut parts = input.split(|line| line.is_empty());
    let mut paper = Paper::parse(parts.next().context("Cannot find end of dots")?)?;
    let fold = parts.next().context("Cannot find folds")?[0].parse::<Fold>()?;

    paper.fold(&fold);
    paper.dedup();

    Ok(paper.count_dots())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut parts = input.split(|line| line.is_empty());
    let mut paper = Paper::parse(parts.next().context("Cannot find end of dots")?)?;

    for fold in parts.next().context("Cannot find folds")? {
        paper.fold(&fold.parse()?);
    }

    Ok(paper.project())
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Dot(u16, u16);

impl Dot {
    fn new(x: u16, y: u16) -> Self {
        Dot(x, y)
    }

    fn x(&self) -> u16 {
        self.0
    }

    fn y(&self) -> u16 {
        self.1
    }
}

impl FromStr for Dot {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (x, y) = string.split_once(',').context("Line is not a valid dot")?;
        Ok(Dot(x.parse()?, y.parse()?))
    }
}

struct Paper(Vec<Dot>);

impl Paper {
    fn parse(input: &[&str]) -> anyhow::Result<Paper> {
        let mut dots = Vec::with_capacity(input.len());

        for dot in input {
            dots.push(dot.parse()?);
        }

        Ok(Paper(dots))
    }

    fn count_dots(&self) -> usize {
        self.0.len()
    }

    fn fold(&mut self, fold: &Fold) {
        match fold.direction() {
            Direction::Horizontal => self.fold_horizontal(fold.position()),
            Direction::Vertical => self.fold_vertical(fold.position()),
        }
    }

    fn fold_horizontal(&mut self, pos: u16) {
        let len = self.0.len();
        let dots = std::mem::replace(&mut self.0, Vec::with_capacity(len));

        for dot in dots {
            match dot.y().cmp(&pos) {
                Ordering::Less => self.0.push(dot),
                Ordering::Equal => {}
                Ordering::Greater => self.0.push(Dot::new(dot.x(), 2 * pos - dot.y())),
            }
        }
    }

    fn fold_vertical(&mut self, pos: u16) {
        let len = self.0.len();
        let dots = std::mem::replace(&mut self.0, Vec::with_capacity(len));

        for dot in dots {
            match dot.x().cmp(&pos) {
                Ordering::Less => self.0.push(dot),
                Ordering::Equal => {}
                Ordering::Greater => self.0.push(Dot::new(2 * pos - dot.x(), dot.y())),
            }
        }
    }

    fn dedup(&mut self) {
        self.0.sort();
        self.0.dedup();
    }

    fn project(&self) -> impl std::fmt::Display {
        Projection::new(&self.0)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Horizontal,
    Vertical,
}

struct Fold(Direction, u16);

impl Fold {
    fn direction(&self) -> Direction {
        self.0
    }

    fn position(&self) -> u16 {
        self.1
    }
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (dir, pos) = string.split_once('=').context("Line is not a valid fold")?;

        let dir = match dir.as_bytes()[dir.len() - 1] {
            b'x' => Direction::Vertical,
            b'y' => Direction::Horizontal,
            byte => return Err(anyhow!("{} is notm a valid direction", char::from(byte))),
        };

        Ok(Fold(dir, pos.parse()?))
    }
}

struct Projection(String);

impl Projection {
    fn new(dots: &[Dot]) -> Self {
        let len_x = dots.iter().map(|p| p.0).max().unwrap_or_default() as usize + 1;
        let len_y = dots.iter().map(|p| p.1).max().unwrap_or_default() as usize + 1;

        let mut buffer = String::with_capacity((len_x + 1) * len_y);
        let mut dot_map = vec![false; len_x * len_y];

        for dot in dots {
            dot_map[dot.x() as usize + dot.y() as usize * len_x] = true;
        }

        for y in 0..len_y {
            for x in 0..len_x {
                if dot_map[x + y * len_x] {
                    buffer.push('█');
                } else {
                    buffer.push(' ');
                }
            }

            buffer.push('\n');
        }

        Projection(buffer)
    }
}

impl std::fmt::Display for Projection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        self.0.fmt(f)
    }
}
