use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum LinkCommand {
    /// List available providers
    Providers,

    /// Generate a link token for a user
    Token {
        /// User ID
        #[arg(long)]
        user_id: String,
        /// Provider to link
        #[arg(long)]
        provider: Option<String>,
    },

    /// Create a demo connection
    Demo {
        /// User ID
        #[arg(long)]
        user_id: String,
        /// Provider (default: apple_health_kit)
        #[arg(long, default_value = "apple_health_kit")]
        provider: String,
    },
}

pub async fn run(cmd: LinkCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        LinkCommand::Providers => {
            let data: serde_json::Value = client.get_raw("/v2/link/providers").await?;
            output::print_json(&data);
        }
        LinkCommand::Token { user_id, provider } => {
            let mut body = serde_json::json!({ "user_id": user_id });
            if let Some(p) = provider {
                body["provider"] = serde_json::Value::String(p);
            }
            let data: serde_json::Value = client.post_json("/v2/link/token", &body).await?;
            output::print_json(&data);
        }
        LinkCommand::Demo { user_id, provider } => {
            let body = serde_json::json!({
                "user_id": user_id,
                "provider": provider,
            });
            let data: serde_json::Value =
                client.post_json("/v2/link/connect/demo", &body).await?;
            output::print_json(&data);
        }
    }

    Ok(())
}
