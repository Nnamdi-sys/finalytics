use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use polars::prelude::*;
use chrono::{TimeZone, NaiveDateTime, Utc};
use crate::utils::date_utils::to_date;


#[derive(Debug, Copy, Clone)]
pub struct IntervalDays {
    pub average: f64,
    pub mode: f64,
}


/// KLINE Struct
///
/// ### Description
/// - Represents historical price/volume data for a financial instrument
/// - Stores OHLCV (Open, High, Low, Close, Volume) data with UNIX timestamps
/// - Supports both adjusted and unadjusted closing prices
/// - Designed for efficient time series analysis and portfolio construction
/// - Flexible data ingestion from multiple sources
///
/// ### Fields
/// - `ticker`: Instrument symbol (e.g., "AAPL", "BTC-USD")
/// - `timestamp`: UNIX timestamps (seconds since epoch) for each data point
/// - `open`: Opening prices (optional)
/// - `high`: Daily high prices (optional)
/// - `low`: Daily low prices (optional)
/// - `close`: Closing prices (required)
/// - `volume`: Trading volumes (optional)
/// - `adjclose`: Adjusted closing prices (optional)
///
/// ### Features
/// - **Multi-format Support**: Load data from CSV, JSON, and Polars DataFrames
/// - **Flexible Integration**: Works seamlessly with `Tickers` and `Portfolio` structs
/// - **Performance Analysis**: Enables risk/return calculations and reporting
///
/// ### Example: Loading Data and Generating Reports
/// ```rust,no_run
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     // Method 1: Load from CSV files
///     let aapl_csv = KLINE::from_csv("AAPL", "../examples/datasets/aapl.csv")?;
///     let goog_csv = KLINE::from_csv("GOOG", "../examples/datasets/goog.csv")?;
///     let gspc_csv = KLINE::from_csv("^GSPC", "../examples/datasets/gspc.csv")?;
///
///     // Method 2: Load from JSON files
///     let msft_json = KLINE::from_json("MSFT", "../examples/datasets/msft.json")?;
///     let btc_json = KLINE::from_json("BTC-USD", "../examples/datasets/btc.json")?;
///
///     // Method 3: Load from Polars DataFrame
///     let df = KLINE::from_csv("NVDA","../examples/datasets/nvda.csv")?.to_dataframe()?;
///     let nvda_df = KLINE::from_dataframe("NVDA", &df)?;
///
///     // Combine all data sources
///     let tickers_data = vec![aapl_csv, goog_csv, msft_json, btc_json, nvda_df];
///
///     // Initialize Tickers Struct
///     let tickers = Tickers::builder()
///         .tickers_data(Some(tickers_data))
///         .benchmark_data(Some(gspc_csv))
///         .confidence_level(0.95)
///         .risk_free_rate(0.02)
///         .build();
///
///     // Generate a Single Ticker Report
///     let ticker = tickers.clone().get_ticker("AAPL").await?;
///     ticker.report(Some(ReportType::Performance)).await?.show()?;
///
///     // Generate a Multiple Ticker Report
///     tickers.report(Some(ReportType::Performance)).await?.show()?;
///
///     // Portfolio optimization
///     let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None).await?;
///     portfolio.report(Some(ReportType::Performance)).await?.show()?;
///
///     Ok(())
/// }
/// ```
///
/// ### Supported Data Sources
/// 1. **CSV Files**:
///    ```csv
///    timestamp,open,high,low,close,volume,adjclose
///     1672704000,130.28,130.90,124.17,125.07,112117500,124.82
///     1672790400,126.89,128.66,125.08,126.36,89113600,126.11
///    ```
///
/// 2. **JSON Files**:
///    ```json
///    {
///      "timestamp": [1672704000, 1672790400],
///      "open": [130.28, 126.89],
///      "high": [130.90, 128.66],
///      "low": [124.17, 125.08],
///      "close": [125.07, 126.36],
///      "volume": [112117500, 89113600],
///      "adjclose": [124.82, 126.11]
///    }
///    ```
///
/// 3. **Polars DataFrames**:
///    ```rust,no_run
///    use polars::prelude::*;
///
///    let df = df!(
///         "timestamp" => &[1672704000, 1672790400],
///         "open" => &[130.28, 126.89],
///         "high" => &[130.90, 128.66],
///         "low" => &[124.17, 125.08],
///         "close" => &[125.07, 126.36],
///         "volume" => &[112117500.0, 89113600.0],
///         "adjclose" => &[124.82, 126.11]
///     );
///    ```
///
/// ### Important Notes
/// - All sources must include `timestamp` and `close` columns
/// - Column names are case-sensitive and must match exactly
/// - Timestamps should be UNIX format (seconds since epoch)
/// - Missing columns will be initialized as `None`
#[derive(Debug, Clone)]
pub struct KLINE {
    pub ticker: String,
    pub timestamp: Vec<i64>,
    pub open: Option<Vec<f64>>,
    pub high: Option<Vec<f64>>,
    pub low: Option<Vec<f64>>,
    pub close: Vec<f64>,
    pub volume: Option<Vec<f64>>,
    pub adjclose: Option<Vec<f64>>,
}

impl KLINE {
    pub fn interval_days(&self) -> IntervalDays {
        let mut intervals = Vec::new();
        let mut total_seconds = 0.0;

        // Calculate all intervals and track total
        for window in self.timestamp.windows(2) {
            let diff = (window[1] - window[0]) as f64;
            let days = diff / 86400.0;
            intervals.push(days);
            total_seconds += diff;
        }

        // Calculate average
        let avg = total_seconds / (intervals.len() as f64 * 86400.0);

        // Calculate mode with precision handling
        let mut interval_counts = HashMap::new();
        let mut max_count = 0;
        let mut mode = 0.0;

        for &interval in &intervals {
            let key = (interval * 10000.0) as i64;
            let count = interval_counts.entry(key).or_insert(0);
            *count += 1;

            if *count > max_count || (*count == max_count && key < (mode * 10000.0) as i64) {
                max_count = *count;
                mode = key as f64 / 10000.0;
            }
        }

        IntervalDays {
            average: avg,
            mode,
        }
    }

