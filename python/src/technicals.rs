use pyo3::prelude::*;
#[pyclass]
#[derive(Clone)]
pub enum IndicatorType {
    /// Simple Moving Average (SMA) with a specified period.
    /// - `usize`: The period for the SMA calculation.
    /// - `Option<String>`: The Column name for the SMA calculation. (default: "close")
    SMA(usize, Option<String>),

    /// Exponential Moving Average (EMA) with a specified period.
    /// - `usize`: The period for the EMA calculation.
    /// - `Option<String>`: The Column name for the EMA calculation. (default: "close")
    EMA(usize, Option<String>),

    /// Relative Strength Index (RSI) with a specified period.
    /// - `usize`: The period for the RSI calculation.
    /// - `Option<String>`: The Column name for the RSI calculation. (default: "close")
    RSI(usize, Option<String>),

    /// Moving Average Convergence Divergence (MACD) with specified fast period, slow period, and signal period.
    /// - `usize`: The fast period for the MACD calculation.
    /// - `usize`: The slow period for the MACD calculation.
    /// - `usize`: The signal period for the MACD calculation.
    /// - `Option<String>`: The Column name for the MACD calculation. (default: "close")
    MACD(usize, usize, usize, Option<String>),

    /// Percentage Price Oscillator (PPO) with specified fast period, slow period, and signal period.
    /// - `usize`: The fast period for the PPO calculation.
    /// - `usize`: The slow period for the PPO calculation.
    /// - `usize`: The signal period for the PPO calculation.
    /// - `Option<String>`: The Column name for the PPO calculation. (default: "close")
    PPO(usize, usize, usize, Option<String>),

    /// Money Flow Index (MFI) with a specified period.
    /// - `usize`: The period for the MFI calculation.
    MFI(usize),

    /// Bollinger Bands (BB) with a specified period and standard deviation.
    /// - `usize`: The period for the Bollinger Bands calculation.
    /// - `f64`: The number of standard deviations for the Bollinger Bands calculation.
    /// - `Option<String>`: The Column name for the Bollinger Bands calculation. (default: "close")
    BB(usize, f64, Option<String>),

    /// Fast Stochastic (FS) with a specified period.
    /// - `usize`: The period for the Fast Stochastic calculation.
    /// - `Option<String>`: The Column name for the Fast Stochastic calculation. (default: "close")
    FS(usize, Option<String>),

    /// Slow Stochastic (SS) with specified stochastic period and EMA period.
    /// - `usize`: The period for the stochastic calculation.
    /// - `usize`: The period for the EMA calculation.
    /// - `Option<String>`: The Column name for the Slow Stochastic calculation. (default: "close")
    SS(usize, usize, Option<String>),

    /// Standard Deviation (SD) with a specified period.
    /// - `usize`: The period for the Standard Deviation calculation.
    /// - `Option<String>`: The Column name for the Standard Deviation calculation. (default: "close")
    SD(usize, Option<String>),

    /// Mean Absolute Deviation (MAD) with a specified period.
    /// - `usize`: The period for the Mean Absolute Deviation calculation.
    /// - `Option<String>`: The Column name for the Mean Absolute Deviation calculation. (default: "close")
    MAD(usize, Option<String>),

    /// Maximum value over a specified period.
    /// - `usize`: The period over which to find the maximum value.
    /// - `Option<String>`: The Column name for the MAX calculation. (default: "close")
    MAX(usize, Option<String>),

    /// Minimum value over a specified period.
    /// - `usize`: The period over which to find the minimum value.
    /// - `Option<String>`: The Column name for the MIN calculation. (default: "close")
    MIN(usize, Option<String>),

    /// Average True Range (ATR) with a specified period.
    /// - `usize`: The period for the ATR calculation.
    ATR(usize),

    /// Rate of Change (ROC) with a specified period.
    /// - `usize`: The period for the Rate of Change calculation.
    /// - `Option<String>`: The Column name for the ROC calculation. (default: "close")
    ROC(usize, Option<String>),

    /// On-Balance Volume (OBV) indicator. This indicator does not require parameters.
    OBV(),
}
