use once_cell::sync::Lazy;
use reqwest::{Client, StatusCode, Url};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;
use anyhow::{Result, Context};
use cached::proc_macro::cached;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

static SESSION_MANAGER: Lazy<RwLock<SessionManager>> = Lazy::new(|| {
    RwLock::new(SessionManager::new().expect("Failed to create session manager"))
});

const MAX_RETRIES: usize = 5;
const RETRY_DELAY_MS: u64 = 500;

pub struct SessionManager {
    client: Client,
    crumb: Option<String>,
}

impl SessionManager {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"));
        headers.insert("Accept", HeaderValue::from_static("application/json, text/plain, */*"));

        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            crumb: None,
        })
    }

    pub async fn ensure_session(&mut self) -> Result<()> {
        // Warm-up session
        self.client
            .get("https://fc.yahoo.com")
            .send()
            .await
            .context("Failed to warm up Yahoo session")?;

        // Retry loop for crumb
        for attempt in 1..=MAX_RETRIES {
            let crumb_result = self.client
                .get("https://query2.finance.yahoo.com/v1/test/getcrumb")
                .header("Accept", "text/plain")
                .send()
                .await;

            match crumb_result {
                Ok(response) => {
                    if response.status() == StatusCode::OK {
                        let crumb_text = response
                            .text()
                            .await
                            .context("Failed to read crumb response")?;

                        self.crumb = Some(crumb_text);
                        return Ok(());
                    } else if response.status() == StatusCode::UNAUTHORIZED {
                        if attempt == MAX_RETRIES {
                            return Err(anyhow::anyhow!("Failed to fetch crumb: unauthorized after {} attempts", attempt));
                        }
                        sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                        continue;
                    } else {
                        return Err(anyhow::anyhow!("Failed to fetch crumb with unexpected status: {}", response.status()));
                    }
                }
                Err(e) => {
                    if attempt == MAX_RETRIES {
                        return Err(e).context(format!("Failed to fetch crumb after {attempt} attempts"));
                    }
                    sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                    continue;
                }
            }
        }
        
        Err(anyhow::anyhow!("Crumb fetch retry loop exhausted unexpectedly"))
    }

    pub async fn get_url_with_crumb(&mut self, url: &str) -> Result<Url> {
        if self.crumb.is_none() {
            self.ensure_session().await?;
        }

        let crumb = self.crumb.as_deref().unwrap_or_default();
        let mut api_url = Url::parse(url)?;
        api_url.query_pairs_mut().append_pair("crumb", crumb);
        Ok(api_url)
    }

    pub async fn request_json(&mut self, url: &str) -> Result<Value> {
        let mut tried_refresh = false;

        loop {
            let full_url = self.get_url_with_crumb(url).await?;

            let response = self.client.get(full_url.clone())
                .send()
                .await
                .context("Failed to send request")?;

            if response.status() == StatusCode::OK {
                let json = response.json::<Value>()
                    .await
                    .context("Failed to parse JSON response")?;
                return Ok(json);
            } else if !tried_refresh && self.crumb.is_some() {
                // Retry once with refreshed crumb
                self.crumb = None;
                self.ensure_session().await?;
                tried_refresh = true;
                continue;
            } else {
                return Err(anyhow::anyhow!("Request failed with status: {}", response.status()));
            }
        }
    }

    pub async fn post_json(&mut self, url: &str, payload: &Value) -> Result<Value> {
        let mut tried_refresh = false;

        loop {
            let full_url = self.get_url_with_crumb(url).await?;

            let response = self.client
                .post(full_url.clone())
                .json(payload)
                .send()
                .await
                .context("Failed to send POST request")?;

            if response.status() == StatusCode::OK {
                let json = response
                    .json::<Value>()
                    .await
                    .context("Failed to parse JSON response")?;
                return Ok(json);
            } else if response.status() == StatusCode::TOO_MANY_REQUESTS {
                // Optional: delay and retry on 429
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            } else if !tried_refresh && self.crumb.is_some() {
                // Retry once with refreshed crumb
                self.crumb = None;
                self.ensure_session().await?;
                tried_refresh = true;
                continue;
            } else {
                return Err(anyhow::anyhow!(
                "POST request failed with status: {}",
                response.status()
            ));
            }
        }
    }
}

#[cached(
    result = true,
    time = 900 // Yahoo Finance API has a 15-minute Delay for Real-Time Data
)]
pub async fn get_json_response(url: String) -> Result<Value> {
    let mut session = SESSION_MANAGER.write().await;
    session.request_json(&url).await
}

#[cached(
    result = true,
    time = 900, // Yahoo Finance API has a 15-minute delay for real-time data
    key = "(String, Value)",
    convert = r#"{ (url.clone(), payload.clone()) }"#
)]
pub async fn post_json_response(url: String, payload: Value) -> Result<Value> {
    let mut session = SESSION_MANAGER.write().await;
    session.post_json(&url, &payload).await
}