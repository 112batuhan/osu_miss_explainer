use reqwest;
use serde::Deserialize;
use serde_json;
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct Summary{
    title:String,
    extract:String,
}

pub async fn get_random_summary() -> Result<Summary>{
    let url = "https://en.wikipedia.org/api/rest_v1/page/random/summary";
    let body = reqwest::get(url)
    .await?
    .text()
    .await?;
    let summary:Summary = serde_json::from_str(&body).unwrap();
    Ok(summary)
}
