pub async fn get<U: reqwest::IntoUrl>(url: U) -> anyhow::Result<String> {
    let cookie = format!("session={}", std::env::var("SESSION")?);

    let response = reqwest::Client::default()
        .get(url)
        .header("cookie", cookie)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(response)
}
