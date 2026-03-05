use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;
use crate::validate;

#[derive(Subcommand)]
pub enum SummaryCommand {
    /// Get sleep summary for a user
    Sleep {
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Provider filter
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get activity summary for a user
    Activity {
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Provider filter
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get workout summary for a user
    Workouts {
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Provider filter
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get body measurements for a user
    Body {
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Provider filter
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get meal/nutrition data for a user
    Meal {
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Provider filter
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get user profile
    Profile {
        /// User ID
        user_id: String,
    },

    /// Get electrocardiogram data
    Electrocardiogram {
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Provider filter
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get menstrual cycle data
    MenstrualCycle {
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Provider filter
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get sleep cycle data
    SleepCycle {
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Provider filter
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get raw data for any summary type
    Raw {
        /// Summary type (sleep, activity, workouts, body, profile, devices)
        #[arg(value_name = "TYPE")]
        summary_type: String,
        /// User ID
        user_id: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
    },
}

pub async fn run(cmd: SummaryCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    validate_summary_dates(&cmd)?;

    match cmd {
        SummaryCommand::Sleep {
            user_id,
            start_date,
            end_date,
            provider,
        } => {
            let path = build_summary_path(
                "sleep",
                &user_id,
                &start_date,
                end_date.as_deref(),
                provider.as_deref(),
            );
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::Activity {
            user_id,
            start_date,
            end_date,
            provider,
        } => {
            let path = build_summary_path(
                "activity",
                &user_id,
                &start_date,
                end_date.as_deref(),
                provider.as_deref(),
            );
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::Workouts {
            user_id,
            start_date,
            end_date,
            provider,
        } => {
            let path = build_summary_path(
                "workouts",
                &user_id,
                &start_date,
                end_date.as_deref(),
                provider.as_deref(),
            );
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::Body {
            user_id,
            start_date,
            end_date,
            provider,
        } => {
            let path = build_summary_path(
                "body",
                &user_id,
                &start_date,
                end_date.as_deref(),
                provider.as_deref(),
            );
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::Meal {
            user_id,
            start_date,
            end_date,
            provider,
        } => {
            let path = build_summary_path(
                "meal",
                &user_id,
                &start_date,
                end_date.as_deref(),
                provider.as_deref(),
            );
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::Profile { user_id } => {
            let path = format!("/v2/summary/profile/{user_id}");
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::Electrocardiogram {
            user_id,
            start_date,
            end_date,
            provider,
        } => {
            let path = build_summary_path(
                "electrocardiogram",
                &user_id,
                &start_date,
                end_date.as_deref(),
                provider.as_deref(),
            );
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::MenstrualCycle {
            user_id,
            start_date,
            end_date,
            provider,
        } => {
            let path = build_summary_path(
                "menstrual_cycle",
                &user_id,
                &start_date,
                end_date.as_deref(),
                provider.as_deref(),
            );
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::SleepCycle {
            user_id,
            start_date,
            end_date,
            provider,
        } => {
            let path = build_summary_path(
                "sleep_cycle",
                &user_id,
                &start_date,
                end_date.as_deref(),
                provider.as_deref(),
            );
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        SummaryCommand::Raw {
            summary_type,
            user_id,
            start_date,
            end_date,
        } => {
            let mut path = format!("/v2/summary/{summary_type}/{user_id}/raw");
            if let Some(sd) = &start_date {
                path.push_str(&format!("?start_date={sd}"));
                if let Some(ed) = &end_date {
                    path.push_str(&format!("&end_date={ed}"));
                }
            }
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
    }

    Ok(())
}

fn validate_summary_dates(cmd: &SummaryCommand) -> Result<()> {
    match cmd {
        SummaryCommand::Sleep {
            start_date,
            end_date,
            ..
        }
        | SummaryCommand::Activity {
            start_date,
            end_date,
            ..
        }
        | SummaryCommand::Workouts {
            start_date,
            end_date,
            ..
        }
        | SummaryCommand::Body {
            start_date,
            end_date,
            ..
        }
        | SummaryCommand::Meal {
            start_date,
            end_date,
            ..
        }
        | SummaryCommand::Electrocardiogram {
            start_date,
            end_date,
            ..
        }
        | SummaryCommand::MenstrualCycle {
            start_date,
            end_date,
            ..
        }
        | SummaryCommand::SleepCycle {
            start_date,
            end_date,
            ..
        } => {
            validate::date(start_date)?;
            if let Some(ed) = end_date {
                validate::date(ed)?;
            }
        }
        SummaryCommand::Profile { .. } => {}
        SummaryCommand::Raw {
            start_date,
            end_date,
            ..
        } => {
            if let Some(sd) = start_date {
                validate::date(sd)?;
            }
            if let Some(ed) = end_date {
                validate::date(ed)?;
            }
        }
    }
    Ok(())
}

pub fn build_summary_path(
    resource: &str,
    user_id: &str,
    start_date: &str,
    end_date: Option<&str>,
    provider: Option<&str>,
) -> String {
    let mut path = format!("/v2/summary/{resource}/{user_id}?start_date={start_date}");
    if let Some(ed) = end_date {
        path.push_str(&format!("&end_date={ed}"));
    }
    if let Some(p) = provider {
        path.push_str(&format!("&provider={p}"));
    }
    path
}
