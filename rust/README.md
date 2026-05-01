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

Finalytics is organized around five core modules, each designed for a specific aspect of financial analytics:

### 1. Screener

Efficiently filter and rank securities using advanced metrics and custom filters.

**Usage Example:**
```rust
use finalytics::prelude::*;

let screener = Screener::builder()
    .quote_type(QuoteType::Equity)
    .add_filter(ScreenerFilter::EqStr(
        ScreenerMetric::Equity(EquityScreener::Exchange),
        Exchange::NASDAQ.as_ref()
    ))
    .add_filter(ScreenerFilter::EqStr(
        ScreenerMetric::Equity(EquityScreener::Sector),
        Sector::Technology.as_ref()
    ))
    .add_filter(ScreenerFilter::Gte(
        ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
        10_000_000_000.0
    ))
    .add_filter(ScreenerFilter::Gte(
        ScreenerMetric::Equity(EquityScreener::ReturnOnEquity),
        0.15
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
let tickers = Tickers::builder()
    .tickers(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
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
Supports rebalancing strategies, scheduled cash flows (DCA), ad-hoc transactions,
and out-of-sample evaluation.

**Objective Functions:**
`MaxSharpe`, `MaxSortino`, `MaxReturn`, `MinVol`, `MinVar`, `MinCVaR`, `MinDrawdown`,
`RiskParity`, `MaxDiversification`, `HierarchicalRiskParity`

**Usage Example: Optimization with Out-of-Sample Evaluation**
```rust
// Optimize on 2023 - 2024 data (in-sample)
let mut portfolio = Portfolio::builder()
    .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
    .benchmark_symbol("^GSPC")
    .start_date("2023-01-01")
    .end_date("2024-12-31")
    .interval(Interval::OneDay)
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .objective_function(ObjectiveFunction::MaxSharpe)
    .build().await?;

portfolio.optimize()?;
portfolio.report(Some(ReportType::Optimization)).await?.show()?;

// Update to 2025 data for out-of-sample evaluation
portfolio.update_dates("2025-01-01", "2026-01-01").await?;
portfolio.performance_stats()?;
portfolio.report(Some(ReportType::Performance)).await?.show()?;
```

**Usage Example: Explicit Allocation with Rebalancing and DCA**
```rust
let mut portfolio = Portfolio::builder()
    .ticker_symbols(vec!["AAPL", "MSFT", "NVDA", "BTC-USD"])
    .benchmark_symbol("^GSPC")
    .start_date("2023-01-01")
    .end_date("2024-12-31")
    .interval(Interval::OneDay)
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .weights(vec![25_000.0, 25_000.0, 25_000.0, 25_000.0])
    .rebalance_strategy(Some(RebalanceStrategy::Calendar(ScheduleFrequency::Quarterly)))
    .scheduled_cash_flows(Some(vec![
        ScheduledCashFlow {
            amount: 2_000.0,
            frequency: ScheduleFrequency::Monthly,
            start_date: None,
            end_date: None,
            allocation: CashFlowAllocation::ProRata,
        }
    ]))
    .build().await?;

portfolio.performance_stats()?;
portfolio.report(Some(ReportType::Performance)).await?.show()?;
```

**Usage Example: Optimization with Weight & Categorical Constraints**
```rust
use finalytics::prelude::*;

let constraints = Constraints {
    // Per-asset (lower_bound, upper_bound) in the same order as ticker_symbols.
    asset_weights: Some(vec![
        (0.05, 0.40), // AAPL
        (0.05, 0.40), // MSFT
        (0.05, 0.40), // NVDA
        (0.05, 0.30), // JPM
        (0.05, 0.20), // XOM
        (0.05, 0.25), // BTC-USD
    ]),
    categorical_weights: Some(vec![
        CategoricalWeights {
            name: "Sector".to_string(),
            category_per_symbol: vec![
                "Tech".to_string(),    // AAPL
                "Tech".to_string(),    // MSFT
                "Tech".to_string(),    // NVDA
                "Finance".to_string(), // JPM
                "Energy".to_string(),  // XOM
                "Crypto".to_string(),  // BTC-USD
            ],
            weight_per_category: vec![
                ("Tech".to_string(),    0.30, 0.60),
                ("Finance".to_string(), 0.05, 0.30),
                ("Energy".to_string(),  0.05, 0.20),
                ("Crypto".to_string(),  0.05, 0.25),
            ],
        },
        CategoricalWeights {
            name: "Asset Class".to_string(),
            category_per_symbol: vec![
                "Equity".to_string(), // AAPL
                "Equity".to_string(), // MSFT
                "Equity".to_string(), // NVDA
                "Equity".to_string(), // JPM
                "Equity".to_string(), // XOM
                "Crypto".to_string(), // BTC-USD
            ],
            weight_per_category: vec![
                ("Equity".to_string(), 0.70, 0.95),
                ("Crypto".to_string(), 0.05, 0.30),
            ],
        },
    ]),
};

