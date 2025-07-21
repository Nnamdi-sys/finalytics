use polars::prelude::*;
use std::error::Error;
use chrono::{Duration, NaiveDate};
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Semaphore;
use crate::data::yahoo::api::get_quote;
use crate::data::google::web::fetch_news;

pub async fn get_news(symbol: &str, start_date: &str, end_date: &str) -> Result<DataFrame, Box<dyn Error>> {
    let quote = get_quote(symbol).await?;
    let symbol = if quote.asset_class == "CRYPTOCURRENCY" {
        symbol.replace("-USD", "")
    } else {
        symbol.to_string()
    };
    let token = format!("({} OR {})", &symbol, &quote.name);

    let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
    let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")?;

    let mut current_date = start_date;
    let mut futures = Vec::new();

    // Define the maximum number of concurrent tasks
    let max_concurrent_tasks = 20;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

    // Create and configure the progress bar
    let total_steps = (end_date - start_date).num_days()/3;
    let pb = ProgressBar::new(total_steps as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-"),
    );

    while current_date < end_date {
        let next_date = (current_date + Duration::days(3)).min(end_date);
        let token_clone = token.clone();
        let current_date_clone = current_date;
        let next_date_clone = next_date;
        let semaphore = semaphore.clone();

        let fut = tokio::task::spawn(async move {
            let permit = semaphore.acquire().await.unwrap();
            let result = fetch_news(&token_clone, current_date_clone, next_date_clone, true).await;
            drop(permit);
            match result {
                Ok(df) => Ok(df),
                Err(e) => Err(format!("Error fetching news for {current_date_clone} to {next_date_clone}: {e}")),
            }
        });

        futures.push(fut);
        current_date = next_date;
    }

    let results = join_all(futures).await;
    let mut combined_df = DataFrame::default();

    for res in results {
        match res {
            Ok(Ok(df)) => {
                match combined_df.vstack(&df) {
                    Ok(jdf) => combined_df = jdf,
                    Err(e) => eprintln!("Unable to stack {:?}: {}", &df, e),
                }
            }
            Ok(Err(_)) => continue,
            Err(e) => eprintln!("Error in task: {e}"),
        }
    }

    combined_df.sort(["Published Date"], SortMultipleOptions::new().with_order_descending(false))?;

    pb.finish_with_message(format!("News Data Fetched for {}", &symbol));

    Ok(combined_df)
}

