use std::error::Error;
use polars::prelude::{DataFrame, NamedFrom, Series};
use plotly::color::NamedColor;
use plotly::{Bar, HeatMap, Histogram, Layout, Plot, Scatter};
use plotly::layout::{Axis, GridPattern, LayoutGrid, RowOrder};
use plotly::common::{ColorScalePalette, Fill, Marker, MarkerSymbol, Mode, Title};

use crate::prelude::TickersData;
use crate::prelude::TickersBuilder;
use crate::models::portfolio::Portfolio;
use crate::reports::table::{DataTable, TableType};
use crate::charts::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::analytics::statistics::{correlation_matrix, cumulative_returns_list, maximum_drawdown};


pub trait PortfolioCharts {
    fn optimal_symbols(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn optimization_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>>;
    fn performance_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>>;
    fn performance_stats_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn returns_table(&self) -> Result<DataTable, Box<dyn Error>>;
    fn returns_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>>;
    fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>>;
}

impl PortfolioCharts for Portfolio {
    /// Returns the Optimal Symbols for the Portfolio
    ///
    /// # Returns
    ///
    /// * `Vec<String>` Vector of optimal symbols
    fn optimal_symbols(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let symbols = self.performance_stats.ticker_symbols.clone();
        let weights = self.performance_stats.optimal_weights.clone();
        let filtered_results: Vec<_> = symbols.iter()
            .zip(weights.iter())
            .filter(|&(_, &weight)| weight.abs() > 0.0)
            .collect();
        let symbols: Vec<String> = filtered_results.iter().map(|&(ticker, _)| ticker.clone()).collect();
        Ok(symbols)
    }

    /// Generates Chart of the Portfolio Optimization Results
    ///
    /// # Arguments
    ///
    /// * `height` - usize - Height of the chart
    /// * `width` - usize - Width of the chart
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    fn optimization_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let days = self.performance_stats.interval.to_days();

        let ef_returns = self.performance_stats.efficient_frontier.clone().iter()
            .map(|x| (1.0 + (x[0]/days)/100.0).powf(252.0) - 1.0).collect::<Vec<f64>>();

        let ef_risk = self.performance_stats.efficient_frontier.clone().iter()
            .map(|x| x[1]/100.0 * 252.0_f64.sqrt()).collect::<Vec<f64>>();

        let ef_trace = Scatter::new(ef_risk, ef_returns)
            .name("Efficient Frontier")
            .mode(Mode::Markers)
            .marker(Marker::new().size(10));

        let opt_return = (1.0 + (self.performance_stats.performance_stats.daily_return/days)/100.0).powf(252.0) - 1.0;
        let opt_risk = self.performance_stats.performance_stats.daily_volatility/100.0 * 252.0_f64.sqrt();

        let optimal_point = Scatter::new(vec![opt_risk],
                                         vec![opt_return])
            .name("Optimal Portfolio")
            .mode(Mode::Markers)
            .marker(Marker::new().size(12).color(NamedColor::Red).symbol(MarkerSymbol::Star));

        let ticker_symbols = self.performance_stats.ticker_symbols.clone();
        let weights = self.performance_stats.optimal_weights.clone().iter()
            .map(|x| x * 100.0).collect::<Vec<f64>>();

        let mut filtered: Vec<_> = ticker_symbols.iter()
            .zip(weights.iter())
            .filter(|&(_, &weight)| weight.abs() > 0.0)
            .collect();

        filtered.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let filtered_ticker_symbols: Vec<String> = filtered.iter().map(|&(ticker, _)| ticker.clone()).collect();
        let filtered_weights: Vec<f64> = filtered.iter().map(|&(_, &weight)| weight).collect();

        let allocation_trace = Bar::new(filtered_ticker_symbols.clone(), filtered_weights.clone())
            .name("Asset Allocation")
            .x_axis("x2")
            .y_axis("y2")
            .text_array(filtered_weights.clone().iter().map(|w| format!("{:.2}%", w).to_string()).collect::<Vec<_>>());


        let mut plot = Plot::new();
        plot.add_trace(ef_trace);
        plot.add_trace(optimal_point);
        plot.add_trace(allocation_trace);

