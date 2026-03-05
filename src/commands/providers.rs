use anyhow::Result;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

pub async fn run() -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;
    let data: serde_json::Value = client.get_raw("/v2/providers").await?;
    output::print_json(&data);
    Ok(())
}
