use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum LabReportCommand {
    /// Create a lab report parsing job
    Create {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Get parsing job status
    Get {
        /// Job ID
        job_id: String,
    },
}

pub async fn run(cmd: LabReportCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        LabReportCommand::Create { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value =
                client.post_json("/lab_report/v1/parser/job", &body).await?;
            output::print_json(&result);
        }
        LabReportCommand::Get { job_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/lab_report/v1/parser/job/{job_id}"))
                .await?;
            output::print_json(&data);
        }
    }

    Ok(())
}
