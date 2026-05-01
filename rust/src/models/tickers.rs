use crate::analytics::performance::prepare_portfolio_data;
use crate::prelude::{
    Constraints, Interval, ObjectiveFunction, Portfolio, Ticker, Transaction, KLINE,
};
use std::error::Error;

pub struct TickersBuilder {
    tickers: Vec<String>,
    start_date: String,
    end_date: String,
    interval: Interval,
    benchmark_symbol: Option<String>,
    confidence_level: f64,
    risk_free_rate: f64,
    tickers_data: Option<Vec<KLINE>>,
    benchmark_data: Option<KLINE>,
}

impl Default for TickersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TickersBuilder {
    pub fn new() -> TickersBuilder {
        TickersBuilder {
            tickers: Vec::new(),
            start_date: String::new(),
            end_date: String::new(),
            interval: Interval::OneDay,
            benchmark_symbol: None,
            confidence_level: 0.95,
            risk_free_rate: 0.0,
            tickers_data: None,
            benchmark_data: None,
        }
    }

    pub fn tickers(mut self, tickers: Vec<&str>) -> TickersBuilder {
        self.tickers = tickers.iter().map(|x| x.to_string()).collect();
        self
    }

    pub fn start_date(mut self, start_date: &str) -> TickersBuilder {
        self.start_date = start_date.to_string();
        self
    }

    pub fn end_date(mut self, end_date: &str) -> TickersBuilder {
        self.end_date = end_date.to_string();
        self
    }

    pub fn interval(mut self, interval: Interval) -> TickersBuilder {
        self.interval = interval;
        self
    }

    pub fn benchmark_symbol(mut self, benchmark_symbol: &str) -> TickersBuilder {
        self.benchmark_symbol = Some(benchmark_symbol.to_string());
        self
    }

    pub fn confidence_level(mut self, confidence_level: f64) -> TickersBuilder {
        self.confidence_level = confidence_level;
        self
    }

    pub fn risk_free_rate(mut self, risk_free_rate: f64) -> TickersBuilder {
        self.risk_free_rate = risk_free_rate;
        self
    }

    pub fn tickers_data(mut self, tickers_data: Option<Vec<KLINE>>) -> TickersBuilder {
        self.tickers_data = tickers_data;
        self
    }

    pub fn benchmark_data(mut self, benchmark_data: Option<KLINE>) -> TickersBuilder {
        self.benchmark_data = benchmark_data;
        self
    }

    pub fn build(self) -> Tickers {
        // Build the benchmark ticker only if benchmark info is available
        let benchmark_ticker: Option<Ticker> =
            if let Some(benchmark_data) = self.benchmark_data.clone() {
                Some(
                    Ticker::builder()
                        .ticker_data(Some(benchmark_data.clone()))
                        .confidence_level(self.confidence_level)
                        .risk_free_rate(self.risk_free_rate)
                        .build(),
                )
            } else if let Some(ref sym) = self.benchmark_symbol {
                Some(
                    Ticker::builder()
                        .ticker(sym)
                        .start_date(&self.start_date)
                        .end_date(&self.end_date)
                        .interval(self.interval)
                        .confidence_level(self.confidence_level)
                        .risk_free_rate(self.risk_free_rate)
                        .build(),
                )
            } else {
                None
            };

        let benchmark_data_for_tickers = benchmark_ticker
            .as_ref()
            .and_then(|bt| bt.ticker_data.clone());

        let tickers = if let Some(tickers_data) = self.tickers_data.clone() {
            tickers_data
                .clone()
                .into_iter()
                .map(|x| {
                    let mut builder = Ticker::builder()
                        .ticker_data(Some(x.clone()))
                        .confidence_level(self.confidence_level)
                        .risk_free_rate(self.risk_free_rate);
                    if let Some(ref bd) = benchmark_data_for_tickers {
                        builder = builder.benchmark_data(Some(bd.clone()));
                    }
                    builder.build()
                })
                .collect::<Vec<Ticker>>()
        } else {
            self.tickers
                .clone()
                .into_iter()
                .map(|x| {
                    let mut builder = Ticker::builder()
                        .ticker(&x)
                        .start_date(&self.start_date)
                        .end_date(&self.end_date)
                        .interval(self.interval)
                        .confidence_level(self.confidence_level)
                        .risk_free_rate(self.risk_free_rate);
                    if let Some(ref sym) = self.benchmark_symbol {
                        builder = builder.benchmark_symbol(sym);
                    }
                    builder.build()
                })
                .collect::<Vec<Ticker>>()
        };

        let benchmark_symbol = benchmark_ticker.as_ref().map(|bt| bt.ticker.clone());

        Tickers {
            tickers: tickers.clone(),
            start_date: tickers[0].start_date.clone(),
            end_date: tickers[0].end_date.clone(),
            interval: tickers[0].interval,
            benchmark_symbol,
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
            tickers_data: self.tickers_data,
            benchmark_data: self.benchmark_data,
            benchmark_ticker,
        }
    }
}

impl Tickers {
    pub fn builder() -> TickersBuilder {
        TickersBuilder::new()
    }

