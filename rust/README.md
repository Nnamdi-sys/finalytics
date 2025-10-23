![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![Crates.io](https://img.shields.io/crates/v/finalytics)](https://crates.io/crates/finalytics)
[![Docs.rs](https://docs.rs/finalytics/badge.svg)](https://docs.rs/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![Crates.io](https://img.shields.io/crates/d/finalytics)](https://crates.io/crates/finalytics)

---

# Finalytics Rust Library

**Finalytics** is a modular, high-performance Rust library for retrieving financial data, performing security analysis, and optimizing portfolios.
It is designed for extensibility and speed, and powers bindings for Python, Node.js, Go, and a web application built with Dioxus.

---

## 🚀 Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
finalytics = "*"
```

Or run:

```bash
cargo add finalytics
```

---

## 🦀 Main Modules

Finalytics is organized around four core modules, each designed for a specific aspect of financial analytics:

### 1. Screener

Efficiently filter and rank securities (equities, crypto, etc.) using advanced metrics and custom filters.

**Usage Example:**
```rust
use finalytics::prelude::*;

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
```

---

### 2. Ticker

Analyze a single security in depth: performance, financials, options, news, and more.

**Usage Example:**
```rust
let ticker = Ticker::builder()
    .ticker("AAPL")
    .start_date("2023-01-01")
    .end_date("2024-12-31")
    .interval(Interval::OneDay)
    .benchmark_symbol("^GSPC")
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .build();

ticker.report(Some(ReportType::Performance)).await?.show()?;
ticker.report(Some(ReportType::Financials)).await?.show()?;
ticker.report(Some(ReportType::Options)).await?.show()?;
ticker.report(Some(ReportType::News)).await?.show()?;
```

---

### 3. Tickers

Work with multiple securities at once—aggregate reports, batch analytics, and portfolio construction.

**Usage Example:**
```rust
let tickers = TickersBuilder::new()
    .tickers(vec!["AAPL", "MSFT", "GOOG"])
    .start_date("2023-01-01")
    .end_date("2024-12-31")
    .interval(Interval::OneDay)
    .benchmark_symbol("^GSPC")
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .build();

tickers.report(Some(ReportType::Performance)).await?.show()?;
```

---

### 4. Portfolio

Optimize and analyze portfolios using advanced objective functions and constraints.

**Usage Example:**
```rust
let ticker_symbols = vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"];

let portfolio = Portfolio::builder()
    .ticker_symbols(ticker_symbols)
    .benchmark_symbol("^GSPC")
    .start_date("2023-01-01")
    .end_date("2024-12-31")
    .interval(Interval::OneDay)
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .objective_function(ObjectiveFunction::MaxSharpe)
    .build().await?;

portfolio.report(Some(ReportType::Performance)).await?.show()?;
```

---

## 📚 Documentation

- See the [API documentation](https://docs.rs/finalytics/) for full details.

---

## 🗂️ Multi-language Bindings

Finalytics is also available in:
- [Python](../python/README.md)
- [Node.js](../js/README.md)
- [Go](../go/README.md)
- [Web Application](../web/README.md)

---

**Finalytics** — Modular, high-performance financial analytics in Rust.
