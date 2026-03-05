mod configure;
mod link;
pub mod summary;
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

    /// Link management (providers, connections)
    #[command(subcommand)]
    Link(link::LinkCommand),

    /// Health data summaries
    #[command(subcommand)]
    Summary(summary::SummaryCommand),

    /// User operations (via summary/profile)
    #[command(subcommand)]
    User(user::UserCommand),
}

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Configure { api_key, base_url } => configure::run(api_key, base_url),
        Command::Link(cmd) => link::run(cmd).await,
        Command::Summary(cmd) => summary::run(cmd).await,
        Command::User(cmd) => user::run(cmd).await,
    }
}
