use anyhow::Result;

use crate::config::Config;
use crate::output;

pub fn run(api_key: Option<String>, base_url: Option<String>) -> Result<()> {
    let mut config = Config::load()?;

    if api_key.is_none() && base_url.is_none() {
        // Show current config
        let path = Config::path()?;
        println!("Config path: {}", path.display());
        println!(
            "API key:     {}",
            config
                .api_key
                .as_deref()
                .map(|k| format!("{}…", &k[..8.min(k.len())]))
                .unwrap_or_else(|| "(not set)".into())
        );
        println!("Base URL:    {}", config.base_url());
        return Ok(());
    }

    if let Some(key) = api_key {
        config.api_key = Some(key);
    }
    if let Some(url) = base_url {
        config.base_url = Some(url);
    }

    config.save()?;
    output::print_success("configuration saved");
    Ok(())
}
