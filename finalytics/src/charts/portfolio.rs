use std::error::Error;
use plotly::{Bar, Histogram, Layout, Plot, Scatter, Table};
use plotly::color::NamedColor;
use plotly::common::{Fill, Marker, MarkerSymbol, Mode, Title};
use plotly::layout::{Axis, GridPattern, LayoutGrid, RowOrder};
use plotly::traces::table::{Cells, Header};
use crate::analytics::statistics::cumulative_returns_list;
use crate::models::portfolio::Portfolio;


pub trait PortfolioCharts {
    fn optimization_chart(&self, height: usize, width: usize) -> Result<Plot, Box<dyn Error>>;
    fn performance_chart(&self, height: usize, width: usize) -> Result<Plot, Box<dyn Error>>;
    fn asset_returns_chart(&self, height: usize, width: usize) -> Result<Plot, Box<dyn Error>>;
    fn performance_stats_table(&self, height: usize, width: usize) -> Result<Plot, Box<dyn Error>>;
}

impl PortfolioCharts for Portfolio {
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
    fn optimization_chart(&self, height: usize, width: usize) -> Result<Plot, Box<dyn Error>> {
        let days = self.performance_stats.interval.to_days();

        let ef_returns = self.performance_stats.efficient_frontier.clone().iter()
            .map(|x| ((1.0 + (x[0]/days)/100.0).powf(252.0) - 1.0)).collect::<Vec<f64>>();

        let ef_risk = self.performance_stats.efficient_frontier.clone().iter()
            .map(|x| (x[1]/100.0 * 252.0_f64.sqrt())).collect::<Vec<f64>>();

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

        let filtered: Vec<_> = ticker_symbols.iter()
            .zip(weights.iter())
            .filter(|&(_, &weight)| weight > 0.0)
            .collect();

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
            .height(height)
            .width(width)
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
    fn performance_chart(&self, height: usize, width: usize) -> Result<Plot, Box<dyn Error>> {
        let dates = self.performance_stats.dates_array.clone();

        let returns = self.performance_stats.optimal_portfolio_returns.f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let benchmark_returns = self.performance_stats.benchmark_returns.f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let cum_returns= cumulative_returns_list(returns.clone());

        let benchmark_cum_returns= cumulative_returns_list(benchmark_returns.clone());

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

        let mut plot = Plot::new();
        plot.add_trace(returns_trace);
        plot.add_trace(returns_dist_trace);
        plot.add_trace(cum_returns_trace);
        plot.add_trace(benchmark_cum_returns_trace);

        // Set layout for the plot
        let layout = Layout::new()
            .height(height)
            .width(width)
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
            );

        plot.set_layout(layout);

        Ok(plot)
    }

    /// Generates Chart of the Portfolio Asset Returns
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    fn asset_returns_chart(&self, height: usize, width: usize) -> Result<Plot, Box<dyn Error>> {
        let symbols = self.performance_stats.ticker_symbols.clone();
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
            .height(height)
            .width(width)
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Assets Cumulative Returns</span>"))
            .y_axis(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
                    .tick_format(".0%")
            );

        plot.set_layout(layout);
        Ok(plot)
    }

    /// Displays the Performance Statistics table for the portfolio
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    fn performance_stats_table(&self, height: usize, width: usize) -> Result<Plot, Box<dyn Error>> {
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
            Header::new(vec![
                format!("<span style=\"font-weight:bold; color:darkgreen;\">{}</span>", "Performance Stats"),
                format!("<span style=\"font-weight:bold; color:darkgreen;\">{}</span>", "Values"),
            ]),
            Cells::new(vec![fields, values]),
        );
        let mut plot = Plot::new();
        plot.add_trace(trace);

        let layout = Layout::new()
            .height(height)
            .width(width)
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\"> Portfolio Performance Stats</span>"));

        plot.set_layout(layout);

        Ok(plot)
    }
}