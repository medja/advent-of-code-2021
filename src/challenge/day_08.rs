pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .flat_map(|line| line.rsplit_once('|').unwrap().1.split_ascii_whitespace())
        .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
        .count())
}
