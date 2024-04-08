use std::error::Error;
use select::document::Document;
use select::predicate::Name;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
use tokio::time::timeout;
use crate::models::ticker::Ticker;
use crate::utils::web_utils::scrape_text;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct News {
    pub title: String,
    pub source: String,
    pub link: String,
    pub timestamp: String,
    pub text: String,
    pub sentiment_score: f64,
    pub positive_score: f64,
    pub negative_score: f64,
    pub positive_keywords: Vec<String>,
    pub negative_keywords: Vec<String>,
}

pub trait NewsSentiment {
    fn get_news(&self, compute_sentiment: bool) ->  impl  std::future::Future<Output = Result<Vec<News>, Box<dyn Error>>>;
}


impl NewsSentiment for Ticker {
    /// Scrapes news articles from Google News RSS feed and computes sentiment scores
    ///
    /// # Arguments
    /// * `compute_sentiment` - Boolean flag to compute sentiment scores and get keywords (expensive)
    ///
    /// # Returns
    ///
    /// * `Vec<News>` Vector of News struct
    async fn get_news(&self, compute_sentiment: bool) -> Result<Vec<News>, Box<dyn Error>> {
        let mut result = vec![];
        let symbol = if self.ticker.asset_class == "CRYPTOCURRENCY" { self.ticker.symbol.replace("-USD", "") } else { self.ticker.symbol.clone() };
        let token = format!("({} OR {})", &symbol, &self.ticker.name);
        let url = format!("https://news.google.com/rss/search?q=allintext:{token}+when:1d");
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let document = Document::from_read(body.as_bytes()).unwrap();

        for item in document.find(Name("item")) {
            let title = item.children().nth(0).map(|n| n.text()).unwrap_or_default();
            let source = item.last_child().map(|n| n.text()).unwrap_or_default();
            let link = item.children().nth(2).map(|n| n.text()).unwrap_or_default();
            let pub_date = item.children().nth(4).map(|n| n.text()).unwrap_or_default();

            let timeout_duration = Duration::from_secs(10); // Set your desired timeout duration in seconds

            let fetch_task = async {
                let news = if !compute_sentiment {
                    News {
                        title: title.clone(),
                        source: source.clone(),
                        link: link.clone(),
                        timestamp: pub_date.clone(),
                        text: " ".to_string(),
                        sentiment_score: 0.0,
                        positive_score: 0.0,
                        negative_score: 0.0,
                        positive_keywords: vec![],
                        negative_keywords: vec![],
                    }
                } else {
                    match timeout(timeout_duration, scrape_text(&link, &title)).await {
                        Ok(Ok(article)) => {
                            News {
                                title: title.clone(),
                                source: source.clone(),
                                link: link.clone(),
                                timestamp: pub_date.clone(),
                                text: article.text.clone(),
                                sentiment_score: article.sentiment_score,
                                positive_score: article.positive_score,
                                negative_score: article.negative_score,
                                positive_keywords: article.positive_keywords,
                                negative_keywords: article.negative_keywords,
                            }
                        }
                        _ => News {
                            title: title.clone(),
                            source: source.clone(),
                            link: link.clone(),
                            timestamp: pub_date.clone(),
                            text: " ".to_string(),
                            sentiment_score: 0.0,
                            positive_score: 0.0,
                            negative_score: 0.0,
                            positive_keywords: vec![],
                            negative_keywords: vec![],
                        }
                    }
                };
                result.push(news);
            };

            fetch_task.await;
        }

        Ok(result)
    }
}






