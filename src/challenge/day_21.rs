const POSITION_COUNT: u8 = 10;
const WINNING_SCORE: u16 = 1000;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut current_player = 0;
    let mut players = parse_players(input);
    let mut die = 6;
    let mut roll_count = 0;

    let result = loop {
        let player = &mut players[current_player];
        player.advance(die);
        let score = player.score();

        current_player = (current_player + 1) % players.len();
        die = (die + 9) % POSITION_COUNT;
        roll_count += 3;

        if score >= WINNING_SCORE {
            break players[current_player].score() as u32 * roll_count;
        }
    };

    Ok(result)
}

fn parse_players(input: &[&str]) -> [Player; 2] {
    [
        Player::new(parse_position(input[0])),
        Player::new(parse_position(input[1])),
    ]
}

fn parse_position(input: &str) -> u8 {
    (input.bytes().last().unwrap() - b'0' + POSITION_COUNT - 1) % POSITION_COUNT
}

struct Player {
    position: u8,
    score: u16,
}

impl Player {
    fn new(position: u8) -> Self {
        Player { position, score: 0 }
    }

    fn score(&self) -> u16 {
        self.score
    }

    fn advance(&mut self, positions: u8) {
        self.position = (self.position + positions) % 10;
        self.score += (self.position as u16) + 1;
    }
}