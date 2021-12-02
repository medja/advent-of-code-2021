mod challenge;
mod http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //println!("Sonar Sweep - Part 1: {}", challenge::day_01::part_01().await?);
    //println!("Sonar Sweep - Part 2: {}", challenge::day_01::part_02().await?);
    //println!("Dive! - Part 1: {}", challenge::day_02::part_01().await?);
    println!("Dive! - Part 2: {}", challenge::day_02::part_02().await?);
    Ok(())
}
