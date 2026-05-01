use finalytics::prelude::*;

#[allow(unused)]
pub async fn save_code_images() {
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

    ticker
        .performance_chart(None, None)
        .await
        .unwrap()
        .write_html("./public/html/ticker.html");

    println!("Ticker Performance Chart Saved");

    let mut portfolio = Portfolio::builder()
        .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
        .benchmark_symbol("^GSPC")
        .start_date("2023-01-01")
        .end_date("2024-12-31")
        .interval(Interval::OneDay)
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .objective_function(ObjectiveFunction::MaxSharpe)
        .build()
        .await
        .unwrap();

    portfolio.optimize().unwrap();

    portfolio
        .optimization_chart(None, None)
        .unwrap()
        .write_html("./public/html/portfolio.html");

    println!("Portfolio Optimization Chart Saved");
}

#[tokio::main]
async fn main() {
    //save_code_images().await;
}
