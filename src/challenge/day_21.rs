const PLAYER_COUNT: usize = 2;
const POSITION_COUNT: u8 = 10;

const WINNING_SCORE_A: u16 = 1000;
const WINNING_SCORE_B: u16 = 21;

const SCORE_MULTIPLIER: usize = POSITION_COUNT as usize;
const ID_MULTIPLIER: usize = SCORE_MULTIPLIER * WINNING_SCORE_B as usize;
const MAX_REALITIES: usize = ID_MULTIPLIER * ID_MULTIPLIER;

const ROLLS: [(u8, u8); 7] = [
    (9, 1), // 1 way to roll 9
    (8, 3), // 3 ways to roll 8
    (7, 6), // 6 ways to roll 7
    (6, 7), // 7 ways to roll 6
    (5, 6), // 6 ways to roll 5
    (4, 3), // 3 ways to roll 4
    (3, 1), // 1 way to roll 3
];

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut current_player = 0;
    let mut players = parse_players(input);
    let mut die = 6;
    let mut roll_count = 0;

    let result = loop {
        let player = &mut players[current_player];
        player.advance(die);
        let score = player.score();

        current_player = (current_player + 1) % PLAYER_COUNT;
        die = (die + 9) % POSITION_COUNT;
        roll_count += 3;

        if score >= WINNING_SCORE_A {
            break players[current_player].score() as u32 * roll_count;
        }
    };

    Ok(result)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    // the least points a player can earn in 2 rounds is 5 (1 and 4)
    // so each player will get to 20 in 4 turns, and to 21 or more in 5 turns

    // there are possible 10 positions and 21 possible (non-winning) scores
    // with 2 players that's 10^2 * 21^2 or 44100 unique realities

    let players = parse_players(input);
    let mut cache = [[0; PLAYER_COUNT]; MAX_REALITIES];
    let counts = simulate([&players[0], &players[1]], &mut cache);

    Ok(*counts.iter().max().unwrap())
}

fn universe_id(players: &[&Player; PLAYER_COUNT]) -> usize {
    players[0].id() * POSITION_COUNT as usize * WINNING_SCORE_B as usize + players[1].id()
}

fn simulate(
    players: [&Player; PLAYER_COUNT],
    cache: &mut [[u64; PLAYER_COUNT]; MAX_REALITIES],
) -> [u64; PLAYER_COUNT] {
    let id = universe_id(&players);
    let mut result = cache[id];

    if result[0] == 0 && result[1] == 0 {
        for (die, multiplier) in ROLLS {
            let mut player = players[0].clone();
            player.advance(die);

            if player.score() >= WINNING_SCORE_B {
                result[0] += multiplier as u64;
            } else {
                let [x, y] = simulate([players[1], &player], cache);
                result[0] += y * multiplier as u64;
                result[1] += x * multiplier as u64;
            }
        }

        cache[id] = result;
    }

    result
}

fn parse_players(input: &[&str]) -> [Player; PLAYER_COUNT] {
    [
        Player::new(parse_position(input[0])),
        Player::new(parse_position(input[1])),
    ]
}

fn parse_position(input: &str) -> u8 {
    (input.bytes().last().unwrap() - b'0' + POSITION_COUNT - 1) % POSITION_COUNT
}

#[derive(Clone)]
struct Player {
    position: u8,
    score: u16,
}

impl Player {
    fn new(position: u8) -> Self {
        Player { position, score: 0 }
    }

    fn id(&self) -> usize {
        self.score as usize * SCORE_MULTIPLIER + self.position as usize
    }

    fn score(&self) -> u16 {
        self.score
    }

    fn advance(&mut self, positions: u8) {
        self.position = (self.position + positions) % POSITION_COUNT;
        self.score += (self.position as u16) + 1;
    }
}
