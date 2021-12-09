const MAX_DEPTH: u8 = 10;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let width = input[0].len();
    let height = input.len();
    let stride = width + 2;

    let mut depths = Vec::<u8>::with_capacity(stride * (height + 2));

    depths.extend(std::iter::repeat(MAX_DEPTH).take(stride));

    for line in input {
        depths.push(MAX_DEPTH);
        depths.extend(line.bytes().map(|byte| byte - b'0'));
        depths.push(MAX_DEPTH);
    }

    depths.extend(std::iter::repeat(MAX_DEPTH).take(stride));

    let mut risk_level = 0usize;

    for y in 1..=height {
        for x in 1..=width {
            let i = x + y * stride;
            let depth = depths[i];

            let is_basin = depth < depths[i - stride]
                && depth < depths[i + stride]
                && depth < depths[i - 1]
                && depth < depths[i + 1];

            if is_basin {
                risk_level += depth as usize + 1;
            }
        }
    }

    Ok(risk_level)
}
