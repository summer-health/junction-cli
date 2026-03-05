use anyhow::Result;
use clap::Subcommand;

use crate::client::JunctionClient;
use crate::config::Config;
use crate::output;
use crate::validate;

/// All known timeseries metrics.
///
/// The CLI does not enforce this list — any metric string is accepted and
/// forwarded to the API, which will return an error for unknown metrics.
/// This list is provided for documentation and shell-completion only.
pub const KNOWN_METRICS: &[&str] = &[
    "afib_burden",
    "basal_body_temperature",
    "blood_oxygen",
    "blood_pressure",
    "body_fat",
    "body_mass_index",
    "body_temperature",
    "body_temperature_delta",
    "body_weight",
    "caffeine",
    "calories_active",
    "calories_basal",
    "carbohydrates",
    "cholesterol",
    "cholesterol/hdl",
    "cholesterol/ldl",
    "cholesterol/total",
    "cholesterol/triglycerides",
    "daylight_exposure",
    "distance",
    "electrocardiogram_voltage",
    "fall",
    "floors_climbed",
    "forced_expiratory_volume_1",
    "forced_vital_capacity",
    "glucose",
    "handwashing",
    "heart_rate_alert",
    "heart_rate_recovery_one_minute",
    "heartrate",
    "hrv",
    "hypnogram",
    "ige",
    "igg",
    "inhaler_usage",
    "insulin_injection",
    "lean_body_mass",
    "mindfulness_minutes",
    "note",
    "peak_expiratory_flow_rate",
    "respiratory_rate",
    "sleep_apnea_alert",
    "sleep_breathing_disturbance",
    "stand_duration",
    "stand_hour",
    "steps",
    "stress_level",
    "uv_exposure",
    "vo2_max",
    "waist_circumference",
    "water",
    "wheelchair_push",
    "workout_distance",
    "workout_duration",
    "workout_swimming_stroke",
];

#[derive(Subcommand)]
pub enum TimeseriesCommand {
    /// Get raw timeseries data for a metric
    ///
    /// Calls GET /v2/timeseries/{user_id}/{metric}
    Get {
        /// User ID
        user_id: String,
        /// Metric name (e.g. heartrate, cholesterol/hdl)
        metric: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Filter by provider slug
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get grouped timeseries data for a metric
    ///
    /// Calls GET /v2/timeseries/{user_id}/{metric}/grouped
    Grouped {
        /// User ID
        user_id: String,
        /// Metric name (e.g. heartrate, cholesterol/hdl)
        metric: String,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
        /// Filter by provider slug
        #[arg(long)]
        provider: Option<String>,
    },

    /// Get the stream for a sleep session
    ///
    /// Calls GET /v2/timeseries/sleep/{sleep_id}/stream
    SleepStream {
        /// Sleep session ID
        sleep_id: String,
    },

    /// Get the stream for a workout session
    ///
    /// Calls GET /v2/timeseries/workouts/{workout_id}/stream
    WorkoutStream {
        /// Workout session ID
        workout_id: String,
    },
}

/// Build a query string from optional parameters.
///
/// `start_date` is always present for get/grouped calls.
fn build_query(start_date: &str, end_date: &Option<String>, provider: &Option<String>) -> String {
    let mut params = vec![format!("start_date={start_date}")];
    if let Some(end) = end_date {
        params.push(format!("end_date={end}"));
    }
    if let Some(prov) = provider {
        params.push(format!("provider={prov}"));
    }
    format!("?{}", params.join("&"))
}

pub async fn run(cmd: TimeseriesCommand) -> Result<()> {
    let config = Config::load()?;
    let client = JunctionClient::new(&config)?;

    match &cmd {
        TimeseriesCommand::Get {
            start_date,
            end_date,
            ..
        }
        | TimeseriesCommand::Grouped {
            start_date,
            end_date,
            ..
        } => {
            validate::date(start_date)?;
            if let Some(ed) = end_date {
                validate::date(ed)?;
            }
        }
        _ => {}
    }

    match cmd {
        TimeseriesCommand::Get {
            user_id,
            metric,
            start_date,
            end_date,
            provider,
        } => {
            let qs = build_query(&start_date, &end_date, &provider);
            let path = format!("/v2/timeseries/{user_id}/{metric}{qs}");
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        TimeseriesCommand::Grouped {
            user_id,
            metric,
            start_date,
            end_date,
            provider,
        } => {
            let qs = build_query(&start_date, &end_date, &provider);
            let path = format!("/v2/timeseries/{user_id}/{metric}/grouped{qs}");
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        TimeseriesCommand::SleepStream { sleep_id } => {
            let path = format!("/v2/timeseries/sleep/{sleep_id}/stream");
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
        TimeseriesCommand::WorkoutStream { workout_id } => {
            let path = format!("/v2/timeseries/workouts/{workout_id}/stream");
            let data: serde_json::Value = client.get_raw(&path).await?;
            output::print_json(&data);
        }
    }

    Ok(())
}
