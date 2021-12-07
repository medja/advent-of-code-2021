const COUNTERS_LEN: usize = 9;
const DAYS_TO_REPRODUCE: usize = 7;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    simulate(input[0], 80)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    simulate(input[0], 256)
}

fn simulate(input: &str, days: usize) -> anyhow::Result<u64> {
    let mut counters = Counters::try_from(input)?;

    for _ in 0..days {
        counters.advance();
    }

    Ok(counters.count())
}

struct Counters(usize, [u64; COUNTERS_LEN]);

impl Counters {
    fn advance(&mut self) {
        self.1[(self.0 + DAYS_TO_REPRODUCE) % COUNTERS_LEN] += self.1[self.0];
        self.0 = (self.0 + 1) % COUNTERS_LEN;
    }

    fn count(&self) -> u64 {
        self.1.iter().sum()
    }
}

impl TryFrom<&str> for Counters {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut counters = [0u64; COUNTERS_LEN];

        for number in value.split(',') {
            counters[number.parse::<usize>()?] += 1;
        }

        Ok(Counters(0, counters))
    }
}
