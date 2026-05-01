use crate::data::ticker::TickerData;
use crate::error::{series_to_vec_f64, FinalyticsError};
use crate::models::ticker::Ticker;
use chrono::{DateTime, NaiveDateTime};
use polars::prelude::*;
use std::error::Error;
use ta::indicators::*;
use ta::{DataItem, Next};

/// Enum of OHLCV DataFrame Columns
pub enum Column {
    Open,
    High,
    Low,
    Close,
    Volume,
    AdjClose,
}

impl Column {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Column::Open => "open",
            Column::High => "high",
            Column::Low => "low",
            Column::Close => "close",
            Column::Volume => "volume",
            Column::AdjClose => "adjclose",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Column {
        match s {
            "open" => Column::Open,
            "high" => Column::High,
            "low" => Column::Low,
            "close" => Column::Close,
            "volume" => Column::Volume,
            "adjclose" => Column::AdjClose,
            _ => Column::Close,
        }
    }
}

pub trait TechnicalIndicators {
    fn sma(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn ema(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn rsi(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn macd(
        &self,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn ppo(
        &self,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn mfi(
        &self,
        period: usize,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn bb(
        &self,
        period: usize,
        std_dev: f64,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn fs(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn ss(
        &self,
        stochastic_period: usize,
        ema_period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn sd(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn mad(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn max(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn min(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn atr(
        &self,
        period: usize,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn roc(
        &self,
        period: usize,
        col: Option<Column>,
    ) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn obv(&self) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extract a `Vec<f64>` from an OHLCV DataFrame column by name, using the
/// safe `series_to_vec_f64` helper so we never panic on dtype/null issues.
fn extract_f64_col(df: &DataFrame, col_name: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let col = df.column(col_name)?;
    let s = col.as_materialized_series();
    Ok(series_to_vec_f64(s, col_name)?)
}

/// Extract the timestamp column as a `Series` clone suitable for building a
/// new DataFrame. Returns an error instead of panicking.
fn extract_series_col(df: &DataFrame, col_name: &str) -> Result<Series, Box<dyn Error>> {
    let col = df.column(col_name)?;
    Ok(col.as_materialized_series().clone())
}

/// Extract the timestamp column as `Vec<NaiveDateTime>`, safely handling
/// potential null values.
fn extract_timestamps(df: &DataFrame) -> Result<Vec<NaiveDateTime>, Box<dyn Error>> {
    let ts_col = df.column("timestamp")?;
    let ca = ts_col.datetime().map_err(|e| -> Box<dyn Error> {
        Box::new(FinalyticsError::DtypeMismatch {
            column: "timestamp".into(),
            expected: "Datetime".into(),
            actual: format!("{:?} — {e}", ts_col.dtype()),
        })
    })?;
    let mut out = Vec::with_capacity(ca.len());
    for opt in ca.into_iter() {
        let millis = opt.ok_or_else(|| -> Box<dyn Error> {
            Box::new(FinalyticsError::NullValues {
                column: "timestamp".into(),
                null_count: 1,
            })
        })?;
        let dt = DateTime::from_timestamp_millis(millis)
            .ok_or_else(|| -> Box<dyn Error> {
                Box::new(FinalyticsError::DataParse {
                    source: "timestamp".into(),
                    message: format!("Cannot convert millis {millis} to DateTime"),
                })
            })?
            .naive_local();
        out.push(dt);
    }
    Ok(out)
}

/// Create a `ta` indicator, mapping the `Result` from `::new()` into our
/// `Box<dyn Error>`.
macro_rules! new_indicator {
    ($ty:ty, $($arg:expr),+ $(,)?) => {{
        <$ty>::new($($arg),+).map_err(|e| -> Box<dyn Error> {
            Box::new(FinalyticsError::InvalidParameter {
                name: stringify!($ty).into(),
                message: format!("Failed to create indicator: {e}"),
            })
        })
    }};
}

/// Build an OHLCV `DataItem` vec together with sanitised parallel vecs
/// (timestamps, open, high, low, close, volume). Rows where the `DataItem`
/// cannot be built are dropped from **all** vectors so they stay aligned.
fn build_data_items(
    timestamps: &[NaiveDateTime],
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
) -> (
    Vec<NaiveDateTime>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<DataItem>,
) {
    let mut ts_out = Vec::with_capacity(close.len());
    let mut o_out = Vec::with_capacity(close.len());
    let mut h_out = Vec::with_capacity(close.len());
    let mut l_out = Vec::with_capacity(close.len());
    let mut c_out = Vec::with_capacity(close.len());
    let mut v_out = Vec::with_capacity(close.len());
    let mut items = Vec::with_capacity(close.len());

    for i in 0..close.len() {
        match DataItem::builder()
            .high(high[i])
            .low(low[i])
            .close(close[i])
            .open(open[i])
            .volume(volume[i])
            .build()
        {
            Ok(di) => {
                ts_out.push(timestamps[i]);
                o_out.push(open[i]);
                h_out.push(high[i]);
                l_out.push(low[i]);
                c_out.push(close[i]);
                v_out.push(volume[i]);
                items.push(di);
            }
            Err(_) => {
                eprintln!("Skipping row {i}: invalid DataItem");
            }
        }
    }

    (ts_out, o_out, h_out, l_out, c_out, v_out, items)
}

impl TechnicalIndicators for Ticker {
    /// Generates a Dataframe of the ticker price data with the Simple Moving Average Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Simple Moving Average Indicator (e.g., 50)
    /// * `col` - Column for the Simple Moving Average Indicator (e.g., Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with the Simple Moving Average Indicator
    async fn sma(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut sma = new_indicator!(SimpleMovingAverage, period)?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let col_name = format!("sma-{period}");
        let sma_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| sma.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(sma_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with the Exponential Moving Average Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Exponential Moving Average Indicator (e.g., 3)
    /// * `col` - Column for the Exponential Moving Average Indicator (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with the Exponential Moving Average Indicator
    async fn ema(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut ema = new_indicator!(ExponentialMovingAverage, period)?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let col_name = format!("ema-{period}");
        let ema_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| ema.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(ema_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with the Relative Strength Index Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Relative Strength Index Indicator (e.g., 14)
    /// * `col` - Column for the Relative Strength Index Indicator (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with the Relative Strength Index Indicator
    async fn rsi(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut rsi = new_indicator!(RelativeStrengthIndex, period)?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let col_name = format!("rsi-{period}");
        let rsi_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| rsi.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(rsi_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with the Moving Average Convergence Divergence Indicators
    ///
    /// # Arguments
    ///
    /// * `fast_period` - Fast period for the Moving Average Convergence Divergence Indicator (e.g., 12)
    /// * `slow_period` - Slow period for the Moving Average Convergence Divergence Indicator (e.g., 26)
    /// * `signal_period` - Signal period for the Moving Average Convergence Divergence Indicator (e.g., 9)
    /// * `col` - Column for the Moving Average Convergence Divergence Indicator (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with the Moving Average Convergence Divergence Indicators
    async fn macd(
        &self,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
        col: Option<Column>,
    ) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut macd = new_indicator!(
            MovingAverageConvergenceDivergence,
            fast_period,
            slow_period,
            signal_period
        )?;
        let macd_str = format!("macd-({fast_period},{slow_period},{signal_period})");
        let signal_str = format!("macd_signal-({fast_period},{slow_period},{signal_period})");
        let divergence_str =
            format!("macd_divergence-({fast_period},{slow_period},{signal_period})");
        let all_series: Vec<MovingAverageConvergenceDivergenceOutput> =
            col_val.iter().map(|x| macd.next(*x)).collect();
        let df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
            macd_str.as_str() => all_series.iter().map(|x| x.macd).collect::<Vec<f64>>(),
            signal_str.as_str() => all_series.iter().map(|x| x.signal).collect::<Vec<f64>>(),
            divergence_str.as_str() => all_series.iter().map(|x| x.histogram).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with the Percentage Price Oscillator Indicators
    ///
    /// # Arguments
    ///
    /// * `fast_period` - Fast period for the Percentage Price Oscillator Indicator (e.g., 12)
    /// * `slow_period` - Slow period for the Percentage Price Oscillator Indicator (e.g., 26)
    /// * `signal_period` - Signal period for the Percentage Price Oscillator Indicator (e.g., 9)
    /// * `col` - Column for the Percentage Price Oscillator Indicator (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with the Percentage Price Oscillator Indicators
    async fn ppo(
        &self,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
        col: Option<Column>,
    ) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut ppo = new_indicator!(
            PercentagePriceOscillator,
            fast_period,
            slow_period,
            signal_period
        )?;
        let ppo_str = format!("ppo-({fast_period},{slow_period},{signal_period})");
        let signal_str = format!("ppo_signal-({fast_period},{slow_period},{signal_period})");
        let divergence_str =
            format!("ppo_divergence-({fast_period},{slow_period},{signal_period})");
        let all_series: Vec<PercentagePriceOscillatorOutput> =
            col_val.iter().map(|x| ppo.next(*x)).collect();
        let df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
            ppo_str.as_str() => all_series.iter().map(|x| x.ppo).collect::<Vec<f64>>(),
            signal_str.as_str() => all_series.iter().map(|x| x.signal).collect::<Vec<f64>>(),
            divergence_str.as_str() => all_series.iter().map(|x| x.histogram).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the Money Flow Index Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Money Flow Index Indicator (e.g., 14)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Money Flow Index Indicator
    async fn mfi(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut mfi = new_indicator!(MoneyFlowIndex, period)?;
        let timestamp = extract_timestamps(&ohlcv)?;
        let open = extract_f64_col(&ohlcv, "open")?;
        let high = extract_f64_col(&ohlcv, "high")?;
        let low = extract_f64_col(&ohlcv, "low")?;
        let close = extract_f64_col(&ohlcv, "close")?;
        let volume = extract_f64_col(&ohlcv, "volume")?;

        let (ts, o, h, l, c, v, data_items) =
            build_data_items(&timestamp, &open, &high, &low, &close, &volume);

        let col_name = format!("mfi-{period}");
        let df = df!(
            "timestamp" => ts,
            "open" => o,
            "high" => h,
            "low" => l,
            "close" => c,
            "volume" => v,
            col_name.as_str() => data_items.iter().map(|x| mfi.next(x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with Bollinger Bands
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Bollinger Bands (e.g., 20)
    /// * `std_dev` - Standard deviation for the Bollinger Bands (e.g., 2.0)
    /// * `col` - Column for the Bollinger Bands (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with Bollinger Bands
    async fn bb(
        &self,
        period: usize,
        std_dev: f64,
        col: Option<Column>,
    ) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut bb = new_indicator!(BollingerBands, period, std_dev)?;
        let bb_str = format!("bb-({period},{std_dev})");
        let upper_str = format!("bb_upper-({period},{std_dev})");
        let lower_str = format!("bb_lower-({period},{std_dev})");
        let all_series: Vec<BollingerBandsOutput> = col_val.iter().map(|x| bb.next(*x)).collect();
        let df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
            bb_str.as_str() => all_series.iter().map(|x| x.average).collect::<Vec<f64>>(),
            upper_str.as_str() => all_series.iter().map(|x| x.upper).collect::<Vec<f64>>(),
            lower_str.as_str() => all_series.iter().map(|x| x.lower).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with the Fast Stochastic Oscillator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Fast Stochastic Oscillator (e.g., 14)
    /// * `col` - Column for the Fast Stochastic Oscillator (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with the Fast Stochastic Oscillator
    async fn fs(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut fs = new_indicator!(FastStochastic, period)?;
        let col_name = format!("fs-{period}");
        let fs_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| fs.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(fs_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with the Slow Stochastic Oscillator
    ///
    /// # Arguments
    ///
    /// * `stochastic_period` - Stochastic period for the Slow Stochastic Oscillator (e.g., 7)
    /// * `ema_period` - Exponential Moving Average period for the Slow Stochastic Oscillator (e.g., 3)
    /// * `col` - Column for the Slow Stochastic Oscillator (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with the Slow Stochastic Oscillator
    async fn ss(
        &self,
        stochastic_period: usize,
        ema_period: usize,
        col: Option<Column>,
    ) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut ss = new_indicator!(SlowStochastic, stochastic_period, ema_period)?;
        let col_name = format!("ss-({stochastic_period},{ema_period}`)");
        let ss_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| ss.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(ss_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with rolling Standard Deviation
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the rolling Standard Deviation (e.g., 20)
    /// * `col` - Column for the rolling Standard Deviation (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with rolling Standard Deviation
    async fn sd(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut sd = new_indicator!(StandardDeviation, period)?;
        let col_name = format!("sd-{period}");
        let sd_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| sd.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(sd_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with rolling Mean Absolute Deviation
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the rolling Mean Absolute Deviation (e.g., 20)
    /// * `col` - Column for the rolling Mean Absolute Deviation (default - Column::Close)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with rolling Mean Absolute Deviation
    async fn mad(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Close.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut mad = new_indicator!(MeanAbsoluteDeviation, period)?;
        let col_name = format!("mad-{period}");
        let mad_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| mad.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(mad_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with rolling Maximum Values
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the rolling Maximum Values (e.g., 20)
    /// * `col` - Column for the rolling Maximum Values (default - Column::High)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price data with rolling Maximum Values
    async fn max(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::High.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut max = new_indicator!(Maximum, period)?;
        let col_name = format!("max-{period}");
        let max_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| max.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(max_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with rolling Minimum Values
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the rolling Minimum Values (e.g., 20)
    /// * `col` - Column for the rolling Minimum Values (default - Column::Low)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the ticker price date data with rolling Minimum Values
    async fn min(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::Low.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut min = new_indicator!(Minimum, period)?;
        let col_name = format!("min-{period}");
        let min_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| min.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(min_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the Average True Range Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Average True Range Indicator (e.g., 14)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Average True Range Indicator
    async fn atr(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut atr = new_indicator!(AverageTrueRange, period)?;
        let col_name = format!("atr-{period}");
        let timestamp = extract_timestamps(&ohlcv)?;
        let open = extract_f64_col(&ohlcv, "open")?;
        let high = extract_f64_col(&ohlcv, "high")?;
        let low = extract_f64_col(&ohlcv, "low")?;
        let close = extract_f64_col(&ohlcv, "close")?;
        let volume = extract_f64_col(&ohlcv, "volume")?;

        let (ts, o, h, l, c, v, data_items) =
            build_data_items(&timestamp, &open, &high, &low, &close, &volume);

        let df = df!(
            "timestamp" => ts,
            "open" => o,
            "high" => h,
            "low" => l,
            "close" => c,
            "volume" => v,
            col_name.as_str() => data_items.iter().map(|x| atr.next(x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the ticker price data with the Rate of Change Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Rate of Change Indicator (e.g., 1)
    /// * `col` - Column for the Rate of Change Indicator (default - Column::AdjClose)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Rate of Change Indicator
    async fn roc(&self, period: usize, col: Option<Column>) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let col_str = match col {
            Some(col) => col.as_str(),
            None => Column::AdjClose.as_str(),
        };
        let col_val = extract_f64_col(&ohlcv, col_str)?;
        let mut roc = new_indicator!(RateOfChange, period)?;
        let col_name = format!("roc-{period}");
        let roc_series = Series::new(
            col_name.as_str().into(),
            col_val.iter().map(|x| roc.next(*x)).collect::<Vec<f64>>(),
        );
        let mut df = df!(
            "timestamp" => extract_series_col(&ohlcv, "timestamp")?,
            col_str => extract_series_col(&ohlcv, col_str)?,
        )?;
        let df = df.with_column(roc_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the On Balance Volume Indicator
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the On Balance Volume Indicator
    async fn obv(&self) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut obv = OnBalanceVolume::new();
        let timestamp = extract_timestamps(&ohlcv)?;
        let open = extract_f64_col(&ohlcv, "open")?;
        let high = extract_f64_col(&ohlcv, "high")?;
        let low = extract_f64_col(&ohlcv, "low")?;
        let close = extract_f64_col(&ohlcv, "close")?;
        let volume = extract_f64_col(&ohlcv, "volume")?;

        let (ts, o, h, l, c, v, data_items) =
            build_data_items(&timestamp, &open, &high, &low, &close, &volume);

        let df = df!(
            "timestamp" => ts,
            "open" => o,
            "high" => h,
            "low" => l,
            "close" => c,
            "volume" => v,
            "obv" => data_items.iter().map(|x| obv.next(x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }
}
