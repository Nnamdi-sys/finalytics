use std::error::Error;
use crate::data::db::{get_symbol, Symbol};
use crate::data::ticker::Interval;

pub struct TickerBuilder {
    ticker: Symbol,
    start_date: String,
    end_date: String,
    interval: Interval,
    benchmark_symbol: String,
    confidence_level: f64,
    risk_free_rate: f64,
}

impl TickerBuilder {
    pub fn new() -> TickerBuilder {
        TickerBuilder {
            ticker: Symbol::new(),
            start_date: String::new(),
            end_date: String::new(),
            interval: Interval::OneDay,
            benchmark_symbol: String::new(),
            confidence_level: 0.95,
            risk_free_rate: 0.02,
        }
    }

    pub fn ticker(mut self, symbol: &str) -> Result<TickerBuilder, Box<dyn Error>> {
        self.ticker = get_symbol(symbol)?;
        Ok(self)
    }

    pub fn start_date(mut self, start_date: &str) -> TickerBuilder {
        self.start_date = start_date.to_string();
        self
    }

    pub fn end_date(mut self, end_date: &str) -> TickerBuilder {
        self.end_date = end_date.to_string();
        self
    }

    pub fn interval(mut self, interval: Interval) -> TickerBuilder {
        self.interval = interval;
        self
    }

    pub fn benchmark_symbol(mut self, benchmark_symbol: &str) -> TickerBuilder {
        self.benchmark_symbol = benchmark_symbol.to_string();
        self
    }

    pub fn confidence_level(mut self, confidence_level: f64) -> TickerBuilder {
        self.confidence_level = confidence_level;
        self
    }

    pub fn risk_free_rate(mut self, risk_free_rate: f64) -> TickerBuilder {
        self.risk_free_rate = risk_free_rate;
        self
    }

    pub fn build(self) -> Result<Ticker, Box<dyn Error>> {
        Ok(Ticker {
            ticker: self.ticker,
            start_date: self.start_date,
            end_date: self.end_date,
            interval: self.interval,
            benchmark_symbol: self.benchmark_symbol,
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
        })
    }
}


/// # Ticker Struct
///
/// ### Description
///    - This is the Security Analysis Module for the `Finalytics` Library.
///    - It provides methods to:
///      - Fetch Ticker Data from Yahoo Finance
///      - Perform Fundamental Analysis, Technical Analysis, and News Sentiment Analysis
///      - Display Ticker Charts
///
/// ### Ticker Data Methods
///    - `get_quote` - Fetches Current Ticker Price from Yahoo Finance
///    - `get_ticker_stats` - Fetches Ticker Current Summary Stats from Yahoo Finance
///    - `get_chart` - Returns the Ticker OHLCV Data from Yahoo Finance for a given time range
///    - `get_options` - Returns Ticker Option Chain Data from Yahoo Finance for all available expirations
///    - `get_fundamentals` - Returns Ticker Fundamental Data from Yahoo Finance
///
/// ### Ticker Fundamental Analysis Methods
///    - `income_statement` - Returns IFRS formatted Ticker Income Statement
///    - `balance_sheet` - Returns IFRS formatted Ticker Balance Sheet
///    - `cash_flow` - Returns IFRS formatted Ticker Cash Flow Statement
///    - `financial_ratios` - Returns Ticker Financial Ratios
///    - `performance_stats` - Returns Ticker Performance Statistics
///
/// ### Ticker Technical Analysis Methods
///    - `sma` - Returns Ticker Simple Moving Averages
///    - `ema` - Returns Ticker Exponential Moving Averages
///    - `macd` - Returns Ticker Moving Average Convergence Divergence
///    - `rsi` - Returns Ticker Relative Strength Index
///    - `fs` - Returns Ticker Fast Stochastic Oscillator
///    - `ss` - Returns Ticker Slow Stochastic Oscillator
///    - `ppo` - Returns Ticker Percentage Price Oscillator
///    - `roc` - Returns Ticker Rate of Change
///    - `mfi` - Returns Ticker Money Flow Index
///    - `bb` - Returns Ticker Bollinger Bands
///    - `sd` - Returns Ticker Rolling Standard Deviation
///    - `mad` - Returns Ticker Rolling Mean Absolute Deviation
///    - `atr` - Returns Ticker Average True Range
///    - `max` - Returns Ticker Rolling Maximum Values
///    - `min` - Returns Ticker Rolling Minimum Values
///    - `obv` - Returns Ticker On-Balance Volume
///

