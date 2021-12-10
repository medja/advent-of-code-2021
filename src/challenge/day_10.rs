pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut stack = Vec::with_capacity(input[0].len());
    let mut score = 0usize;

    for line in input {
        match find_invalid(line, &mut stack) {
            Some(b')') => score += 3,
            Some(b']') => score += 57,
            Some(b'}') => score += 1197,
            Some(b'>') => score += 25137,
            _ => {}
        }
    }

    Ok(score)
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
