![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![Crates.io](https://img.shields.io/crates/v/finalytics)](https://crates.io/crates/finalytics)
[![Docs.rs](https://docs.rs/finalytics/badge.svg)](https://docs.rs/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![CodeFactor](https://www.codefactor.io/repository/github/nnamdi-sys/finalytics/badge)](https://www.codefactor.io/repository/github/nnamdi-sys/finalytics)

**Finalytics** is a Rust library designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
finalytics = {version = "0.4.0", features = ["kaleido"]}
```

Or run the following command:

```bash
cargo install finalytics --features kaleido
```

## Documentation

View the library documentation on [docs.rs](https://docs.rs/finalytics/) or visit the [homepage](https://finalytics.rs/)


### Security Analysis

```rust
use std::error::Error;
use finalytics::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Instantiate the Ticker Object
    let ticker = TickerBuilder::new().ticker("AAPL")?
        .start_date("2023-01-01")
        .end_date("2023-02-01")
        .interval(Interval::OneDay)
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .build()?;

    // Fetch Ticker Data
    let quote = ticker.get_quote().await?;
    let stats = ticker.get_ticker_stats().await?;
    let chart = ticker.get_chart().await?;
    let options = ticker.get_options().await?;

    println!("{:?}", quote);
    println!("{:?}", stats);
    println!("{:?}", chart);
    println!("{:?}", options);

    // Fundamental Analysis
    let income_statement = ticker.income_statement().await?;
    let balance_sheet = ticker.balance_sheet().await?;
    let cash_flow = ticker.cashflow_statement().await?;
    let financial_ratios = ticker.financial_ratios().await?;
    let performance_stats = ticker.performance_stats().await?;
    let volatility_surface = ticker.volatility_surface().await?;

    println!("{:?}", income_statement);
    println!("{:?}", balance_sheet);
    println!("{:?}", cash_flow);
    println!("{:?}", financial_ratios);
    println!("{:?}", performance_stats);
    println!("{:?}", volatility_surface);

    // Technical Analysis
    let sma = ticker.sma(50).await?;
    let ema = ticker.ema(3).await?;
    let macd = ticker.macd(12, 26, 9).await?;
    let rsi = ticker.rsi(14).await?;
    let fs = ticker.fs(14).await?;
    let ss = ticker.ss(7, 3).await?;
    let ppo = ticker.ppo(12, 26, 9).await?;
    let roc = ticker.roc(1).await?;
    let mfi = ticker.mfi(14).await?;
    let bb = ticker.bb(20, 2.0).await?;
    let sd = ticker.sd(20).await?;
    let mad = ticker.mad(20).await?;
    let atr = ticker.atr(14).await?;
    let max = ticker.max(20).await?;
    let min = ticker.min(20).await?;
    let obv = ticker.obv().await?;

    println!("SMA:{:?}\nEMA:{:?}\nMACD:{:?}\nRSI:{:?}\nFS:{:?}\nSS:{:?}\nPPO:{:?}\nROC:{:?}\nMFI:{:?}\
            \nBB:{:?}\nSD:{:?}\nMAD:{:?}\nATR:{:?}\nMAX:{:?}\nMIN:{:?}\nOBV:{:?}\n",
             sma, ema, macd, rsi, fs, ss, ppo, roc, mfi, bb, sd, mad, atr, max, min, obv);

    // News Sentiment Analysis
    let news_sentiment = ticker.get_news(true).await?;
    println!("{:?}", news_sentiment);

    // Display Ticker Charts
    let candlestick_chart = ticker.candlestick_chart().await?;
    let performance_chart = ticker.performance_chart().await?;
    let volatility_charts = ticker.volatility_charts().await?;
    let summary_stats_table = ticker.summary_stats_table().await?;
    let performance_stats_table = ticker.performance_stats_table().await?;
    let financials_tables = ticker.financials_tables().await?;

    candlestick_chart.show();
    performance_chart.show();
    summary_stats_table.show();
    performance_stats_table.show();
    volatility_charts["Volatility Surface"].show();
    volatility_charts["Volatility Smile"].show();
    volatility_charts["Volatility Term Structure"].show();
    financials_tables["Income Statement"].show();
    financials_tables["Balance Sheet"].show();
    financials_tables["Cashflow Statement"].show();
    financials_tables["Financial Ratios"].show();

    Ok(())

}
```

### Portfolio Optimization

```rust
use std::error::Error;
use finalytics::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Construct the Portfolio Object
    let ticker_symbols = Vec::from(["NVDA", "BRK-A", "AAPL", "ZN=F"]);
    let portfolio = PortfolioBuilder::new().ticker_symbols(ticker_symbols)
                                            .benchmark_symbol("^GSPC")
                                            .start_date("2017-01-01")
                                            .end_date("2023-01-01")
                                            .interval(Interval::OneDay)
                                            .confidence_level(0.95)
                                            .risk_free_rate(0.02)
                                            .max_iterations(1000)
                                            .objective_function(ObjectiveFunction::MaxSharpe)
                                            .build().await?;

    // Display Portfolio Optimization Results
    println!("{:?}", portfolio.performance_stats);

   // Display Portfolio Analytics Charts
    portfolio.optimization_chart()?.show();
    portfolio.performance_chart()?.show();
    portfolio.asset_returns_chart()?.show();
    portfolio.performance_stats_table()?.show();

    Ok(())
}
```

## Sample Applications

<h3><a href="https://finalytics.rs/ticker">Ticker Charts Viewer</a></h3>

This sample application allows you to perform security analysis based on the Finalytics Library.

<h3><a href="https://finalytics.rs/portfolio">Portfolio Charts Viewer</a></h3>

This sample application enables you to perform portfolio optimization based on the Finalytics Library.

<h3><a href="https://t.me/finalytics_bot">Telegram Bot</a></h3>
The Finalytics Telegram Bot allows you to perform security analysis, portfolio optimization and news sentiment analysis directly from Telegram.


## Python Binding

[![pypi](https://img.shields.io/pypi/v/finalytics)](https://pypi.org/project/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![Documentation Status](https://readthedocs.org/projects/finalytics-py/badge/?version=latest)](https://finalytics-py.readthedocs.io/en/latest/?badge=latest)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)
![Python Version](https://img.shields.io/badge/Python-3.7%20%7C%203.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12-blue)
![PePy](https://static.pepy.tech/personalized-badge/finalytics?period=total&units=international_system&left_color=black&right_color=blue&left_text=Downloads)
[![CodeFactor](https://www.codefactor.io/repository/github/nnamdi-sys/finalytics-py/badge)](https://www.codefactor.io/repository/github/nnamdi-sys/finalytics-py)


## Installation

```bash
pip install finalytics
```

## Documentation

View Library documentation on readthedocs [here](https://finalytics-py.readthedocs.io/en/latest/)


### Security Analysis

```python
from finalytics import Ticker

ticker = Ticker(symbol="AAPL")
print(ticker.get_current_price())
print(ticker.get_summary_stats())
print(ticker.get_price_history(start="2023-01-01", end="2023-10-31", interval="1d"))
print(ticker.get_options_chain())
print(ticker.get_news(compute_sentiment=True))
print(ticker.get_income_statement())
print(ticker.get_balance_sheet())
print(ticker.get_cashflow_statement())
print(ticker.get_financial_ratios())
print(ticker.compute_performance_stats(start="2023-01-01", end="2023-10-31", interval="1d", benchmark="^GSPC", 
                                       confidence_level=0.95, risk_free_rate=0.02))
ticker.display_performance_chart(start="2023-01-01", end="2023-10-31", interval="1d", benchmark="^GSPC", 
                                 confidence_level=0.95, risk_free_rate=0.02, display_format="notebook")
ticker.display_candlestick_chart(start="2023-01-01", end="2023-10-31", interval="1d", display_format="html")
ticker.display_options_chart(risk_free_rate=0.02, chart_type="surface", display_format="png")
```

### Portfolio Optimization

```python
from finalytics import Portfolio

portfolio = Portfolio(ticker_symbols=["AAPL", "GOOG", "MSFT", "BTC-USD"], 
                      benchmark_symbol="^GSPC", start_date="2020-01-01", end_date="2022-01-01", interval="1d", 
                      confidence_level=0.95, risk_free_rate=0.02, max_iterations=1000, 
                      objective_function="max_sharpe")
print(portfolio.get_optimization_results())
portfolio.display_portfolio_charts("performance", "html")
```



