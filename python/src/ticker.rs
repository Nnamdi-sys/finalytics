use polars::export::chrono;
use tokio::task;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use finalytics::prelude::*;
use crate::ffi::{rust_df_to_py_df, rust_plot_to_py_plot, rust_series_to_py_series};

#[pyclass]
#[pyo3(name = "Ticker")]
pub struct PyTicker {
    ticker: Ticker
}


#[pymethods]
impl PyTicker {
    #[new]
    /// Create a new Ticker object
    ///
    /// # Arguments
    ///
    /// * `symbol` - `str` - The ticker symbol of the asset
    /// * `start_date` - `str` - The start date of the time period in the format YYYY-MM-DD
    /// * `end_date` - `str` - The end date of the time period in the format YYYY-MM-DD
    /// * `interval` - `str` - The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo)
    /// * `benchmark_symbol` - `str` - The ticker symbol of the benchmark to compare against
    /// * `confidence_level` - `float` - The confidence level for the VaR and ES calculations
    /// * `risk_free_rate` - `float` - The risk free rate to use in the calculations
    ///
    /// # Returns
    ///
    /// `Ticker` - A Ticker object
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker(symbol="AAPL", start_date="2020-01-01", end_date="2021-01-01", interval="1d",
    /// benchmark_symbol="^GSPC", confidence_level=0.95, risk_free_rate=0.02)
    /// ```
    pub fn new(symbol: &str, start_date: Option<String>, end_date: Option<String>, interval: Option<String>, benchmark_symbol: Option<String>,
    confidence_level: Option<f64>, risk_free_rate: Option<f64>) -> Self {
        let default_start = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(365))
            .unwrap().format("%Y-%m-%d").to_string();
        let defualt_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let interval = Interval::from_str(&interval.unwrap_or("1d".to_string()));
        task::block_in_place(move || {
            let ticker = TickerBuilder::new()
                .ticker(symbol)
                .start_date(&start_date.unwrap_or(default_start))
                .end_date(&end_date.unwrap_or(defualt_end))
                .interval(interval)
                .benchmark_symbol(&benchmark_symbol.unwrap_or("^GSPC".to_string()))
                .confidence_level(confidence_level.unwrap_or(0.95))
                .risk_free_rate(risk_free_rate.unwrap_or(0.02))
                .build();
            PyTicker {
                ticker
            }
        })
    }

