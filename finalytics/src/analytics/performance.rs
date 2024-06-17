use polars::prelude::*;
use std::error::Error;

use crate::data::config::Interval;
use crate::models::ticker::{Ticker, TickerBuilder};
use crate::analytics::technicals::TechnicalIndicators;
use crate::analytics::optimization::{ObjectiveFunction, portfolio_optimization};
use crate::analytics::statistics::{PerformanceStats, covariance_matrix, daily_portfolio_returns};


#[derive(Debug, Clone)]
pub struct TickerPerformanceStats {
    pub ticker_symbol: String,
    pub benchmark_symbol: String,
    pub start_date: String,
    pub end_date: String,
    pub interval: Interval,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    pub security_prices: Series,
    pub security_returns: Series,
    pub benchmark_returns: Series,
    pub performance_stats: PerformanceStats,
}

pub trait TickerPerformance {
    fn performance_stats(&self) -> impl std::future::Future<Output = Result<TickerPerformanceStats, Box<dyn Error>>>;
}

impl TickerPerformance for Ticker {
    /// Computes the performance statistics for the ticker
    ///
    /// # Returns
    ///
    /// * `TickerPerformanceStats` struct
    async fn performance_stats(&self) -> Result<TickerPerformanceStats, Box<dyn Error>> {
        let security_df = self.roc(1).await?;
        let security_prices = security_df.column("close")?.clone();
        let security_returns = DataFrame::new(vec![
            security_df.column("timestamp")?.clone(),
            security_df.column("roc-1")?.clone().with_name(&*self.ticker)
        ])?;
        let benchmark_ticker = TickerBuilder::new().ticker(&self.benchmark_symbol.clone())
            .start_date(&self.start_date.clone())
            .end_date(&self.end_date.clone())
            .interval(self.interval.clone())
            .confidence_level(self.confidence_level)
            .risk_free_rate(self.risk_free_rate)
            .build();
        let benchmark_returns = benchmark_ticker.roc(1).await?;
        let benchmark_returns = benchmark_returns.join(
            &security_returns,
            &["timestamp"],
            &["timestamp"],
            JoinArgs::new(JoinType::Inner),
        )?;
        let benchmark_returns = benchmark_returns.sort(&["timestamp"], false, false)?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Forward(None))?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Backward(None))?;
        let benchmark_returns = benchmark_returns.column("roc-1")?.clone();
        let security_returns = security_returns.column(&*self.ticker)?.clone();
        let performance_stats = PerformanceStats::compute_stats(
            security_returns.clone(), benchmark_returns.clone(),
            self.risk_free_rate, self.confidence_level, self.interval)?;
        Ok(TickerPerformanceStats {
            ticker_symbol: self.ticker.clone(),
            benchmark_symbol: self.benchmark_symbol.clone(),
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            interval: self.interval.clone(),
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
            security_prices: security_prices.clone(),
            security_returns: security_returns.clone(),
            benchmark_returns: benchmark_returns.clone(),
            performance_stats
        })
    }

}

/// # Portfolio Performance Struct
/// Helps compute the performance statistics for a portfolio
#[derive(Debug, Clone)]
pub struct PortfolioPerformanceStats {
    pub ticker_symbols: Vec<String>,
    pub benchmark_symbol: String,
    pub start_date: String,
    pub end_date: String,
    pub interval: Interval,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    pub portfolio_returns: DataFrame,
    pub benchmark_returns: Series,
    pub objective_function: ObjectiveFunction,
    pub optimization_method: String,
    pub max_iterations: u64,
    pub optimal_weights: Vec<f64>,
    pub optimal_portfolio_returns: Series,
    pub performance_stats: PerformanceStats,
    pub efficient_frontier: Vec<Vec<f64>>,
}


