use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum IntrospectCommand {
    /// Get historical pull status
    HistoricalPull,

    /// Get available resources
    Resources,
}

pub async fn run(cmd: IntrospectCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        IntrospectCommand::HistoricalPull => {
            let data: serde_json::Value = client.get_raw("/v2/introspect/historical_pull").await?;
            output::print_json(&data);
        }
        IntrospectCommand::Resources => {
            let data: serde_json::Value = client.get_raw("/v2/introspect/resources").await?;
            output::print_json(&data);
        }
    }

    Ok(())
}
