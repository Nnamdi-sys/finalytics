use ta::{DataItem, Next};
use ta::indicators::*;
use std::error::Error;
use chrono::NaiveDateTime;
use polars::prelude::*;
use crate::data::ticker::{Interval, Ticker};

pub struct TechnicalIndicators {
    pub timestamp: Vec<NaiveDateTime>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<f64>,
}

impl TechnicalIndicators {
    /// Creates a new TechnicalIndicators struct
    ///
    /// # Arguments
    ///
    /// * `symbol` - Ticker symbol (e.g. "AAPL")
    /// * `start_date` - Start date in YYYY-MM-DD format (e.g. "2020-01-01")
    /// * `end_date` - End date in YYYY-MM-DD format (e.g. "2020-12-31")
    /// * `interval` - Time interval enum (e.g. Interval::OneDay)
    ///
    /// # Returns
    ///
    /// * `TechnicalIndicators` struct
    pub async fn new(
        symbol: &str,
        start_date: &str,
        end_date: &str,
        interval: Interval
    ) -> Result<TechnicalIndicators, Box<dyn Error>> {
        let ticker = Ticker::new(symbol).await?;
        let df = ticker.get_chart(start_date, end_date, interval).await?;
        let timestamp = df.column("timestamp")?.datetime()?.to_vec().iter().map(|x|
            NaiveDateTime::from_timestamp_millis( x.unwrap()).unwrap()).collect::<Vec<NaiveDateTime>>();
        let open = df.column("open")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let high = df.column("high")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let low = df.column("low")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let close = df.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let volume = df.column("volume")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        Ok(TechnicalIndicators {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        })
    }

