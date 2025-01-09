use std::error::Error;
use polars::prelude::*;
use chrono::{DateTime, NaiveDateTime};
use num_format::{Locale, ToFormattedString};
use plotly::common::{AxisSide, Fill, Line, LineShape, Mode, Title};
use plotly::{Bar, Candlestick, Histogram, Layout, Plot, Scatter, Surface};
use plotly::layout::{Axis, GridPattern, LayoutGrid, LayoutScene, RangeSelector, RangeSlider, RowOrder, SelectorButton, SelectorStep, StepMode};

use crate::models::ticker::Ticker;
use crate::data::ticker::TickerData;
use crate::prelude::StatementFrequency;
use crate::prelude::TechnicalIndicators;
use crate::analytics::fundamentals::Financials;
use crate::analytics::performance::TickerPerformance;
use crate::analytics::stochastics::VolatilitySurface;
use crate::analytics::statistics::{cumulative_returns_list, maximum_drawdown};
use crate::utils::date_utils::to_date;
use crate::reports::table::{DataTable, TableType};
use crate::charts::{DEFAULT_HEIGHT, DEFAULT_WIDTH};


pub struct FinancialsTables {
    pub income_statement: DataTable,
    pub balance_sheet: DataTable,
    pub cashflow_statement: DataTable,
    pub financial_ratios: DataTable,
}

pub struct OptionsCharts {
    pub volatility_surface: Plot,
    pub volatility_smile: Plot,
    pub volatility_term_structure: Plot,
}

pub struct OptionsTables {
    pub options_chain: DataTable,
    pub volatility_surface: DataTable,
}

pub trait TickerCharts {
    fn ohlcv_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn candlestick_chart(&self, height: Option<usize>, width: Option<usize>) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
    fn performance_chart(&self, height: Option<usize>, width: Option<usize>) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
    fn summary_stats_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn performance_stats_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn financials_tables(&self, frequency: StatementFrequency) -> impl std::future::Future<Output = Result<FinancialsTables, Box<dyn Error>>>;
    fn options_charts(&self, height: Option<usize>, width: Option<usize>) -> impl std::future::Future<Output = Result<OptionsCharts, Box<dyn Error>>>;
    fn options_tables(&self) -> impl std::future::Future<Output = Result<OptionsTables, Box<dyn Error>>>;
    fn news_sentiment_chart(&self, height: Option<usize>, width: Option<usize>) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
}

impl TickerCharts for Ticker {
    /// Displays the OHLCV Table for the ticker
    ///
    /// # Returns
    ///
    /// * `DataTable` - Interactive Table Chart struct
    fn ohlcv_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>> {
        async move {
            let data = self.get_chart().await?;
            let table_chart = DataTable::new(data, TableType::OHLCV);
            Ok(table_chart)
        }
    }

