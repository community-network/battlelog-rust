use anyhow::Result;

pub async fn get_battlelog_keeper_data(guid: String) -> Result<serde_json::Value> {
    let url = format!("https://keeper.battlelog.com/snapshot/{}", guid);
    let client = reqwest::Client::new();

    let resp = client.get(url).send().await?.json::<serde_json::Value>().await?;

    Ok(resp)
}
