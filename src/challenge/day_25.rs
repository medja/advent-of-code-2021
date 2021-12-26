pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut map = Game::parse(input);
    let mut counter = 1;

    while map.advance() {
        counter += 1;
    }

    Ok(counter)
}

#[derive(Copy, Clone)]
enum Spot {
    Empty,
    East,
    South,
}

impl From<u8> for Spot {
    fn from(byte: u8) -> Self {
        match byte {
            b'.' => Spot::Empty,
            b'>' => Spot::East,
            b'v' => Spot::South,
            _ => unreachable!(),
        }
    }
}

struct Game {
    width: usize,
    height: usize,
    spots: Vec<Spot>,
    swap: Vec<Spot>,
}

impl Game {
    fn parse(input: &[&str]) -> Self {
        let width = input[0].len();
        let height = input.len();
        let swap = vec![Spot::Empty; width * height];

        let spots = input
            .iter()
            .flat_map(|line| line.bytes())
            .map(|byte| byte.into())
            .collect();

        Game {
            width,
            height,
            spots,
            swap,
        }
    }

    fn advance(&mut self) -> bool {
        let mut moved = false;
        moved |= self.move_east();
        moved |= self.move_south();
        moved
    }

    fn move_east(&mut self) -> bool {
        let mut moved = false;

        self.swap.copy_from_slice(&self.spots);

        for y in 0..self.height {
            for x in 0..(self.width - 1) {
                let index = x + y * self.width;
                let next_index = index + 1;

                if matches!(self.spots[index], Spot::East)
                    && matches!(self.spots[next_index], Spot::Empty)
                {
                    moved = true;
                    self.swap[index] = Spot::Empty;
                    self.swap[next_index] = Spot::East;
                }
            }
        }

        for y in 0..self.height {
            let next_index = y * self.width;
            let index = next_index + self.width - 1;

            if matches!(self.spots[index], Spot::East)
                && matches!(self.spots[next_index], Spot::Empty)
            {
                moved = true;
                self.swap[index] = Spot::Empty;
                self.swap[next_index] = Spot::East;
            }
        }

        moved
    }

    fn move_south(&mut self) -> bool {
        let mut moved = false;

        self.spots.copy_from_slice(&self.swap);

        for y in 0..(self.height - 1) {
            for x in 0..self.width {
                let index = x + y * self.width;
                let next_index = index + self.width;

                if matches!(self.swap[index], Spot::South)
                    && matches!(self.swap[next_index], Spot::Empty)
                {
                    moved = true;
                    self.spots[index] = Spot::Empty;
                    self.spots[next_index] = Spot::South;
                }
            }
        }

        for x in 0..self.width {
            let index = self.spots.len() - self.width + x;
            let next_index = x;

            if matches!(self.swap[index], Spot::South)
                && matches!(self.swap[next_index], Spot::Empty)
            {
                moved = true;
                self.spots[index] = Spot::Empty;
                self.spots[next_index] = Spot::South;
            }
        }

        moved
    }
}
