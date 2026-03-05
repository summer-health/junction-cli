use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum TeamCommand {
    /// Get team info
    Get {
        /// Team ID
        team_id: String,
    },

    /// Get link configuration
    LinkConfig,

    /// Get source priorities
    SourcePriorities,

    /// Update source priorities
    UpdateSourcePriorities {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Get Svix webhook URL
    SvixUrl,

    /// Search users in team
    SearchUsers {
        /// Search query parameters as JSON
        #[arg(long)]
        query: Option<String>,
    },

    /// List physicians
    Physicians {
        /// Team ID
        team_id: String,
    },
}

pub async fn run(cmd: TeamCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        TeamCommand::Get { team_id } => {
            let data: serde_json::Value = client.get_raw(&format!("/v2/team/{team_id}")).await?;
            output::print_json(&data);
        }
        TeamCommand::LinkConfig => {
            let data: serde_json::Value = client.get_raw("/v2/team/link/config").await?;
            output::print_json(&data);
        }
        TeamCommand::SourcePriorities => {
            let data: serde_json::Value = client.get_raw("/v2/team/source/priorities").await?;
            output::print_json(&data);
        }
        TeamCommand::UpdateSourcePriorities { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .patch_json("/v2/team/source/priorities", &body)
                .await?;
            output::print_json(&result);
        }
        TeamCommand::SvixUrl => {
            let data: serde_json::Value = client.get_raw("/v2/team/svix/url").await?;
            output::print_json(&data);
        }
        TeamCommand::SearchUsers { query } => {
            let mut path = "/v2/team/users/search".to_string();
            if let Some(q) = query {
                path.push_str(&format!("?{q}"));
            }
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        TeamCommand::Physicians { team_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v2/team/{team_id}/physicians"))
                .await?;
            output::print_json(&data);
        }
    }

    Ok(())
}
