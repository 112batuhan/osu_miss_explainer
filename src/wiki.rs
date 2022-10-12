use anyhow::Result;

pub async fn get_random_summary() -> Result<String> {
    let url = "https://tr.wikipedia.org/api/rest_v1/page/random/summary";
    let body = reqwest::get(url).await?.text().await?;
    let summary: String = serde_json::from_str(&body).unwrap();
    Ok(summary)
}
