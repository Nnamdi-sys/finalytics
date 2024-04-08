use tokio::task;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use finalytics::prelude::*;
use crate::ffi::{rust_df_to_py_df, rust_series_to_py_series, display_html_with_iframe};

#[pyclass]
#[pyo3(name = "Ticker")]
pub struct PyTicker {
    #[pyo3(get, set)]
    pub symbol: String,
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub category: String,
    #[pyo3(get, set)]
    pub asset_class: String,
    #[pyo3(get, set)]
    pub exchange: String,
}


#[pymethods]
impl PyTicker {
    #[new]
    /// Create a new Ticker object
    ///
    /// # Arguments
    ///
    /// * `symbol` - `str` - The ticker symbol of the asset
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
    /// ticker = finalytics.Ticker("AAPL")
    /// print(ticker.symbol, ticker.name, ticker.category, ticker.asset_class, ticker.exchange)
    /// ```
    pub fn new(symbol: &str) -> Self {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(symbol).unwrap().build().unwrap();
            PyTicker {
                symbol: ticker.ticker.symbol,
                name: ticker.ticker.name,
                category: ticker.ticker.category,
                asset_class: ticker.ticker.asset_class,
                exchange: ticker.ticker.exchange,
            }
        })
    }

    /// Get the current price of the ticker
    ///
    /// # Returns
    ///
    /// `float` - The current price of the ticker
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// current_price = ticker.get_current_price()
    /// ```
    pub fn get_current_price(&self) -> f64 {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().build().unwrap();
            let current_price = tokio::runtime::Runtime::new().unwrap().block_on(ticker.get_quote()).unwrap();
            current_price
        })
    }

    /// Get summary technical and fundamental statistics for the ticker
    ///
    /// # Returns
    ///
    /// `dict` - A dictionary containing the summary statistics
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// summary_stats = ticker.get_summary_stats()
    /// ```
    pub fn get_summary_stats(&self) -> Py<PyDict>  {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().build().unwrap();
            let ticker_stats = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.get_ticker_stats()
            ).unwrap();
            // convert ticker stats to pydict
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("Symbol", ticker_stats.symbol).unwrap();
                locals.set_item("Name", ticker_stats.display_name).unwrap();
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
    /// # Arguments
    ///
    /// * `start` - `str` - The start date of the time period in the format YYYY-MM-DD
    /// * `end` - `str` - The end date of the time period in the format YYYY-MM-DD
    /// * `interval` - `str` - The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo)
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the ohlcv data
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// ohlcv = ticker.get_price_history("2020-01-01", "2020-12-31", "1d")
    /// ```
    pub fn get_price_history(&self, start: String, end: String, interval: String) -> PyObject {
        task::block_in_place(move || {
            let interval = Interval::from_str(&interval);
            let price_history = tokio::runtime::Runtime::new().unwrap().block_on(
                TickerBuilder::new().ticker(&self.symbol).unwrap().start_date(&start).end_date(&end)
                    .interval(interval).build().unwrap().get_chart()
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
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// options_chain = ticker.get_options_chain()
    /// ```
    pub fn get_options_chain(&self) -> PyObject {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().build().unwrap();
            let options_chain = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.get_options()
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
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// news = ticker.get_news(False)
    /// ```
    pub fn get_news(&self, compute_sentiment: bool) -> PyObject {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().build().unwrap();
            let news = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.get_news(compute_sentiment)
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
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// income_statement = ticker.get_income_statement()
    /// ```
    pub fn get_income_statement(&self) -> PyObject {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().build().unwrap();
            let income_statement = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.income_statement()).unwrap();
            let df = rust_df_to_py_df(&income_statement).unwrap();
            df
        })
    }

    /// Get the Balance Sheet for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Balance Sheet
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// balance_sheet = ticker.get_balance_sheet()
    /// ```
    pub fn get_balance_sheet(&self) -> PyObject {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().build().unwrap();
            let balance_sheet = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.balance_sheet()).unwrap();
            let df = rust_df_to_py_df(&balance_sheet).unwrap();
            df
        })
    }

    /// Get the Cashflow Statement for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Cashflow Statement
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// cashflow_statement = ticker.get_cashflow_statement()
    /// ```
    pub fn get_cashflow_statement(&self) -> PyObject {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().build().unwrap();
            let cashflow_statement = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.cashflow_statement()).unwrap();
            let df = rust_df_to_py_df(&cashflow_statement).unwrap();
            df
        })
    }

    /// Get the Financial Ratios for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Financial Ratios
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// financial_ratios = ticker.get_financial_ratios()
    /// ```
    pub fn get_financial_ratios(&self) -> PyObject {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().build().unwrap();
            let ratios = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.financial_ratios()).unwrap();
            let df = rust_df_to_py_df(&ratios).unwrap();
            df
        })
    }

    /// Compute the performance statistics for the ticker
    ///
    /// # Arguments
    ///
    /// * `start` - `str` - The start date of the time period in the format YYYY-MM-DD
    /// * `end` - `str` - The end date of the time period in the format YYYY-MM-DD
    /// * `interval` - `str` - The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo)
    /// * `benchmark` - `str` - The ticker symbol of the benchmark to compare against
    /// * `confidence_level` - `float` - The confidence level for the VaR and ES calculations
    /// * `risk_free_rate` - `float` - The risk free rate to use in the calculations
    ///
    /// # Returns
    ///
    /// `dict` - A dictionary containing the performance statistics
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// performance_stats = ticker.compute_performance_stats("2020-01-01", "2020-12-31", "1d", "^GSPC", 0.95, 0.02)
    /// ```
    pub fn compute_performance_stats(&self, start: String, end: String, interval: String, benchmark: String,
                                     confidence_level: f64, risk_free_rate: f64) -> PyObject {
        task::block_in_place(move || {
            let interval = Interval::from_str(&interval);
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().start_date(&start).end_date(&end)
                .interval(interval).benchmark_symbol(&benchmark).confidence_level(confidence_level)
                .risk_free_rate(risk_free_rate).build().unwrap();
            let performance_stats = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.performance_stats()).unwrap();
            // convert ticker performance stats struct to pydict
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
    /// * `start` - `str` - The start date of the time period in the format YYYY-MM-DD
    /// * `end` - `str` - The end date of the time period in the format YYYY-MM-DD
    /// * `interval` - `str` - The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo)
    /// * `benchmark` - `str` - The ticker symbol of the benchmark to compare against
    /// * `confidence_level` - `float` - The confidence level for the VaR and ES calculations
    /// * `risk_free_rate` - `float` - The risk free rate to use in the calculations
    /// * `display_format` - `str` - The format to display the chart in (png, html, notebook)
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// ticker.display_performance_chart("2020-01-01", "2020-12-31", "1d", "^GSPC", 0.95, 0.02, "html")
    /// ```
    pub fn display_performance_chart(&self, start: String, end: String, interval: String, benchmark: String,
                                     confidence_level: f64, risk_free_rate: f64, display_format: String)  {
        task::block_in_place(move || {
            let interval = Interval::from_str(&interval);
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().start_date(&start).end_date(&end)
                .interval(interval).benchmark_symbol(&benchmark).confidence_level(confidence_level)
                .risk_free_rate(risk_free_rate).build().unwrap();
            let performance_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.performance_chart()).unwrap();

            match display_format.as_str() {
                "png" => {
                    performance_chart.to_png("ticker_performance_chart.png",  1500, 1200, 1.0);
                    println!("Chart Saved to ticker_performance_chart.png");
                },
                "html" => {
                    performance_chart.write_html("ticker_performance_chart.html");
                    println!("Chart Saved to ticker_performance_chart.html");
                },
                "notebook" => {
                    if let Err(err) = display_html_with_iframe(Some(performance_chart), "performance_chart") {
                        eprintln!("Error displaying HTML with iframe: {:?}", err);
                    }
                },
                _ => {
                    println!("Invalid output format. Please choose either 'png', 'html', or 'notebook'");
                }
            }
        })
    }

    /// Display the candlestick chart for the ticker
    ///
    /// # Arguments
    ///
    /// * `start` - `str` - The start date of the time period in the format YYYY-MM-DD
    /// * `end` - `str` - The end date of the time period in the format YYYY-MM-DD
    /// * `interval` - `str` - The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo)
    /// * `display_format` - `str` - The format to display the chart in (png, html, notebook)
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// ticker.display_candlestick_chart("2020-01-01", "2020-12-31", "1d", "html")
    /// ```
    pub fn display_candlestick_chart(&self, start: String, end: String, interval: String, display_format: String)  {
        task::block_in_place(move || {
            let interval = Interval::from_str(&interval);
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().start_date(&start).end_date(&end)
                .interval(interval).build().unwrap();
            let candlestick_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.candlestick_chart()).unwrap();

            match display_format.as_str() {
                "png" => {
                    candlestick_chart.to_png("candlestick_chart.png",  1500, 1200, 1.0);
                    println!("Chart Saved to candlestick_chart.png")
                },
                "html" => {
                    candlestick_chart.write_html("candlestick_chart.html");
                    println!("Chart Saved to candlestick_chart.html");
                },
                "notebook" => {
                    if let Err(err) = display_html_with_iframe(Some(candlestick_chart), "candlestick_chart") {
                        eprintln!("Error displaying HTML with iframe: {:?}", err);
                    }
                },
                _ => {
                    println!("Invalid output format. Please choose either 'png', 'html' or 'notebook'");
                }
            }
        })
    }

    /// Display the options volatility surface, smile and term structure charts for the ticker
    ///
    /// # Arguments
    ///
    /// * `risk_free_rate` - `float` - The risk free rate to use in the calculations
    /// * `chart_type` - `str` - The type of options volatility chart to display (surface, smile, term_structure)
    /// * `display_format` - `str` - The format to display the chart in (png, html, notebook)
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// ticker = finalytics.Ticker("AAPL")
    /// ticker.display_options_chart(0.02, "surface", "html")
    /// ```
    pub fn display_options_chart(&self, risk_free_rate: f64, chart_type: String,  display_format: String)  {
        task::block_in_place(move || {
            let ticker = TickerBuilder::new().ticker(&self.symbol).unwrap().risk_free_rate(risk_free_rate).build().unwrap();

            let options_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                ticker.volatility_charts()).unwrap();

            let plot = match chart_type.as_str() {
                "surface" => options_chart.get("Volatility Surface").unwrap().clone(),
                "smile" => options_chart.get("Volatility Smile").unwrap().clone(),
                "term_structure" => options_chart.get("Volatility Term Structure").unwrap().clone(),
                _ => panic!("Invalid chart type. Please choose either 'surface', 'smile' or 'term_structure'"),
            };

            match display_format.as_str() {
                "png" => {
                    plot.to_png(format!("{}.png", chart_type).as_str(),  1500, 1200, 1.0);
                    println!("{}.png", chart_type);
                },
                "html" => {
                    plot.write_html(format!("{}.html", chart_type).as_str());
                    println!("{}.html", chart_type);
                },
                "notebook" => {
                    if let Err(err) = display_html_with_iframe(Some(plot), &chart_type) {
                        eprintln!("Error displaying HTML with iframe: {:?}", err);
                    }
                },
                _ => {
                    println!("Invalid output format. Please choose either 'png', 'html' or 'notebook'");
                }
            }

        })
    }
}

