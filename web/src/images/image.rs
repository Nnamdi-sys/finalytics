use finalytics::prelude::*;

#[allow(unused)]
pub async fn save_code_images()  {

    println!("Saving Code Images");

    let ticker = TickerBuilder::new()
        .ticker("AAPL")
        .start_date("2023-01-01")
        .end_date("2024-12-31")
        .interval(Interval::OneDay)
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .build();

    let _ = ticker.performance_chart(Some(800), Some(1200)).await.unwrap()
        .to_png("ticker.png", 900, 800, 1.0);

    println!("Ticker Performance Chart Saved");

    let portfolio = PortfolioBuilder::new()
        .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
        .benchmark_symbol("^GSPC")
        .start_date("2023-01-01")
        .end_date("2024-12-31")
        .interval(Interval::OneDay)
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .objective_function(ObjectiveFunction::MaxSharpe)
        .build().await.unwrap();

    let _ = portfolio.optimization_chart(Some(800), Some(1200)).unwrap()
        .to_png("portfolio.png", 900, 800, 1.0);

    println!("Portfolio Optimization Chart Saved");

}

#[tokio::main]
async fn main() {
    save_code_images().await;
}