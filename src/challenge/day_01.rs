pub async fn part_01() -> anyhow::Result<usize> {
    let input = crate::http::get("https://adventofcode.com/2021/day/1/input").await?;

    let depths = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(depths.windows(2).filter(|x| x[1] > x[0]).count())
}

pub async fn part_02() -> anyhow::Result<usize> {
    let input = crate::http::get("https://adventofcode.com/2021/day/1/input").await?;

    let depths = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    let averages = depths
        .windows(3)
        .map(|depths| depths.iter().sum())
        .collect::<Vec<u32>>();

    Ok(averages.windows(2).filter(|x| x[1] > x[0]).count())
}