/// ### Ticker Options Analysis Methods
///    - `volatility_surface` - Returns Ticker Volatility Surface
///
/// ### Ticker Sentiment Analysis Methods
///    - `news_sentiment` - Returns Ticker News and Sentiment Scores for a given time range
///
/// ### Ticker Chart Methods
///    - `candlestick_chart` - Returns Ticker Candlestick Chart
///    - `performance_chart` - Returns Ticker Performance Chart
///    - `volatility_charts` - Returns Ticker Volatility Charts
///    - `summary_stats_table` - Returns Ticker Summary Stats Table plot
///    - `performance_stats_table` - Returns Ticker Performance Stats Table plot
///    - `financials_tables` - Returns Ticker Financials Table plots
///
/// ### Constructor
///    - The `Ticker` struct can be instantiated using the `TickerBuilder` struct.
///
/// ### Example
///
/// ```
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///
///  // Instantiate the Ticker Object
/// let ticker = TickerBuilder::new().ticker("AAPL")?
///                                     .start_date("2023-01-01")
///                                     .end_date("2023-02-01")
///                                     .interval(Interval::OneDay)
///                                     .benchmark_symbol("^GSPC")
///                                     .confidence_level(0.95)
///                                     .risk_free_rate(0.02)
///                                     .build()?;
///
///    // Fetch Ticker Data
///   let quote = ticker.get_quote().await?;
///   let stats = ticker.get_ticker_stats().await?;
///   let chart = ticker.get_chart().await?;
///   let options = ticker.get_options().await?;
///   let fundamentals = ticker.get_fundamentals("income-statement", "annual").await?;
///
///   println!("{:?}", quote);
///   println!("{:?}", stats);
///   println!("{:?}", chart);
///   println!("{:?}", options);
///   println!("{:?}", fundamentals);
///
///  // Fundamental Analysis
///  let income_statement = ticker.income_statement().await?;
///  let balance_sheet = ticker.balance_sheet().await?;
///  let cash_flow = ticker.cashflow_statement().await?;
///  let financial_ratios = ticker.financial_ratios().await?;
///  let performance_stats = ticker.performance_stats().await?;
///  let volatility_surface = ticker.volatility_surface().await?;
///
///  println!("{:?}", income_statement);
///  println!("{:?}", balance_sheet);
///  println!("{:?}", cash_flow);
///  println!("{:?}", financial_ratios);
///  println!("{:?}", performance_stats);
///  println!("{:?}", volatility_surface);
///
///  // Technical Analysis
///  let sma = ticker.sma(50).await?;
///  let ema = ticker.ema(3).await?;
///  let macd = ticker.macd(12, 26, 9).await?;
///  let rsi = ticker.rsi(14).await?;
///  let fs = ticker.fs(14).await?;
///  let ss = ticker.ss(7, 3).await?;
///  let ppo = ticker.ppo(12, 26, 9).await?;
///  let roc = ticker.roc(1).await?;
///  let mfi = ticker.mfi(14).await?;
///  let bb = ticker.bb(20, 2.0).await?;
///  let sd = ticker.sd(20).await?;
///  let mad = ticker.mad(20).await?;
///  let atr = ticker.atr(14).await?;
///  let max = ticker.max(20).await?;
///  let min = ticker.min(20).await?;
///  let obv = ticker.obv().await?;
///
///  println!("SMA:{:?}\nEMA:{:?}\nMACD:{:?}\nRSI:{:?}\nFS:{:?}\nSS:{:?}\nPPO:{:?}\nROC:{:?}\nMFI:{:?}\
///             \nBB:{:?}\nSD:{:?}\nMAD:{:?}\nATR:{:?}\nMAX:{:?}\nMIN:{:?}\nOBV:{:?}\n",
///              sma, ema, macd, rsi, fs, ss, ppo, roc, mfi, bb, sd, mad, atr, max, min, obv);
///
/// // News Sentiment Analysis
/// let news_sentiment = ticker.get_news(false).await?;
/// println!("{:?}", news_sentiment);
///
///  // Display Ticker Charts
///  let candlestick_chart = ticker.candlestick_chart().await?;
///  let performance_chart = ticker.performance_chart().await?;
///  let volatility_charts = ticker.volatility_charts().await?;
///  let summary_stats_table = ticker.summary_stats_table().await?;
///  let performance_stats_table = ticker.performance_stats_table().await?;
///  let financials_tables = ticker.financials_tables().await?;
///
///  candlestick_chart.show();
///  performance_chart.show();
///  summary_stats_table.show();
///  performance_stats_table.show();
///  volatility_charts["Volatility Surface"].show();
///  volatility_charts["Volatility Smile"].show();
///  volatility_charts["Volatility Term Structure"].show();
///  financials_tables["Income Statement"].show();
///  financials_tables["Balance Sheet"].show();
///  financials_tables["Cashflow Statement"].show();
///  financials_tables["Financial Ratios"].show();
///
///  Ok(())
///
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Ticker {
    pub ticker: Symbol,
    pub start_date: String,
    pub end_date: String,
    pub interval: Interval,
    pub benchmark_symbol: String,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
}















