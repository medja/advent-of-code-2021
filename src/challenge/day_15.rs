use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Graph::new(input).score_best_path())
}

struct Graph {
    size: usize,
    weights: Vec<u16>,
    scores: Vec<u16>,
    queue: BinaryHeap<Entry>,
}

impl Graph {
    fn new(input: &[&str]) -> Self {
        let weights = input
            .iter()
            .flat_map(|line| line.bytes())
            .map(|byte| (byte - b'0') as u16)
            .collect::<Vec<_>>();

        let length = weights.len();

        Graph {
            weights,
            size: input.len(),
            scores: vec![u16::MAX; length],
            queue: BinaryHeap::new(),
        }
    }

    fn score_best_path(&mut self) -> u16 {
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

    fn visit(&mut self, index: usize, parent_score: u16) {
        let score = parent_score + self.weights[index];

        if score < self.scores[index] {
            self.scores[index] = score;
            self.queue.push(Entry::new(index, score));
        }
    }
}

#[derive(Eq, PartialEq)]
struct Entry {
    index: u16,
    score: u16,
}

impl Entry {
    fn new(index: usize, score: u16) -> Self {
        let index = index as u16;
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
