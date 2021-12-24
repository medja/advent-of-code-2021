use anyhow::anyhow;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

const ROOM_COUNT: usize = 4;

const ENERGY_PER_STEP: [u32; ROOM_COUNT] = [1, 10, 100, 1000];

const HALLWAY_LENGTH: usize = 11;
const DOOR_POSITIONS: [usize; ROOM_COUNT] = [2, 4, 6, 8];

const INVALID_SPOT: u8 = 254;
const EMPTY_SPOT: u8 = 255;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Search::find_best_path(GameState::<8>::parse(input))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let extended = [
        input[0],
        input[1],
        input[2],
        "  #D#C#B#A#  ",
        "  #D#B#A#C#  ",
        input[3],
        input[4],
    ];

    Search::find_best_path(GameState::<16>::parse(&extended))
}

type PlayerPositions<const N: usize> = [u8; N];

struct Search<const N: usize> {
    queue: BinaryHeap<SearchState<N>>,
    costs: HashMap<PlayerPositions<N>, u32>,
    buffer: Vec<GameState<N>>,
}

impl<const N: usize> Search<N> {
    fn find_best_path(state: GameState<N>) -> anyhow::Result<u32> {
        let search = Search {
            queue: BinaryHeap::new(),
            costs: HashMap::new(),
            buffer: Vec::new(),
        };

        search.solve(state)
    }

    fn solve(mut self, state: GameState<N>) -> anyhow::Result<u32> {
        self.costs.insert(*state.positions(), state.cost());
        self.queue.push(SearchState::from_game_state(state));

        while let Some(state) = self.queue.pop() {
            let state = match self.costs.get(state.positions()) {
                Some(cost) => state.build_game_state(*cost),
                None => unreachable!(),
            };

            if state.is_won() {
                return Ok(state.cost());
            }

            state.find_next_states(&mut self.buffer);

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

        Err(anyhow!("Cannot find solution"))
    }
}

#[derive(Eq, PartialEq)]
struct SearchState<const N: usize> {
    estimate: u32,
    positions: PlayerPositions<N>,
}

impl<const N: usize> SearchState<N> {
    fn from_game_state(state: GameState<N>) -> Self {
        SearchState {
            estimate: state.estimate(),
            positions: *state.positions(),
        }
    }

    fn positions(&self) -> &PlayerPositions<N> {
        &self.positions
    }

    fn build_game_state(self, cost: u32) -> GameState<N> {
        GameState::new(cost, self.positions)
    }
}

impl<const N: usize> Ord for SearchState<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

impl<const N: usize> PartialOrd for SearchState<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct GameState<const N: usize> {
    cost: u32,
    positions: PlayerPositions<N>,
}

impl<const N: usize> GameState<N> {
    fn new(cost: u32, positions: PlayerPositions<N>) -> Self {
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

    fn positions(&self) -> &PlayerPositions<N> {
        &self.positions
    }

    fn is_won(&self) -> bool {
        let room_size = N / ROOM_COUNT;

        self.positions()
            .chunks(room_size)
            .enumerate()
            .all(|(i, room)| {
                let start = (HALLWAY_LENGTH + room_size * i) as u8;
                let stop = start + room_size as u8;

                room.iter()
                    .all(|&position| position >= start && position < stop)
            })
    }

    fn find_next_states(&self, buffer: &mut Vec<Self>) {
        let room_size = N / ROOM_COUNT;

        let mut hallway_positions = [EMPTY_SPOT; HALLWAY_LENGTH];
        let mut room_positions = [EMPTY_SPOT; N];

        for position in DOOR_POSITIONS {
            hallway_positions[position] = INVALID_SPOT;
        }

        for (index, position) in self.positions.iter().enumerate() {
            let position = *position as usize;

            if position < HALLWAY_LENGTH {
                hallway_positions[position] = index as u8;
            } else {
                room_positions[position - HALLWAY_LENGTH] = index as u8;
            }
        }

        for (index, position) in self.positions.iter().enumerate() {
            let position = *position as usize;
            let r#type = index / room_size;

            if position < HALLWAY_LENGTH {
                // we're in the hallway
                let door_position = DOOR_POSITIONS[r#type];

                let mut hallway_range = if position < door_position {
                    position + 1..door_position
                } else {
                    door_position + 1..position
                };

                let distance = hallway_range.len();

                if !hallway_range.all(|i| hallway_positions[i] >= INVALID_SPOT) {
                    // cannot access the room
                    continue;
                }

                let room_start = r#type * room_size;
                let mut position = room_start + room_size;

                while position > room_start {
                    position -= 1;
                    let room = room_positions[position];

                    if room == EMPTY_SPOT {
                        let distance = distance + 2 + position - room_start;
                        let position = position + HALLWAY_LENGTH;
                        buffer.push(self.move_player(distance, index, r#type, position));
                        break;
                    }

                    if room as usize / room_size != r#type {
                        // room is occupied by a different kind
                        break;
                    }
                }
            } else {
                // we're in a room
                let position = position - HALLWAY_LENGTH;
                let room = position / room_size;
                let depth = position % room_size;
                let room_start = room * room_size;

                if room == r#type
                    && (depth + 1..room_size)
                        .all(|i| room_positions[room_start + i] as usize / room_size == r#type)
                {
                    // we're in the right room
                    continue;
                }

                if depth > 0 && (0..depth).any(|i| room_positions[room_start + i] != EMPTY_SPOT) {
                    // we're blocked in the room
                    continue;
                }

                let door_position = DOOR_POSITIONS[room];
                let mut distance = 2 + depth;

                for (i, &spot) in hallway_positions
                    .iter()
                    .enumerate()
                    .take(door_position)
                    .rev()
                {
                    if spot == EMPTY_SPOT {
                        buffer.push(self.move_player(distance, index, r#type, i));
                    } else if spot != INVALID_SPOT {
                        break;
                    }

                    distance += 1;
                }

                distance = 2 + depth;

                for (i, &spot) in hallway_positions.iter().enumerate().skip(door_position + 1) {
                    if spot == EMPTY_SPOT {
                        buffer.push(self.move_player(distance, index, r#type, i));
                    } else if spot != INVALID_SPOT {
                        break;
                    }

                    distance += 1;
                }
            }
        }
    }

    fn estimate(&self) -> u32 {
        let room_size = N / ROOM_COUNT;

        let estimate = self
            .positions
            .iter()
            .enumerate()
            .map(|(index, position)| {
                let r#type = index / room_size;
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

    fn move_player(
        &self,
        distance: usize,
        index: usize,
        r#type: usize,
        destination: usize,
    ) -> Self {
        let cost = self.cost + distance as u32 * ENERGY_PER_STEP[r#type];

        let mut positions = self.positions;
        positions[index] = destination as u8;

        GameState { cost, positions }
    }
}

struct GameStateBuilder<const N: usize> {
    positions: PlayerPositions<N>,
    indices: [usize; ROOM_COUNT],
    position: u8,
}

impl<const N: usize> GameStateBuilder<N> {
    fn new() -> Self {
        let room_size = N / ROOM_COUNT;
        let positions = [0; N];
        let indices = [0, room_size, 2 * room_size, 3 * room_size];
        let position = HALLWAY_LENGTH as u8;

        GameStateBuilder {
            positions,
            indices,
            position,
        }
    }

    fn parse(&mut self, input: &[&str]) {
        let room_size = N / ROOM_COUNT;

        for x in 0..ROOM_COUNT {
            for y in 0..room_size {
                self.insert(input[2 + y].as_bytes()[3 + 2 * x]);
            }
        }
    }

    fn build(self) -> GameState<N> {
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
