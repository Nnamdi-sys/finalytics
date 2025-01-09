use std::error::Error;
use chrono::{NaiveDate, NaiveDateTime};
use once_cell::sync::Lazy;
use polars::prelude::*;
use reqwest::{Client, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use select::document::Document;
use select::predicate::Name;
use tokio::task::spawn_blocking;
use cached::proc_macro::cached;
use serde_json::Value;
use anyhow::{Result, Context};
use vader_sentiment::SentimentIntensityAnalyzer;


pub static REQUEST_CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
});

#[cached(
    result = true,
    time = 900 // Yahoo Finance API has a 15-minute Delay for Real-Time Data
)]
pub async fn get_json_response(url: String) -> Result<Value> {
    let response = REQUEST_CLIENT.get(&url).send().await.context("Failed to send request")?;
    if response.status() != StatusCode::OK {
        return Err(anyhow::anyhow!("Request failed with status: {}", response.status()));
    }
    let result = response.json::<Value>().await.context("Failed to parse JSON response")?;
    Ok(result)
}

pub async fn fetch_news(token: &str, start_date: NaiveDate, end_date: NaiveDate, compute_sentiment: bool) -> Result<DataFrame, Box<dyn Error + Send + Sync>> {
    let url = format!(
        "https://news.google.com/rss/search?q=allintext:{}+after:{}+before:{}",
        token,
        start_date.format("%Y-%m-%d"),
        end_date.format("%Y-%m-%d")
    );
    let body = fetch_html(url).await?;
    let df = spawn_blocking(move || extract_news_details(body, compute_sentiment)).await?;
    Ok(df)
}

#[cached(
    result = true,
    time = 3600 // Cache Google News Results for 1 Hour
)]
async fn fetch_html(url: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let response = REQUEST_CLIENT.get(url).send().await?;

   if response.status() != StatusCode::OK {
       let body = response.text().await?;
       println!("Request failed with error: {}", &body);
       return Err(format!("Request failed with error: {}", body).into());
    }

    let body = response.text().await?;
    Ok(body)
}

fn extract_news_details(body: String, compute_sentiment: bool) -> DataFrame {
    let document = Document::from_read(body.as_bytes()).unwrap();

    // Collect data into vectors
    let mut titles = Vec::new();
    let mut sources = Vec::new();
    let mut links = Vec::new();
    let mut pub_dates = Vec::new();
    let mut sentiment_scores = Vec::new();

    for item in document.find(Name("item")) {
        let title = item.children().nth(0).map(|n| n.text()).unwrap_or_default();
        let source = item.last_child().map(|n| n.text()).unwrap_or_default();
        let link = item.children().nth(2).map(|n| n.text()).unwrap_or_default();
        let pub_date = item.children().nth(4).map(|n| n.text()).unwrap_or_default();
        if title.is_empty() || link.is_empty() || pub_date.is_empty() {
            continue;
        }
        let pub_date = NaiveDateTime::parse_from_str(&pub_date, "%a, %d %b %Y %H:%M:%S GMT").unwrap();
        titles.push(title.clone());
        links.push(format!(r#"<a href="{}">{}</a>"#, link, title.replace(format!("- {}", source).as_str(), "")));
        sources.push(source);
        pub_dates.push(pub_date);
        if compute_sentiment {
            let analyzer = SentimentIntensityAnalyzer::new();
            let sentiment = analyzer.polarity_scores(&title);
            sentiment_scores.push(sentiment["compound"]);
        }
    }

    let df = DataFrame::new(vec![
        Series::new("Published Date", pub_dates),
        Series::new("Source", sources),
        Series::new("Title", titles),
        Series::new("Link", links),
    ]).unwrap();

    if compute_sentiment {
        let mut new_df = df.clone();
        match new_df.with_column(Series::new("Sentiment Score", sentiment_scores)) {
            Ok(_) => new_df,
            Err(e) => {
                eprintln!("Error Computing Sentiment Scores: {}", e);
                df
            }
        }
    } else {
        df
    }

}



