use anyhow::anyhow;

const BOARD_SIZE: usize = 5;
const SENTINEL: u8 = 0xff;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut boards = parse_boards(&input[1..])?;

    for number in input[0].split(',') {
        let number = number.parse::<u8>()?;

        for board in &mut boards {
            if board.mark(number) {
                return Ok(board.score(number));
            }
        }
    }

    return Err(anyhow!("Could not find winning board"));
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut boards = parse_boards(&input[1..])?;
    let mut remaining = boards.len();

    for number in input[0].split(',') {
        let number = number.parse::<u8>()?;

        for board in &mut boards {
            if board.mark(number) {
                if remaining == 1 {
                    return Ok(board.score(number));
                } else {
                    remaining -= 1;
                }
            }
        }
    }

    return Err(anyhow!("Could not find last winning board"));
}

fn parse_boards(input: &[&str]) -> anyhow::Result<Vec<Board>> {
    input
        .chunks_exact(BOARD_SIZE + 1)
        .map(|chunk| chunk[1..].try_into())
        .collect::<Result<Vec<Board>, _>>()
}

struct Board([u8; BOARD_SIZE * BOARD_SIZE], bool);

impl Board {
    fn score(&self, number: u8) -> u32 {
        let mut score = 0u32;

        for number in self.0 {
            if number != SENTINEL {
                score += number as u32;
            }
        }

        score * number as u32
    }

    fn mark(&mut self, number: u8) -> bool {
        if self.1 {
            return false;
        }

        let index = match self.0.iter().position(|&x| x == number) {
            Some(index) => index,
            None => return false,
        };

        self.0[index] = SENTINEL;

        if self.check_column(index % BOARD_SIZE) || self.check_row(index / BOARD_SIZE) {
            self.1 = true;
            true
        } else {
            false
        }
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
            .flat_map(|line| line.split_whitespace())
            .map(|number| number.parse())
            .take(BOARD_SIZE * BOARD_SIZE);

        for (i, number) in input.enumerate() {
            board[i] = number?;
        }

        Ok(Board(board, false))
    }
}