    /// Generates an OHLCV candlestick chart for the ticker with technical indicators
    ///
    /// # Arguments
    ///
    /// * `height` - `usize` - Height of the chart
    /// * `width` - `usize` - Width of the chart
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    async fn candlestick_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let data = self.get_chart().await?;
        let x = data.column("timestamp")?.datetime()?.to_vec().iter().map(|x|
            DateTime::from_timestamp_millis( x.unwrap()).unwrap().naive_local()).collect::<Vec<NaiveDateTime>>();
        let x = x.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let open = data.column("open")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let high = data.column("high")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let low = data.column("low")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let close = data.column("close")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let volume = data.column("volume")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let rsi_df = self.rsi(14, None).await?;
        let rsi_values = rsi_df.column("rsi-14")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let ma_50_df = self.sma(50, None).await?;
        let ma_50_values = ma_50_df.column("sma-50")?.f64()?.to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let ma_200_df = self.sma(200, None).await?;
        let ma_200_values = ma_200_df.column("sma-200")?.f64()?.to_vec()
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
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
            .title(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{} Candlestick Chart</span>", self.ticker))
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
    /// # Arguments
    ///
    /// * `height` - `usize` - Height of the chart
    /// * `width` - `usize` - Width of the chart
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    async fn performance_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let performance_stats = self.performance_stats().await?;
        let dates = performance_stats.dates_array;
        let returns = performance_stats.security_returns.clone().f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let benchmark_returns = performance_stats.benchmark_returns.f64().unwrap().to_vec()
            .iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

        let cum_returns= cumulative_returns_list(returns.clone());

        let benchmark_cum_returns= cumulative_returns_list(benchmark_returns.clone());

        let (drawdowns, _) = maximum_drawdown(&performance_stats.security_returns);
        let drawdowns = drawdowns.iter().map(|x| x/100.0).collect::<Vec<f64>>();

        let returns_trace = Scatter::new(dates.clone(), returns.clone().iter().map(|x| x/100.0).collect::<Vec<f64>>())
            .name(format!("{} Returns", self.ticker))
            .mode(Mode::Markers)
            .fill(Fill::ToZeroY);

        let returns_dist_trace = Histogram::new(returns.clone().iter().map(|x| x/100.0).collect::<Vec<f64>>())
            .name(format!("{} Returns Distribution", self.ticker))
            .x_axis("x2")
            .y_axis("y2");

        let cum_returns_trace = Scatter::new(dates.clone(), cum_returns.clone())
            .name(format!("{} Cumulative Returns", self.ticker))
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

        let drawdown_trace = Scatter::new(dates.clone(), drawdowns.clone())
            .name(format!("{} Drawdown", self.ticker))
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
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{} Performance Chart</span>",
                                         self.ticker)))
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

    /// Displays the Summary Statistics table for the ticker
    ///
    /// # Returns
    ///
    /// * `DataTable` - Table Chart struct
    async fn summary_stats_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let stats = self.get_ticker_stats().await?;

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
            format!("{}", stats.long_name),
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

        let df = DataFrame::new(vec![
            Series::new("Items", fields),
            Series::new("Values", values),
        ])?;

        let data_table = DataTable::new(df, TableType::SummaryStats);

        Ok(data_table)
    }

    /// Displays the Performance Statistics table for the ticker
    ///
    /// # Returns
    ///
    /// * `DataTable` - Table Chart struct
    async fn performance_stats_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let stats = self.performance_stats().await?;

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
            format!("{:.2}%",stats.performance_stats.daily_return),
            format!("{:.2}%",stats.performance_stats.daily_volatility),
            format!("{:.2}%",stats.performance_stats.cumulative_return),
            format!("{:.2}%",stats.performance_stats.annualized_return),
            format!("{:.2}%",stats.performance_stats.annualized_volatility),
            format!("{:.2}",stats.performance_stats.alpha),
            format!("{:.2}",stats.performance_stats.beta),
            format!("{:.2}",stats.performance_stats.sharpe_ratio),
            format!("{:.2}",stats.performance_stats.sortino_ratio),
            format!("{:.2}%",stats.performance_stats.active_return),
            format!("{:.2}%",stats.performance_stats.active_risk),
            format!("{:.2}",stats.performance_stats.information_ratio),
            format!("{:.2}",stats.performance_stats.calmar_ratio),
            format!("{:.2}%",stats.performance_stats.maximum_drawdown),
            format!("{:.2}%",stats.performance_stats.value_at_risk),
            format!("{:.2}%",stats.performance_stats.expected_shortfall),
        ];

        let df = DataFrame::new(vec![
            Series::new("Items", fields),
            Series::new("Values", values),
        ])?;

        let data_table = DataTable::new(df, TableType::PerformanceStats);

        Ok(data_table)
    }

    /// Generates Table Plots for the Ticker's Financial Statements
    ///
    /// # Arguments
    /// * `frequency` - `StatementFrequency` - Frequency of the Financial Statements
    ///
    /// # Returns
    ///
    /// * `FinancialsTables` - Financials Tables struct
    async fn financials_tables(&self, frequency: StatementFrequency) -> Result<FinancialsTables, Box<dyn Error>> {
        let data = self.income_statement(frequency).await?;
        let table_type = match frequency {
            StatementFrequency::Annual => TableType::AnnualIncomeStatement,
            StatementFrequency::Quarterly => TableType::QuarterlyIncomeStatement,
        };
        let income_statement = DataTable::new(data, table_type);

        let data = self.balance_sheet(frequency).await?;
        let table_type = match frequency {
            StatementFrequency::Annual => TableType::AnnualBalanceSheet,
            StatementFrequency::Quarterly => TableType::QuarterlyBalanceSheet,
        };
        let balance_sheet = DataTable::new(data, table_type);

        let data = self.cashflow_statement(frequency).await?;
        let table_type = match frequency {
            StatementFrequency::Annual => TableType::AnnualCashflowStatement,
            StatementFrequency::Quarterly => TableType::QuarterlyCashflowStatement,
        };
        let cashflow_statement = DataTable::new(data, table_type);

        let data = self.financial_ratios(frequency).await?;
        let table_type = match frequency {
            StatementFrequency::Annual => TableType::AnnualFinancialRatios,
            StatementFrequency::Quarterly => TableType::QuarterlyFinancialRatios,
        };
        let financial_ratios = DataTable::new(data, table_type);

        Ok(FinancialsTables {
            income_statement,
            balance_sheet,
            cashflow_statement,
            financial_ratios,
        })
    }

    /// Generates Charts of the Ticker's Option Volatility Surface, Smile, and Term Structure
    ///
    /// # Arguments
    ///
    /// * `height` - `usize` - Height of the chart
    /// * `width` - `usize` - Width of the chart
    ///
    /// # Returns
    ///
    /// * `OptionsCharts` - Options Charts struct
    async fn options_charts(&self, height: Option<usize>, width: Option<usize>) -> Result<OptionsCharts, Box<dyn Error>> {
        let vol_surface = self.volatility_surface().await?;
        let symbol = vol_surface.symbol;
        let ivols = vol_surface.ivols;
        let strikes = vol_surface.strikes;
        let ttms = vol_surface.ttms;

        // Volatility Surface
        let trace = Surface::new(ivols.clone()).x(strikes.clone()).y(ttms.clone());
        let mut surface_plot = Plot::new();
        surface_plot.add_trace(trace);

        let layout = Layout::new()
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{symbol} Volatility Surface</span>")))
            .scene(
                LayoutScene::new()
                    .x_axis(
                        Axis::new()
                            .title(Title::from("Strike"))
                    )
                    .y_axis(
                        Axis::new()
                            .title(Title::from("Time to Maturity"))
                    )
                    .z_axis(Axis::new()
                        .title(Title::from("Implied Volatility")))

            );
        surface_plot.set_layout(layout);

        // Volatility Smile
        let mut traces = Vec::new();

        for (index, ttm) in ttms.iter().enumerate() {
            let ivols = ivols[index].clone();
            let trace = Scatter::new(strikes.clone(), ivols)
                .mode(Mode::LinesMarkers)
                .line(Line::new().shape(LineShape::Spline))
                .name(&*format!("Volatility Smile - {:.1} Months Expiration", ttm));

            traces.push(trace);
        }

        let layout = Layout::new()
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{symbol} Volatility Smile</span>")))
            .x_axis(Axis::new().title(Title::from("Strike")))
            .y_axis(Axis::new().title(Title::from("Implied Volatility")));

        let mut smile_plot = Plot::new();
        for trace in traces {
            smile_plot.add_trace(trace);
        }
        smile_plot.set_layout(layout);


        // Volatility Term Structure
        let rows = ivols[0].len();
        let cols = ivols.len();
        let mut strike_vols: Vec<Vec<f64>>= vec![vec![Default::default(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                strike_vols[i][j] = ivols[j][i].clone();
            }
        }
        let mut traces = Vec::new();


        for (index, strike) in strikes.iter().enumerate() {
            let ivols = strike_vols[index].clone();
            let trace = Scatter::new(ttms.clone(), ivols)
                .mode(Mode::LinesMarkers)
                .line(Line::new().shape(LineShape::Spline))
                .name(&*format!("Volatility Cone - {} Strike", strike));

            traces.push(trace);
        }

        let layout = Layout::new()
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{symbol} Volatility Term Structure</span>")))
            .x_axis(Axis::new().title(Title::from("Time to Maturity (Months)")))
            .y_axis(Axis::new().title(Title::from("Implied Volatility")));

        let mut term_plot = Plot::new();
        for trace in traces {
            term_plot.add_trace(trace);
        }
        term_plot.set_layout(layout);


        Ok(OptionsCharts {
            volatility_surface: surface_plot,
            volatility_smile: smile_plot,
            volatility_term_structure: term_plot,
        })
    }

    /// Generates Tables of the Ticker's Options Chain and Volatility Surface Data
    ///
    /// # Returns
    ///
    /// * `OptionsTables` - Options Tables struct
    async fn options_tables(&self) -> Result<OptionsTables, Box<dyn Error>> {
        // Options Chain
        let data = self.get_options().await?.chain;
        let options_chain = DataTable::new(data, TableType::OptionsChain);

        // Volatility Surface
        let data = self.volatility_surface().await?.ivols_df;
        let volatility_surface = DataTable::new(data, TableType::VolatilitySurface);

        Ok(OptionsTables {
            options_chain,
            volatility_surface,
        })
    }

    /// Generates a News Sentiment Chart for the Ticker
    ///
    /// # Arguments
    ///
    /// * `height` - `Option<usize>` - Height of the chart
    /// * `width` - `Option<usize>` - Width of the chart
    ///
    /// # Returns
    ///
    /// * `Plot` - Plotly Chart struct
    async fn news_sentiment_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let data = self.get_news().await?;
        let data = data.lazy()
            .with_column(col("Published Date").dt().date().alias("Published Date"));
        let grouped = data.clone().lazy().group_by_stable([col("Published Date")])
            .agg([
                col("Sentiment Score").mean().alias("Average Sentiment Score"),
                col("Sentiment Score").count().alias("Number of Articles"),
            ]).collect()?;
        let grouped = grouped.sort(&["Published Date"], SortMultipleOptions::new().with_order_descending(false))?
            .lazy()
            .with_column(col("Published Date").cast(DataType::Datetime(TimeUnit::Milliseconds, None)).alias("Published Date"))
            .collect()?;


        // Convert to Vec for plotting
        let dates = grouped.column("Published Date")?.datetime()?
            .into_no_null_iter().map(|x| DateTime::from_timestamp_millis(x).unwrap()
            .naive_local().date().to_string()).collect::<Vec<_>>();
        let scores = grouped.column("Average Sentiment Score")?.f64()?.into_no_null_iter().collect::<Vec<_>>();
        let counts = grouped.column("Number of Articles")?.u32()?.into_no_null_iter().collect::<Vec<_>>();

        // Create Plotly traces
        let bar_trace = Bar::new(dates.clone(), counts)
            .name("Articles Count")
            .opacity(0.7);

        let line_trace = Scatter::new(dates, scores)
            .mode(Mode::LinesMarkers)
            .name("Sentiment Score")
            .y_axis("y2");

        // Create the Plotly plot
        let mut plot = Plot::new();
        plot.add_trace(bar_trace);
        plot.add_trace(line_trace);

        // Set the layout
        let layout = Layout::new()
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{} News Sentiment Chart</span>", &self.ticker)))
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
            //.bar_mode(BarMode::Group)
            .x_axis(Axis::new()
                .title("Published Date")
                .color("purple")
                .show_grid(false))
            .y_axis(Axis::new()
                .title("Number of Articles")
                .color("purple")
                .show_grid(false))
            .y_axis2(Axis::new()
                .title("Average Sentiment Score")
                .color("purple")
                .show_grid(false)
                .overlaying("y")
                .side(AxisSide::Right)
            );

        plot.set_layout(layout);

        Ok(plot)
    }
}
