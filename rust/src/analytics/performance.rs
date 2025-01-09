use polars::prelude::*;
use std::error::Error;
use chrono::{DateTime, NaiveDateTime};

use crate::data::config::Interval;
use crate::models::ticker::{Ticker, TickerBuilder};
use crate::analytics::technicals::TechnicalIndicators;
use crate::analytics::optimization::{ObjectiveFunction, portfolio_optimization};
use crate::analytics::statistics::{PerformanceStats, covariance_matrix, daily_portfolio_returns};
use crate::prelude::{Column, TickersBuilder, TickersData};


#[derive(Debug, Clone)]
pub struct TickerPerformanceStats {
    pub ticker_symbol: String,
    pub benchmark_symbol: String,
    pub start_date: String,
    pub end_date: String,
    pub dates_array: Vec<String>,
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
        let security_df = self.roc(1, Some(Column::AdjClose)).await?;
        let security_prices = security_df.column(Column::AdjClose.as_str())?.clone();
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
        let benchmark_returns = benchmark_ticker.roc(1, Some(Column::AdjClose)).await?;
        let benchmark_returns = security_returns.join(
            &benchmark_returns,
            &["timestamp"],
            &["timestamp"],
            JoinArgs::new(JoinType::Left),
        )?;
        let benchmark_returns = benchmark_returns.sort(&["timestamp"], SortMultipleOptions::new().with_order_descending(false))?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Forward(None))?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Backward(None))?;
        let dates_array = benchmark_returns.column("timestamp")?.datetime()?
            .into_no_null_iter().map(|x| DateTime::from_timestamp_millis(x).unwrap()
            .naive_local()).collect::<Vec<NaiveDateTime>>();
        let dates_array = dates_array.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let security_returns = benchmark_returns.column(&*self.ticker)?.clone();
        let benchmark_returns = benchmark_returns.column("roc-1")?.clone();

        let performance_stats = PerformanceStats::compute_stats(
            security_returns.clone(), benchmark_returns.clone(),
            self.risk_free_rate, self.confidence_level, self.interval)?;
        Ok(TickerPerformanceStats {
            ticker_symbol: self.ticker.clone(),
            benchmark_symbol: self.benchmark_symbol.clone(),
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            dates_array,
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
    pub dates_array: Vec<String>,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    pub portfolio_returns: DataFrame,
    pub benchmark_returns: Series,
    pub objective_function: ObjectiveFunction,
    pub optimization_method: String,
    pub constraints: Vec<(f64, f64)>,
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
        objective_function: ObjectiveFunction,
        constraints: Option<Vec<(f64, f64)>>
    ) -> Result<PortfolioPerformanceStats, Box<dyn Error>> {
        let ticker_symbols = ticker_symbols.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        let tickers = TickersBuilder::new()
            .tickers(ticker_symbols.clone())
            .start_date(start_date)
            .end_date(end_date)
            .build();
        let mut portfolio_returns = tickers.returns().await?;
        let portfolio_dates = portfolio_returns
            .column("timestamp").unwrap()
            .str().unwrap()
            .into_no_null_iter()
            .map(|x| NaiveDateTime::parse_from_str(x, "%Y-%m-%d %H:%M:%S").unwrap())
            .collect::<Vec<NaiveDateTime>>();
        let _ = portfolio_returns.drop_in_place("timestamp")?;
        let _=  portfolio_returns.insert_column(0, Series::new("timestamp", portfolio_dates))?;

        let benchmark_ticker = TickerBuilder::new().ticker(benchmark_symbol)
            .start_date(start_date)
            .end_date(end_date)
            .interval(interval)
            .confidence_level(confidence_level)
            .risk_free_rate(risk_free_rate)
            .build();
        let benchmark_returns = benchmark_ticker.roc(1, Some(Column::AdjClose)).await?;
        let benchmark_returns =  portfolio_returns.join(
            &benchmark_returns,
            &["timestamp"],
            &["timestamp"],
            JoinArgs::new(JoinType::Left),
        )?;

        let benchmark_returns = benchmark_returns.sort(&["timestamp"], SortMultipleOptions::new().with_order_descending(false))?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Zero)?;
        let dates_array = benchmark_returns.column("timestamp")?.datetime()?
            .into_no_null_iter().map(|x| DateTime::from_timestamp_millis(x).unwrap()
            .naive_local()).collect::<Vec<NaiveDateTime>>();
        let dates_array = dates_array.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let benchmark_returns = benchmark_returns.column("roc-1")?.clone();

        let _ = portfolio_returns.drop_in_place("timestamp")?;

        let fetched_symbols = portfolio_returns.get_column_names().iter().map(|x| x.to_string()).collect::<Vec<String>>();

        let constraints = match constraints {
            Some(c) => {
                let symbols = ticker_symbols.clone();
                let constraints = c.iter()
                    .zip(symbols.iter())
                    .filter(|(_, symbol)| fetched_symbols.contains(&symbol.to_string()))
                    .map(|(x, _)| x.clone())
                    .collect::<Vec<(f64, f64)>>();
                constraints
            }
            None => vec![(0.0, 1.0); fetched_symbols.len()]
        };

        Ok(PortfolioPerformanceStats {
            ticker_symbols: fetched_symbols.clone(),
            benchmark_symbol: benchmark_symbol.to_string(),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            interval,
            dates_array,
            confidence_level,
            risk_free_rate,
            portfolio_returns: portfolio_returns.clone(),
            benchmark_returns: benchmark_returns.clone(),
            objective_function,
            optimization_method: "Simple Gradient Descent".to_string(),
            constraints,
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
        let mean_returns = self.portfolio_returns
            .get_columns()
            .iter()
            .map(|col| {
                col.f64()
                    .unwrap()
                    .mean()
                    .unwrap()
            })
            .collect::<Vec<f64>>();
        let cov_matrix = covariance_matrix(&self.portfolio_returns)?;

        let opt_result = portfolio_optimization(&mean_returns, &cov_matrix, &self.portfolio_returns, self.risk_free_rate,
                                                     self.confidence_level, self.objective_function, self.constraints.clone());
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
            dates_array: self.dates_array.clone(),
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
            portfolio_returns: self.portfolio_returns.clone(),
            benchmark_returns: self.benchmark_returns.clone(),
            objective_function: self.objective_function.clone(),
            optimization_method: self.optimization_method.clone(),
            constraints: self.constraints.clone(),
            optimal_weights: optimal_weights.clone(),
            optimal_portfolio_returns: daily_portfolio_returns.clone(),
            performance_stats,
            efficient_frontier: opt_result.efficient_frontier,
        })
    }
}










