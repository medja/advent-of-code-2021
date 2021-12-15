use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let weights = input
        .iter()
        .flat_map(|line| line.bytes())
        .map(|byte| byte - b'0')
        .collect::<Vec<_>>();

    Ok(Graph::new(input.len(), &weights).score_best_path())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let size = input.len() * 5;
    let mut buffer = Vec::with_capacity(input.len());
    let mut weights = Vec::with_capacity(size * size);

    for &line in input {
        for byte in line.bytes() {
            let weight = byte - b'0';
            buffer.push(weight);
            weights.push(weight);
        }

        for i in 1..5 {
            for weight in &buffer {
                weights.push((*weight + i - 1) % 9 + 1);
            }
        }

        buffer.clear();
    }

    for i in 1..5 {
        for index in 0..input.len() * size {
            weights.push((weights[index] + i - 1) % 9 + 1);
        }
    }

    Ok(Graph::new(size, &weights).score_best_path())
}

struct Graph<'a> {
    size: usize,
    weights: &'a [u8],
    scores: Vec<u32>,
    queue: BinaryHeap<Entry>,
}

impl<'a> Graph<'a> {
    fn new(size: usize, weights: &'a [u8]) -> Self {
        let length = weights.len();

        Graph {
            weights,
            size,
            scores: vec![u32::MAX; length],
            queue: BinaryHeap::new(),
        }
    }

    fn score_best_path(&mut self) -> u32 {
        let end = self.scores.len() - 1;

        self.scores[0] = 0;
        self.queue.push(Entry::new(0, 0));

        while let Some(entry) = self.queue.pop() {
            let index = entry.index();

            if index == end {
                break;
            }

            let score = self.scores[index];

            let x = index % self.size;
            let y = index / self.size;

            if x > 0 {
                self.visit(index - 1, score);
            }

            if x + 1 < self.size {
                self.visit(index + 1, score);
            }

            if y > 0 {
                self.visit(index - self.size, score);
            }

            if y + 1 < self.size {
                self.visit(index + self.size, score);
            }
        }

        self.scores[end]
    }

    fn visit(&mut self, index: usize, parent_score: u32) {
        let score = parent_score + self.weights[index] as u32;

        if score < self.scores[index] {
            self.scores[index] = score;
            self.queue.push(Entry::new(index, score));
        }
    }
}

#[derive(Eq, PartialEq)]
struct Entry {
    index: u32,
    score: u32,
}

impl Entry {
    fn new(index: usize, score: u32) -> Self {
        let index = index as u32;
        Entry { index, score }
    }

    fn index(&self) -> usize {
        self.index as usize
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
