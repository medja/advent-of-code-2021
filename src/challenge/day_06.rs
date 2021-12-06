pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    simulate(input[0], 80)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    simulate(input[0], 256)
}

fn simulate(input: &str, days: usize) -> anyhow::Result<u64> {
    let mut counters = Counters::new();

    for number in input.split(',') {
        counters.add(number.parse()?);
    }

    for _ in 0..days {
        counters.advance();
    }

    Ok(counters.count())
}

struct Counters([u64; 9]);

impl Counters {
    fn new() -> Self {
        Counters([0u64; 9])
    }

    fn add(&mut self, counter: u8) {
        self.0[counter as usize] += 1;
    }

    fn advance(&mut self) {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
    }

    fn count(&self) -> u64 {
        self.0.iter().sum()
    }
}
