use std::error::Error;
use plotly::{Bar, Candlestick, Histogram, Layout, Plot, Scatter, Table};
use plotly::common::{Fill, Line, LineShape, Mode, Title};
use plotly::layout::{Axis, GridPattern, LayoutGrid, RangeSelector, RangeSlider, RowOrder, SelectorButton, SelectorStep, StepMode};
use polars::datatypes::AnyValue;
use num_format::{Locale, ToFormattedString};
use crate::analytics::fundamentals::Financials;
use crate::analytics::performance::TickerPerformanceStats;
use crate::analytics::statistics::cumulative_returns_list;
use crate::analytics::technicals::TechnicalIndicators;
use crate::charts::options::OptionCharts;
use crate::data::ticker::{Interval, Ticker};
use crate::utils::date_utils::{generate_dates, to_date};


/// # Ticker Charts Struct
///
/// Helps generate Financial Analytics charts for a given ticker
///
/// # Example
///
/// ```
/// use std::error::Error;
/// use finalytics::data::ticker::Interval;
/// use finalytics::charts::ticker::TickerCharts;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let tc = TickerCharts::new("AAPL", "2019-01-01", "2023-01-01", Interval::OneDay
///     ,"^GSPC", 0.95, 0.02).unwrap();
///     let result = tc.candlestick_chart().await?.show();
///     println!("{:?}", result);
///     let result = tc.performance_chart().await?.show();
///     println!("{:?}", result);
///     let result = tc.performance_stats_table().await?.show();
///     println!("{:?}", result);
///    let fin_plots = tc.financial_statements().await?;
///     for plot in fin_plots {
///         let _ = plot.show();
///     }
///     let result = tc.summary_stats_table().await?.show();
///     println!("{:?}", result);
///    let vol_plots = tc.options_volatility_charts().await?;
///     for plot in vol_plots {
///         let _ = plot.show();
///     }
///     Ok(())
/// }
/// ```
pub struct TickerCharts {
    symbol: String,
    start_date: String,
    end_date: String,
    interval: Interval,
    benchmark_symbol: String,
    confidence_level: f64,
    risk_free_rate: f64,
}


impl TickerCharts {
    /// Creates a new TickerCharts struct
    ///
    /// # Arguments
    ///
    /// * `symbol` - Ticker symbol
    /// * `start_date` - Start date in YYYY-MM-DD format
    /// * `end_date` - End date in YYYY-MM-DD format
    /// * `interval` - Time interval enum
    /// * `benchmark_symbol` - Benchmark ticker symbol
    /// * `confidence_level` - Confidence level for VaR and CVaR in decimal (e.g 0.95 for 95%)
    /// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
    ///
    /// # Returns
    ///
    /// * `TickerCharts` struct
    pub fn new(symbol: &str, start_date: &str, end_date: &str, interval: Interval,
               benchmark_symbol: &str, confidence_level: f64, risk_free_rate: f64) -> Result<TickerCharts, Box<dyn Error>> {
        Ok(TickerCharts {
            symbol: symbol.to_string(),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            interval,
            benchmark_symbol: benchmark_symbol.to_string(),
            confidence_level,
            risk_free_rate,
        })
    }

    /// Generates an OHLCV candlestick chart for the ticker with technical indicators
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub async fn candlestick_chart(&self) -> Result<Plot, Box<dyn Error>> {
        let data = TechnicalIndicators::new(&*self.symbol, &*self.start_date, &*self.end_date,
                                            self.interval).await?;
        let x = data.timestamp.clone().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let open = data.open.clone();
        let high = data.high.clone();
        let low = data.low.clone();
        let close = data.close.clone();
        let volume = data.volume.clone();
        let rsi_values = data.rsi(14)?.column("rsi-14")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let ma_50_values = data.sma(50)?.column("sma-50")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let ma_200_values = data.sma(200)?.column("sma-200")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let candlestick_trace = Candlestick::new(x.clone(), open, high, low, close)
            .name("Prices");
        let volume_trace = Bar::new(x.clone(), volume)
            .name("Volume")
            //.marker(Marker::new().color(NamedColor::Blue))
            .x_axis("x")
            .y_axis("y2");
        let rsi_trace = Scatter::new(x.clone(), rsi_values)
            .name("RSI 14")
            .mode(Mode::Lines)
            .line(Line::new().shape(LineShape::Spline))
            .x_axis("x")
            .y_axis("y3");
        let ma50_trace = Scatter::new(x.clone(), ma_50_values)
            .name("MA 50")
            .mode(Mode::Lines)
            .line(Line::new().shape(LineShape::Spline));
        let ma200_trace = Scatter::new(x.clone(), ma_200_values)
            .name("MA 200")
            .mode(Mode::Lines)
            .line(Line::new().shape(LineShape::Spline));

        let layout = Layout::new()
            .height(800)
            .width(1200)
            .title(Title::new(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{} Candlestick Chart</span>",
                                        self.symbol)))
            .grid(
                LayoutGrid::new()
                    .rows(3)
                    .columns(1)
                    .pattern(GridPattern::Coupled)
                    .row_order(RowOrder::TopToBottom)
            )
            .x_axis(
            Axis::new()
                .range_slider(RangeSlider::new().visible(true))
                .range_selector(RangeSelector::new().buttons(vec![
                    SelectorButton::new()
                        .count(1)
                        .label("1H")
                        .step(SelectorStep::Hour)
                        .step_mode(StepMode::Backward),
                    SelectorButton::new()
                        .count(1)
                        .label("1D")
                        .step(SelectorStep::Day)
                        .step_mode(StepMode::Backward),
                    SelectorButton::new()
                        .count(1)
                        .label("1M")
                        .step(SelectorStep::Month)
                        .step_mode(StepMode::Backward),
                    SelectorButton::new()
                        .count(6)
                        .label("6M")
                        .step(SelectorStep::Month)
                        .step_mode(StepMode::Backward),
                    SelectorButton::new()
                        .count(1)
                        .label("YTD")
                        .step(SelectorStep::Year)
                        .step_mode(StepMode::ToDate),
                    SelectorButton::new()
                        .count(1)
                        .label("1Y")
                        .step(SelectorStep::Year)
                        .step_mode(StepMode::Backward),
                    SelectorButton::new()
                        .label("MAX")
                        .step(SelectorStep::All),
                ])),
            )
            .y_axis(
                Axis::new()
                    .domain(&[0.4, 1.0])
            )
            .y_axis2(
                Axis::new()
                    .domain(&[0.2, 0.4])
            )
            .y_axis3(
                Axis::new()
                    .domain(&[0.0, 0.2])
            );

