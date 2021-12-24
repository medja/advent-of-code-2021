use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

const PLAYER_COUNT: usize = 8;
const ROOM_COUNT: usize = 4;

const ENERGY_PER_STEP: [u32; ROOM_COUNT] = [1, 10, 100, 1000];

const HALLWAY_LENGTH: usize = 11;
const DOOR_POSITIONS: [usize; ROOM_COUNT] = [2, 4, 6, 8];

const INVALID_SPOT: u8 = 254;
const EMPTY_SPOT: u8 = 255;

type PlayerPositions = [u8; PLAYER_COUNT];

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Search::find_best_path(GameState::parse(input)))
}

struct Search {
    queue: BinaryHeap<SearchState>,
    costs: HashMap<PlayerPositions, u32>,
    buffer: Vec<GameState>,
}

impl Search {
    fn find_best_path(state: GameState) -> u32 {
        let search = Search {
            queue: BinaryHeap::new(),
            costs: HashMap::new(),
            buffer: Vec::new(),
        };

        search.solve(state)
    }

    fn solve(mut self, state: GameState) -> u32 {
        self.costs.insert(*state.positions(), state.cost());
        self.queue.push(SearchState::from_game_state(state));

        while let Some(state) = self.queue.pop() {
            let state = match self.costs.get(state.positions()) {
                Some(cost) => state.build_game_state(*cost),
                None => unreachable!(),
            };

            if state.is_won() {
                return state.cost();
            }

            state.find_next_states(&mut self.buffer);

            // Is removing one by one faster?
            for state in self.buffer.drain(..) {
                let cost = state.cost();

                match self.costs.entry(*state.positions()) {
                    Entry::Occupied(mut entry) => {
                        if *entry.get() > cost {
                            entry.insert(cost);
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(cost);
                        self.queue.push(SearchState::from_game_state(state));
                    }
                }
            }
        }

        unreachable!()
    }
}

#[derive(Eq, PartialEq)]
struct SearchState {
    estimate: u32,
    positions: PlayerPositions,
}

impl SearchState {
    fn from_game_state(state: GameState) -> Self {
        SearchState {
            estimate: state.estimate(),
            positions: state.into_positions(),
        }
    }

    fn positions(&self) -> &PlayerPositions {
        &self.positions
    }

