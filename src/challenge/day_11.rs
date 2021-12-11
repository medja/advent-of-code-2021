const MAP_SIZE: usize = 10;
const MAX_INDEX: usize = MAP_SIZE - 1;
const OCTOPUS_COUNT: usize = MAP_SIZE * MAP_SIZE;
const MAX_ENERGY: u8 = 10;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut octopuses = parse(input);
    let mut flashes = 0usize;

    for _ in 0..100 {
        flashes += tick(&mut octopuses);
    }

    Ok(flashes)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut octopuses = parse(input);
    let mut iteration = 1usize;

    while tick(&mut octopuses) != OCTOPUS_COUNT {
        iteration += 1;
    }

    Ok(iteration)
}

fn parse(input: &[&str]) -> [u8; OCTOPUS_COUNT] {
    let mut octopuses = [0u8; OCTOPUS_COUNT];

    for (i, byte) in input.iter().flat_map(|line| line.bytes()).enumerate() {
        octopuses[i] = byte - b'0';
    }

    octopuses
}

fn tick(octopuses: &mut [u8; OCTOPUS_COUNT]) -> usize {
    let mut flashes = 0usize;

    for index in 0..octopuses.len() {
        simulate(index, octopuses);
    }

    for octopus in octopuses {
        if *octopus >= MAX_ENERGY {
            *octopus = 0;
            flashes += 1;
        }
    }

    flashes
}

fn simulate(index: usize, octopuses: &mut [u8; OCTOPUS_COUNT]) {
    let octopus = match octopuses.get_mut(index) {
        Some(octopus) => octopus,
        None => return,
    };

    let value = *octopus + 1;
    *octopus = value;

    if value != MAX_ENERGY {
        return;
    }

    let x = index % MAP_SIZE;
    let y = index / MAP_SIZE;

    if y > 0 {
        let top = index - MAP_SIZE;
        simulate(top, octopuses);

        if x > 0 {
            simulate(top - 1, octopuses);
        }

        if x < MAX_INDEX {
            simulate(top + 1, octopuses);
        }
    }

    if x > 0 {
        simulate(index - 1, octopuses);
    }

    if x < MAX_INDEX {
        simulate(index + 1, octopuses);
    }

    if y < MAX_INDEX {
        let bottom = index + MAP_SIZE;
        simulate(bottom, octopuses);

        if x > 0 {
            simulate(bottom - 1, octopuses);
        }

        if x < MAX_INDEX {
            simulate(bottom + 1, octopuses);
        }
    }
}