    /// Fetch a single Ticker Struct from the Tickers Struct
    ///
    /// ### Arguments
    /// - `symbol` - The Ticker Symbol
    ///
    /// ### Returns
    ///
    /// - A `Ticker` Struct
    pub async fn get_ticker(self, symbol: &str) -> Result<Ticker, Box<dyn Error>> {
        let ticker = self.tickers.iter().find(|x| x.ticker == symbol);
        match ticker {
            Some(t) => Ok(t.clone()),
            None => Err("Ticker not found".into()),
        }
    }

    /// Optimize a Portfolio of multiple tickers within the Tickers Struct.
    ///
    /// Builds a Portfolio with the given objective function and constraints,
    /// then runs optimization to find optimal weights.
    ///
    /// ### Arguments
    /// - `objective_function` - The Objective Function to optimize the Portfolio
    /// - `constraints` - The Portfolio Constraints
    ///
    /// ### Returns
    ///
    /// - A `Portfolio` Struct with optimization result computed
    pub async fn optimize(
        &self,
        objective_function: Option<ObjectiveFunction>,
        constraints: Option<Constraints>,
    ) -> Result<Portfolio, Box<dyn Error>> {
        let data = prepare_portfolio_data(self, self.benchmark_ticker.as_ref()).await?;
        let mut portfolio = Portfolio::new_raw(
            self.clone(),
            data,
            objective_function.unwrap_or(ObjectiveFunction::MaxSharpe),
            constraints,
            None,
            None,
        );

        portfolio.optimize()?;

        Ok(portfolio)
    }

    /// Compute portfolio performance statistics from explicit weights.
    ///
    /// Builds a Portfolio and computes performance analysis using the
    /// provided weights (dollar amounts) and optional per-asset transactions.
    ///
    /// ### Arguments
    /// - `weights` - Dollar amounts per asset (same order as tickers)
    /// - `transactions` - Optional per-asset transactions (additions/withdrawals)
    ///
    /// ### Returns
    ///
    /// - A `Portfolio` Struct with performance stats computed
    pub async fn portfolio_performance_stats(
        &self,
        weights: Vec<f64>,
        transactions: Option<Vec<Transaction>>,
    ) -> Result<Portfolio, Box<dyn Error>> {
        let data = prepare_portfolio_data(self, self.benchmark_ticker.as_ref()).await?;
        let mut portfolio = Portfolio::new_raw(
            self.clone(),
            data,
            ObjectiveFunction::MaxSharpe,
            None,
            Some(weights),
            transactions,
        );

        portfolio.performance_stats()?;

        Ok(portfolio)
    }
}

/// Tickers Struct
///
/// ### Description
/// - This is the main Interface for the `Finalytics` Library.
/// - It provides methods to:
///     - fetch data for multiple tickers asynchronously.
///     - compute performance statistics for multiple tickers and display HTML reports.
///     - initialize the Ticker and Portfolio Structs, providing an interface for calling their respective methods.
///
/// ### Constructor
/// - The `Tickers` struct can be instantiated using the `TickersBuilder` struct.
///
/// ### Example
/// ```rust
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>>  {
///   // Instantiate a Tickers Object
///    let symbols = vec!["AAPL", "MSFT", "NVDA", "BTC-USD"];
///    let tickers = Tickers::builder()
///        .tickers(symbols)
///        .start_date("2023-01-01")
///        .end_date("2023-12-31")
///        .interval(Interval::OneDay)
///        .benchmark_symbol("^GSPC")
///        .confidence_level(0.95)
///        .risk_free_rate(0.02)
///        .build();
///
///   // Generate a Single Ticker Report
///    let ticker = tickers.clone().get_ticker("NVDA").await?;
///    ticker.report(Some(ReportType::Performance)).await?.show()?;
///
///    // Generate a Multiple Ticker Report
///    tickers.report(Some(ReportType::Performance)).await?.show()?;
///
///    // Perform a Portfolio Optimization (in-sample performance stats computed automatically)
///    let mut portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None).await?;
///    portfolio.report(Some(ReportType::Optimization)).await?.show()?;
///
///    // Optionally: update dates for out-of-sample evaluation, then recompute performance stats
///    portfolio.update_dates("2024-01-01", "2024-12-31").await?;
///    portfolio.performance_stats()?;
///    portfolio.report(Some(ReportType::Performance)).await?.show()?;
///
///    // Or evaluate an explicit allocation directly (no optimization needed)
///    let allocation = vec![25_000.0, 25_000.0, 25_000.0, 25_000.0];
///    let portfolio = tickers.portfolio_performance_stats(allocation, None).await?;
///    portfolio.report(Some(ReportType::Performance)).await?.show()?;
///
///    Ok(())
/// }
/// ```

#[derive(Debug, Clone)]
pub struct Tickers {
    pub tickers: Vec<Ticker>,
    pub start_date: String,
    pub end_date: String,
    pub interval: Interval,
    pub benchmark_symbol: Option<String>,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    pub tickers_data: Option<Vec<KLINE>>,
    pub benchmark_data: Option<KLINE>,
    pub benchmark_ticker: Option<Ticker>,
}
