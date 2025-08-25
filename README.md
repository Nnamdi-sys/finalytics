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
cargo add finalytics
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
    let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None, None).await?;

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
![Python Version](https://img.shields.io/badge/Python-3.9%20%7C%203.10%20%7C%203.11%20%7C%203.12%20%7C%203.13-blue)
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

## Go Binding

[![Go Reference](https://pkg.go.dev/badge/github.com/Nnamdi-sys/finalytics/go/finalytics.svg)](https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)

This is a Go binding for the [Finalytics Rust Library](https://github.com/Nnamdi-sys/finalytics), designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

To install the Finalytics Go binding, add it to your Go project using:

```bash
go get github.com/Nnamdi-sys/finalytics/go/finalytics
```

After installing the Go module, **download the required native binaries** by running:
```bash
curl -O https://raw.githubusercontent.com/Nnamdi-sys/finalytics/refs/heads/main/go/download_binaries.sh
bash download_binaries.sh
```

## Example

View the [Go documentation](https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics) for more information.
You can also check the [`main.go` file](https://github.com/Nnamdi-sys/finalytics/blob/main/go/main.go) for more usage examples.

```go
package main

import (
    "fmt"
    "github.com/Nnamdi-sys/finalytics/go/finalytics"
)

func main() {
    // Screen for Large Cap NASDAQ Stocks
    screener, err := finalytics.NewScreener(
        "EQUITY",
        []string{
            `{"operator":"eq","operands":["exchange","NMS"]}`,
            `{"operator":"gte","operands":["intradaymarketcap",10000000000]}`,
        },
        "intradaymarketcap",
        true,
        0,
        10,
    )
    if err != nil {
        fmt.Printf("Error creating Screener: %v\n", err)
        return
    }
    defer screener.Free()

    // Get screened symbols
    symbols, err := screener.Symbols()
    if err != nil {
        fmt.Printf("Failed to get symbols: %v\n", err)
        return
    }
    fmt.Printf("Screened Symbols: %v\n", symbols)

    tickers, err := finalytics.NewTickersBuilder().
        Symbols(symbols).
        StartDate("2023-01-01").
        EndDate("2024-12-31").
        Interval("1d").
        BenchmarkSymbol("^GSPC").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        Build()
    if err != nil {
        fmt.Printf("Failed to create Tickers: %v\n", err)
        return
    }
    defer tickers.Free()

    // Generate a Single Ticker Report
    if len(symbols) > 0 {
        ticker, err := tickers.GetTicker(symbols[0])
        if err != nil {
            fmt.Printf("Failed to get Ticker: %v\n", err)
            return
        }
        defer ticker.Free()

        for _, reportType := range []string{"performance", "financials", "options", "news"} {
            report, err := ticker.Report(reportType)
            if err != nil {
                fmt.Printf("Failed to get %s report: %v\n", reportType, err)
                continue
            }
            report.Show()
        }
    }

    // Generate a Multiple Ticker Report
    tickersReport, err := tickers.Report("performance")
    if err != nil {
        fmt.Printf("Failed to get Tickers report: %v\n", err)
        return
    }
    tickersReport.Show()

    // Perform Portfolio Optimization
    portfolio, err := tickers.Optimize("max_sharpe", "{}", "{}", "{}")
    if err != nil {
        fmt.Printf("Failed to optimize portfolio: %v\n", err)
        return
    }
    defer portfolio.Free()

    // Generate a Portfolio Report
    portfolioReport, err := portfolio.Report("performance")
    if err != nil {
        fmt.Printf("Failed to get Portfolio report: %v\n", err)
        return
    }
    portfolioReport.Show()
}
```


## Node.js Binding

[![NPM Version](https://img.shields.io/npm/v/finalytics)](https://www.npmjs.com/package/finalytics)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)

This is a JavaScript (ESM) binding for the [Finalytics Rust Library](https://github.com/Nnamdi-sys/finalytics), designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

To install the Finalytics JavaScript binding, add it to your Node.js project using:

```bash
npm install finalytics
```

## Example

View the [npm package documentation](https://www.npmjs.com/package/finalytics) for more information. You can also check the [index.js file](https://github.com/Nnamdi-sys/finalytics/blob/main/js/index.js) for more usage examples.

```javascript
import { Screener, TickersBuilder } from 'finalytics';

async function main() {
  console.log('=== Finalytics Example ===');

  let screener;
  try {
    screener = await Screener.new(
      'EQUITY',
      [
        JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] }),
        JSON.stringify({ operator: 'gte', operands: ['intradaymarketcap', 10000000000] }),
      ],
      'intradaymarketcap',
      true,
      0,
      10
    );
  } catch (err) {
    console.error('Error creating Screener:', err.message);
    return;
  }

  let symbols;
  try {
    symbols = await screener.symbols();
    console.log('Screened Symbols:', symbols);
  } catch (err) {
    console.error('Failed to get symbols:', err.message);
    screener.free();
    return;
  }

  let tickers;
  try {
    tickers = await new TickersBuilder()
      .symbols(symbols)
      .startDate('2023-01-01')
      .endDate('2024-12-31')
      .interval('1d')
      .benchmarkSymbol('^GSPC')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .build();
  } catch (err) {
    console.error('Failed to create Tickers:', err.message);
    screener.free();
    return;
  }

  if (symbols.length > 0) {
    let ticker;
    try {
      ticker = await tickers.getTicker(symbols[0]);
      for (const reportType of ['performance', 'financials', 'options', 'news']) {
        try {
          const report = await ticker.report(reportType);
          console.log(`Ticker ${reportType} report: Opening in browser...`);
          await report.show();
        } catch (err) {
          console.error(`Failed to get ${reportType} report:`, err.message);
        }
      }
    } catch (err) {
      console.error('Failed to get Ticker:', err.message);
    } finally {
      if (ticker) ticker.free();
    }
  }

  try {
    const tickersReport = await tickers.report('performance');
    console.log('Tickers report: Opening in browser...');
    await tickersReport.show();
  } catch (err) {
    console.error('Failed to get Tickers report:', err.message);
  }

  let portfolio;
  try {
    portfolio = await tickers.optimize('max_sharpe', '{}', '{}', '{}');
    const portfolioReport = await portfolio.report('performance');
    console.log('Portfolio report: Opening in browser...');
    await portfolioReport.show();
  } catch (err) {
    console.error('Failed to optimize portfolio or get report:', err.message);
  } finally {
    if (portfolio) portfolio.free();
  }

  tickers.free();
  screener.free();
}

main();
```


## Web Application

The Finalytics web application integrates the Ticker, Portfolio, and Screener dashboards, built with the Finalytics Rust Library using the [Dioxus Web Framework](https://dioxuslabs.com/). It allows users to perform security analysis, portfolio optimization, and screen for securities, all accessible at [finalytics.rs](https://finalytics.rs).

### Running Locally

To run the web application locally, follow these steps:

```bash
# Install the Dioxus CLI
cargo install dioxus-cli

# Clone the repository
git clone https://github.com/Nnamdi-sys/finalytics.git

# Navigate to the web directory
cd finalytics/web

# Serve the application
dx serve --platform web
```
