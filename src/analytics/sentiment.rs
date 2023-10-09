use std::error::Error;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use rust_bert::pipelines::keywords_extraction::{Keyword, KeywordExtractionModel};
use rust_bert::pipelines::sentiment::{Sentiment, SentimentModel, SentimentPolarity};
use rust_bert::pipelines::summarization::SummarizationModel;

#[derive(Debug)]
pub struct Article {
    text: String,
    keywords: Vec<Vec<Keyword>>,
    sentiment: Vec<Sentiment>,
    summary: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct News {
    title: String,
    source: String,
    link: String,
    timestamp: String,
    text: String,
    keywords: Vec<(String, f64)>,
    sentiment: f64,
    summary: Vec<String>,
}


/// Scrapes news articles from Google News RSS feed
///
/// # Arguments
///
/// * `token` - Search token (e.g. "AAPL")
/// * `start` - Start date in YYYY-MM-DD format (e.g. "2021-01-01")
/// * `end` - End date in YYYY-MM-DD format (e.g. "2021-01-31")
///
/// # Returns
///
/// * `Vec<News>` Vector of News struct
pub async fn scrape_news(
    token: &str,
    start: &str,
    end: &str
) -> Result<Vec<News>, Box<dyn Error>> {
    let mut result = vec![];
    let url = format!("https://news.google.com/rss/search?q=allintext:{token}+when:after:{start}+before:{end}");
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let document = Document::from_read(body.as_bytes()).unwrap();
    for item in document.find(Name("item")) {
        let title = item.children().nth(0).map(|n| n.text()).unwrap_or_default();
        let source = item.last_child().map(|n| n.text()).unwrap_or_default();
        let link = item.children().nth(2).map(|n| n.text()).unwrap_or_default();
        let pub_date = item.children().nth(4).map(|n| n.text()).unwrap_or_default();
        if let Ok(article) = tokio::task::block_in_place(|| scrape_text(&link, &title)) {
            let news = News {
                title: title.clone(),
                source: source.clone(),
                link: link.clone(),
                timestamp: pub_date.clone(),
                text: article.text.clone(),
                keywords: article.keywords[0].iter().map(|x| (x.text.clone(), x.score as f64)).collect::<Vec<(String, f64)>>(),
                sentiment: if article.sentiment[0].polarity == SentimentPolarity::Negative { article.sentiment[0].score * -1.0 } else { article.sentiment[0].score },
                summary: article.summary.clone(),
            };
            dbg!(&news);
            result.push(news);
        }
        else {
            eprintln!("Error scraping article from {}", &link);
            continue;
        }

    }
    Ok(result)
}

fn scrape_text(url: &str, title: &str) -> Result<Article, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let body = response.text()?;

    // Parse the HTML content of the article
    let document = Document::from_read(body.as_bytes()).unwrap();

    // Extract the text of the article
    let mut link = String::new();
    let mut text = String::new();

    for node in document.find(Name("a")) {
        link.push_str(&node.text());
        let response = client.get(&node.text()).send()?;
        let body = response.text()?;
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

    let keyword_extraction_model = KeywordExtractionModel::new(Default::default())?;
    let keywords = keyword_extraction_model.predict(&[text.clone()])?;

    let sentiment_model = SentimentModel::new(Default::default())?;
    let sentiment = sentiment_model.predict(&[&text[..]]);

    let model = SummarizationModel::new(Default::default())?;
    let summary = model.summarize(&[text.clone()]);

    let article = Article {text, keywords, sentiment, summary};

    Ok(article)
}
