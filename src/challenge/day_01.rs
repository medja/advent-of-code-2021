pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let depths = input
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(depths.windows(2).filter(|x| x[1] > x[0]).count())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let depths = input
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    let averages = depths
        .windows(3)
        .map(|depths| depths.iter().sum())
        .collect::<Vec<u32>>();

    Ok(averages.windows(2).filter(|x| x[1] > x[0]).count())
}
