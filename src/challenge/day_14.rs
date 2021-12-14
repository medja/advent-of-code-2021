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

struct Polymer(Vec<u8>);

impl Polymer {
    fn new(string: &str) -> Self {
        Polymer(string.bytes().collect())
    }

    fn apply(&mut self, insertions: &Insertions) {
        let mut bytes = Vec::with_capacity(self.0.len() * 2 - 1);

        bytes.push(self.0[0]);

        for (&x, &y) in self.0.iter().zip(&self.0[1..]) {
            if let Some(insertion) = insertions.lookup(x, y) {
                bytes.push(insertion);
            }

            bytes.push(y);
        }

        self.0 = bytes;
    }

    fn score(&self) -> usize {
        let mut counts = [0usize; LETTER_COUNT];

        for byte in &self.0 {
            counts[(byte - b'A') as usize] += 1;
        }

        let mut min = usize::MAX;
        let mut max = usize::MIN;

        for count in counts {
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
            rules[(line[0] - b'A') as usize * LETTER_COUNT + (line[1] - b'A') as usize] = line[6];
        }

        Insertions(rules)
    }

    fn lookup(&self, a: u8, b: u8) -> Option<u8> {
        match self.0[(a - b'A') as usize * LETTER_COUNT + (b - b'A') as usize] {
            0 => None,
            insertion => Some(insertion),
        }
    }
}
