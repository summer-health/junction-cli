use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum LabTestsCommand {
    /// List lab tests (paginated)
    List {
        /// Pagination cursor
        #[arg(long)]
        next_cursor: Option<String>,
        /// Page size
        #[arg(long)]
        page_size: Option<u32>,
    },

    /// Get a single lab test (non-paginated)
    Get {
        /// Lab test ID
        lab_test_id: String,
    },

    /// Create a new lab test
    Create {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Update a lab test
    Update {
        /// Lab test ID
        lab_test_id: String,
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// List available labs
    Labs,

    /// List all markers
    Markers {
        /// Pagination cursor
        #[arg(long)]
        next_cursor: Option<String>,
        /// Page size
        #[arg(long)]
        page_size: Option<u32>,
    },

    /// Get markers for a specific lab and provider
    LabMarkers {
        /// Lab ID
        lab_id: String,
        /// Provider ID
        provider_id: String,
    },

    /// Get markers for a lab test
    TestMarkers {
        /// Lab test ID
        lab_test_id: String,
    },

    /// Get collection instruction PDF
    CollectionInstructionPdf {
        /// Lab test ID
        lab_test_id: String,
        /// Output file path
        #[arg(long, short)]
        output: String,
    },

    /// List order set markers
    OrderSetMarkers {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Get lab test by ID (v3/lab_test endpoint)
    GetV3 {
        /// Query parameters as needed
        #[arg(long)]
        next_cursor: Option<String>,
        #[arg(long)]
        page_size: Option<u32>,
    },
}

pub async fn run(cmd: LabTestsCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        LabTestsCommand::List {
            next_cursor,
            page_size,
        } => {
            let mut path = "/v3/lab_tests".to_string();
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
        LabTestsCommand::Get { lab_test_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/lab_tests/{lab_test_id}"))
                .await?;
            output::print_json(&data);
        }
        LabTestsCommand::Create { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client.post_json("/v3/lab_tests", &body).await?;
            output::print_json(&result);
        }
        LabTestsCommand::Update { lab_test_id, data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .patch_json(&format!("/v3/lab_tests/{lab_test_id}"), &body)
                .await?;
            output::print_json(&result);
        }
        LabTestsCommand::Labs => {
            let data: serde_json::Value = client.get_raw("/v3/lab_tests/labs").await?;
            output::print_json(&data);
        }
        LabTestsCommand::Markers {
            next_cursor,
            page_size,
        } => {
            let mut path = "/v3/lab_tests/markers".to_string();
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
        LabTestsCommand::LabMarkers {
            lab_id,
            provider_id,
        } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/lab_tests/{lab_id}/markers/{provider_id}"))
                .await?;
            output::print_json(&data);
        }
        LabTestsCommand::TestMarkers { lab_test_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/lab_tests/{lab_test_id}/markers"))
                .await?;
            output::print_json(&data);
        }
        LabTestsCommand::CollectionInstructionPdf {
            lab_test_id,
            output: output_path,
        } => {
            let bytes = client
                .get_bytes(&format!(
                    "/v3/lab_test/{lab_test_id}/collection_instruction_pdf"
                ))
                .await?;
            std::fs::write(&output_path, &bytes)?;
            output::print_success(&format!("saved to {output_path}"));
        }
        LabTestsCommand::OrderSetMarkers { data } => {
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let result: serde_json::Value = client
                .post_json("/v3/lab_tests/list_order_set_markers", &body)
                .await?;
            output::print_json(&result);
        }
        LabTestsCommand::GetV3 {
            next_cursor,
            page_size,
        } => {
            let mut path = "/v3/lab_test".to_string();
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
    }

    Ok(())
}
