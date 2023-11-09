use std::error::Error;
use plotly::common::{Fill, Marker, MarkerSymbol, Mode, Title};
use plotly::{Layout, Scatter, Plot, Bar, Histogram, Table};
use plotly::color::NamedColor;
use plotly::layout::{Axis, GridPattern, LayoutGrid, RowOrder};
use crate::analytics::optimization::ObjectiveFunction;
use crate::analytics::performance::PortfolioPerformanceStats;
use crate::analytics::statistics::cumulative_returns_list;
use crate::data::ticker::Interval;
use crate::utils::date_utils::generate_dates;

/// # Portfolio Charts Struct
///
/// Helps generate financial analytics charts for portfolio optimization and performance
///
/// # Example
///
/// ```
/// use std::error::Error;
/// use finalytics::data::ticker::Interval;
/// use finalytics::analytics::optimization::ObjectiveFunction;
/// use finalytics::charts::portfolio::PortfolioCharts;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let pc = PortfolioCharts::new(
///         Vec::from(["NVDA".to_string(), "BRK-A".to_string(), "AAPL".to_string(), "ZN=F".to_string()]),
///         "^GSPC", "2017-01-01", "2023-01-01", Interval::OneDay, 0.95, 0.02, 1000, ObjectiveFunction::MaxSharpe).await?;
///     let _ = pc.optimization_chart().unwrap().show();
///     let _ = pc.performance_chart().unwrap().show();
///     let _ = pc.asset_returns_chart().unwrap().show();
///     let _ = pc.performance_stats_table().unwrap().show();
///     Ok(())
/// }
/// ```
pub struct PortfolioCharts {
    pub performance_stats: PortfolioPerformanceStats,
}