        let mut plot = Plot::new();
        plot.add_trace(Box::new(candlestick_trace));
        plot.add_trace(volume_trace);
        plot.add_trace(ma50_trace);
        plot.add_trace(ma200_trace);
        plot.add_trace(rsi_trace);
        plot.set_layout(layout);

        Ok(plot)

    }

    /// Generates a performance chart for the ticker
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub async fn performance_chart(&self) -> Result<Plot, Box<dyn Error>> {
        let performance_stats = TickerPerformanceStats::new(
            &*self.symbol, &*self.benchmark_symbol, &*self.start_date, &*self.end_date,
            self.interval, self.confidence_level, self.risk_free_rate).await?.compute_stats()?;
        let dates = generate_dates(&*performance_stats.start_date,
                                   &*performance_stats.end_date, 1);

        let returns = performance_stats.security_returns.f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let benchmark_returns = performance_stats.benchmark_returns.f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let cum_returns= cumulative_returns_list(returns.clone());

        let benchmark_cum_returns= cumulative_returns_list(benchmark_returns.clone());

        let returns_trace = Scatter::new(dates.clone(), returns.clone().iter().map(|x| x/100.0).collect::<Vec<f64>>())
            .name(format!("{} Returns", self.symbol))
            .mode(Mode::Markers)
            .fill(Fill::ToZeroY);

        let returns_dist_trace = Histogram::new(returns.clone().iter().map(|x| x/100.0).collect::<Vec<f64>>())
            .name(format!("{} Returns Distribution", self.symbol))
            .x_axis("x2")
            .y_axis("y2");

        let cum_returns_trace = Scatter::new(dates.clone(), cum_returns.clone())
            .name(format!("{} Cumulative Returns", self.symbol))
            .mode(Mode::Lines)
            .fill(Fill::ToZeroY)
            .x_axis("x3")
            .y_axis("y3");

        let benchmark_cum_returns_trace = Scatter::new(dates.clone(), benchmark_cum_returns.clone())
            .name(format!("{} Cumulative Returns", performance_stats.benchmark_symbol))
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
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{} Performance Chart</span>",
                                         self.symbol)))
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

    /// Displays the Summary Statistics table for the ticker
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub async fn summary_stats_table(&self) -> Result<Plot, Box<dyn Error>> {
        let stats = Ticker::new(&*self.symbol).await?.get_ticker_stats().await?;

        let fields = vec![
            "Symbol".to_string(),
            "Name".to_string(),
            "Exchange".to_string(),
            "Currency".to_string(),
            "Timestamp".to_string(),
            "Price".to_string(),
            "Change (%)".to_string(),
            "Volume".to_string(),
            "Open".to_string(),
            "Day High".to_string(),
            "Day Low".to_string(),
            "Previous Close".to_string(),
            "52 Week High".to_string(),
            "52 Week Low".to_string(),
            "52 Week Change (%)".to_string(),
            "50 Day Average".to_string(),
            "200 Day Average".to_string(),
            "Trailing EPS".to_string(),
            "Current EPS".to_string(),
            "Forward EPS".to_string(),
            "Trailing P/E".to_string(),
            "Current P/E".to_string(),
            "Forward P/E".to_string(),
            "Dividend Rate".to_string(),
            "Dividend Yield".to_string(),
            "Book Value".to_string(),
            "Price to Book".to_string(),
            "Market Cap".to_string(),
            "Shares Outstanding".to_string(),
            "Average Analyst Rating".to_string(),
        ];

        let values = vec![
            format!("{}", stats.symbol),
            format!("{}", stats.display_name),
            format!("{}", stats.full_exchange_name),
            format!("{}", stats.currency),
            format!("{}", to_date(stats.regular_market_time)),
            format!("{:.2}", stats.regular_market_price),
            format!("{:.2}%", stats.regular_market_change_percent),
            format!("{}", (stats.regular_market_volume as i64).to_formatted_string(&Locale::en)),
            format!("{:.2}", stats.regular_market_open),
            format!("{:.2}", stats.regular_market_day_high),
            format!("{:.2}", stats.regular_market_day_low),
            format!("{:.2}", stats.regular_market_previous_close),
            format!("{:.2}", stats.fifty_two_week_high),
            format!("{:.2}", stats.fifty_two_week_low),
            format!("{:.2}", stats.fifty_two_week_change_percent),
            format!("{:.2}", stats.fifty_day_average),
            format!("{:.2}", stats.two_hundred_day_average),
            format!("{:.2}", stats.trailing_eps),
            format!("{:.2}", stats.current_eps),
            format!("{:.2}", stats.eps_forward),
            format!("{:.2}", stats.trailing_pe),
            format!("{:.2}", stats.current_pe),
            format!("{:.2}", stats.forward_pe),
            format!("{:.2}%", stats.dividend_rate),
            format!("{:.2}%", stats.dividend_yield),
            format!("{}", (stats.book_value as i64).to_formatted_string(&Locale::en)),
            format!("{:.2}", stats.price_to_book),
            format!("{}", (stats.market_cap as i64).to_formatted_string(&Locale::en)),
            format!("{}", (stats.shares_outstanding as i64).to_formatted_string(&Locale::en)),
            format!("{}", stats.average_analyst_rating),
        ];

        let trace = Table::new(
            vec![
                format!("<span style=\"font-weight:bold; color:darkgreen;\">{}</span>", "Summary Stats"),
                format!("<span style=\"font-weight:bold; color:darkgreen;\">{}</span>", "Values"),
            ],
            vec![fields, values],
        );
        let mut plot = Plot::new();
        plot.add_trace(trace);

        let layout = Layout::new()
            .height(1000)
            .width(1200)
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{} Summary Stats</span>",
                                         self.symbol)));

        plot.set_layout(layout);

        Ok(plot)
    }

    /// Displays the Performance Statistics table for the ticker
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub async fn performance_stats_table(&self) -> Result<Plot, Box<dyn Error>> {
        let stats = TickerPerformanceStats::new(
            &*self.symbol, &*self.benchmark_symbol, &*self.start_date, &*self.end_date,
            self.interval, self.confidence_level, self.risk_free_rate).await?.compute_stats()?.performance_stats;

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
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{} Performance Stats</span>",
                                         self.symbol)));

        plot.set_layout(layout);

        Ok(plot)
    }

    /// Generates Table Plots for the Ticker's Financial Statements
    ///
    /// # Returns
    ///
    /// * `Vec<Plot>` Vector of Plotly Chart structs
    pub async fn financial_statements(&self) -> Result<Vec<Plot>, Box<dyn Error>> {
        let financials = Financials::new(&*self.symbol).await?;
        let dfs = vec![
            ("Income Statement", financials.format_income_statement()?),
            ("Balance Sheet", financials.format_balance_sheet()?),
            ("Cashflow Statement", financials.format_cashflow_statement()?),
            ("Financial Ratios", financials.compute_ratios()?),
        ];
        let mut plots = Vec::new();

        for (name, df) in dfs.iter() {
            let fields = df.get_column_names();
            let fields= fields.iter().map(|x| x.to_string())
                .collect::<Vec<String>>();
            let values = df
                .iter()
                .map(|x| {
                    x.iter()
                        .map(|y| match y {
                            AnyValue::Null => "".to_string(),
                            AnyValue::Utf8(str_val) => str_val.to_string(),
                            AnyValue::Float64(val) => if name == &"Financial Ratios" {format!("{:.2}", val)}
                            else if val > -999.0 && val < 999.0 {format!("${:.2}", val)}
                            else{format!("${}", (val as i64).to_formatted_string(&Locale::en))
                            },
                            _ => format!("{}", y),
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>();
            let trace = Table::new(
                fields,
                values,
            );
            let mut plot = Plot::new();
            plot.add_trace(trace);

            let layout = Layout::new()
                .height(1000)
                .width(1200)
                .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{} {}</span>",
                                             self.symbol, name)));

            plot.set_layout(layout);
            plots.push(plot);
        };
        Ok(plots)
    }

    /// Generates charts of the ticker's options volatility surface, smile, and term structure
    ///
    /// # Returns
    ///
    /// * `Vec<Plot>` Vector of Plotly Chart structs
    pub async fn options_volatility_charts(&self) -> Result<Vec<Plot>, Box<dyn Error>> {
        let oc = OptionCharts::new(&*self.symbol, self.risk_free_rate).await?;
        let vol_surface = oc.volatility_surface();
        let vol_smile = oc.volatility_smile();
        let vol_cone = oc.volatility_cone();
        let plots = vec![vol_surface, vol_smile, vol_cone];
        Ok(plots)
    }
}