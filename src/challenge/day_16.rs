use anyhow::anyhow;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(parse(&parse_bits(input[0])?).version)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(parse(&parse_bits(input[0])?).value)
}

const BITS: [[bool; 4]; 16] = [
    [false, false, false, false],
    [false, false, false, true],
    [false, false, true, false],
    [false, false, true, true],
    [false, true, false, false],
    [false, true, false, true],
    [false, true, true, false],
    [false, true, true, true],
    [true, false, false, false],
    [true, false, false, true],
    [true, false, true, false],
    [true, false, true, true],
    [true, true, false, false],
    [true, true, false, true],
    [true, true, true, false],
    [true, true, true, true],
];

struct ParseResult {
    version: usize,
    value: u64,
    length: usize,
}

struct LiteralResult {
    value: u64,
    length: usize,
}

struct NestedResult {
    version: usize,
    values: Vec<u64>,
    length: usize,
}

fn parse(bits: &[bool]) -> ParseResult {
    let version = (bits[0] as usize) << 2 | (bits[1] as usize) << 1 | bits[2] as usize;
    let type_id = (bits[3] as usize) << 2 | (bits[4] as usize) << 1 | bits[5] as usize;

    if type_id == 4 {
        let LiteralResult { value, length } = parse_literal(&bits[6..]);
        let length = length + 6;

        return ParseResult {
            version,
            value,
            length,
        };
    }

    let result = if bits[6] {
        parse_nested_by_count(&bits[7..])
    } else {
        parse_nested_by_length(&bits[7..])
    };

    let version = version + result.version;
    let length = result.length + 7;

    let value = match type_id {
        0 => result.values.iter().copied().sum(),
        1 => result.values.iter().copied().product(),
        2 => result.values.iter().copied().min().unwrap(),
        3 => result.values.iter().copied().max().unwrap(),
        5 => (result.values[0] > result.values[1]) as u64,
        6 => (result.values[0] < result.values[1]) as u64,
        7 => (result.values[0] == result.values[1]) as u64,
        _ => 0,
    };

    ParseResult {
        version,
        value,
        length,
    }
}

fn parse_literal(bits: &[bool]) -> LiteralResult {
    let mut i = 0;
    let mut value = parse_number(&bits[1..5]);

    while bits[i] {
        i += 5;
        value = parse_number_with_base(value, &bits[i + 1..i + 5]);
    }

    let length = i + 5;
    LiteralResult { value, length }
}

fn parse_nested_by_count(bits: &[bool]) -> NestedResult {
    let count = parse_number(&bits[..11]);

    let mut index = 11;
    let mut version = 0;
    let mut values = Vec::with_capacity(count);

    for _ in 0..count {
        let result = parse(&bits[index..]);
        index += result.length;
        version += result.version;
        values.push(result.value);
    }

    let length = index;

    NestedResult {
        version,
        values,
        length,
    }
}

fn parse_nested_by_length(bits: &[bool]) -> NestedResult {
    let length = 15 + parse_number::<usize>(&bits[..15]);

    let mut index = 15;
    let mut version = 0;
    let mut values = Vec::with_capacity(2);

    while index < length {
        let result = parse(&bits[index..]);
        index += result.length;
        version += result.version;
        values.push(result.value);
    }

    NestedResult {
        version,
        values,
        length,
    }
}

fn parse_number<N: Default + FromBits>(bits: &[bool]) -> N {
    parse_number_with_base(N::default(), bits)
}

fn parse_number_with_base<N: FromBits>(base: N, bits: &[bool]) -> N {
    bits.iter().fold(base, |acc, bit| acc.push_bit(*bit))
}

fn parse_bits(input: &str) -> anyhow::Result<Vec<bool>> {
    let mut bits = Vec::with_capacity(input.len() * 4);

    for byte in input.bytes() {
        let value = match byte {
            b'0'..=b'9' => byte - b'0',
            b'A'..=b'F' => byte - b'A' + 10,
            _ => return Err(anyhow!("{} is not a hex character", char::from(byte))),
        };

        bits.extend_from_slice(&BITS[value as usize]);
    }

    Ok(bits)
}

trait FromBits {
    fn push_bit(self, bit: bool) -> Self;
}

impl FromBits for usize {
    fn push_bit(self, bit: bool) -> Self {
        self << 1 | (bit as usize)
    }
}

impl FromBits for u64 {
    fn push_bit(self, bit: bool) -> Self {
        self << 1 | (bit as u64)
    }
}
