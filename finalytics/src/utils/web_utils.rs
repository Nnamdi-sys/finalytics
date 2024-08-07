use std::error::Error;
use once_cell::sync::Lazy;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use select::document::Document;
use select::predicate::Name;
use sentiment::analyze;


pub static REQUEST_CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
});


#[derive(Debug)]
pub struct Article {
    pub text: String,
    pub sentiment_score: f64,
    pub positive_score: f64,
    pub negative_score: f64,
    pub positive_keywords: Vec<String>,
    pub negative_keywords: Vec<String>,
}

pub async fn scrape_text(url: &str, title: &str) -> Result<Article, Box<dyn Error>> {
    let response = REQUEST_CLIENT.get(url).send().await?;
    let body = response.text().await?;

    // Parse the HTML content of the article
    let document = Document::from_read(body.as_bytes()).unwrap();

    // Extract the text of the article
    let mut link = String::new();
    let mut text = String::new();

    for node in document.find(Name("a")) {
        link.push_str(&node.text());
        let response = REQUEST_CLIENT.get(&node.text()).send().await?;
        let body = response.text().await?;
        let document = Document::from_read(body.as_bytes()).unwrap();
        for node in document.find(Name("p")) {
            let mut include_node = false;
            let mut parent = node.parent();

            // Check if any ancestor node is in the list of unwanted tags
            while let Some(p) = parent {
                if &p.name().unwrap() == &"article" || (&p.name().unwrap() == &"div" && p.attr("class").unwrap_or_default().contains("content")) {
                    include_node = true;
                    break;
                }
                parent = p.parent();
            }

            if include_node {
                text.push_str(&node.text());
            }
        }
    }

    if text.is_empty() {
        text = title.to_string();
    }

    let sentiment_result = analyze(text.clone());
    let sentiment_score = sentiment_result.score as f64;
    let positive_score = sentiment_result.positive.score as f64;
    let negative_score = sentiment_result.negative.score as f64;
    let positive_keywords = sentiment_result.positive.words;
    let negative_keywords = sentiment_result.negative.words;

    let article = Article {
        text: text.to_string(),
        sentiment_score,
        positive_score,
        negative_score,
        positive_keywords,
        negative_keywords,
    };

    Ok(article)
}