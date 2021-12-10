pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut stack = Vec::with_capacity(input[0].len());
    let mut score = 0usize;

    for line in input {
        if let Some(byte) = find_invalid(line, &mut stack) {
            score += score_invalid(byte);
        }
    }

    Ok(score)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut scores = Vec::with_capacity(input.len());
    let mut stack = Vec::with_capacity(input[0].len());

    for line in input {
        if find_invalid(line, &mut stack).is_some() {
            continue;
        }

        let score = stack
            .iter()
            .rev()
            .fold(0usize, |acc, byte| acc * 5 + score_missing(*byte));

        scores.push(score);
    }

    let middle = scores.len() / 2;
    Ok(*scores.select_nth_unstable(middle).1)
}

fn score_missing(byte: u8) -> usize {
    match byte {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => 0,
    }
}

fn score_invalid(byte: u8) -> usize {
    match byte {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => 0,
    }
}

fn find_invalid(line: &str, stack: &mut Vec<u8>) -> Option<u8> {
    stack.clear();

    for byte in line.bytes() {
        match byte {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b'<' => stack.push(b'>'),
            _ if !matches!(stack.pop(), Some(expected) if byte == expected) => return Some(byte),
            _ => {}
        }
    }

    None
}
