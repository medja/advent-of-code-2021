pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, false))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, true))
}

fn solve(input: &[&str], large: bool) -> u64 {
    let mut cuboids = Vec::<Cuboid>::new();
    let mut count = 0;

    for line in input {
        if let Some(cuboid) = parse_cuboid(line, large) {
            let mut index = 0;
            let mut length = cuboids.len();

            while index < length {
                let existing = &cuboids[index];
                let current_index = index;
                index += 1;

                let intersection = match existing.intersection(&cuboid) {
                    Some(intersection) => intersection,
                    None => continue,
                };

                if intersection.enabled {
                    count += intersection.size();
                } else {
                    count -= intersection.size();
                }

                if !intersection.negates(existing) {
                    cuboids.push(intersection);
                    continue;
                }

                cuboids.swap_remove(current_index);

                if length > cuboids.len() {
                    index = current_index;
                    length -= 1;
                }
            }

            if cuboid.enabled {
                count += cuboid.size();
                cuboids.push(cuboid);
            }
        }
    }

    count
}

#[derive(Debug)]
struct Cuboid {
    enabled: bool,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl Cuboid {
    fn size(&self) -> u64 {
        ((self.max_x - self.min_x + 1)
            * (self.max_y - self.min_y + 1)
            * (self.max_z - self.min_z + 1)) as u64
    }

    fn negates(&self, other: &Self) -> bool {
        self.enabled != other.enabled
            && self.min_x == other.min_x
            && self.max_x == other.max_x
            && self.min_y == other.min_y
            && self.max_y == other.max_y
            && self.min_z == other.min_z
            && self.max_z == other.max_z
    }

    fn intersection(&self, other: &Self) -> Option<Cuboid> {
        let min_x = self.min_x.max(other.min_x);
        let max_x = self.max_x.min(other.max_x);
        let min_y = self.min_y.max(other.min_y);
        let max_y = self.max_y.min(other.max_y);
        let min_z = self.min_z.max(other.min_z);
        let max_z = self.max_z.min(other.max_z);

        if min_x <= max_x && min_y <= max_y && min_z <= max_z {
            let enabled = !self.enabled;
            Some(Cuboid {
                enabled,
                min_x,
                max_x,
                min_y,
                max_y,
                min_z,
                max_z,
            })
        } else {
            None
        }
    }
}

fn parse_cuboid(input: &str, large: bool) -> Option<Cuboid> {
    let (enabled, input) = if *input.as_bytes().get(1).unwrap() == b'n' {
        (true, &input[3..])
    } else {
        (false, &input[4..])
    };

    let (min_x, input) = input[2..].split_once('.').unwrap();
    let (max_x, input) = input[1..].split_once(',').unwrap();
    let (min_y, input) = input[2..].split_once('.').unwrap();
    let (max_y, input) = input[1..].split_once(',').unwrap();
    let (min_z, input) = input[2..].split_once('.').unwrap();
    let max_z = &input[1..];

    let min_x = min_x.parse::<isize>().unwrap();
    let max_x = max_x.parse::<isize>().unwrap();
    let min_y = min_y.parse::<isize>().unwrap();
    let max_y = max_y.parse::<isize>().unwrap();
    let min_z = min_z.parse::<isize>().unwrap();
    let max_z = max_z.parse::<isize>().unwrap();

    if !large && min_x.abs() > 50 {
        None
    } else {
        Some(Cuboid {
            enabled,
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        })
    }
}
