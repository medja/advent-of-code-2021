mod challenge;
mod http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //println!("Sonar Sweep: {}", challenge::day_01::part_01().await?);
    println!("Sonar Sweep: {}", challenge::day_01::part_02().await?);
    Ok(())
}
