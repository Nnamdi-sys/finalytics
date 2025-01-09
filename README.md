![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![Crates.io](https://img.shields.io/crates/v/finalytics)](https://crates.io/crates/finalytics)
[![Docs.rs](https://docs.rs/finalytics/badge.svg)](https://docs.rs/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![CodeFactor](https://www.codefactor.io/repository/github/nnamdi-sys/finalytics/badge)](https://www.codefactor.io/repository/github/nnamdi-sys/finalytics)
[![Crates.io](https://img.shields.io/crates/d/finalytics)](https://crates.io/crates/finalytics)


**Finalytics** is a Rust library designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
finalytics = "*"
```

Or run the following command:

```bash
cargo install finalytics
```

## Example

View the [documentation](https://docs.rs/finalytics/) for more information.

```rust
use std::error::Error;
use finalytics::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Instantiate a Multiple Ticker Object
    let ticker_symbols = vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"];

    let tickers = TickersBuilder::new()
        .tickers(ticker_symbols.clone())
        .start_date("2023-01-01")
        .end_date("2024-12-31")
        .interval(Interval::OneDay)
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .build();

    // Generate a Single Ticker Report
    let ticker = tickers.clone().get_ticker("AAPL").await?;
    ticker.report(Some(ReportType::Performance)).await?.show()?;
    ticker.report(Some(ReportType::Financials)).await?.show()?;
    ticker.report(Some(ReportType::Options)).await?.show()?;
    ticker.report(Some(ReportType::News)).await?.show()?;

    // Generate a Multiple Ticker Report
    tickers.report(Some(ReportType::Performance)).await?.show()?;

    // Perform a Portfolio Optimization
    let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None).await?;

    // Generate a Portfolio Report
    portfolio.report(Some(ReportType::Performance)).await?.show()?;

    Ok(())
}
```


## Python Binding

[![pypi](https://img.shields.io/pypi/v/finalytics)](https://pypi.org/project/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![Documentation Status](https://img.shields.io/badge/docs-quarto-blue)](https://nnamdi.quarto.pub/finalytics/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)
![Python Version](https://img.shields.io/badge/Python-3.7%20%7C%203.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12%20%7C%203.13-blue)
![PePy](https://static.pepy.tech/personalized-badge/finalytics?period=total&units=international_system&left_color=black&right_color=blue&left_text=Downloads)


## Installation

```bash
pip install finalytics
```

## Example

View the [documentation](https://nnamdi.quarto.pub/finalytics/) for more information.

```python
from finalytics import Tickers

# Instantiate a Multiple Ticker Object
tickers = Tickers(symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
                  start_date="2023-01-01",
                  end_date="2024-12-31",
                  interval="1d",
                  confidence_level=0.95,
                  risk_free_rate=0.02)

# Generate a Single Ticker Report
ticker = tickers.get_ticker("AAPL")
ticker.report("performance")
ticker.report("financials")
ticker.report("options")
ticker.report("news")

# Generate a Multiple Ticker Report
tickers.report("performance")

# Perform a Portfolio Optimization
portfolio = tickers.optimize(objective_function="max_sharpe")

# Generate a Portfolio Report
portfolio.report("performance")

```


## Sample Applications

<h3><a href="https://finalytics.rs/ticker">Ticker Dashboard</a></h3>

This sample application allows you to perform security analysis based on the Finalytics Library.

<h3><a href="https://finalytics.rs/portfolio">Portfolio Dashboard</a></h3>

This sample application enables you to perform portfolio optimization based on the Finalytics Library.


