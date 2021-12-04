use anyhow::anyhow;

const BOARD_SIZE: usize = 5;
const SENTINEL: u8 = 0xff;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut boards = input[1..]
        .chunks_exact(BOARD_SIZE + 1)
        .map(|chunk| chunk[1..].try_into())
        .collect::<Result<Vec<Board>, _>>()?;

    for number in input[0].split(',') {
        let number = number.parse::<u8>()?;

        for board in &mut boards {
            if board.mark(number) {
                return Ok(board.remaining().map(|&x| x as u32).sum::<u32>() * number as u32);
            }
        }
    }

    return Err(anyhow!("Could not find a winning board"));
}

struct Board([u8; BOARD_SIZE * BOARD_SIZE]);

impl Board {
    fn remaining(&self) -> impl Iterator<Item = &u8> {
        self.0.iter().filter(|&&x| x != SENTINEL)
    }

    fn mark(&mut self, number: u8) -> bool {
        let index = match self.0.iter().position(|&x| x == number) {
            Some(index) => index,
            None => return false,
        };

        self.0[index] = SENTINEL;

        self.check_column(index % BOARD_SIZE) || self.check_row(index / BOARD_SIZE)
    }

    fn check_column(&self, x: usize) -> bool {
        (0..BOARD_SIZE)
            .map(|i| x + i * BOARD_SIZE)
            .all(|i| self.0[i] == SENTINEL)
    }

    fn check_row(&self, y: usize) -> bool {
        (0..BOARD_SIZE)
            .map(|i| i + y * BOARD_SIZE)
            .all(|i| self.0[i] == SENTINEL)
    }
}

impl TryFrom<&[&str]> for Board {
    type Error = anyhow::Error;

    fn try_from(lines: &[&str]) -> Result<Self, Self::Error> {
        let mut board = [SENTINEL; BOARD_SIZE * BOARD_SIZE];

        let input = lines
            .iter()
            .flat_map(|line| line.split(' '))
            .filter(|number| !number.is_empty())
            .map(|number| number.parse())
            .take(BOARD_SIZE * BOARD_SIZE);

        for (i, number) in input.enumerate() {
            board[i] = number?;
        }

        Ok(Board(board))
    }
}