        // Set layout for the plot
        let layout = Layout::new()
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Optimization Chart</span>"))
            .grid(
                LayoutGrid::new()
                    .rows(2)
                    .columns(1)
                    .pattern(GridPattern::Independent)
                    .row_order(RowOrder::TopToBottom)
            )
            .x_axis(
                Axis::new()
                    .title(Title::from("Annualized Risk"))
                    .tick_format(".0%")
            )
            .y_axis(
                Axis::new()
                    .title(Title::from("Annualized Returns"))
                    .tick_format(".0%")
            )
            .x_axis2(
                Axis::new()
                    .title(Title::from("Portfolio Assets"))
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
    /// # Arguments
    ///
    /// * `height` - usize - Height of the chart
    /// * `width` - usize - Width of the chart
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    fn performance_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let dates = self.performance_stats.dates_array.clone();

        let returns = self.performance_stats.optimal_portfolio_returns.clone().f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let benchmark_returns = self.performance_stats.benchmark_returns.f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let cum_returns= cumulative_returns_list(returns.clone());

        let benchmark_cum_returns= cumulative_returns_list(benchmark_returns.clone());

        let (drawdowns, _) = maximum_drawdown(&self.performance_stats.optimal_portfolio_returns);
        let drawdowns = drawdowns.iter().map(|x| x/100.0).collect::<Vec<f64>>();

        let returns_trace = Scatter::new(dates.clone(), returns.clone().iter().map(|x| x/100.0).collect::<Vec<f64>>())
            .name("Portfolio Returns")
            .mode(Mode::Markers)
            .fill(Fill::ToZeroY);

        let returns_dist_trace = Histogram::new(returns.clone().iter().map(|x| x/100.0).collect::<Vec<f64>>())
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

        let drawdown_trace = Scatter::new(dates.clone(), drawdowns.clone())
            .name("Portfolio Drawdown")
            .mode(Mode::Lines)
            .fill(Fill::ToZeroY)
            .x_axis("x4")
            .y_axis("y4");

        let mut plot = Plot::new();
        plot.add_trace(returns_trace);
        plot.add_trace(returns_dist_trace);
        plot.add_trace(cum_returns_trace);
        plot.add_trace(benchmark_cum_returns_trace);
        plot.add_trace(drawdown_trace);

        // Set layout for the plot
        let layout = Layout::new()
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Performance Chart</span>"))
            .grid(
                LayoutGrid::new()
                    .rows(4)
                    .columns(1)
                    .pattern(GridPattern::Independent)
                    .row_order(RowOrder::TopToBottom)
            )
            .y_axis(
                Axis::new()
                    .title(Title::from("Returns"))
                    .tick_format(".0%")
            )
            .y_axis2(
                Axis::new()
                    .title(Title::from("Returns Distribution"))
            )
            .x_axis2(
                Axis::new()
                    .tick_format(".0%")
            )
            .y_axis3(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
                    .tick_format(".0%")
            )
            .y_axis4(
                Axis::new()
                    .title(Title::from("Drawdown"))
                    .tick_format(".0%")
            );

        plot.set_layout(layout);

        Ok(plot)
    }

    /// Displays the Performance Statistics table for the portfolio
    ///
    /// # Returns
    ///
    /// * `DataTable` Table Chart struct
    async fn performance_stats_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let symbols = self.optimal_symbols()?;

        let tickers = TickersBuilder::new()
            .tickers(symbols.iter().map(|x| x.as_str()).collect::<Vec<&str>>())
            .start_date(&self.performance_stats.start_date)
            .end_date(&self.performance_stats.end_date)
            .interval(self.performance_stats.interval)
            .benchmark_symbol(&self.performance_stats.benchmark_symbol)
            .confidence_level(self.performance_stats.confidence_level)
            .risk_free_rate(self.performance_stats.risk_free_rate)
            .build();

        let symbols_stats = tickers.performance_stats().await?;

        let stats = &self.performance_stats.performance_stats;

