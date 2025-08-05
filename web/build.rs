use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use finalytics::prelude::*;
use yahoo_finance_symbols::keys::{AssetClass, Category, Exchange};
use yahoo_finance_symbols::get_symbols;

#[allow(unused)]
pub async fn save_code_images()  {

    println!("Saving Code Images");

    let ticker = Ticker::builder()
        .ticker("AAPL")
        .start_date("2023-01-01")
        .end_date("2024-12-31")
        .interval(Interval::OneDay)
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .build();

    ticker.performance_chart(Some(1000), Some(1200)).await.unwrap()
        .to_png("./public/images/ticker.png", 1200, 1000, 1.0);

    println!("Ticker Performance Chart Saved");

    let portfolio = Portfolio::builder()
        .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
        .benchmark_symbol("^GSPC")
        .start_date("2023-01-01")
        .end_date("2024-12-31")
        .interval(Interval::OneDay)
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .objective_function(ObjectiveFunction::MaxSharpe)
        .build().await.unwrap();

    portfolio.optimization_chart(Some(1000), Some(1200)).unwrap()
        .to_png("./public/images/portfolio.png", 1200, 1000, 1.0);

    println!("Portfolio Optimization Chart Saved");

}

#[allow(unused)]
async fn update_symbols() {
    let tickers = get_symbols(AssetClass::All, Category::All, Exchange::All).await.unwrap();

    let mut map = HashMap::new();
    for ticker in tickers {
        map.insert(ticker.symbol, ticker.name);
    }

    let serialized_data = bincode::serialize(&map).unwrap();

    let mut file = File::create("datalist.bin").unwrap();
    file.write_all(&serialized_data).unwrap();
}

#[tokio::main]
async fn main() {
    //update_symbols().await;
    //save_code_images().await;
}