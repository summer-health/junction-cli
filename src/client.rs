use anyhow::{Context, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use crate::config::Config;

pub struct JunctionClient {
    http: Client,
    base_url: String,
    api_key: String,
}

impl JunctionClient {
    pub fn new(config: &Config) -> Result<Self> {
        let api_key = config.resolve_api_key()?;
        let base_url = config.base_url().to_string();
        let http = Client::new();
        Ok(Self {
            http,
            base_url,
            api_key,
        })
    }

    pub fn from_parts(base_url: String, api_key: String) -> Self {
        Self {
            http: Client::new(),
            base_url,
            api_key,
        }
    }

    pub async fn get(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("x-vital-api-key", &self.api_key)
            .send()
            .await
            .with_context(|| format!("request to {} failed", url))?;
        Ok(resp)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self.get(path).await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("API error ({}): {}", status, body);
        }
        let value = resp.json::<T>().await.context("failed to parse response")?;
        Ok(value)
    }

    pub async fn post_json<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("x-vital-api-key", &self.api_key)
            .json(body)
            .send()
            .await
            .with_context(|| format!("request to {} failed", url))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("API error ({}): {}", status, body);
        }
        resp.json::<T>().await.context("failed to parse response")
    }

    pub async fn delete(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .delete(&url)
            .header("x-vital-api-key", &self.api_key)
            .send()
            .await
            .with_context(|| format!("request to {} failed", url))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(resp)
    }

    pub async fn get_raw(&self, path: &str) -> Result<serde_json::Value> {
        self.get_json(path).await
    }
}