    /// Generates a Dataframe of the OHLVC data with the Simple Moving Average Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Simple Moving Average Indicator (e.g. 50)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Simple Moving Average Indicator
    pub fn sma(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut sma = SimpleMovingAverage::new(period).unwrap();
        let col = format!("sma-{period}");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| sma.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the Exponential Moving Average Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Exponential Moving Average Indicator (e.g. 3)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Exponential Moving Average Indicator
    pub fn ema(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ema = ExponentialMovingAverage::new(period).unwrap();
        let col = format!("ema-{period}");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| ema.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the Relative Strength Index Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Relative Strength Index Indicator (e.g. 14)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Relative Strength Index Indicator
    pub fn rsi(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut rsi = RelativeStrengthIndex::new(period).unwrap();
        let col = format!("rsi-{period}");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| rsi.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the Moving Average Convergence Divergence Indicators
    ///
    /// # Arguments
    ///
    /// * `fast_period` - Fast period for the Moving Average Convergence Divergence Indicator (e.g. 12)
    /// * `slow_period` - Slow period for the Moving Average Convergence Divergence Indicator (e.g. 26)
    /// * `signal_period` - Signal period for the Moving Average Convergence Divergence Indicator (e.g. 9)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Moving Average Convergence Divergence Indicators
    pub fn macd(&self, fast_period: usize, slow_period: usize, signal_period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut macd = MovingAverageConvergenceDivergence::new(fast_period, slow_period, signal_period).unwrap();
        let macd_str = format!("macd-({fast_period},{slow_period},{signal_period})");
        let signal_str = format!("macd_signal-({fast_period},{slow_period},{signal_period})");
        let divergence_str = format!("macd_divergence-({fast_period},{slow_period},{signal_period})");
        let all_series:Vec<MovingAverageConvergenceDivergenceOutput> = self.close.iter().map(|x| macd.next(*x)).collect();
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            macd_str.as_str() => all_series.iter().map(|x| x.macd).collect::<Vec<f64>>(),
            signal_str.as_str() => all_series.iter().map(|x| x.signal).collect::<Vec<f64>>(),
            divergence_str.as_str() => all_series.iter().map(|x| x.histogram).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the Percentage Price Oscillator Indicators
    ///
    /// # Arguments
    ///
    /// * `fast_period` - Fast period for the Percentage Price Oscillator Indicator (e.g. 12)
    /// * `slow_period` - Slow period for the Percentage Price Oscillator Indicator (e.g. 26)
    /// * `signal_period` - Signal period for the Percentage Price Oscillator Indicator (e.g. 9)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Percentage Price Oscillator Indicators
    pub fn ppo(&self, fast_period: usize, slow_period: usize, signal_period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ppo = PercentagePriceOscillator::new(fast_period, slow_period, signal_period).unwrap();
        let ppo_str = format!("ppo-({fast_period},{slow_period},{signal_period})");
        let signal_str = format!("ppo_signal-({fast_period},{slow_period},{signal_period})");
        let divergence_str = format!("ppo_divergence-({fast_period},{slow_period},{signal_period})");
        let all_series:Vec<PercentagePriceOscillatorOutput> = self.close.iter().map(|x| ppo.next(*x)).collect();
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
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
    /// * `period` - Period for the Money Flow Index Indicator (e.g. 14)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Money Flow Index Indicator
    pub fn mfi(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut mfi = MoneyFlowIndex::new(period).unwrap();
        let mut timestamp = self.timestamp.clone();
        let mut open = self.open.clone();
        let mut high = self.high.clone();
        let mut low = self.low.clone();
        let mut close = self.close.clone();
        let mut volume = self.volume.clone();
        let items = vec![&self.high, &self.low, &self.close, &self.open, &self.volume];
        let mut data_items:Vec<DataItem> = Vec::new();
        for i in 0..self.close.len() {
            let di = match DataItem::builder()
                .high(items[0][i])
                .low(items[1][i])
                .close(items[2][i])
                .open(items[3][i])
                .volume(items[4][i])
                .build() {
                Ok(di) => {
                    di
                },
                Err(_) => {
                    timestamp.remove(i);
                    open.remove(i);
                    high.remove(i);
                    low.remove(i);
                    close.remove(i);
                    volume.remove(i);
                    eprintln!("Error creating DataItem");
                    continue
                }
            };
            data_items.push(di);
        }
        let col = format!("mfi-{period}");
        let df = df!(
            "timestamp" => timestamp,
            "open" => open,
            "high" => high,
            "low" => low,
            "close" => close,
            "volume" => volume,
            col.as_str() => data_items.iter().map(|x| mfi.next(x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with Bollinger Bands
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Bollinger Bands (e.g. 20)
    /// * `std_dev` - Standard deviation for the Bollinger Bands (e.g. 2.0)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with Bollinger Bands
    pub fn bb(&self, period: usize, std_dev: f64) -> Result<DataFrame, Box<dyn Error>> {
        let mut bb = BollingerBands::new(period, std_dev).unwrap();
        let bb_str = format!("bb-({period},{std_dev})", period=period, std_dev=std_dev);
        let upper_str = format!("bb_upper-({period},{std_dev})", period=period, std_dev=std_dev);
        let lower_str = format!("bb_lower-({period},{std_dev})", period=period, std_dev=std_dev);
        let all_series:Vec<BollingerBandsOutput> = self.close.iter().map(|x| bb.next(*x)).collect();
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            bb_str.as_str() => all_series.iter().map(|x| x.average).collect::<Vec<f64>>(),
            upper_str.as_str() => all_series.iter().map(|x| x.upper).collect::<Vec<f64>>(),
            lower_str.as_str() => all_series.iter().map(|x| x.lower).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the Fast Stochastic Oscillator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Fast Stochastic Oscillator (e.g. 14)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Fast Stochastic Oscillator
    pub fn fs(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut fs = FastStochastic::new(period).unwrap();
        let col = format!("fs-{period}");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| fs.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with the Slow Stochastic Oscillator
    ///
    /// # Arguments
    ///
    /// * `stochastic_period` - Stochastic period for the Slow Stochastic Oscillator (e.g. 7)
    /// * `ema_period` - Exponential Moving Average period for the Slow Stochastic Oscillator (e.g. 3)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Slow Stochastic Oscillator
    pub fn ss(&self, stochastic_period: usize, ema_period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ss = SlowStochastic::new(stochastic_period, ema_period).unwrap();
        let col = format!("ss-({stochastic_period},{ema_period}`)");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| ss.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with rolling Standard Deviation
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the rolling Standard Deviation (e.g. 20)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with rolling Standard Deviation
    pub fn sd(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut sd = StandardDeviation::new(period).unwrap();
        let col = format!("sd-{period}");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| sd.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with rolling Mean Absolute Deviation
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the rolling Mean Absolute Deviation (e.g. 20)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with rolling Mean Absolute Deviation
    pub fn mad(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut mad = MeanAbsoluteDeviation::new(period).unwrap();
        let col = format!("mad-{period}");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| mad.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with rolling Maximum Values
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the rolling Maximum Values (e.g. 20)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with rolling Maximum Values
    pub fn max(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut max = Maximum::new(period).unwrap();
        let col = format!("max-{period}");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.high.iter().map(|x| max.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLCV data with rolling Minimum Values
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the rolling Minimum Values (e.g. 20)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with rolling Minimum Values
    pub fn min(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut min = Minimum::new(period).unwrap();
        let col = format!("min-{period}", period=period);
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.low.iter().map(|x| min.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLVC data with the Average True Range Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Average True Range Indicator (e.g. 14)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Average True Range Indicator
    pub fn atr(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut atr = AverageTrueRange::new(period).unwrap();
        let col = format!("atr-{period}");
        let mut timestamp = self.timestamp.clone();
        let mut open = self.open.clone();
        let mut high = self.high.clone();
        let mut low = self.low.clone();
        let mut close = self.close.clone();
        let mut volume = self.volume.clone();
        let items = vec![&self.high, &self.low, &self.close, &self.open, &self.volume];
        let mut data_items:Vec<DataItem> = Vec::new();
        for i in 0..self.close.len() {
            let di = match DataItem::builder()
                .high(items[0][i])
                .low(items[1][i])
                .close(items[2][i])
                .open(items[3][i])
                .volume(items[4][i])
                .build() {
                Ok(di) => {
                    di
                },
                Err(_) => {
                    timestamp.remove(i);
                    open.remove(i);
                    high.remove(i);
                    low.remove(i);
                    close.remove(i);
                    volume.remove(i);
                    eprintln!("Error creating DataItem");
                    continue
                }
            };
            data_items.push(di);
        }
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| atr.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLVC data with the Rate of Change Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Rate of Change Indicator (e.g. 1)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Rate of Change Indicator
    pub fn roc(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut roc = RateOfChange::new(period).unwrap();
        let col = format!("roc-{period}");
        let df = df!(
            "timestamp" => &self.timestamp,
            "open" => &self.open,
            "high" => &self.high,
            "low" => &self.low,
            "close" => &self.close,
            "volume" => &self.volume,
            col.as_str() => self.close.iter().map(|x| roc.next(*x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }

    /// Generates a Dataframe of the OHLVC data with the On Balance Volume Indicator
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the On Balance Volume Indicator
    pub fn obv(&self) -> Result<DataFrame, Box<dyn Error>> {
        let mut obv = OnBalanceVolume::new();
        let mut timestamp = self.timestamp.clone();
        let mut open = self.open.clone();
        let mut high = self.high.clone();
        let mut low = self.low.clone();
        let mut close = self.close.clone();
        let mut volume = self.volume.clone();
        let items = vec![&self.high, &self.low, &self.close, &self.open, &self.volume];
        let mut data_items:Vec<DataItem> = Vec::new();
        for i in 0..self.close.len() {
            let di = match DataItem::builder()
                .high(items[0][i])
                .low(items[1][i])
                .close(items[2][i])
                .open(items[3][i])
                .volume(items[4][i])
                .build() {
                Ok(di) => {
                    di
                },
                Err(_) => {
                    timestamp.remove(i);
                    open.remove(i);
                    high.remove(i);
                    low.remove(i);
                    close.remove(i);
                    volume.remove(i);
                    eprintln!("Error creating DataItem");
                    continue
                }
            };
            data_items.push(di);
        }
        let df = df!(
            "timestamp" => timestamp,
            "open" => open,
            "high" => high,
            "low" => low,
            "close" => close,
            "volume" => volume,
            "obv" => data_items.iter().map(|x| obv.next(x)).collect::<Vec<f64>>()
        )?;
        Ok(df)
    }
}