let mut portfolio = Portfolio::builder()
    .ticker_symbols(vec!["AAPL", "MSFT", "NVDA", "JPM", "XOM", "BTC-USD"])
    .benchmark_symbol("^GSPC")
    .start_date("2023-01-01")
    .end_date("2024-12-31")
    .interval(Interval::OneDay)
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .objective_function(ObjectiveFunction::MaxSharpe)
    .constraints(Some(constraints))
    .build().await?;

portfolio.optimize()?;
portfolio.report(Some(ReportType::Optimization)).await?.show()?;
```

---

### 5. Custom Data (KLINE)

Load your own price data from CSV, JSON, or DataFrames and use it with any Finalytics module.
The `KLINE` struct expects columns: `timestamp` (unix epoch i64), `open`, `high`, `low`, `close`, `volume`, `adjclose`.

**Usage Example:**
```rust
// Load data from CSV files
let aapl = KLINE::from_csv("AAPL", "examples/datasets/aapl.csv")?;
let msft = KLINE::from_csv("MSFT", "examples/datasets/msft.csv")?;
let nvda = KLINE::from_csv("NVDA", "examples/datasets/nvda.csv")?;
let goog = KLINE::from_csv("GOOG", "examples/datasets/goog.csv")?;
let btcusd = KLINE::from_csv("BTC-USD", "examples/datasets/btcusd.csv")?;
let gspc = KLINE::from_csv("^GSPC", "examples/datasets/gspc.csv")?;

// Also available: KLINE::from_json("AAPL", "aapl.json")?
// Also available: KLINE::from_dataframe("AAPL", &polars_df)?

// Single Ticker from custom data
let ticker = Ticker::builder()
    .ticker("AAPL")
    .benchmark_symbol("^GSPC")
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .ticker_data(Some(aapl.clone()))
    .benchmark_data(Some(gspc.clone()))
    .build();

ticker.report(Some(ReportType::Performance)).await?.show()?;

// Multiple Tickers from custom data
let tickers = Tickers::builder()
    .tickers(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
    .benchmark_symbol("^GSPC")
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .tickers_data(Some(vec![nvda, goog, aapl, msft, btcusd]))
    .benchmark_data(Some(gspc.clone()))
    .build();

tickers.report(Some(ReportType::Performance)).await?.show()?;

// Portfolio optimization from custom data
let mut portfolio = Portfolio::builder()
    .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
    .benchmark_symbol("^GSPC")
    .confidence_level(0.95)
    .risk_free_rate(0.02)
    .objective_function(ObjectiveFunction::MaxSharpe)
    .tickers_data(Some(vec![
        KLINE::from_csv("NVDA", "examples/datasets/nvda.csv")?,
        KLINE::from_csv("GOOG", "examples/datasets/goog.csv")?,
        KLINE::from_csv("AAPL", "examples/datasets/aapl.csv")?,
        KLINE::from_csv("MSFT", "examples/datasets/msft.csv")?,
        KLINE::from_csv("BTC-USD", "examples/datasets/btcusd.csv")?,
    ]))
    .benchmark_data(Some(KLINE::from_csv("^GSPC", "examples/datasets/gspc.csv")?))
    .build().await?;

portfolio.optimize()?;
portfolio.report(Some(ReportType::Optimization)).await?.show()?;
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
