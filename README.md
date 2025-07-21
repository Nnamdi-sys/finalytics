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

    // Screen for Large-Cap NASDAQ Stocks
    let screener = Screener::builder()
        .quote_type(QuoteType::Equity)
        .add_filter(ScreenerFilter::EqStr(
            ScreenerMetric::Equity(EquityScreener::Exchange),
            Exchange::NASDAQ.as_ref()
        ))
        .sort_by(
            ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
            true
        )
        .size(10)
        .build()
        .await?;

    screener.overview().show()?;
    screener.metrics().await?.show()?;

    // Instantiate a Multiple Ticker Object
    let ticker_symbols = screener.symbols.iter()
        .map(|x| x.as_str()).collect::<Vec<&str>>();

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
    let symbol = ticker_symbols.first().unwrap();
    let ticker = tickers.clone().get_ticker(symbol).await?;
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
[![PyPI Downloads](https://static.pepy.tech/badge/finalytics)](https://pepy.tech/projects/finalytics)


## Installation

```bash
pip install finalytics
```

## Example

View the [documentation](https://nnamdi.quarto.pub/finalytics/) for more information.

```python
from finalytics import Screener, Tickers

# Screen for Large Cap NASDAQ Stocks
screener = Screener(
    quote_type="EQUITY",
    filters=[
        '{"operator": "eq", "operands": ["exchange", "NMS"]}'
    ],
    sort_field="intradaymarketcap",
    sort_descending=True,
    offset=0,
    size=10
)
screener.display()


# Instantiate a Multiple Ticker Object
symbols = screener.symbols()
tickers = Tickers(symbols=symbols,
                  start_date="2023-01-01",
                  end_date="2024-12-31",
                  interval="1d",
                  confidence_level=0.95,
                  risk_free_rate=0.02)

# Generate a Single Ticker Report
ticker = tickers.get_ticker(symbols[0])
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

<h3><a href="https://finalytics.rs/screener">Screener Dashboard</a></h3>

This sample application allows you to screen for securities based on the Finalytics Library.


