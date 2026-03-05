use anyhow::Result;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

pub async fn run(data: String) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;
    let body: serde_json::Value = serde_json::from_str(&data)?;
    let result: serde_json::Value = client.post_json("/v3/payor", &body).await?;
    output::print_json(&result);
    Ok(())
}
