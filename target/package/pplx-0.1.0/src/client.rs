use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use serde_json::Value;

pub struct PplxClient {
    client: Client,
    api_key: String,
}

impl PplxClient {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("PERPLEXITY_API_KEY")
            .context("PERPLEXITY_API_KEY not set. Add it to ~/.secrets")?;
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub fn query(&self, model: &str, query: &str) -> Result<Value> {
        let body = serde_json::json!({
            "model": model,
            "messages": [{"role": "user", "content": query}]
        });

        let resp = self
            .client
            .post("https://api.perplexity.ai/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .context("Failed to reach Perplexity API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().unwrap_or_default();
            bail!("Perplexity API returned {status}: {body}");
        }

        resp.json().context("Failed to parse API response")
    }
}
