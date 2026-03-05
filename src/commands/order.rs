use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;

#[derive(Subcommand)]
pub enum OrderCommand {
    /// Create a new order
    Create {
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Import an existing order
    Import {
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Get an order by ID
    Get {
        /// Order ID
        order_id: String,
    },

    /// List orders
    List,

    /// Cancel an order
    Cancel {
        /// Order ID
        order_id: String,
        /// JSON request body (optional cancellation details)
        #[arg(long)]
        data: Option<String>,
    },

    /// Mark an order's draw as completed
    DrawCompleted {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: Option<String>,
    },

    /// Add a test to an order
    AddTest {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Resend order events
    ResendEvents {
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Get area info for orders
    AreaInfo,

    /// Get order result
    Result {
        /// Order ID
        order_id: String,
    },

    /// Get order result metadata
    ResultMetadata {
        /// Order ID
        order_id: String,
    },

    /// Download order result PDF
    ResultPdf {
        /// Order ID
        order_id: String,
        /// Output file path
        #[arg(long, default_value = "result.pdf")]
        output: PathBuf,
    },

    /// Download ABN PDF for an order
    AbnPdf {
        /// Order ID
        order_id: String,
        /// Output file path
        #[arg(long, default_value = "abn.pdf")]
        output: PathBuf,
    },

    /// Download collection instruction PDF
    CollectionInstructionPdf {
        /// Order ID
        order_id: String,
        /// Output file path
        #[arg(long, default_value = "collection_instruction.pdf")]
        output: PathBuf,
    },

    /// Download labels PDF
    LabelsPdf {
        /// Order ID
        order_id: String,
        /// Output file path
        #[arg(long, default_value = "labels.pdf")]
        output: PathBuf,
    },

    /// Download requisition PDF
    RequisitionPdf {
        /// Order ID
        order_id: String,
        /// Output file path
        #[arg(long, default_value = "requisition.pdf")]
        output: PathBuf,
    },

    /// Testkit operations
    #[command(subcommand)]
    Testkit(TestkitCommand),

    /// Phlebotomy appointment operations
    #[command(subcommand)]
    Phlebotomy(PhlebotomyCommand),

    /// PSC (Patient Service Center) appointment operations
    #[command(subcommand)]
    Psc(PscCommand),

    /// Order transaction operations
    #[command(subcommand)]
    Transaction(TransactionCommand),
}

#[derive(Subcommand)]
pub enum TestkitCommand {
    /// Create a testkit order
    Create {
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Register a testkit
    Register {
        /// JSON request body
        #[arg(long)]
        data: String,
    },
}

#[derive(Subcommand)]
pub enum PhlebotomyCommand {
    /// Get phlebotomy appointment for an order
    Get {
        /// Order ID
        order_id: String,
    },

    /// Book a phlebotomy appointment
    Book {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Request a phlebotomy appointment
    Request {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Cancel a phlebotomy appointment
    Cancel {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: Option<String>,
    },

    /// Reschedule a phlebotomy appointment
    Reschedule {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Check phlebotomy appointment availability
    Availability {
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Get phlebotomy appointment cancellation reasons
    CancellationReasons,
}

#[derive(Subcommand)]
pub enum PscCommand {
    /// Get PSC appointment for an order
    Get {
        /// Order ID
        order_id: String,
    },

    /// Get PSC info for an order
    Info {
        /// Order ID
        order_id: String,
    },

    /// Get general PSC info
    GeneralInfo,

    /// Book a PSC appointment
    Book {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Cancel a PSC appointment
    Cancel {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: Option<String>,
    },

    /// Reschedule a PSC appointment
    Reschedule {
        /// Order ID
        order_id: String,
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Check PSC appointment availability
    Availability {
        /// JSON request body
        #[arg(long)]
        data: String,
    },

    /// Get PSC appointment cancellation reasons
    CancellationReasons,
}

#[derive(Subcommand)]
pub enum TransactionCommand {
    /// Get an order transaction by ID
    Get {
        /// Transaction ID
        transaction_id: String,
    },

    /// Get order transaction result
    Result {
        /// Transaction ID
        transaction_id: String,
    },

    /// Download order transaction result PDF
    ResultPdf {
        /// Transaction ID
        transaction_id: String,
        /// Output file path
        #[arg(long, default_value = "transaction_result.pdf")]
        output: PathBuf,
    },
}

async fn save_pdf(client: &JunctionClient, path: &str, output: &PathBuf) -> Result<()> {
    let bytes = client.get_bytes(path).await?;
    std::fs::write(output, &bytes)?;
    println!("Saved PDF to {}", output.display());
    Ok(())
}

fn parse_data(data: &str) -> Result<serde_json::Value> {
    crate::validate::json(data)
}

fn empty_body() -> serde_json::Value {
    serde_json::json!({})
}

pub async fn run(cmd: OrderCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match cmd {
        OrderCommand::Create { data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client.post_json("/v3/order", &body).await?;
            output::print_json(&resp);
        }

        OrderCommand::Import { data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client.post_json("/v3/order/import", &body).await?;
            output::print_json(&resp);
        }

        OrderCommand::Get { order_id } => {
            let data: serde_json::Value =
                client.get_raw(&format!("/v3/order/{}", order_id)).await?;
            output::print_json(&data);
        }

        OrderCommand::List => {
            let data: serde_json::Value = client.get_raw("/v3/orders").await?;
            output::print_json(&data);
        }

        OrderCommand::Cancel { order_id, data } => {
            let body = match data {
                Some(d) => parse_data(&d)?,
                None => empty_body(),
            };
            let resp: serde_json::Value = client
                .post_json(&format!("/v3/order/{}/cancel", order_id), &body)
                .await?;
            output::print_json(&resp);
        }

        OrderCommand::DrawCompleted { order_id, data } => {
            let body = match data {
                Some(d) => parse_data(&d)?,
                None => empty_body(),
            };
            let resp: serde_json::Value = client
                .patch_json(&format!("/v3/order/{}/draw_completed", order_id), &body)
                .await?;
            output::print_json(&resp);
        }

        OrderCommand::AddTest { order_id, data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .post_json(&format!("/v3/order/{}/test", order_id), &body)
                .await?;
            output::print_json(&resp);
        }

        OrderCommand::ResendEvents { data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value =
                client.post_json("/v3/order/resend_events", &body).await?;
            output::print_json(&resp);
        }

        OrderCommand::AreaInfo => {
            let data: serde_json::Value = client.get_raw("/v3/order/area/info").await?;
            output::print_json(&data);
        }

        OrderCommand::Result { order_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/order/{}/result", order_id))
                .await?;
            output::print_json(&data);
        }

        OrderCommand::ResultMetadata { order_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/order/{}/result/metadata", order_id))
                .await?;
            output::print_json(&data);
        }

        OrderCommand::ResultPdf { order_id, output } => {
            save_pdf(
                &client,
                &format!("/v3/order/{}/result/pdf", order_id),
                &output,
            )
            .await?;
        }

        OrderCommand::AbnPdf { order_id, output } => {
            save_pdf(&client, &format!("/v3/order/{}/abn_pdf", order_id), &output).await?;
        }

        OrderCommand::CollectionInstructionPdf { order_id, output } => {
            save_pdf(
                &client,
                &format!("/v3/order/{}/collection_instruction_pdf", order_id),
                &output,
            )
            .await?;
        }

        OrderCommand::LabelsPdf { order_id, output } => {
            save_pdf(
                &client,
                &format!("/v3/order/{}/labels/pdf", order_id),
                &output,
            )
            .await?;
        }

        OrderCommand::RequisitionPdf { order_id, output } => {
            save_pdf(
                &client,
                &format!("/v3/order/{}/requisition/pdf", order_id),
                &output,
            )
            .await?;
        }

        OrderCommand::Testkit(testkit_cmd) => run_testkit(&client, testkit_cmd).await?,
        OrderCommand::Phlebotomy(phleb_cmd) => run_phlebotomy(&client, phleb_cmd).await?,
        OrderCommand::Psc(psc_cmd) => run_psc(&client, psc_cmd).await?,
        OrderCommand::Transaction(tx_cmd) => run_transaction(&client, tx_cmd).await?,
    }

    Ok(())
}

async fn run_testkit(client: &JunctionClient, cmd: TestkitCommand) -> Result<()> {
    match cmd {
        TestkitCommand::Create { data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client.post_json("/v3/order/testkit", &body).await?;
            output::print_json(&resp);
        }

        TestkitCommand::Register { data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .post_json("/v3/order/testkit/register", &body)
                .await?;
            output::print_json(&resp);
        }
    }

    Ok(())
}

async fn run_phlebotomy(client: &JunctionClient, cmd: PhlebotomyCommand) -> Result<()> {
    match cmd {
        PhlebotomyCommand::Get { order_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/order/{}/phlebotomy/appointment", order_id))
                .await?;
            output::print_json(&data);
        }

        PhlebotomyCommand::Book { order_id, data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .post_json(
                    &format!("/v3/order/{}/phlebotomy/appointment/book", order_id),
                    &body,
                )
                .await?;
            output::print_json(&resp);
        }

        PhlebotomyCommand::Request { order_id, data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .post_json(
                    &format!("/v3/order/{}/phlebotomy/appointment/request", order_id),
                    &body,
                )
                .await?;
            output::print_json(&resp);
        }

        PhlebotomyCommand::Cancel { order_id, data } => {
            let body = match data {
                Some(d) => parse_data(&d)?,
                None => empty_body(),
            };
            let resp: serde_json::Value = client
                .patch_json(
                    &format!("/v3/order/{}/phlebotomy/appointment/cancel", order_id),
                    &body,
                )
                .await?;
            output::print_json(&resp);
        }

        PhlebotomyCommand::Reschedule { order_id, data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .patch_json(
                    &format!("/v3/order/{}/phlebotomy/appointment/reschedule", order_id),
                    &body,
                )
                .await?;
            output::print_json(&resp);
        }

        PhlebotomyCommand::Availability { data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .post_json("/v3/order/phlebotomy/appointment/availability", &body)
                .await?;
            output::print_json(&resp);
        }

        PhlebotomyCommand::CancellationReasons => {
            let data: serde_json::Value = client
                .get_raw("/v3/order/phlebotomy/appointment/cancellation-reasons")
                .await?;
            output::print_json(&data);
        }
    }

    Ok(())
}

async fn run_psc(client: &JunctionClient, cmd: PscCommand) -> Result<()> {
    match cmd {
        PscCommand::Get { order_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/order/{}/psc/appointment", order_id))
                .await?;
            output::print_json(&data);
        }

        PscCommand::Info { order_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/order/{}/psc/info", order_id))
                .await?;
            output::print_json(&data);
        }

        PscCommand::GeneralInfo => {
            let data: serde_json::Value = client.get_raw("/v3/order/psc/info").await?;
            output::print_json(&data);
        }

        PscCommand::Book { order_id, data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .post_json(
                    &format!("/v3/order/{}/psc/appointment/book", order_id),
                    &body,
                )
                .await?;
            output::print_json(&resp);
        }

        PscCommand::Cancel { order_id, data } => {
            let body = match data {
                Some(d) => parse_data(&d)?,
                None => empty_body(),
            };
            let resp: serde_json::Value = client
                .patch_json(
                    &format!("/v3/order/{}/psc/appointment/cancel", order_id),
                    &body,
                )
                .await?;
            output::print_json(&resp);
        }

        PscCommand::Reschedule { order_id, data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .patch_json(
                    &format!("/v3/order/{}/psc/appointment/reschedule", order_id),
                    &body,
                )
                .await?;
            output::print_json(&resp);
        }

        PscCommand::Availability { data } => {
            let body = parse_data(&data)?;
            let resp: serde_json::Value = client
                .post_json("/v3/order/psc/appointment/availability", &body)
                .await?;
            output::print_json(&resp);
        }

        PscCommand::CancellationReasons => {
            let data: serde_json::Value = client
                .get_raw("/v3/order/psc/appointment/cancellation-reasons")
                .await?;
            output::print_json(&data);
        }
    }

    Ok(())
}

async fn run_transaction(client: &JunctionClient, cmd: TransactionCommand) -> Result<()> {
    match cmd {
        TransactionCommand::Get { transaction_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/order_transaction/{}", transaction_id))
                .await?;
            output::print_json(&data);
        }

        TransactionCommand::Result { transaction_id } => {
            let data: serde_json::Value = client
                .get_raw(&format!("/v3/order_transaction/{}/result", transaction_id))
                .await?;
            output::print_json(&data);
        }

        TransactionCommand::ResultPdf {
            transaction_id,
            output,
        } => {
            save_pdf(
                client,
                &format!("/v3/order_transaction/{}/result/pdf", transaction_id),
                &output,
            )
            .await?;
        }
    }

    Ok(())
}
