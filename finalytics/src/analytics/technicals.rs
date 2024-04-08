use ta::{DataItem, Next};
use ta::indicators::*;
use std::error::Error;
use chrono::{DateTime, NaiveDateTime};
use polars::prelude::*;
use crate::data::ticker::TickerData;
use crate::models::ticker::Ticker;


pub trait TechnicalIndicators {
    fn sma(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn ema(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn rsi(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn macd(&self, fast_period: usize, slow_period: usize, signal_period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn ppo(&self, fast_period: usize, slow_period: usize, signal_period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn mfi(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn bb(&self, period: usize, std_dev: f64) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn fs(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn ss(&self, stochastic_period: usize, ema_period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn sd(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn mad(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn max(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn min(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn atr(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn roc(&self, period: usize) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn obv(&self) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
}


impl TechnicalIndicators for Ticker {

    /// Generates a Dataframe of the OHLVC data with the Simple Moving Average Indicator
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the Simple Moving Average Indicator (e.g. 50)
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the Simple Moving Average Indicator
    async fn sma(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let mut sma = SimpleMovingAverage::new(period).unwrap();
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let col = format!("sma-{period}");
        let sma_series = Series::new(&*col, close.iter().map(|x| sma.next(*x)).collect::<Vec<f64>>());
        let df= ohlcv.with_column(sma_series)?.clone();
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
    async fn ema(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let mut ema = ExponentialMovingAverage::new(period).unwrap();
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let col = format!("ema-{period}");
        let ema_series = Series::new(&*col, close.iter().map(|x| ema.next(*x)).collect::<Vec<f64>>());
        let df= ohlcv.with_column(ema_series)?.clone();
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
    async fn rsi(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let mut rsi = RelativeStrengthIndex::new(period).unwrap();
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let col = format!("rsi-{period}");
        let rsi_series = Series::new(&*col, close.iter().map(|x| rsi.next(*x)).collect::<Vec<f64>>());
        let df = ohlcv.with_column(rsi_series)?.clone();
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
    async fn macd(&self, fast_period: usize, slow_period: usize, signal_period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut macd = MovingAverageConvergenceDivergence::new(fast_period, slow_period, signal_period).unwrap();
        let macd_str = format!("macd-({fast_period},{slow_period},{signal_period})");
        let signal_str = format!("macd_signal-({fast_period},{slow_period},{signal_period})");
        let divergence_str = format!("macd_divergence-({fast_period},{slow_period},{signal_period})");
        let all_series:Vec<MovingAverageConvergenceDivergenceOutput> = close.iter().map(|x| macd.next(*x)).collect();
        let df = df!(
            "timestamp" => ohlcv.column("timestamp")?.clone(),
            "open" => ohlcv.column("open")?.clone(),
            "high" => ohlcv.column("high")?.clone(),
            "low" => ohlcv.column("low")?.clone(),
            "close" => ohlcv.column("close")?.clone(),
            "volume" => ohlcv.column("volume")?.clone(),
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
    async fn ppo(&self, fast_period: usize, slow_period: usize, signal_period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut ppo = PercentagePriceOscillator::new(fast_period, slow_period, signal_period).unwrap();
        let ppo_str = format!("ppo-({fast_period},{slow_period},{signal_period})");
        let signal_str = format!("ppo_signal-({fast_period},{slow_period},{signal_period})");
        let divergence_str = format!("ppo_divergence-({fast_period},{slow_period},{signal_period})");
        let all_series:Vec<PercentagePriceOscillatorOutput> = close.iter().map(|x| ppo.next(*x)).collect();
        let df = df!(
            "timestamp" => ohlcv.column("timestamp")?.clone(),
            "open" => ohlcv.column("open")?.clone(),
            "high" => ohlcv.column("high")?.clone(),
            "low" => ohlcv.column("low")?.clone(),
            "close" => ohlcv.column("close")?.clone(),
            "volume" => ohlcv.column("volume")?.clone(),
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
    async fn mfi(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut mfi = MoneyFlowIndex::new(period).unwrap();
        let mut timestamp = ohlcv.column("timestamp")?.datetime()?.to_vec().iter().map(|x|
            DateTime::from_timestamp_millis( x.unwrap()).unwrap().naive_local()).collect::<Vec<NaiveDateTime>>();
        let mut open = ohlcv.column("open")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut high = ohlcv.column("high")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut low = ohlcv.column("low")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut volume = ohlcv.column("volume")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let items = vec![high.clone(), low.clone(), close.clone(), open.clone(), volume.clone()];
        let mut data_items:Vec<DataItem> = Vec::new();
        for i in 0..close.len() {
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
    async fn bb(&self, period: usize, std_dev: f64) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut bb = BollingerBands::new(period, std_dev).unwrap();
        let bb_str = format!("bb-({period},{std_dev})", period=period, std_dev=std_dev);
        let upper_str = format!("bb_upper-({period},{std_dev})", period=period, std_dev=std_dev);
        let lower_str = format!("bb_lower-({period},{std_dev})", period=period, std_dev=std_dev);
        let all_series:Vec<BollingerBandsOutput> = close.iter().map(|x| bb.next(*x)).collect();
        let df = df!(
            "timestamp" => ohlcv.column("timestamp")?.clone(),
            "open" => ohlcv.column("open")?.clone(),
            "high" => ohlcv.column("high")?.clone(),
            "low" => ohlcv.column("low")?.clone(),
            "close" => ohlcv.column("close")?.clone(),
            "volume" => ohlcv.column("volume")?.clone(),
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
    async fn fs(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut fs = FastStochastic::new(period).unwrap();
        let col = format!("fs-{period}");
        let fs_series = Series::new(&*col, close.iter().map(|x| fs.next(*x)).collect::<Vec<f64>>());
        let df = ohlcv.with_column(fs_series)?.clone();
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
    async fn ss(&self, stochastic_period: usize, ema_period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut ss = SlowStochastic::new(stochastic_period, ema_period).unwrap();
        let col = format!("ss-({stochastic_period},{ema_period}`)");
        let ss_series = Series::new(&*col, close.iter().map(|x| ss.next(*x)).collect::<Vec<f64>>());
        let df = ohlcv.with_column(ss_series)?.clone();
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
    async fn sd(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut sd = StandardDeviation::new(period).unwrap();
        let col = format!("sd-{period}");
        let sd_series = Series::new(&*col, close.iter().map(|x| sd.next(*x)).collect::<Vec<f64>>());
        let df = ohlcv.with_column(sd_series)?.clone();
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
    async fn mad(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut mad = MeanAbsoluteDeviation::new(period).unwrap();
        let col = format!("mad-{period}");
        let mad_series = Series::new(&*col, close.iter().map(|x| mad.next(*x)).collect::<Vec<f64>>());
        let df = ohlcv.with_column(mad_series)?.clone();
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
    async fn max(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let high = ohlcv.column("high")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut max = Maximum::new(period).unwrap();
        let col = format!("max-{period}");
        let max_series = Series::new(&*col, high.iter().map(|x| max.next(*x)).collect::<Vec<f64>>());
        let df = ohlcv.with_column(max_series)?.clone();
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
    async fn min(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let low = ohlcv.column("low")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut min = Minimum::new(period).unwrap();
        let col = format!("min-{period}", period=period);
        let min_series = Series::new(&*col, low.iter().map(|x| min.next(*x)).collect::<Vec<f64>>());
        let df = ohlcv.with_column(min_series)?.clone();
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
    async fn atr(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut atr = AverageTrueRange::new(period).unwrap();
        let col = format!("atr-{period}");
        let mut timestamp = ohlcv.column("timestamp")?.datetime()?.to_vec().iter().map(|x|
            DateTime::from_timestamp_millis( x.unwrap()).unwrap().naive_local()).collect::<Vec<NaiveDateTime>>();
        let mut open = ohlcv.column("open")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut high = ohlcv.column("high")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut low = ohlcv.column("low")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut volume = ohlcv.column("volume")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let items = vec![high.clone(), low.clone(), close.clone(), open.clone(), volume.clone()];
        let mut data_items:Vec<DataItem> = Vec::new();
        for i in 0..close.len() {
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
            col.as_str() => data_items.iter().map(|x| atr.next(x)).collect::<Vec<f64>>()
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
    async fn roc(&self, period: usize) -> Result<DataFrame, Box<dyn Error>> {
        let mut ohlcv = self.get_chart().await?;
        let close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut roc = RateOfChange::new(period).unwrap();
        let col = format!("roc-{period}");
        let roc_series = Series::new(&*col, close.iter().map(|x| roc.next(*x)).collect::<Vec<f64>>());
        let df = ohlcv.with_column(roc_series)?.clone();
        Ok(df)
    }

    /// Generates a Dataframe of the OHLVC data with the On Balance Volume Indicator
    ///
    /// # Returns
    ///
    /// * `DataFrame` of the OHLCV data with the On Balance Volume Indicator
    async fn obv(&self) -> Result<DataFrame, Box<dyn Error>> {
        let ohlcv = self.get_chart().await?;
        let mut obv = OnBalanceVolume::new();
        let mut timestamp = ohlcv.column("timestamp")?.datetime()?.to_vec().iter().map(|x|
            DateTime::from_timestamp_millis( x.unwrap()).unwrap().naive_local()).collect::<Vec<NaiveDateTime>>();
        let mut open = ohlcv.column("open")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut high = ohlcv.column("high")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut low = ohlcv.column("low")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut close = ohlcv.column("close")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let mut volume = ohlcv.column("volume")?.f64()?.to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
        let items = vec![high.clone(), low.clone(), close.clone(), open.clone(), volume.clone()];
        let mut data_items:Vec<DataItem> = Vec::new();
        for i in 0..close.len() {
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