impl PortfolioPerformanceStats {
    /// Creates a new PortfolioPerformanceStats struct
    ///
    /// # Arguments
    ///
    /// * `ticker_symbols` - Vector of ticker symbols (e.g. ["AAPL", "NVDA", "GOOG"])
    /// * `benchmark_symbol` - Benchmark ticker symbol (e.g. "^GSPC")
    /// * `start_date` - Start date in YYYY-MM-DD format (e.g. "2021-01-01")
    /// * `end_date` - End date in YYYY-MM-DD format (e.g. "2021-01-31")
    /// * `interval` - Time interval enum (e.g. Interval::OneDay)
    /// * `confidence_level` - Confidence level for VaR and CVaR in decimal (e.g 0.95 for 95%)
    /// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
    /// * `max_iterations` - Maximum number of iterations for the optimization (e.g. 1000)
    /// * `objective_function` - Objective function for the optimization (e.g. ObjectiveFunction::MaxSharpe)
    ///
    /// # Returns
    ///
    /// * `PortfolioPerformanceStats` struct
    pub async fn new(
        ticker_symbols: Vec<String>,
        benchmark_symbol: &str,
        start_date: &str,
        end_date: &str,
        interval: Interval,
        confidence_level: f64,
        risk_free_rate: f64,
        max_iterations: u64,
        objective_function: ObjectiveFunction
    ) -> Result<PortfolioPerformanceStats, Box<dyn Error>> {
        let mut dfs: Vec<DataFrame> = Vec::new();
        for ticker_symbol in ticker_symbols.iter() {
            let ticker = TickerBuilder::new().ticker(ticker_symbol)
                .start_date(start_date)
                .end_date(end_date)
                .interval(interval)
                .confidence_level(confidence_level)
                .risk_free_rate(risk_free_rate)
                .build();
            let security_df = ticker.roc(1).await?;
            let security_returns_df = DataFrame::new(vec![
                security_df.column("timestamp")?.clone(),
                security_df.column("roc-1")?.clone().with_name(ticker_symbol)
            ])?;
            dfs.push(security_returns_df);
        }

        let mut portfolio_returns = dfs[0].clone();

        for df in dfs[1..].iter() {
            portfolio_returns = portfolio_returns
                .join(
                    df,
                    &["timestamp"],
                    &["timestamp"],
                    JoinArgs::new(JoinType::Outer),
                )?;
        }
        portfolio_returns = portfolio_returns.sort(&["timestamp"], false, false)?;
        portfolio_returns = portfolio_returns.fill_null(FillNullStrategy::Forward(None))?;
        portfolio_returns = portfolio_returns.fill_null(FillNullStrategy::Backward(None))?;

        let benchmark_ticker = TickerBuilder::new().ticker(benchmark_symbol)
            .start_date(start_date)
            .end_date(end_date)
            .interval(interval)
            .confidence_level(confidence_level)
            .risk_free_rate(risk_free_rate)
            .build();
        let benchmark_returns = benchmark_ticker.roc(1).await?;
        let benchmark_returns = if portfolio_returns.height() > benchmark_returns.height() {
            benchmark_returns.join(
            &portfolio_returns,
            &["timestamp"],
            &["timestamp"],
            JoinArgs::new(JoinType::Outer),
        )?} else {
            portfolio_returns.join(
            &benchmark_returns,
            &["timestamp"],
            &["timestamp"],
            JoinArgs::new(JoinType::Inner),
        )?};

        let benchmark_returns = benchmark_returns.sort(&["timestamp"], false, false)?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Forward(None))?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Backward(None))?;
        let benchmark_returns = benchmark_returns.column("roc-1")?.clone();

        let _ = portfolio_returns.drop_in_place("timestamp")?;

        Ok(PortfolioPerformanceStats {
            ticker_symbols: ticker_symbols.clone(),
            benchmark_symbol: benchmark_symbol.to_string(),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            interval,
            confidence_level,
            risk_free_rate,
            portfolio_returns: portfolio_returns.clone(),
            benchmark_returns: benchmark_returns.clone(),
            objective_function,
            optimization_method: "Simple Gradient Descent".to_string(),
            max_iterations,
            optimal_weights: Vec::new(),
            optimal_portfolio_returns: Series::default(),
            performance_stats: PerformanceStats::default(),
            efficient_frontier: Vec::new(),
        })
    }

    /// Computes the performance statistics for the portfolio
    ///
    /// # Returns
    ///
    /// * `PortfolioPerformanceStats` struct
    pub fn compute_stats(&self) -> Result<PortfolioPerformanceStats, Box<dyn Error>> {
        let mean_returns = self.portfolio_returns.mean().iter().map(|x| x.f64().unwrap()
            .get(0).unwrap()).collect::<Vec<f64>>();
        let cov_matrix = covariance_matrix(&self.portfolio_returns)?;

        let opt_result = portfolio_optimization(&mean_returns, &cov_matrix, &self.portfolio_returns, self.risk_free_rate,
                                                     self.confidence_level, self.max_iterations, self.objective_function);
        let optimal_weights = opt_result.optimal_weights;
        let daily_portfolio_returns = daily_portfolio_returns(&optimal_weights, &self.portfolio_returns);

        let performance_stats = PerformanceStats::compute_stats(
            daily_portfolio_returns.clone(), self.benchmark_returns.clone(),
            self.risk_free_rate, self.confidence_level, self.interval)?;


        Ok(Self{
            ticker_symbols: self.ticker_symbols.clone(),
            benchmark_symbol: self.benchmark_symbol.clone(),
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            interval: self.interval.clone(),
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
            portfolio_returns: self.portfolio_returns.clone(),
            benchmark_returns: self.benchmark_returns.clone(),
            objective_function: self.objective_function.clone(),
            optimization_method: self.optimization_method.clone(),
            max_iterations: self.max_iterations,
            optimal_weights: optimal_weights.clone(),
            optimal_portfolio_returns: daily_portfolio_returns.clone(),
            performance_stats,
            efficient_frontier: opt_result.efficient_frontier,
        })
    }
}










