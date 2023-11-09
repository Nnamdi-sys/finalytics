use polars::prelude::*;
use std::error::Error;
use crate::analytics::optimization::{ObjectiveFunction, portfolio_optimization};
use crate::analytics::technicals::TechnicalIndicators;
use crate::analytics::statistics::{PerformanceStats, covariance_matrix, daily_portfolio_returns};
use crate::data::ticker::Interval;


/// # Ticker Performance Struct
///
/// Helps compute the performance statistics for a ticker
///
/// # Example
///
/// ```
/// use std::error::Error;
/// use finalytics::data::ticker::Interval;
/// use finalytics::analytics::performance::TickerPerformanceStats;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let result = TickerPerformanceStats::new(
///         "AAPL", "^GSPC", "2022-01-01", "2022-12-31", Interval::OneDay,
///         0.95, 0.02).await?.compute_stats()?;
///     println!("{:?}", result);
///     Ok(())
/// }
/// ```
#[derive(Debug)]
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

/// # Portfolio Performance Struct
///
/// Helps compute the performance statistics for a portfolio
///
/// # Example
///
/// ```
/// use std::error::Error;
/// use finalytics::data::ticker::Interval;
/// use finalytics::analytics::optimization::ObjectiveFunction;
/// use finalytics::analytics::performance::PortfolioPerformanceStats;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let result = PortfolioPerformanceStats::new(
///         Vec::from(["AAPL".to_string(), "GOOG".to_string(), "NVDA".to_string(), "ZN=F".to_string()]),
///         "^GSPC", "2021-01-01", "2023-01-01",
///         Interval::OneDay, 0.95, 0.02, 1000,
///         ObjectiveFunction::MaxSharpe).await?.compute_stats()?;
///     println!("{:?}", result);
///     Ok(())
/// }
/// ```
#[derive(Debug)]
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
            let security_df = TechnicalIndicators::new(
                ticker_symbol, start_date, end_date, interval).await?.roc(1)?;
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

        let benchmark_returns = TechnicalIndicators::new(benchmark_symbol, start_date, end_date, interval).await?.roc(1)?;
        let benchmark_returns = benchmark_returns.join(
            &portfolio_returns,
            &["timestamp"],
            &["timestamp"],
            JoinArgs::new(JoinType::Outer),
        )?;
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


impl TickerPerformanceStats {
    /// Creates a new TickerPerformanceStats struct
    ///
    /// # Arguments
    ///
    /// * `ticker_symbol` - Ticker symbol (e.g. "AAPL")
    /// * `benchmark_symbol` - Benchmark ticker symbol (e.g. "^GSPC")
    /// * `start_date` - Start date in YYYY-MM-DD format (e.g. "2021-01-01")
    /// * `end_date` - End date in YYYY-MM-DD format (e.g. "2021-01-31")
    /// * `interval` - Time interval enum (e.g. Interval::OneDay)
    /// * `confidence_level` - Confidence level for VaR and CVaR in decimal (e.g 0.95 for 95%)
    /// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
    ///
    /// # Returns
    ///
    /// * `TickerPerformanceStats` struct
    pub async fn new(
        ticker_symbol: &str,
        benchmark_symbol: &str,
        start_date: &str,
        end_date: &str,
        interval: Interval,
        confidence_level: f64,
        risk_free_rate: f64
    ) -> Result<TickerPerformanceStats, Box<dyn Error>> {
        let security_df = TechnicalIndicators::new(
            ticker_symbol, start_date, end_date, interval).await?.roc(1)?;
        let security_prices = security_df.column("close")?.clone();
        let security_returns = DataFrame::new(vec![
            security_df.column("timestamp")?.clone(),
            security_df.column("roc-1")?.clone().with_name(ticker_symbol)
        ])?;
        let benchmark_returns = TechnicalIndicators::new(benchmark_symbol, start_date, end_date, interval).await?.roc(1)?;
        let benchmark_returns = benchmark_returns.join(
            &security_returns,
            &["timestamp"],
            &["timestamp"],
            JoinArgs::new(JoinType::Outer),
        )?;
        let benchmark_returns = benchmark_returns.sort(&["timestamp"], false, false)?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Forward(None))?;
        let benchmark_returns = benchmark_returns.fill_null(FillNullStrategy::Backward(None))?;
        let benchmark_returns = benchmark_returns.column("roc-1")?.clone();
        let security_returns = security_returns.column(ticker_symbol)?.clone();

        Ok(TickerPerformanceStats {
            ticker_symbol: ticker_symbol.to_string(),
            benchmark_symbol: benchmark_symbol.to_string(),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            interval,
            confidence_level,
            risk_free_rate,
            security_prices: security_prices.clone(),
            security_returns: security_returns.clone(),
            benchmark_returns: benchmark_returns.clone(),
            performance_stats: PerformanceStats::default(),
        })
    }

    /// Computes the performance statistics for the ticker
    ///
    /// # Returns
    ///
    /// * `TickerPerformanceStats` struct
    pub fn compute_stats(&self) -> Result<TickerPerformanceStats, Box<dyn Error>> {
        let performance_stats = PerformanceStats::compute_stats(
            self.security_returns.clone(), self.benchmark_returns.clone(),
            self.risk_free_rate, self.confidence_level, self.interval)?;
        Ok(Self{
            ticker_symbol: self.ticker_symbol.clone(),
            benchmark_symbol: self.benchmark_symbol.clone(),
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            interval: self.interval.clone(),
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
            security_prices: self.security_prices.clone(),
            security_returns: self.security_returns.clone(),
            benchmark_returns: self.benchmark_returns.clone(),
            performance_stats
        })
    }

}








