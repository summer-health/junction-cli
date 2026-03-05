use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum UserCommand {
    /// Get user profile
    Get {
        /// User ID
        user_id: String,
    },

    /// Get user devices
    Devices {
        /// User ID
        user_id: String,
    },
}

pub async fn run(cmd: UserCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        UserCommand::Get { user_id } => {
            let data: serde_json::Value =
                client.get_raw(&format!("/v2/summary/profile/{user_id}")).await?;
            output::print_json(&data);
        }
        UserCommand::Devices { user_id } => {
            let data: serde_json::Value =
                client.get_raw(&format!("/v2/summary/devices/{user_id}/raw")).await?;
            output::print_json(&data);
        }
    }

    Ok(())
}