impl PortfolioCharts {
    /// Creates a new PortfolioCharts struct
    ///
    /// # Arguments
    ///
    /// * `ticker_symbols` - Vector of ticker symbols
    /// * `benchmark_symbol` - Benchmark ticker symbol
    /// * `start_date` - Start date in YYYY-MM-DD format
    /// * `end_date` - End date in YYYY-MM-DD format
    /// * `interval` - Time interval enum
    /// * `confidence_level` - Confidence level for VaR and CVaR in decimal (e.g 0.95 for 95%)
    /// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
    /// * `max_iterations` - Maximum number of iterations for the optimization algorithm
    /// * `objective_function` - Objective function for the optimization algorithm
    ///
    /// # Returns
    ///
    /// * `PortfolioCharts` struct
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
    ) -> Result<PortfolioCharts, Box<dyn Error>> {

        let performance_stats = PortfolioPerformanceStats::new(
            ticker_symbols, benchmark_symbol, start_date, end_date, interval,
            confidence_level, risk_free_rate, max_iterations, objective_function).await?.compute_stats()?;

        Ok(PortfolioCharts {
            performance_stats,
        })
    }

    /// Generates Chart of the Portfolio Optimization Results
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub fn optimization_chart(&self) -> Result<Plot, Box<dyn Error>> {
        let ef_returns = self.performance_stats.efficient_frontier.clone().iter()
            .map(|x| x[0]).collect::<Vec<f64>>();

        let ef_risk = self.performance_stats.efficient_frontier.clone().iter()
            .map(|x| x[1]).collect::<Vec<f64>>();

        let ef_trace = Scatter::new(ef_risk, ef_returns)
            .name("Efficient Frontier")
            .mode(Mode::Markers)
            .marker(Marker::new().size(10));

        let optimal_point = Scatter::new(vec![self.performance_stats.performance_stats.daily_volatility],
                                         vec![self.performance_stats.performance_stats.daily_return])
            .name("Optimal Portfolio")
            .mode(Mode::Markers)
            .marker(Marker::new().size(12).color(NamedColor::Red).symbol(MarkerSymbol::Star));

        let ticker_symbols = self.performance_stats.ticker_symbols.clone();
        let weights = self.performance_stats.optimal_weights.clone().iter()
            .map(|x| x * 100.0).collect::<Vec<f64>>();
        let allocation_trace = Bar::new(ticker_symbols.clone(), weights.clone())
            .name("Asset Allocation")
            .x_axis("x2")
            .y_axis("y2")
            .text_array(weights.clone().iter().map(|w| format!("{:.2}%", w).to_string()).collect::<Vec<_>>());


        let mut plot = Plot::new();
        plot.add_trace(ef_trace);
        plot.add_trace(optimal_point);
        plot.add_trace(allocation_trace);

        // Set layout for the plot
        let layout = Layout::new()
            .height(800)
            .width(1200)
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Optimization Chart</span>"))
            .grid(
                LayoutGrid::new()
                    .rows(2)
                    .columns(1)
                    .pattern(GridPattern::Independent)
                    .row_order(RowOrder::TopToBottom)
            )
            .y_axis(
                Axis::new()
                    .title(Title::from("Efficient Frontier"))
            )
            .y_axis2(
                Axis::new()
                    .title(Title::from("Asset Allocation"))
            );

        plot.set_layout(layout);

        Ok(plot)
    }

    /// Generates Chart of the Portfolio Performance Results
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub fn performance_chart(&self) -> Result<Plot, Box<dyn Error>> {
        let dates = generate_dates(&*self.performance_stats.start_date,
                                   &*self.performance_stats.end_date, 1);

        let returns = self.performance_stats.optimal_portfolio_returns.f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let benchmark_returns = self.performance_stats.benchmark_returns.f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let cum_returns= cumulative_returns_list(returns.clone());

        let benchmark_cum_returns= cumulative_returns_list(benchmark_returns.clone());

        let returns_trace = Scatter::new(dates.clone(), returns.clone())
            .name("Portfolio Returns")
            .mode(Mode::Markers)
            .fill(Fill::ToZeroY);

        let returns_dist_trace = Histogram::new(returns.clone())
            .name("Portfolio Returns Distribution")
            .x_axis("x2")
            .y_axis("y2");

        let cum_returns_trace = Scatter::new(dates.clone(), cum_returns.clone())
            .name("Portfolio Cumulative Returns")
            .mode(Mode::Lines)
            .fill(Fill::ToZeroY)
            .x_axis("x3")
            .y_axis("y3");

        let benchmark_cum_returns_trace = Scatter::new(dates.clone(), benchmark_cum_returns.clone())
            .name("Benchmark Cumulative Returns")
            .mode(Mode::Lines)
            .fill(Fill::ToZeroY)
            .x_axis("x3")
            .y_axis("y3");

        let mut plot = Plot::new();
        plot.add_trace(returns_trace);
        plot.add_trace(returns_dist_trace);
        plot.add_trace(cum_returns_trace);
        plot.add_trace(benchmark_cum_returns_trace);

        // Set layout for the plot
        let layout = Layout::new()
            .height(800)
            .width(1200)
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Performance Chart</span>"))
            .grid(
                LayoutGrid::new()
                    .rows(3)
                    .columns(1)
                    .pattern(GridPattern::Independent)
                    .row_order(RowOrder::TopToBottom)
            )
            .y_axis(
                Axis::new()
                    .title(Title::from("Returns"))
            )
            .y_axis2(
                Axis::new()
                    .title(Title::from("Returns Distribution"))
            )
            .y_axis3(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
            );

        plot.set_layout(layout);

        Ok(plot)
    }

    /// Generates Chart of the Portfolio Asset Returns
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub fn asset_returns_chart(&self) -> Result<Plot, Box<dyn Error>> {
        let symbols = self.performance_stats.ticker_symbols.clone();
        let asset_returns = self.performance_stats.portfolio_returns.clone();
        let dates = generate_dates(&*self.performance_stats.start_date, &*self.performance_stats.end_date, 1);
        let mut plot = Plot::new();

        for symbol in symbols {
            let returns = asset_returns.column(&symbol).unwrap().f64().unwrap().to_vec()
                .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
            let cum_returns = cumulative_returns_list(returns.clone());
            let cum_returns_trace = Scatter::new(dates.clone(), cum_returns.clone())
                .name(format!("{}", symbol))
                .mode(Mode::Lines);
            plot.add_trace(cum_returns_trace);
        }

        let layout = Layout::new()
            .height(800)
            .width(1200)
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Assets Cumulative Returns</span>"));

        plot.set_layout(layout);
        Ok(plot)
    }

    /// Displays the Performance Statistics table for the portfolio
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub fn performance_stats_table(&self) -> Result<Plot, Box<dyn Error>> {
        let stats = &self.performance_stats.performance_stats;

        let fields = vec![
            "Daily Return".to_string(),
            "Daily Volatility".to_string(),
            "Cumulative Return".to_string(),
            "Annualized Return".to_string(),
            "Annualized Volatility".to_string(),
            "Alpha".to_string(),
            "Beta".to_string(),
            "Sharpe Ratio".to_string(),
            "Sortino Ratio".to_string(),
            "Active Return".to_string(),
            "Active Risk".to_string(),
            "Information Ratio".to_string(),
            "Calmar Ratio".to_string(),
            "Maximum Drawdown".to_string(),
            "Value At Risk".to_string(),
            "Expected Shortfall".to_string(),
        ];

        let values = vec![
            format!("{:.2}%",stats.daily_return),
            format!("{:.2}%",stats.daily_volatility),
            format!("{:.2}%",stats.cumulative_return),
            format!("{:.2}%",stats.annualized_return),
            format!("{:.2}%",stats.annualized_volatility),
            format!("{:.2}",stats.alpha),
            format!("{:.2}",stats.beta),
            format!("{:.2}",stats.sharpe_ratio),
            format!("{:.2}",stats.sortino_ratio),
            format!("{:.2}%",stats.active_return),
            format!("{:.2}%",stats.active_risk),
            format!("{:.2}",stats.information_ratio),
            format!("{:.2}",stats.calmar_ratio),
            format!("{:.2}%",stats.maximum_drawdown),
            format!("{:.2}%",stats.value_at_risk),
            format!("{:.2}%",stats.expected_shortfall),
        ];


        let trace = Table::new(
            vec![
                format!("<span style=\"font-weight:bold; color:darkgreen;\">{}</span>", "Performance Stats"),
                format!("<span style=\"font-weight:bold; color:darkgreen;\">{}</span>", "Values"),
            ],
            vec![fields, values],
        );
        let mut plot = Plot::new();
        plot.add_trace(trace);

        let layout = Layout::new()
            .height(1000)
            .width(1200)
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\"> Portfolio Performance Stats</span>"));

        plot.set_layout(layout);

        Ok(plot)
    }
}

