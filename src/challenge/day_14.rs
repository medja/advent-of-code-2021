const LETTER_COUNT: usize = 26;
const LOOKUP_SIZE: usize = LETTER_COUNT * LETTER_COUNT;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let insertions = Insertions::new(&input[2..]);
    let mut polymer = Polymer::new(input[0]);

    for _ in 0..10 {
        polymer.apply(&insertions);
    }

    Ok(polymer.score())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let insertions = Insertions::new(&input[2..]);
    let mut polymer = Polymer::new(input[0]);

    for _ in 0..40 {
        polymer.apply(&insertions);
    }

    Ok(polymer.score())
}

type Pair = usize;

fn normalize(byte: u8) -> u8 {
    byte - b'A'
}

fn pair(x: u8, y: u8) -> Pair {
    x as usize + y as usize * LETTER_COUNT
}

fn split(pair: Pair) -> (u8, u8) {
    let x = pair % LETTER_COUNT;
    let y = pair / LETTER_COUNT;

    (x as u8, y as u8)
}

struct Polymer([u64; LOOKUP_SIZE], [u64; LETTER_COUNT]);

impl Polymer {
    fn new(string: &str) -> Self {
        let mut pairs = [0u64; LOOKUP_SIZE];
        let mut counts = [0u64; LETTER_COUNT];

        let bytes = string.as_bytes();

        counts[normalize(bytes[0]) as usize] = 1;

        for (&x, &y) in bytes.iter().zip(&bytes[1..]) {
            let x = normalize(x);
            let y = normalize(y);
            pairs[pair(x, y)] += 1;
            counts[y as usize] += 1;
        }

        Polymer(pairs, counts)
    }

    fn apply(&mut self, insertions: &Insertions) {
        let mut pairs = [0u64; LOOKUP_SIZE];

        for (p, &count) in self.0.iter().enumerate() {
            if count == 0 {
                continue;
            }

            if let Some(insertion) = insertions.lookup(p) {
                let (x, y) = split(p);
                pairs[pair(x, insertion)] += count;
                pairs[pair(insertion, y)] += count;
                self.1[insertion as usize] += count;
            } else {
                pairs[p] += count;
            }
        }

        self.0 = pairs;
    }

    fn score(&self) -> u64 {
        let mut min = u64::MAX;
        let mut max = u64::MIN;

        for count in self.1 {
            if count == 0 {
                continue;
            }

            if count < min {
                min = count;
            }

            if count > max {
                max = count;
            }
        }

        max - min
    }
}

struct Insertions([u8; LOOKUP_SIZE]);

impl Insertions {
    fn new(lines: &[&str]) -> Insertions {
        let mut rules = [0; LOOKUP_SIZE];

        for line in lines {
            let line = line.as_bytes();
            rules[pair(normalize(line[0]), normalize(line[1]))] = normalize(line[6]);
        }

        Insertions(rules)
    }

    fn lookup(&self, pair: Pair) -> Option<u8> {
        match self.0[pair] {
            0 => None,
            insertion => Some(insertion),
        }
    }
}
