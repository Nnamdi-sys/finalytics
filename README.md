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
finalytics = {version = "0.5.0", features = ["kaleido"]}
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
    let ticker = TickerBuilder::new()
        .ticker("AAPL")
        .start_date("2023-01-01")
        .end_date("2023-02-01")
        .interval(Interval::OneDay)
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .build();
    
    // Display the Security Analysis Results
    let performance_stats = ticker.performance_stats().await?;
    println!("{:?}", performance_stats);
    
    let performance_chart = ticker.performance_chart().await?;
    performance_chart.show();

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
    portfolio.performance_chart()?.show();

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
[![Documentation Status](https://readthedocs.org/projects/finalytics/badge/?version=latest)](https://finalytics.readthedocs.io/en/latest/?badge=latest)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)
![Python Version](https://img.shields.io/badge/Python-3.7%20%7C%203.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12-blue)
![PePy](https://static.pepy.tech/personalized-badge/finalytics?period=total&units=international_system&left_color=black&right_color=blue&left_text=Downloads)


## Installation

```bash
pip install finalytics
```

## Documentation

View Library documentation on readthedocs [here](https://finalytics.readthedocs.io/en/latest/)


### Security Analysis

```python
from finalytics import Ticker

ticker = Ticker(symbol="AAPL",
                start="2023-01-01",
                end="2023-10-31",
                interval="1d",
                confidence_level=0.95,
                risk_free_rate=0.02)

print(ticker.performance_stats())
ticker.performance_chart().show()
```

### Portfolio Optimization

```python
from finalytics import Portfolio

portfolio = Portfolio(ticker_symbols=["AAPL", "GOOG", "MSFT", "BTC-USD"], 
                      benchmark_symbol="^GSPC", start_date="2020-01-01", end_date="2022-01-01", interval="1d", 
                      confidence_level=0.95, risk_free_rate=0.02, max_iterations=1000, 
                      objective_function="max_sharpe")

print(portfolio.get_optimization_results())
portfolio.portfolio_chart(chart_type="performance").show()
```