        let df = DataFrame::new(vec![
            Series::new("Symbol", &["Portfolio".to_string()]),
            Series::new("Daily Return", &[format!("{:.2}%", stats.daily_return)]),
            Series::new("Daily Volatility", &[format!("{:.2}%", stats.daily_volatility)]),
            Series::new("Cumulative Return", &[format!("{:.2}%", stats.cumulative_return)]),
            Series::new("Annualized Return", &[format!("{:.2}%", stats.annualized_return)]),
            Series::new("Annualized Volatility", &[format!("{:.2}%", stats.annualized_volatility)]),
            Series::new("Alpha", &[format!("{:.2}", stats.alpha)]),
            Series::new("Beta", &[format!("{:.2}", stats.beta)]),
            Series::new("Sharpe Ratio", &[format!("{:.2}", stats.sharpe_ratio)]),
            Series::new("Sortino Ratio", &[format!("{:.2}", stats.sortino_ratio)]),
            Series::new("Active Return", &[format!("{:.2}%", stats.active_return)]),
            Series::new("Active Risk", &[format!("{:.2}%", stats.active_risk)]),
            Series::new("Information Ratio", &[format!("{:.2}", stats.information_ratio)]),
            Series::new("Calmar Ratio", &[format!("{:.2}", stats.calmar_ratio)]),
            Series::new("Maximum Drawdown", &[format!("{:.2}%", stats.maximum_drawdown)]),
            Series::new("Value at Risk", &[format!("{:.2}%", stats.value_at_risk)]),
            Series::new("Expected Shortfall", &[format!("{:.2}%", stats.expected_shortfall)])
        ])?;

        let mut stats_df = symbols_stats.vstack(&df)?;

        let columns = stats_df.column("Symbol")?.str()?.into_no_null_iter()
            .map(|x| x.to_string()).collect::<Vec<String>>();
        stats_df = stats_df.drop("Symbol")?;
        let items = Series::new("Items", stats_df.get_column_names());
        let mut stats_df = stats_df.transpose(None, None)?;
        let _ =  stats_df.set_column_names(&columns)?;
        let _ = stats_df.insert_column(0, items)?;

        let data_table = DataTable::new(stats_df, TableType::PerformanceStats);

        Ok(data_table)
    }

    fn returns_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let returns = self.performance_stats.portfolio_returns.clone();
        let optimal_returns = self.performance_stats.optimal_portfolio_returns.clone();
        let dates = self.performance_stats.dates_array.clone();
        let symbols = self.optimal_symbols()?;
        let mut returns = returns.select(&symbols)?;
        let _=  returns.insert_column(0, Series::new("Timestamp", dates))?;
        returns = returns.hstack(&[Series::new("Portfolio", optimal_returns)])?;
        let table = DataTable::new(returns, TableType::Returns);
        Ok(table)
    }

    /// Generates Chart of the Portfolio Asset Returns
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    fn returns_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let symbols = self.optimal_symbols()?;
        let asset_returns = self.performance_stats.portfolio_returns.clone();
        let dates = self.performance_stats.dates_array.clone();
        let mut plot = Plot::new();

        for symbol in symbols {
            match asset_returns.column(&symbol) {
                Ok(returns_series) => {
                    let returns = returns_series.f64().unwrap().to_vec()
                        .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
                    let cum_returns = cumulative_returns_list(returns.clone());
                    let cum_returns_trace = Scatter::new(dates.clone(), cum_returns.clone())
                        .name(format!("{}", symbol))
                        .mode(Mode::Lines);
                    plot.add_trace(cum_returns_trace);
                }
                Err(e) => {
                    eprintln!("Unable to fetch returns for {}: {}", symbol, e);
                }
            }
        }

        let layout = Layout::new()
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Assets Cumulative Returns</span>"))
            .y_axis(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
                    .tick_format(".0%")
            );

        plot.set_layout(layout);
        Ok(plot)
    }

    /// Generates Chart of the Portfolio Returns Correlation Matrix
    ///
    /// # Arguments
    ///
    /// * `height` - usize - Height of the chart
    /// * `width` - usize - Width of the chart
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let symbols = self.optimal_symbols()?;
        let returns = self.performance_stats.portfolio_returns.clone();
        let returns = returns.select(&symbols)?;
        let labels = returns.get_column_names().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let corr_matrix = correlation_matrix(&returns)?;
        let corr_matrix = corr_matrix.outer_iter()
            .map(|row| row.to_vec())
            .collect();
        let heatmap = HeatMap::new(labels.to_vec(), labels.to_vec(), corr_matrix)
            .zmin(-1.0)
            .zmax(1.0)
            .color_scale(ColorScalePalette::Jet.into());

        let mut plot = Plot::new();
        plot.add_trace(heatmap);
        plot.set_layout(
            Layout::new()
                .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Returns Correlation Matrix</span>"))
                .height(height.unwrap_or(DEFAULT_HEIGHT))
                .width(width.unwrap_or(DEFAULT_WIDTH))
        );

        Ok(plot)
    }
}