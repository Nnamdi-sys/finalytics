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
    let ticker_symbols = Vec::from(["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"]);

    let tickers = TickersBuilder::new()
        .tickers(ticker_symbols)
        .start_date("2020-01-01")
        .end_date("2024-01-01")
        .interval(Interval::OneDay)
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .build();

    // Calculate the Performance Statistics of all the Tickers
    let performance_stats = tickers.performance_stats().await?;
    println!("{:?}", performance_stats);

    // Display the Security Analysis Charts
    tickers.returns_chart(None, None).await?.show();
    tickers.returns_matrix(None, None).await?.show();
    
    // Perform a Portfolio Optimization
    let portfolio = tickers.optimize_portfolio(ObjectiveFunction::MaxSharpe).await?;
    println!("{:?}", portfolio.performance_stats);

    // Display the Portfolio Optimization Charts
    portfolio.performance_stats_table(None, None)?.show();
    portfolio.optimization_chart(None, None)?.show();
    portfolio.performance_chart(None, None)?.show();
    portfolio.asset_returns_chart(None, None)?.show();

    Ok(())
}
```


## Python Binding

[![pypi](https://img.shields.io/pypi/v/finalytics)](https://pypi.org/project/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![Documentation Status](https://img.shields.io/badge/docs-quarto-blue)](https://nnamdi.quarto.pub/finalytics/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)
![Python Version](https://img.shields.io/badge/Python-3.7%20%7C%203.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12-blue)
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
                  start_date="2020-01-01",
                  end_date="2024-01-01",
                  interval="1d",
                  confidence_level=0.95,
                  risk_free_rate=0.02)

# Calculate the Performance Statistics of all the Tickers
print(tickers.performance_stats())

# Display the Security Analysis Charts
tickers.returns_chart().show()
tickers.returns_matrix().show()

# Perform Portfolio Optimization
portfolio = tickers.optimize()
print(portfolio.optimization_results())

# Display the Portfolio Optimization Charts
portfolio.optimization_chart().show()
portfolio.performance_chart().show()
portfolio.asset_returns_chart().show()
portfolio.performance_stats_table().show()

```


## Sample Applications

<h3><a href="https://finalytics.rs/ticker">Ticker Charts Viewer</a></h3>

This sample application allows you to perform security analysis based on the Finalytics Library.

<h3><a href="https://finalytics.rs/portfolio">Portfolio Charts Viewer</a></h3>

This sample application enables you to perform portfolio optimization based on the Finalytics Library.


