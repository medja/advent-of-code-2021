pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let bit_count = input[0].as_bytes().len();
    let majority_count = input.len() / 2;

    let max = 2u32.pow(bit_count as u32) - 1;
    let mut counters = vec![0; bit_count];

    for line in input {
        let bytes = line.as_bytes();

        for i in 0..bit_count {
            if bytes[i] == b'1' {
                counters[i] += 1
            }
        }
    }

    let gamma = counters.iter().fold(0u32, |acc, count| {
        if *count < majority_count {
            acc << 1
        } else {
            acc << 1 | 1
        }
    });

    let epsilon = max & !gamma;

    Ok(gamma * epsilon)
}
