use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum AggregateCommand {
    /// Run an ad-hoc query
    Query {
        /// User ID
        user_id: String,
        /// JSON query body
        #[arg(long)]
        data: String,
    },

    /// Get continuous query result table
    ResultTable {
        /// User ID
        user_id: String,
        /// Query ID or slug
        query_id: String,
    },

    /// Get continuous query task history
    TaskHistory {
        /// User ID
        user_id: String,
        /// Query ID or slug
        query_id: String,
    },
}

pub async fn run(cmd: AggregateCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        AggregateCommand::Query { user_id, data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .post_json(&format!("/aggregate/v1/user/{user_id}/query"), &body)
                .await?;
            output::print_json(&result);
        }
        AggregateCommand::ResultTable { user_id, query_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!(
                    "/aggregate/v1/user/{user_id}/continuous_query/{query_id}/result_table"
                ))
                .await?;
            output::print_json(&data);
        }
        AggregateCommand::TaskHistory { user_id, query_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!(
                    "/aggregate/v1/user/{user_id}/continuous_query/{query_id}/task_history"
                ))
                .await?;
            output::print_json(&data);
        }
    }

    Ok(())
}
