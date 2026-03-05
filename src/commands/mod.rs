pub mod aggregate;
mod compendium;
mod configure;
mod insurance;
mod introspect;
mod lab_account;
mod lab_report;
mod lab_tests;
mod link;
mod order;
mod payor;
mod providers;
pub mod summary;
mod team;
mod timeseries;
mod user;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "junction", version, about = "CLI for the Junction API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Configure API key and settings
    Configure {
        /// API key
        #[arg(long)]
        api_key: Option<String>,
        /// Base URL override
        #[arg(long)]
        base_url: Option<String>,
    },

    /// Link management (providers, connections, bulk ops)
    #[command(subcommand)]
    Link(link::LinkCommand),

    /// Health data summaries
    #[command(subcommand)]
    Summary(summary::SummaryCommand),

    /// Timeseries health metrics
    #[command(subcommand)]
    Timeseries(timeseries::TimeseriesCommand),

    /// User management
    #[command(subcommand)]
    User(user::UserCommand),

    /// Lab test orders (create, manage, results)
    #[command(subcommand)]
    Order(order::OrderCommand),

    /// Lab test definitions and markers
    #[command(subcommand)]
    LabTests(lab_tests::LabTestsCommand),

    /// Lab report parsing
    #[command(subcommand)]
    LabReport(lab_report::LabReportCommand),

    /// Lab accounts
    LabAccounts,

    /// Team management
    #[command(subcommand)]
    Team(team::TeamCommand),

    /// Insurance operations
    #[command(subcommand)]
    Insurance(insurance::InsuranceCommand),

    /// Create a payor
    PayorCreate {
        /// JSON body
        #[arg(long)]
        data: String,
    },

    /// Compendium search and conversion
    #[command(subcommand)]
    Compendium(compendium::CompendiumCommand),

    /// Aggregate queries (Horizon AI)
    #[command(subcommand)]
    Aggregate(aggregate::AggregateCommand),

    /// Introspect API resources and status
    #[command(subcommand)]
    Introspect(introspect::IntrospectCommand),

    /// List all available providers
    Providers,
}

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Configure { api_key, base_url } => configure::run(api_key, base_url),
        Command::Link(cmd) => link::run(cmd).await,
        Command::Summary(cmd) => summary::run(cmd).await,
        Command::Timeseries(cmd) => timeseries::run(cmd).await,
        Command::User(cmd) => user::run(cmd).await,
        Command::Order(cmd) => order::run(cmd).await,
        Command::LabTests(cmd) => lab_tests::run(cmd).await,
        Command::LabReport(cmd) => lab_report::run(cmd).await,
        Command::LabAccounts => lab_account::run().await,
        Command::Team(cmd) => team::run(cmd).await,
        Command::Insurance(cmd) => insurance::run(cmd).await,
        Command::PayorCreate { data } => payor::run(data).await,
        Command::Compendium(cmd) => compendium::run(cmd).await,
        Command::Aggregate(cmd) => aggregate::run(cmd).await,
        Command::Introspect(cmd) => introspect::run(cmd).await,
        Command::Providers => providers::run().await,
    }
}
