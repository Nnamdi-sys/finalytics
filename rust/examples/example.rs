// Finalytics — Rust Examples
//
// Installation
// ────────────
// Add to your Cargo.toml:
//
//   [dependencies]
//   finalytics = "*"
//
// Or run:
//
//   cargo add finalytics
//
// Full docs: https://docs.rs/finalytics/
//
// Run this example (from the repo root)
// ───────────────────────────────────────
//   bash examples/example.sh rust
//
// (The script copies this file to rust/examples/ and runs:
//   cd rust && cargo run --example example)

use finalytics::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    screener().await?;
    ticker().await?;
    tickers().await?;
    portfolio_optimization_oos().await?;
    portfolio_optimization_constraints().await?;
    portfolio_allocation_rebalancing_dca().await?;
    custom_data().await?;
    Ok(())
}

// ── 1. Screener — Large-Cap NASDAQ Technology Stocks with ROE >= 15% ────────

async fn screener() -> Result<(), Box<dyn Error>> {
    println!("=== 1. Screener ===");

    let screener = Screener::builder()
        .quote_type(QuoteType::Equity)
        .add_filter(ScreenerFilter::EqStr(
            ScreenerMetric::Equity(EquityScreener::Exchange),
            Exchange::NASDAQ.as_ref(),
        ))
        .add_filter(ScreenerFilter::EqStr(
            ScreenerMetric::Equity(EquityScreener::Sector),
            Sector::Technology.as_ref(),
        ))
        .add_filter(ScreenerFilter::Gte(
            ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
            10_000_000_000.0,
        ))
        .add_filter(ScreenerFilter::Gte(
            ScreenerMetric::Equity(EquityScreener::ReturnOnEquity),
            0.15,
        ))
        .sort_by(
            ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
            true,
        )
        .size(10)
        .build()
        .await?;

    screener.overview().show()?;
    screener.metrics().await?.show()?;

    Ok(())
}

// ── 2. Ticker — Single security analysis with all report types ───────────────

async fn ticker() -> Result<(), Box<dyn Error>> {
    println!("=== 2. Ticker ===");

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

    Ok(())
}

// ── 3. Tickers — Multiple securities analysis ────────────────────────────────

async fn tickers() -> Result<(), Box<dyn Error>> {
    println!("=== 3. Tickers ===");

    let tickers = Tickers::builder()
        .tickers(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
        .start_date("2023-01-01")
        .end_date("2024-12-31")
        .interval(Interval::OneDay)
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .build();

    tickers
        .report(Some(ReportType::Performance))
        .await?
        .show()?;

    Ok(())
}

// ── 4. Portfolio — Optimization with Out-of-Sample Evaluation ───────────────

// Optimize on 2023-2024 data (in-sample)
async fn portfolio_optimization_oos() -> Result<(), Box<dyn Error>> {
    println!("=== 4. Portfolio — Optimization with Out-of-Sample Evaluation ===");

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
        .await?;

    portfolio.optimize()?;
    portfolio
        .report(Some(ReportType::Optimization))
        .await?
        .show()?;

    // Update to 2025 data for out-of-sample evaluation
    portfolio.update_dates("2025-01-01", "2026-01-01").await?;
    portfolio.performance_stats()?;
    portfolio
        .report(Some(ReportType::Performance))
        .await?
        .show()?;

    Ok(())
}

// ── 5. Portfolio — Optimization with Weight & Categorical Constraints ─────────

async fn portfolio_optimization_constraints() -> Result<(), Box<dyn Error>> {
    println!("=== 5. Portfolio — Optimization with Weight & Categorical Constraints ===");

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
            // ── Sector constraint ──────────────────────────────────────
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
            // ── Asset-class constraint ─────────────────────────────────
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
        .build()
        .await?;

    portfolio.optimize()?;
    portfolio
        .report(Some(ReportType::Optimization))
        .await?
        .show()?;

    Ok(())
}

// ── 6. Portfolio — Explicit Allocation with Rebalancing and DCA ─────────────

async fn portfolio_allocation_rebalancing_dca() -> Result<(), Box<dyn Error>> {
    println!("=== 6. Portfolio — Explicit Allocation with Rebalancing and DCA ===");

    let mut portfolio_alloc = Portfolio::builder()
        .ticker_symbols(vec!["AAPL", "MSFT", "NVDA", "BTC-USD"])
        .benchmark_symbol("^GSPC")
        .start_date("2023-01-01")
        .end_date("2024-12-31")
        .interval(Interval::OneDay)
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .weights(vec![25_000.0, 25_000.0, 25_000.0, 25_000.0])
        .rebalance_strategy(Some(RebalanceStrategy::Calendar(
            ScheduleFrequency::Quarterly,
        )))
        .scheduled_cash_flows(Some(vec![ScheduledCashFlow {
            amount: 2_000.0,
            frequency: ScheduleFrequency::Monthly,
            start_date: None,
            end_date: None,
            allocation: CashFlowAllocation::ProRata,
        }]))
        .build()
        .await?;

    portfolio_alloc.performance_stats()?;
    portfolio_alloc
        .report(Some(ReportType::Performance))
        .await?
        .show()?;

    Ok(())
}

// ── 7. Custom Data (KLINE) — Load CSV data and use with Ticker, Tickers, Portfolio ──

async fn custom_data() -> Result<(), Box<dyn Error>> {
    println!("=== 7. Custom Data (KLINE) ===");

    // Load data from CSV files
    let aapl = KLINE::from_csv("AAPL", "examples/datasets/aapl.csv")?;
    let msft = KLINE::from_csv("MSFT", "examples/datasets/msft.csv")?;
    let nvda = KLINE::from_csv("NVDA", "examples/datasets/nvda.csv")?;
    let goog = KLINE::from_csv("GOOG", "examples/datasets/goog.csv")?;
    let btcusd = KLINE::from_csv("BTC-USD", "examples/datasets/btcusd.csv")?;
    let gspc = KLINE::from_csv("^GSPC", "examples/datasets/gspc.csv")?;

    // Single Ticker from custom data
    println!("--- Custom Ticker ---");
    let custom_ticker = Ticker::builder()
        .ticker("AAPL")
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .ticker_data(Some(aapl.clone()))
        .benchmark_data(Some(gspc.clone()))
        .build();

    custom_ticker
        .report(Some(ReportType::Performance))
        .await?
        .show()?;

    // Multiple Tickers from custom data
    println!("--- Custom Tickers ---");
    let custom_tickers = Tickers::builder()
        .tickers(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .tickers_data(Some(vec![
            nvda.clone(),
            goog.clone(),
            aapl.clone(),
            msft.clone(),
            btcusd.clone(),
        ]))
        .benchmark_data(Some(gspc.clone()))
        .build();

    custom_tickers
        .report(Some(ReportType::Performance))
        .await?
        .show()?;

    // Portfolio optimization from custom data
    println!("--- Custom Portfolio ---");
    let mut custom_portfolio = Portfolio::builder()
        .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .objective_function(ObjectiveFunction::MaxSharpe)
        .tickers_data(Some(vec![nvda, goog, aapl, msft, btcusd]))
        .benchmark_data(Some(gspc))
        .build()
        .await?;

    custom_portfolio.optimize()?;
    custom_portfolio
        .report(Some(ReportType::Optimization))
        .await?
        .show()?;

    Ok(())
}
