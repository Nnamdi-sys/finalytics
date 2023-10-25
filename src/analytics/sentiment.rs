use std::error::Error;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use sentiment::analyze;

#[derive(Debug)]
pub struct Article {
    text: String,
    sentiment_score: f64,
    positive_score: f64,
    negative_score: f64,
    positive_keywords: Vec<String>,
    negative_keywords: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
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
///
/// # Example
///
/// ```
/// use std::error::Error;
/// use finalytics::analytics::sentiment::scrape_news;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let result = scrape_news("MSFT", "2023-01-01", "2023-01-02").await?;
///     println!("{:?}", result);
///     Ok(())
/// }
/// ```
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
                sentiment_score: article.sentiment_score,
                positive_score: article.positive_score,
                negative_score: article.negative_score,
                positive_keywords: article.positive_keywords,
                negative_keywords: article.negative_keywords,
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
