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

    /// Check if a token is valid (deprecated)
    TokenIsValid {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Create an invitation code
    CreateCode {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Check link token state (deprecated)
    State {
        /// Link token
        #[arg(long)]
        vital_link_token: String,
    },

    /// Generate an OAuth link
    Oauth {
        /// OAuth provider slug
        provider: String,
        /// Vital link token
        #[arg(long)]
        vital_link_token: String,
    },

    /// Connect a password-based provider
    ConnectPassword {
        /// Provider slug
        provider: String,
        /// JSON body with credentials
        #[arg(long)]
        data: String,
        /// Vital link token
        #[arg(long)]
        vital_link_token: Option<String>,
    },

    /// Complete MFA for password provider
    CompleteMfa {
        /// Provider slug
        provider: String,
        /// JSON body with MFA code
        #[arg(long)]
        data: String,
        /// Vital link token
        #[arg(long)]
        vital_link_token: Option<String>,
    },

    /// Connect an email auth provider
    ConnectEmail {
        /// Provider slug
        provider: String,
        /// JSON body with email
        #[arg(long)]
        data: String,
        /// Vital link token
        #[arg(long)]
        vital_link_token: Option<String>,
    },

    /// Connect a manual provider (deprecated)
    ConnectManual {
        /// Provider slug
        provider: String,
        /// JSON body
        #[arg(long)]
        data: String,
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

    /// List bulk operations
    BulkOps {
        /// Pagination cursor
        #[arg(long)]
        next_cursor: Option<String>,
        /// Page size
        #[arg(long)]
        page_size: Option<u32>,
    },

    /// Bulk import connections
    BulkImport {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Bulk export connections
    BulkExport {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Bulk pause connections
    BulkPause {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Bulk trigger historical data pull
    BulkHistoricalPull {
        /// JSON body
        #[arg(long)]
        data: String,
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
        LinkCommand::TokenIsValid { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value =
                client.post_json("/v2/link/token/isValid", &body).await?;
            output::print_json(&result);
        }
        LinkCommand::CreateCode { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client.post_json("/v2/link/code/create", &body).await?;
            output::print_json(&result);
        }
        LinkCommand::State { vital_link_token } => {
            let data: serde_json::Value = client
                .get_raw(&format!(
                    "/v2/link/state?vital_link_token={vital_link_token}"
                ))
                .await?;
            output::print_json(&data);
        }
        LinkCommand::Oauth {
            provider,
            vital_link_token,
        } => {
            let data: serde_json::Value = client
                .get_raw(&format!(
                    "/v2/link/provider/oauth/{provider}?vital_link_token={vital_link_token}"
                ))
                .await?;
            output::print_json(&data);
        }
        LinkCommand::ConnectPassword {
            provider,
            data,
            vital_link_token,
        } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let mut path = format!("/v2/link/provider/password/{provider}");
            if let Some(token) = vital_link_token {
                path.push_str(&format!("?vital_link_token={token}"));
            }
            let result: serde_json::Value = client.post_json(&path, &body).await?;
            output::print_json(&result);
        }
        LinkCommand::CompleteMfa {
            provider,
            data,
            vital_link_token,
        } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let mut path = format!("/v2/link/provider/password/{provider}/complete_mfa");
            if let Some(token) = vital_link_token {
                path.push_str(&format!("?vital_link_token={token}"));
            }
            let result: serde_json::Value = client.post_json(&path, &body).await?;
            output::print_json(&result);
        }
        LinkCommand::ConnectEmail {
            provider,
            data,
            vital_link_token,
        } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let mut path = format!("/v2/link/provider/email/{provider}");
            if let Some(token) = vital_link_token {
                path.push_str(&format!("?vital_link_token={token}"));
            }
            let result: serde_json::Value = client.post_json(&path, &body).await?;
            output::print_json(&result);
        }
        LinkCommand::ConnectManual { provider, data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .post_json(&format!("/v2/link/provider/manual/{provider}"), &body)
                .await?;
            output::print_json(&result);
        }
        LinkCommand::Demo { user_id, provider } => {
            let body = serde_json::json!({
                "user_id": user_id,
                "provider": provider,
            });
            let data: serde_json::Value = client.post_json("/v2/link/connect/demo", &body).await?;
            output::print_json(&data);
        }
        LinkCommand::BulkOps {
            next_cursor,
            page_size,
        } => {
            let mut path = "/v2/link/bulk_op".to_string();
            let mut params = Vec::new();
            if let Some(c) = next_cursor {
                params.push(format!("next_cursor={c}"));
            }
            if let Some(s) = page_size {
                params.push(format!("page_size={s}"));
            }
            if !params.is_empty() {
                path.push('?');
                path.push_str(&params.join("&"));
            }
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        LinkCommand::BulkImport { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client.post_json("/v2/link/bulk_import", &body).await?;
            output::print_json(&result);
        }
        LinkCommand::BulkExport { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client.post_json("/v2/link/bulk_export", &body).await?;
            output::print_json(&result);
        }
        LinkCommand::BulkPause { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client.post_json("/v2/link/bulk_pause", &body).await?;
            output::print_json(&result);
        }
        LinkCommand::BulkHistoricalPull { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .post_json("/v2/link/bulk_trigger_historical_pull", &body)
                .await?;
            output::print_json(&result);
        }
    }

    Ok(())
}
