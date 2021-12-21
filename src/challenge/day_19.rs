use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul, Sub};

const MIN_BEACONS: usize = 12;
const MIN_FINGERPRINTS: usize = 66; // 12! / (2! * 10!)

const ROTATIONS: [Rotation; 24] = [
    Rotation::new([1, 0, 0, 0, 1, 0, 0, 0, 1]),
    Rotation::new([1, 0, 0, 0, 0, 1, 0, -1, 0]),
    Rotation::new([1, 0, 0, 0, -1, 0, 0, 0, -1]),
    Rotation::new([1, 0, 0, 0, 0, -1, 0, 1, 0]),
    Rotation::new([0, 1, 0, 0, 0, 1, 1, 0, 0]),
    Rotation::new([0, 1, 0, 1, 0, 0, 0, 0, -1]),
    Rotation::new([0, 1, 0, 0, 0, -1, -1, 0, 0]),
    Rotation::new([0, 1, 0, -1, 0, 0, 0, 0, 1]),
    Rotation::new([0, 0, 1, 1, 0, 0, 0, 1, 0]),
    Rotation::new([0, 0, 1, 0, 1, 0, -1, 0, 0]),
    Rotation::new([0, 0, 1, -1, 0, 0, 0, -1, 0]),
    Rotation::new([0, 0, 1, 0, -1, 0, 1, 0, 0]),
    Rotation::new([-1, 0, 0, 0, -1, 0, 0, 0, 1]),
    Rotation::new([-1, 0, 0, 0, 0, 1, 0, 1, 0]),
    Rotation::new([-1, 0, 0, 0, 1, 0, 0, 0, -1]),
    Rotation::new([-1, 0, 0, 0, 0, -1, 0, -1, 0]),
    Rotation::new([0, -1, 0, 0, 0, -1, 1, 0, 0]),
    Rotation::new([0, -1, 0, 1, 0, 0, 0, 0, 1]),
    Rotation::new([0, -1, 0, 0, 0, 1, -1, 0, 0]),
    Rotation::new([0, -1, 0, -1, 0, 0, 0, 0, -1]),
    Rotation::new([0, 0, -1, -1, 0, 0, 0, 1, 0]),
    Rotation::new([0, 0, -1, 0, 1, 0, 1, 0, 0]),
    Rotation::new([0, 0, -1, 1, 0, 0, 0, -1, 0]),
    Rotation::new([0, 0, -1, 0, -1, 0, -1, 0, 0]),
];

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input).0)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let scanners = solve(input).1;

    let max_distance = pairs(&scanners)
        .map(|(a, b)| (a.x() - b.x()).abs() + (a.y() - b.y()).abs() + (a.z() - b.z()).abs())
        .max()
        .unwrap();

    Ok(max_distance)
}

