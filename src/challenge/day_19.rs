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
    let scanners = parse(input);

    let mut beacons = scanners[0]
        .beacons()
        .iter()
        .cloned()
        .collect::<HashSet<_>>();

    let mut matched = scanners[0]
        .fingerprint()
        .map(|(fingerprint, beacons)| (fingerprint, [beacons[0].clone(), beacons[1].clone()]))
        .collect::<HashMap<_, _>>();

    let mut unmatched = scanners[1..]
        .iter()
        .map(|scanner| (scanner, scanner.fingerprint().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    while !unmatched.is_empty() {
        let mut still_unmatched = Vec::with_capacity(unmatched.len());

        for (scanner, fingerprints) in unmatched {
            if let Some(scanner) = solve_scanner(scanner, &fingerprints, &beacons, &matched) {
                for (fingerprint, beacons) in scanner.fingerprint() {
                    let entry = match matched.entry(fingerprint) {
                        Entry::Occupied(_) => continue,
                        Entry::Vacant(entry) => entry,
                    };

                    entry.insert([beacons[0].clone(), beacons[1].clone()]);
                }

                beacons.extend(scanner.into_beacons());
            } else {
                still_unmatched.push((scanner, fingerprints));
            }
        }

        unmatched = still_unmatched;
    }

    Ok(beacons.len())
}

fn solve_scanner(
    scanner: &Scanner,
    fingerprints: &[(Fingerprint, [&Beacon; 2])],
    beacons: &HashSet<Beacon>,
    matched: &HashMap<Fingerprint, [Beacon; 2]>,
) -> Option<Scanner> {
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

        let scanner = scanner.transform(rotation, &translation);

        let count = scanner
            .beacons()
            .iter()
            .filter(|beacon| beacons.contains(beacon))
            .count();

        if count < MIN_BEACONS {
            previous = Some(rotation);
            None
        } else {
            Some(scanner)
        }
    })
}

fn find_rotation(
    observed: &[&Beacon; 2],
    known: &[Beacon; 2],
) -> Option<(&'static Rotation, Beacon)> {
    ROTATIONS
        .iter()
        .find(|&rotation| {
            &known[0] - &(rotation * observed[0]) == &known[1] - &(rotation * observed[1])
        })
        .map(|rotation| (rotation, &known[0] - &(rotation * observed[0])))
}

fn parse(input: &[&str]) -> Vec<Scanner> {
    input
        .split(|line| line.is_empty())
        .map(|input| {
            let beacons = input[1..]
                .iter()
                .map(|line| {
                    let (x, line) = line.split_once(',').unwrap();
                    let (y, z) = line.split_once(',').unwrap();

                    Beacon::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
                })
                .collect();

            Scanner::new(beacons)
        })
        .collect()
}

type Fingerprint = usize;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Beacon(i16, i16, i16);

impl Beacon {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Beacon(x, y, z)
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

impl Add for &Beacon {
    type Output = Beacon;

    fn add(self, rhs: Self) -> Self::Output {
        Beacon(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for &Beacon {
    type Output = Beacon;

    fn sub(self, rhs: Self) -> Self::Output {
        Beacon(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

struct Rotation([i16; 9]);

impl Rotation {
    const fn new(values: [i16; 9]) -> Self {
        Rotation(values)
    }
}

impl Mul<&Beacon> for &Rotation {
    type Output = Beacon;

    fn mul(self, rhs: &Beacon) -> Self::Output {
        let sx = rhs.x();
        let sy = rhs.y();
        let sz = rhs.z();

        let x = self.0[0] * sx + self.0[1] * sy + self.0[2] * sz;
        let y = self.0[3] * sx + self.0[4] * sy + self.0[5] * sz;
        let z = self.0[6] * sx + self.0[7] * sy + self.0[8] * sz;

        Beacon::new(x, y, z)
    }
}

struct Scanner(Vec<Beacon>);

impl Scanner {
    fn new(beacons: Vec<Beacon>) -> Self {
        Scanner(beacons)
    }

    fn beacons(&self) -> &[Beacon] {
        &self.0
    }

    fn into_beacons(self) -> Vec<Beacon> {
        self.0
    }

    fn transform(&self, rotation: &Rotation, translation: &Beacon) -> Self {
        let beacons = self
            .0
            .iter()
            .map(|beacon| &(rotation * beacon) + translation)
            .collect::<Vec<_>>();

        Scanner(beacons)
    }

    fn fingerprint(&self) -> impl Iterator<Item = (Fingerprint, [&Beacon; 2])> + '_ {
        self.0[..self.0.len() - 1]
            .iter()
            .enumerate()
            .flat_map(|(i, a)| self.0[i..].iter().map(move |b| (a, b)))
            .map(|(a, b)| (a.fingerprint(b), [a, b]))
    }
}
