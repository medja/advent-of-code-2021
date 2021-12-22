pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut cubes = vec![false; 100 * 100 * 100];

    for line in input {
        if let Some(command) = Command::from_str(line) {
            for z in command.min_z..=command.max_z {
                let index = z * 100;

                for y in command.min_y..=command.max_y {
                    let index = (y + index) * 100;

                    for x in command.min_x..=command.max_x {
                        cubes[x + index] = command.enable;
                    }
                }
            }
        }
    }

    Ok(cubes.iter().filter(|enable| **enable).count())
}

struct Command {
    enable: bool,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    min_z: usize,
    max_z: usize,
}

impl Command {
    fn from_str(input: &str) -> Option<Self> {
        let (enable, input) = if *input.as_bytes().get(1).unwrap() == b'n' {
            (true, &input[3..])
        } else {
            (false, &input[4..])
        };

        let (min_x, input) = input[2..].split_once('.').unwrap();

        // ignore values below -50 and above +50
        if min_x.len() > 3 || (min_x.len() == 3 && !min_x.starts_with('-')) {
            return None;
        }

        let (max_x, input) = input[1..].split_once(',').unwrap();
        let (min_y, input) = input[2..].split_once('.').unwrap();
        let (max_y, input) = input[1..].split_once(',').unwrap();
        let (min_z, input) = input[2..].split_once('.').unwrap();
        let max_z = &input[1..];

        let command = Command {
            enable,
            min_x: parse_coordinate(min_x),
            max_x: parse_coordinate(max_x),
            min_y: parse_coordinate(min_y),
            max_y: parse_coordinate(max_y),
            min_z: parse_coordinate(min_z),
            max_z: parse_coordinate(max_z),
        };

        Some(command)
    }
}

fn parse_coordinate(value: &str) -> usize {
    (value.parse::<isize>().unwrap() + 50) as usize
}
