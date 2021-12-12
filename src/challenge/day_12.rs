use anyhow::Context;
use std::ops::{Index, IndexMut};

const LOWER_CASE_MASK: u8 = 0x20;

const START_CAVE_ID: u8 = 96;
const END_CAVE_ID: u8 = 95;

const CAVE_SET_MIN_ID: usize = 65;
const CAVE_SET_MAX_ID: usize = 122;
const CAVE_SET_SIZE: usize = CAVE_SET_MAX_ID - CAVE_SET_MIN_ID + 1;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Graph::parse(input)?.count_paths())
}

#[derive(Copy, Clone)]
enum CaveSize {
    Small,
    Big,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct CaveId(u8);

impl CaveId {
    fn start() -> Self {
        CaveId(START_CAVE_ID)
    }

    fn new(name: &str) -> Self {
        match name.len() {
            3 => CaveId(END_CAVE_ID),
            5 => CaveId(START_CAVE_ID),
            _ => CaveId(name.as_bytes()[0]),
        }
    }

    fn is_end(&self) -> bool {
        self.0 == END_CAVE_ID
    }

    fn cave_size(&self) -> CaveSize {
        if self.0 & LOWER_CASE_MASK == LOWER_CASE_MASK {
            CaveSize::Small
        } else {
            CaveSize::Big
        }
    }

    fn cave_hash(&self) -> usize {
        self.0 as usize - CAVE_SET_MIN_ID
    }
}

struct Cave {
    size: CaveSize,
    neighbors: Vec<CaveId>,
}

impl Cave {
    fn new(size: CaveSize) -> Self {
        let neighbors = Vec::new();
        Cave { size, neighbors }
    }

    fn size(&self) -> CaveSize {
        self.size
    }

    fn set_size(&mut self, size: CaveSize) {
        self.size = size;
    }

    fn neighbors(&self) -> &[CaveId] {
        &self.neighbors
    }

    fn add_neighbor(&mut self, id: CaveId) {
        self.neighbors.push(id)
    }
}

struct CaveLookup<T>([T; CAVE_SET_SIZE]);

impl<T: Default + Copy> CaveLookup<T> {
    fn new() -> Self {
        CaveLookup([T::default(); CAVE_SET_SIZE])
    }
}

impl<T> CaveLookup<T> {
    fn from_fn(f: impl Fn() -> T) -> Self {
        CaveLookup([0usize; CAVE_SET_SIZE].map(|_| f()))
    }
}

impl<T> Index<CaveId> for CaveLookup<T> {
    type Output = T;

    fn index(&self, index: CaveId) -> &Self::Output {
        &self.0[index.cave_hash()]
    }
}

impl<T> IndexMut<CaveId> for CaveLookup<T> {
    fn index_mut(&mut self, index: CaveId) -> &mut Self::Output {
        &mut self.0[index.cave_hash()]
    }
}

struct Graph(CaveLookup<Cave>);

impl Graph {
    fn parse(input: &[&str]) -> anyhow::Result<Self> {
        let mut lookup = CaveLookup::from_fn(|| Cave::new(CaveSize::Small));

        for line in input {
            let (left, right) = line
                .split_once('-')
                .context("Unexpected end of line, expecting `-`")?;

            let left_id = CaveId::new(left);
            let right_id = CaveId::new(right);

            let left_cave = &mut lookup[left_id];
            left_cave.set_size(left_id.cave_size());
            left_cave.add_neighbor(right_id);

            let right_cave = &mut lookup[right_id];
            right_cave.set_size(right_id.cave_size());
            right_cave.add_neighbor(left_id);
        }

        Ok(Graph(lookup))
    }

    fn count_paths(&self) -> usize {
        let mut visited = CaveLookup::new();
        self.count_paths_from(CaveId::start(), &mut visited)
    }

    fn count_paths_from(&self, id: CaveId, visited: &mut CaveLookup<bool>) -> usize {
        if id.is_end() {
            return 1;
        }

        if visited[id] {
            return 0;
        }

        let cave = &self.0[id];
        let is_small = matches!(cave.size(), CaveSize::Small);

        if is_small {
            visited[id] = true;
        }

        let count = cave
            .neighbors()
            .iter()
            .map(|neighbor| self.count_paths_from(*neighbor, visited))
            .sum();

        if is_small {
            visited[id] = false;
        }

        count
    }
}