fn solve(input: &[&str]) -> (usize, Vec<Point>) {
    let reports = parse(input);
    let mut beacons = reports[0].beacons().iter().cloned().collect::<HashSet<_>>();

    let mut scanners = Vec::with_capacity(reports.len());
    scanners.push(Point::new(0, 0, 0));

    let mut matched = reports[0]
        .fingerprint()
        .map(|(fingerprint, beacons)| (fingerprint, (beacons.0.clone(), beacons.1.clone())))
        .collect::<HashMap<_, _>>();

    let mut unmatched = reports[1..]
        .iter()
        .map(|report| (report, report.fingerprint().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    while !unmatched.is_empty() {
        let mut still_unmatched = Vec::with_capacity(unmatched.len());

        for (report, fingerprints) in unmatched {
            let result = solve_report(report, &fingerprints, &beacons, &matched);

            if let Some((scanner, report)) = result {
                for (fingerprint, beacons) in report.fingerprint() {
                    let entry = match matched.entry(fingerprint) {
                        Entry::Occupied(_) => continue,
                        Entry::Vacant(entry) => entry,
                    };

                    entry.insert((beacons.0.clone(), beacons.1.clone()));
                }

                beacons.extend(report.into_beacons());
                scanners.push(scanner);
            } else {
                still_unmatched.push((report, fingerprints));
            }
        }

        unmatched = still_unmatched;
    }

    (beacons.len(), scanners)
}

fn solve_report(
    report: &Report,
    fingerprints: &[(Fingerprint, (&Point, &Point))],
    beacons: &HashSet<Point>,
    matched: &HashMap<Fingerprint, (Point, Point)>,
) -> Option<(Point, Report)> {
    let pairs = fingerprints
        .iter()
        .filter_map(|(fingerprint, observed)| {
            matched.get(fingerprint).map(|known| (observed, known))
        })
        .collect::<Vec<_>>();

    if pairs.len() < MIN_FINGERPRINTS {
        return None;
    }

    let mut previous = Option::None;

    let mut transformations = pairs
        .iter()
        .filter_map(|(observed, known)| find_rotation(observed, known));

    transformations.find_map(|(rotation, translation)| {
        if matches!(previous, Some(previous) if std::ptr::eq(rotation, previous)) {
            return None;
        }

        let report = report.transform(rotation, &translation);

        let count = report
            .beacons()
            .iter()
            .filter(|beacon| beacons.contains(beacon))
            .count();

        if count < MIN_BEACONS {
            previous = Some(rotation);
            None
        } else {
            let scanner = &(rotation * &Point::new(0, 0, 0)) + &translation;
            Some((scanner, report))
        }
    })
}

fn find_rotation(
    observed: &(&Point, &Point),
    known: &(Point, Point),
) -> Option<(&'static Rotation, Point)> {
    ROTATIONS
        .iter()
        .find(|&rotation| {
            &known.0 - &(rotation * observed.0) == &known.1 - &(rotation * observed.1)
        })
        .map(|rotation| (rotation, &known.0 - &(rotation * observed.0)))
}

fn pairs<T>(items: &[T]) -> impl Iterator<Item = (&T, &T)> {
    items
        .iter()
        .enumerate()
        .flat_map(|(i, a)| items[i..].iter().map(move |b| (a, b)))
}

fn parse(input: &[&str]) -> Vec<Report> {
    input
        .split(|line| line.is_empty())
        .map(|input| {
            let beacons = input[1..]
                .iter()
                .map(|line| {
                    let (x, line) = line.split_once(',').unwrap();
                    let (y, z) = line.split_once(',').unwrap();

                    Point::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
                })
                .collect();

            Report::new(beacons)
        })
        .collect()
}

type Fingerprint = usize;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point(i16, i16, i16);

impl Point {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Point(x, y, z)
    }

    fn x(&self) -> i16 {
        self.0
    }

    fn y(&self) -> i16 {
        self.1
    }

    fn z(&self) -> i16 {
        self.2
    }

    fn fingerprint(&self, other: &Self) -> Fingerprint {
        let dx = (self.0 - other.0).abs() as usize;
        let dy = (self.1 - other.1).abs() as usize;
        let dz = (self.2 - other.2).abs() as usize;

        (dx + dy + dz) | (dx.max(dy).max(dz) << 16)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

struct Rotation([i16; 9]);

impl Rotation {
    const fn new(values: [i16; 9]) -> Self {
        Rotation(values)
    }
}

impl Mul<&Point> for &Rotation {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        let sx = rhs.x();
        let sy = rhs.y();
        let sz = rhs.z();

        let x = self.0[0] * sx + self.0[1] * sy + self.0[2] * sz;
        let y = self.0[3] * sx + self.0[4] * sy + self.0[5] * sz;
        let z = self.0[6] * sx + self.0[7] * sy + self.0[8] * sz;

        Point::new(x, y, z)
    }
}

struct Report(Vec<Point>);

impl Report {
    fn new(beacons: Vec<Point>) -> Self {
        Report(beacons)
    }

    fn beacons(&self) -> &[Point] {
        &self.0
    }

    fn into_beacons(self) -> Vec<Point> {
        self.0
    }

    fn transform(&self, rotation: &Rotation, translation: &Point) -> Self {
        let beacons = self
            .0
            .iter()
            .map(|beacon| &(rotation * beacon) + translation)
            .collect::<Vec<_>>();

        Report(beacons)
    }

    fn fingerprint(&self) -> impl Iterator<Item = (Fingerprint, (&Point, &Point))> {
        pairs(&self.0).map(|(a, b)| (a.fingerprint(b), (a, b)))
    }
}