    /// Get the current ticker quote stats
    ///
    /// # Returns
    ///
    /// `dict` - The current ticker quote stats
    pub fn get_quote(&self) -> Py<PyDict> {
        task::block_in_place(move || {
            let quote = tokio::runtime::Runtime::new().unwrap().block_on(self.ticker.get_quote()).unwrap();
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("Symbol", quote.symbol).unwrap();
                locals.set_item("Name", quote.name).unwrap();
                locals.set_item("Exchange", quote.exchange).unwrap();
                locals.set_item("Currency", quote.currency).unwrap();
                locals.set_item("Timestamp", quote.timestamp).unwrap();
                locals.set_item("Current Price", quote.price).unwrap();
                locals.set_item("24H Volume", quote.volume).unwrap();
                locals.set_item("24H Open", quote.open).unwrap();
                locals.set_item("24H High", quote.high).unwrap();
                locals.set_item("24H Low", quote.low).unwrap();
                locals.set_item("24H Close", quote.close).unwrap();
                locals.into()
            })
        })
    }

    /// Get summary technical and fundamental statistics for the ticker
    ///
    /// # Returns
    ///
    /// `dict` - A dictionary containing the summary statistics
    pub fn get_summary_stats(&self) -> Py<PyDict>  {
        task::block_in_place(move || {
            let ticker_stats = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_ticker_stats()
            ).unwrap();
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("Symbol", ticker_stats.symbol).unwrap();
                locals.set_item("Name", ticker_stats.long_name).unwrap();
                locals.set_item("Exchange", ticker_stats.full_exchange_name).unwrap();
                locals.set_item("Currency", ticker_stats.currency).unwrap();
                locals.set_item("Timestamp", ticker_stats.regular_market_time).unwrap();
                locals.set_item("Current Price", ticker_stats.regular_market_price).unwrap();
                locals.set_item("24H Change", ticker_stats.regular_market_change_percent).unwrap();
                locals.set_item("24H Volume", ticker_stats.regular_market_volume).unwrap();
                locals.set_item("24H Open", ticker_stats.regular_market_open).unwrap();
                locals.set_item("24H High", ticker_stats.regular_market_day_high).unwrap();
                locals.set_item("24H Low", ticker_stats.regular_market_day_low).unwrap();
                locals.set_item("24H Close", ticker_stats.regular_market_previous_close).unwrap();
                locals.set_item("52 Week High", ticker_stats.fifty_two_week_high).unwrap();
                locals.set_item("52 Week Low", ticker_stats.fifty_two_week_low).unwrap();
                locals.set_item("52 Week Change", ticker_stats.fifty_two_week_change_percent).unwrap();
                locals.set_item("50 Day Average", ticker_stats.fifty_day_average).unwrap();
                locals.set_item("200 Day Average", ticker_stats.two_hundred_day_average).unwrap();
                locals.set_item("Trailing EPS", ticker_stats.trailing_eps).unwrap();
                locals.set_item("Current EPS", ticker_stats.current_eps).unwrap();
                locals.set_item("Forward EPS", ticker_stats.eps_forward).unwrap();
                locals.set_item("Trailing P/E", ticker_stats.trailing_pe).unwrap();
                locals.set_item("Current P/E", ticker_stats.current_pe).unwrap();
                locals.set_item("Forward P/E", ticker_stats.forward_pe).unwrap();
                locals.set_item("Dividend Rate", ticker_stats.dividend_rate).unwrap();
                locals.set_item("Dividend Yield", ticker_stats.dividend_yield).unwrap();
                locals.set_item("Book Value", ticker_stats.book_value).unwrap();
                locals.set_item("Price to Book", ticker_stats.price_to_book).unwrap();
                locals.set_item("Market Cap", ticker_stats.market_cap).unwrap();
                locals.set_item("Shares Outstanding", ticker_stats.shares_outstanding).unwrap();
                locals.set_item("Average Analyst Rating", ticker_stats.average_analyst_rating).unwrap();
                locals.into()
            })
        })
    }

    /// Get the ohlcv data for the ticker for a given time period
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the ohlcv data
    pub fn get_price_history(&self) -> PyObject {
        task::block_in_place(move || {
            let price_history = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_chart()
            ).unwrap();
            let df = rust_df_to_py_df(&price_history).unwrap();
            df
        })
    }

    /// Get the options chain for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the options chain
    pub fn get_options_chain(&self) -> PyObject {
        task::block_in_place(move || {
            let options_chain = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_options()
            ).unwrap().chain;
            let df = rust_df_to_py_df(&options_chain).unwrap();
            df
        })
    }

    /// Get the latest news for the given ticker
    ///
    /// # Arguments
    ///
    /// * `compute_sentiment` - `bool` - Whether to compute the sentiment of the news articles (set to false to speed up the process)
    ///
    /// # Returns
    ///
    /// `dict` - A dictionary containing the news articles (and sentiment results if requested)
    pub fn get_news(&self, compute_sentiment: bool) -> PyObject {
        task::block_in_place(move || {
            let news = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_news(compute_sentiment)
            ).unwrap();

            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                for (i, article) in news.iter().enumerate() {
                    let article_dict = PyDict::new(py);
                    article_dict.set_item("Title", article.title.clone()).unwrap();
                    article_dict.set_item("Source", article.source.clone()).unwrap();
                    article_dict.set_item("Link", article.link.clone()).unwrap();
                    article_dict.set_item("Timestamp", article.timestamp.clone()).unwrap();
                    article_dict.set_item("Text", article.text.clone()).unwrap();
                    article_dict.set_item("Sentiment Score", article.sentiment_score).unwrap();
                    article_dict.set_item("Positive Score", article.positive_score).unwrap();
                    article_dict.set_item("Negative Score", article.negative_score).unwrap();
                    article_dict.set_item("Positive Keywords", article.positive_keywords.clone()).unwrap();
                    article_dict.set_item("Negative Keywords", article.negative_keywords.clone()).unwrap();
                    locals.set_item(i, article_dict).unwrap();
                }
                locals.into()
            })
        })
    }

    /// Get the Income Statement for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Income Statement
    pub fn get_income_statement(&self) -> PyObject {
        task::block_in_place(move || {
            let income_statement = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.income_statement()).unwrap();
            let df = rust_df_to_py_df(&income_statement).unwrap();
            df
        })
    }

    /// Get the Balance Sheet for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Balance Sheet
    pub fn get_balance_sheet(&self) -> PyObject {
        task::block_in_place(move || {
            let balance_sheet = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.balance_sheet()).unwrap();
            let df = rust_df_to_py_df(&balance_sheet).unwrap();
            df
        })
    }

    /// Get the Cashflow Statement for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Cashflow Statement
    pub fn get_cashflow_statement(&self) -> PyObject {
        task::block_in_place(move || {
            let cashflow_statement = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.cashflow_statement()).unwrap();
            let df = rust_df_to_py_df(&cashflow_statement).unwrap();
            df
        })
    }

    /// Get the Financial Ratios for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Financial Ratios
    pub fn get_financial_ratios(&self) -> PyObject {
        task::block_in_place(move || {
            let ratios = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.financial_ratios()).unwrap();
            let df = rust_df_to_py_df(&ratios).unwrap();
            df
        })
    }

    /// Get the implied volatility surface for the ticker options chain
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the implied volatility surface

    pub fn volatility_surface(&self) -> PyObject {
        task::block_in_place(move || {
            let volatility_surface = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.volatility_surface()).unwrap();
            let df = rust_df_to_py_df(&volatility_surface.ivols_df).unwrap();
            df
        })
    }

    /// Compute the performance statistics for the ticker
    ///
    /// # Returns
    ///
    /// `dict` - A dictionary containing the performance statistics
    pub fn performance_stats(&self) -> PyObject {
        task::block_in_place(move || {
            let performance_stats = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.performance_stats()).unwrap();
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("Symbol", performance_stats.ticker_symbol).unwrap();
                locals.set_item("Benchmark", performance_stats.benchmark_symbol).unwrap();
                locals.set_item("Start Date", performance_stats.start_date).unwrap();
                locals.set_item("End Date", performance_stats.end_date).unwrap();
                locals.set_item("Interval", performance_stats.interval.to_string()).unwrap();
                locals.set_item("Confidence Level", performance_stats.confidence_level).unwrap();
                locals.set_item("Risk Free Rate", performance_stats.risk_free_rate).unwrap();
                locals.set_item("Daily Return", performance_stats.performance_stats.daily_return).unwrap();
                locals.set_item("Daily Volatility", performance_stats.performance_stats.daily_volatility).unwrap();
                locals.set_item("Total Return", performance_stats.performance_stats.cumulative_return).unwrap();
                locals.set_item("Annualized Return", performance_stats.performance_stats.annualized_return).unwrap();
                locals.set_item("Annualized Volatility", performance_stats.performance_stats.annualized_volatility).unwrap();
                locals.set_item("Alpha", performance_stats.performance_stats.alpha).unwrap();
                locals.set_item("Beta", performance_stats.performance_stats.beta).unwrap();
                locals.set_item("Sharpe Ratio", performance_stats.performance_stats.sharpe_ratio).unwrap();
                locals.set_item("Sortino Ratio", performance_stats.performance_stats.sortino_ratio).unwrap();
                locals.set_item("Active Return", performance_stats.performance_stats.active_return).unwrap();
                locals.set_item("Active Risk", performance_stats.performance_stats.active_risk).unwrap();
                locals.set_item("Information Ratio", performance_stats.performance_stats.information_ratio).unwrap();
                locals.set_item("Calmar Ratio", performance_stats.performance_stats.calmar_ratio).unwrap();
                locals.set_item("Maximum Drawdown", performance_stats.performance_stats.maximum_drawdown).unwrap();
                locals.set_item("Value at Risk", performance_stats.performance_stats.value_at_risk).unwrap();
                locals.set_item("Expected Shortfall", performance_stats.performance_stats.expected_shortfall).unwrap();
                locals.set_item("Security Prices", rust_series_to_py_series(&performance_stats.security_prices).unwrap()).unwrap();
                locals.set_item("Security Returns", rust_series_to_py_series(&performance_stats.security_returns).unwrap()).unwrap();
                locals.set_item("Benchmark Returns", rust_series_to_py_series(&performance_stats.benchmark_returns).unwrap()).unwrap();
                locals.into()
            })
        })
    }

    /// Display the performance chart for the ticker
    ///
    /// # Arguments
    ///
    /// * `height` - `int` - The height of the chart
    /// * `width` - `int` - The width of the chart
    ///
    /// # Returns
    ///
    /// `Plot` object
    pub fn performance_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        let plot = task::block_in_place(move || {
            let performance_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.performance_chart(height.unwrap_or(800), width.unwrap_or(1200))).unwrap();
            performance_chart
        });

        rust_plot_to_py_plot(plot).unwrap()
    }

    /// Display the candlestick chart for the ticker
    ///
    /// # Arguments
    ///
    /// * `height` - `int` - The height of the chart
    /// * `width` - `int` - The width of the chart
    ///
    /// # Returns
    ///
    /// `Plot` object
    pub fn candlestick_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject  {
        let plot = task::block_in_place(move || {
            let candlestick_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.candlestick_chart(height.unwrap_or(800), width.unwrap_or(1200))).unwrap();
            candlestick_chart
        });

        rust_plot_to_py_plot(plot).unwrap()
    }

    /// Display the options volatility surface, smile and term structure charts for the ticker
    ///
    /// # Arguments
    ///
    /// * `chart_type` - `str` - The type of chart to display (surface, smile, term_structure)
    /// * `height` - `int` - The height of the chart
    /// * `width` - `int` - The width of the chart
    ///
    /// # Returns
    ///
    /// `Plot` object
    pub fn options_chart(&self, chart_type: String, height: Option<usize>, width: Option<usize>) -> PyObject {
        let plot = task::block_in_place(move || {

            let options_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.options_charts(height.unwrap_or(800), width.unwrap_or(1200))).unwrap();

            match chart_type.as_str() {
                "surface" => options_chart.get("Volatility Surface").unwrap().clone(),
                "smile" => options_chart.get("Volatility Smile").unwrap().clone(),
                "term_structure" => options_chart.get("Volatility Term Structure").unwrap().clone(),
                _ => panic!("Invalid chart type. Please choose either 'surface', 'smile' or 'term_structure'"),
            }

        });

        rust_plot_to_py_plot(plot).unwrap()
    }
}