    fn build_game_state(self, cost: u32) -> GameState {
        GameState::new(cost, self.positions)
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct GameState {
    cost: u32,
    positions: PlayerPositions,
}

impl GameState {
    fn new(cost: u32, positions: PlayerPositions) -> Self {
        GameState { cost, positions }
    }

    fn parse(input: &[&str]) -> Self {
        let mut builder = GameStateBuilder::new();
        builder.parse(input);
        builder.build()
    }

    fn cost(&self) -> u32 {
        self.cost
    }

    fn positions(&self) -> &PlayerPositions {
        &self.positions
    }

    fn into_positions(self) -> PlayerPositions {
        self.positions
    }

    fn is_won(&self) -> bool {
        (self.positions[0] == 11 || self.positions[0] == 12)
            && (self.positions[1] == 11 || self.positions[1] == 12)
            && (self.positions[2] == 13 || self.positions[2] == 14)
            && (self.positions[3] == 13 || self.positions[3] == 14)
            && (self.positions[4] == 15 || self.positions[4] == 16)
            && (self.positions[5] == 15 || self.positions[5] == 16)
            && (self.positions[6] == 17 || self.positions[6] == 18)
            && (self.positions[7] == 17 || self.positions[7] == 18)
    }

    fn find_next_states(&self, buffer: &mut Vec<GameState>) {
        let mut positions = [EMPTY_SPOT; HALLWAY_LENGTH + PLAYER_COUNT];

        for position in DOOR_POSITIONS {
            positions[position] = INVALID_SPOT;
        }

        for (index, position) in self.positions.iter().enumerate() {
            positions[*position as usize] = index as u8;
        }

        for (index, position) in self.positions.iter().enumerate() {
            let position = *position as usize;
            let r#type = index / 2;

            if position < HALLWAY_LENGTH {
                // we're in the hallway
                let door_position = DOOR_POSITIONS[r#type];

                // are we next to our room?
                if position + 1 == door_position || door_position + 1 == position {
                    let room_position = HALLWAY_LENGTH + r#type * 2 + 1;
                    let room = positions[HALLWAY_LENGTH + r#type * 2 + 1];

                    // can enter our room?
                    if room == EMPTY_SPOT {
                        buffer.push(self.move_player(3, index, room_position));
                        continue;
                    } else if room as usize / 2 == r#type {
                        buffer.push(self.move_player(2, index, room_position - 1));
                        continue;
                    }
                }

                // seems like we can't enter our room

                // can get to one of the edges?
                if position == 1 && positions[0] == EMPTY_SPOT {
                    buffer.push(self.move_player(1, index, 0));
                }

                if position == 9 && positions[10] == EMPTY_SPOT {
                    buffer.push(self.move_player(1, index, 10));
                }

                if position == 0 && positions[1] == EMPTY_SPOT {
                    buffer.push(self.move_player(1, index, 1));
                }

                if position == 10 && positions[9] == EMPTY_SPOT {
                    buffer.push(self.move_player(1, index, 9));
                }

                // what about move left (in the hallway)?
                if position > 2 && positions[position - 2] == EMPTY_SPOT {
                    buffer.push(self.move_player(2, index, position - 2));
                }

                // or move right?
                if position < 8 && positions[position + 2] == EMPTY_SPOT {
                    buffer.push(self.move_player(2, index, position + 2));
                }
            } else {
                // we're in a room
                let position_in_room = position - HALLWAY_LENGTH;
                let room = position_in_room / 2;
                let front = position_in_room % 2 == 0;

                if front {
                    // are we in our room with our kind?
                    if room == r#type && positions[position + 1] as usize / 2 == r#type {
                        continue;
                    }
                } else {
                    // is this our room or are we stuck?
                    if room == r#type || positions[position - 1] != EMPTY_SPOT {
                        continue;
                    }
                }

                // we're in the wrong room and we can get to the door
                let door_position = DOOR_POSITIONS[room];
                let distance = if front { 2 } else { 3 };

                if positions[door_position - 1] == EMPTY_SPOT {
                    buffer.push(self.move_player(distance, index, door_position - 1));
                }

                if positions[door_position + 1] == EMPTY_SPOT {
                    buffer.push(self.move_player(distance, index, door_position + 1));
                }
            }
        }
    }

    fn estimate(&self) -> u32 {
        let estimate = self
            .positions
            .iter()
            .enumerate()
            .map(|(index, position)| {
                let r#type = index / 2;
                let destination = DOOR_POSITIONS[r#type];
                let mut position = *position as usize;

                if position >= HALLWAY_LENGTH {
                    position -= HALLWAY_LENGTH;
                }

                let distance = if position < destination {
                    destination - position
                } else {
                    position - destination
                };

                distance as u32 * ENERGY_PER_STEP[r#type]
            })
            .sum::<u32>();

        self.cost + estimate
    }

    fn move_player(&self, distance: usize, index: usize, destination: usize) -> GameState {
        let cost = self.cost + distance as u32 * ENERGY_PER_STEP[index / 2];

        let mut positions = self.positions;
        positions[index] = destination as u8;

        GameState { cost, positions }
    }
}

struct GameStateBuilder {
    positions: PlayerPositions,
    indices: [usize; ROOM_COUNT],
    position: u8,
}

impl GameStateBuilder {
    fn new() -> Self {
        let positions = [0; PLAYER_COUNT];
        let indices = [0, 2, 4, 6];
        let position = HALLWAY_LENGTH as u8;

        GameStateBuilder {
            positions,
            indices,
            position,
        }
    }

    fn parse(&mut self, input: &[&str]) {
        self.insert(input[2].as_bytes()[3]);
        self.insert(input[3].as_bytes()[3]);
        self.insert(input[2].as_bytes()[5]);
        self.insert(input[3].as_bytes()[5]);
        self.insert(input[2].as_bytes()[7]);
        self.insert(input[3].as_bytes()[7]);
        self.insert(input[2].as_bytes()[9]);
        self.insert(input[3].as_bytes()[9]);
    }

    fn build(self) -> GameState {
        GameState::new(0, self.positions)
    }

    fn insert(&mut self, r#type: u8) {
        let r#type = (r#type - b'A') as usize;
        let index = self.indices[r#type];
        self.positions[index] = self.position;
        self.indices[r#type] += 1;
        self.position += 1;
    }
}