    pub fn start_date(&self) -> String {
        let start_timestamp = self.timestamp[0];
        to_date(start_timestamp)
    }

    pub fn end_date(&self) -> String {
        let end_timestamp = self.timestamp[self.timestamp.len() - 1];
        to_date(end_timestamp)
    }

    pub fn to_dataframe(&self) -> Result<DataFrame, Box<dyn Error>> {
        let len = self.timestamp.len();

        if self.close.len() != len {
            return Err( "close length does not match timestamp".into());
        }

        let check_length = |name: &str, opt_vec: &Option<Vec<f64>>| -> Result<(), Box<dyn Error>> {
            if let Some(vec) = opt_vec {
                if vec.len() != len {
                    return Err(format!("{name} length does not match timestamp").into());
                }
            }
            Ok(())
        };

        check_length("open", &self.open)?;
        check_length("high", &self.high)?;
        check_length("low", &self.low)?;
        check_length("volume", &self.volume)?;
        check_length("adjclose", &self.adjclose)?;

        // Handle optional fields with defaults
        let open_data = self.open.clone().unwrap_or_else(|| self.close.clone());
        let high_data = self.high.clone().unwrap_or_else(|| self.close.clone());
        let low_data = self.low.clone().unwrap_or_else(|| self.close.clone());
        let volume_data = self.volume.clone().unwrap_or_else(|| vec![0.0; len]);
        let adj_close_data = self.adjclose.clone().unwrap_or_else(|| self.close.clone());

        // Convert timestamps to datetime
        let datetime_vec: Result<Vec<NaiveDateTime>, _> = self.timestamp
            .iter()
            .map(|&ts| {
                Utc.timestamp_opt(ts, 0)
                    .single()
                    .ok_or_else(|| PolarsError::ComputeError(
                        format!("Invalid timestamp: {ts}").into()
                    ))
                    .map(|dt| dt.naive_utc())
            })
            .collect();

        // Create series
        let cols = vec![
            Column::new("timestamp".into(), datetime_vec?),
            Column::new("open".into(), open_data),
            Column::new("high".into(), high_data),
            Column::new("low".into(), low_data),
            Column::new("close".into(), self.close.clone()),
            Column::new("volume".into(), volume_data),
            Column::new("adjclose".into(), adj_close_data),
        ];

        let df = DataFrame::new(cols)?;

        Ok(df)
    }

    pub fn from_dataframe(ticker: &str, df: &DataFrame) -> Result<KLINE, Box<dyn Error>> {
        let timestamp = df.column("timestamp")?.i64()?.to_vec().iter()
            .map(|&x| x.unwrap_or_default()).collect::<Vec<i64>>();
        let open = df.column("open")?.f64()?.to_vec().iter()
            .map(|&x| x.unwrap_or_default()).collect::<Vec<f64>>();
        let high = df.column("high")?.f64()?.to_vec().iter()
            .map(|&x| x.unwrap_or_default()).collect::<Vec<f64>>();
        let low = df.column("low")?.f64()?.to_vec().iter()
            .map(|&x| x.unwrap_or_default()).collect::<Vec<f64>>();
        let close = df.column("close")?.f64()?.to_vec().iter()
            .map(|&x| x.unwrap_or_default()).collect::<Vec<f64>>();
        let volume = df.column("volume")?.f64()?.to_vec().iter()
            .map(|&x| x.unwrap_or_default()).collect::<Vec<f64>>();
        let adjclose = df.column("adjclose")?.f64()?.to_vec().iter()
            .map(|&x| x.unwrap_or_default()).collect::<Vec<f64>>();

        Ok(KLINE {
            ticker: ticker.to_string(),
            timestamp,
            open: Some(open),
            high: Some(high),
            low: Some(low),
            close,
            volume: Some(volume),
            adjclose: Some(adjclose),
        })
    }

    pub fn from_csv(ticker: &str, path: &str) -> Result<Self, Box<dyn Error>> {
        let df = CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()?;

        KLINE::from_dataframe(ticker, &df)
    }

    pub fn from_json(ticker: &str, path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let json_data: serde_json::Value = serde_json::from_reader(reader)?;

        let timestamp = json_data["timestamp"]
            .as_array()
            .ok_or("Missing 'timestamp' field")?
            .iter()
            .map(|v| v.as_i64().ok_or("Invalid 'timestamp' value"))
            .collect::<Result<Vec<_>, _>>()?;

        let open = json_data["open"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_f64().ok_or("Invalid 'open' value"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        let high = json_data["high"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_f64().ok_or("Invalid 'high' value"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        let low = json_data["low"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_f64().ok_or("Invalid 'low' value"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        let close = json_data["close"]
            .as_array()
            .ok_or("Missing 'close' field")?
            .iter()
            .map(|v| v.as_f64().ok_or("Invalid 'close' value"))
            .collect::<Result<Vec<_>, _>>()?;

        let volume = json_data["volume"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_f64().ok_or("Invalid 'volume' value"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        let adjclose = json_data["adjclose"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_f64().ok_or("Invalid 'adjclose' value"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        let kline = KLINE {
            ticker: ticker.to_string(),
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            adjclose,
        };

        Ok(kline)
    }

}