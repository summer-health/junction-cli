use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum UserCommand {
    /// List all users
    List {
        /// Pagination cursor
        #[arg(long)]
        next_cursor: Option<String>,
        /// Page size
        #[arg(long)]
        page_size: Option<u32>,
    },

    /// Create a new user
    Create {
        /// Client user ID
        #[arg(long)]
        client_user_id: String,
    },

    /// Get user by ID
    Get {
        /// User ID
        user_id: String,
    },

    /// Update user
    Update {
        /// User ID
        user_id: String,
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Delete user
    Delete {
        /// User ID
        user_id: String,
    },

    /// Resolve user by client user ID
    Resolve {
        /// Client user ID
        client_user_id: String,
    },

    /// Get user devices
    Devices {
        /// User ID
        user_id: String,
    },

    /// Get a specific device
    Device {
        /// User ID
        user_id: String,
        /// Device ID
        device_id: String,
    },

    /// Get connected providers for a user
    Providers {
        /// User ID
        user_id: String,
    },

    /// Get latest user info
    Info {
        /// User ID
        user_id: String,
    },

    /// Update user info
    UpdateInfo {
        /// User ID
        user_id: String,
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Get latest insurance
    Insurance {
        /// User ID
        user_id: String,
    },

    /// Create insurance profile
    CreateInsurance {
        /// User ID
        user_id: String,
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Refresh user data
    Refresh {
        /// User ID
        user_id: String,
    },

    /// Undo user deletion
    UndoDelete {
        /// JSON body with user info
        #[arg(long)]
        data: String,
    },

    /// Deregister a provider connection
    Deregister {
        /// User ID
        user_id: String,
        /// Provider slug
        provider: String,
    },

    /// Create a sign-in token
    SignInToken {
        /// User ID
        user_id: String,
    },

    /// Create a portal URL
    PortalUrl {
        /// User ID
        user_id: String,
    },

    /// Get user metrics
    Metrics,
}

pub async fn run(cmd: UserCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        UserCommand::List {
            next_cursor,
            page_size,
        } => {
            let mut path = "/v2/user".to_string();
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
        UserCommand::Create { client_user_id } => {
            let body = serde_json::json!({ "client_user_id": client_user_id });
            let data: serde_json::Value = client.post_json("/v2/user", &body).await?;
            output::print_json(&data);
        }
        UserCommand::Get { user_id } => {
            let data: serde_json::Value = client.get_raw(&format!("/v2/user/{user_id}")).await?;
            output::print_json(&data);
        }
        UserCommand::Update { user_id, data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .patch_json(&format!("/v2/user/{user_id}"), &body)
                .await?;
            output::print_json(&result);
        }
        UserCommand::Delete { user_id } => {
            client.delete(&format!("/v2/user/{user_id}")).await?;
            output::print_success("user deleted");
        }
        UserCommand::Resolve { client_user_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v2/user/resolve/{client_user_id}"))
                .await?;
            output::print_json(&data);
        }
        UserCommand::Devices { user_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v2/user/{user_id}/device"))
                .await?;
            output::print_json(&data);
        }
        UserCommand::Device { user_id, device_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v2/user/{user_id}/device/{device_id}"))
                .await?;
            output::print_json(&data);
        }
        UserCommand::Providers { user_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v2/user/providers/{user_id}"))
                .await?;
            output::print_json(&data);
        }
        UserCommand::Info { user_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v2/user/{user_id}/info/latest"))
                .await?;
            output::print_json(&data);
        }
        UserCommand::UpdateInfo { user_id, data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .patch_json(&format!("/v2/user/{user_id}/info"), &body)
                .await?;
            output::print_json(&result);
        }
        UserCommand::Insurance { user_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v2/user/{user_id}/insurance/latest"))
                .await?;
            output::print_json(&data);
        }
        UserCommand::CreateInsurance { user_id, data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .post_json(&format!("/v2/user/{user_id}/insurance"), &body)
                .await?;
            output::print_json(&result);
        }
        UserCommand::Refresh { user_id } => {
            let body = serde_json::json!({});
            let data: serde_json::Value = client
                .post_json(&format!("/v2/user/refresh/{user_id}"), &body)
                .await?;
            output::print_json(&data);
        }
        UserCommand::UndoDelete { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client.post_json("/v2/user/undo_delete", &body).await?;
            output::print_json(&result);
        }
        UserCommand::Deregister { user_id, provider } => {
            client
                .delete(&format!("/v2/user/{user_id}/{provider}"))
                .await?;
            output::print_success("provider deregistered");
        }
        UserCommand::SignInToken { user_id } => {
            let body = serde_json::json!({});
            let data: serde_json::Value = client
                .post_json(&format!("/v2/user/{user_id}/sign_in_token"), &body)
                .await?;
            output::print_json(&data);
        }
        UserCommand::PortalUrl { user_id } => {
            let body = serde_json::json!({});
            let data: serde_json::Value = client
                .post_json(&format!("/v2/user/{user_id}/create_portal_url"), &body)
                .await?;
            output::print_json(&data);
        }
        UserCommand::Metrics => {
            let data: serde_json::Value = client.get_raw("/v2/user/metrics").await?;
            output::print_json(&data);
        }
    }

    Ok(())
}
