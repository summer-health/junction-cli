use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum InsuranceCommand {
    /// Search for a payor
    SearchPayor {
        /// Search query
        #[arg(long)]
        query: Option<String>,
        /// JSON body (for POST search)
        #[arg(long)]
        data: Option<String>,
    },

    /// Search diagnosis codes
    SearchDiagnosis {
        /// Search query
        #[arg(long)]
        query: Option<String>,
    },

    /// Validate ICD codes
    ValidateIcdCodes {
        /// JSON body
        #[arg(long)]
        data: String,
    },
}

pub async fn run(cmd: InsuranceCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        InsuranceCommand::SearchPayor { query, data } => {
            if let Some(body_str) = data {
                let body: serde_json::Value = serde_json::from_str(&body_str)?;
                let result: serde_json::Value = client
                    .post_json("/v3/insurance/search/payor", &body)
                    .await?;
                output::print_json(&result);
            } else {
                let mut path = "/v3/insurance/search/payor".to_string();
                if let Some(q) = query {
                    path.push_str(&format!("?query={q}"));
                }
                let result: serde_json::Value = client.get_raw(&path).await?;
                output::print_json(&result);
            }
        }
        InsuranceCommand::SearchDiagnosis { query } => {
            let mut path = "/v3/insurance/search/diagnosis".to_string();
            if let Some(q) = query {
                path.push_str(&format!("?query={q}"));
            }
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        InsuranceCommand::ValidateIcdCodes { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .post_json("/v3/insurance/validate_icd_codes", &body)
                .await?;
            output::print_json(&result);
        }
    }

    Ok(())
}
