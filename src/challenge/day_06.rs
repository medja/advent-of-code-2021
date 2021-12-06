pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut counters = Counters::new();

    for number in input[0].split(',') {
        counters.add(number.parse()?);
    }

    for _ in 0..80 {
        counters.advance();
    }

    Ok(counters.count())
}

struct Counters([usize; 9]);

impl Counters {
    fn new() -> Self {
        Counters([0usize; 9])
    }

    fn add(&mut self, counter: u8) {
        self.0[counter as usize] += 1;
    }

    fn advance(&mut self) {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
    }

    fn count(&self) -> usize {
        self.0.iter().sum()
    }
}
