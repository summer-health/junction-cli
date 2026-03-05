use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum CompendiumCommand {
    /// Search the compendium
    Search {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Convert compendium entries
    Convert {
        /// JSON body
        #[arg(long)]
        data: String,
    },
}

pub async fn run(cmd: CompendiumCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        CompendiumCommand::Search { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value =
                client.post_json("/v3/compendium/search", &body).await?;
            output::print_json(&result);
        }
        CompendiumCommand::Convert { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value =
                client.post_json("/v3/compendium/convert", &body).await?;
            output::print_json(&result);
        }
    }

    Ok(())
}
