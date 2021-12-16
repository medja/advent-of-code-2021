use anyhow::anyhow;

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

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(sum_versions(&parse_bits(input[0])?).0)
}

fn sum_versions(bits: &[bool]) -> (usize, usize) {
    let mut sum = (bits[0] as usize) << 2 | (bits[1] as usize) << 1 | bits[2] as usize;
    let type_id = (bits[3] as usize) << 2 | (bits[4] as usize) << 1 | bits[5] as usize;

    let consumed = if type_id == 4 {
        let mut i = 6;

        while bits[i] {
            i += 5;
        }

        i + 5
    } else if bits[6] {
        let count = bits[7..18]
            .iter()
            .fold(0, |acc, bit| acc << 1 | (*bit as usize));

        let mut index = 18;

        for _ in 0..count {
            let (nested_sum, consumed) = sum_versions(&bits[index..]);
            sum += nested_sum;
            index += consumed;
        }

        index
    } else {
        let length = bits[7..22]
            .iter()
            .fold(0, |acc, bit| acc << 1 | (*bit as usize));

        let mut index = 22;
        let end_index = index + length;
        let max_index = end_index - 4;

        while index < max_index {
            let (nested_sum, consumed) = sum_versions(&bits[index..]);
            sum += nested_sum;
            index += consumed;
        }

        end_index
    };

    (sum, consumed)
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
