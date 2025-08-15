use std::cmp::Ordering;
use polars::prelude::*;
use std::error::Error;
use std::ops::Mul;
use smartcore::linalg::basic::arrays::{Array, Array2};
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::linear_regression::LinearRegression;
use statrs::statistics::Statistics;
use statrs::distribution::{ContinuousCDF, Normal};
use crate::prelude::IntervalDays;

#[derive(Debug, Clone, Default)]
pub struct PerformanceStats {
    pub daily_return: f64,
    pub daily_volatility: f64,
    pub cumulative_return: f64,
    pub annualized_return: f64,
    pub annualized_volatility: f64,
    pub alpha: f64,
    pub beta: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub active_return: f64,
    pub active_risk: f64,
    pub information_ratio: f64,
    pub calmar_ratio: f64,
    pub maximum_drawdown: f64,
    pub value_at_risk: f64,
    pub expected_shortfall: f64,
}

impl PerformanceStats {
    /// Computes the performance statistics of a series of security returns
    ///
    /// # Arguments
    ///
    /// * `returns` - Polars Series of security returns
    /// * `benchmark_returns` - Polars Series of benchmark returns
    /// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
    /// * `confidence_level` - Confidence level for the VaR and CVaR calculations in decimal (e.g. 0.95 for 95%)
    ///
    /// # Returns
    ///
    /// * `PerformanceStats` struct
    pub fn compute_stats(
        returns: Series,
        benchmark_returns: Series,
        risk_free_rate: f64,
        confidence_level: f64,
        interval: IntervalDays,
    ) -> Result<PerformanceStats, Box<dyn Error>> {
        let _len = returns.len();
        let days = interval.mode;
        let annual_days = 365.0/interval.average;
        let risk_free_rate = risk_free_rate * 100.0;
        let cumulative_return = cumulative_return(&returns);
        let daily_return = returns.mean().ok_or("Error calculating mean return")?/days;
        let daily_volatility = std_dev(&returns);
        let annualized_return = ((1.0 + daily_return/100.0).powf(annual_days) - 1.0) * 100.0;
        let annualized_volatility = daily_volatility * annual_days.sqrt();
        let (alpha, beta) = ols_regression(&returns.clone(), &benchmark_returns.clone());
        let sharpe_ratio = (annualized_return - risk_free_rate) / annualized_volatility;
        let sortino_ratio = (annualized_return - risk_free_rate) / (downside_deviation(&returns) * annual_days.sqrt());
        let excess_returns = (returns.clone() - benchmark_returns.clone())?;
        let active_return = excess_returns.mean().ok_or("Error calculating active return")?;
        let active_return = ((1.0 + active_return/100.0).powf(annual_days) - 1.0) * 100.0;
        let active_risk = std_dev(&excess_returns) * annual_days.sqrt();
        let information_ratio = active_return / active_risk;
        let (_, maximum_drawdown) = maximum_drawdown(&returns);
        let calmar_ratio = annualized_return / maximum_drawdown;
        let value_at_risk = value_at_risk(&returns, confidence_level);
        let expected_shortfall = expected_shortfall(&returns, confidence_level);
        Ok(PerformanceStats {
            daily_return,
            daily_volatility,
            cumulative_return,
            annualized_return,
            annualized_volatility,
            alpha,
            beta,
            sharpe_ratio,
            sortino_ratio,
            active_return,
            active_risk,
            information_ratio,
            calmar_ratio,
            maximum_drawdown,
            value_at_risk,
            expected_shortfall,
        })
    }
}

/// computes the standard deviation of a series of security returns
///
/// # Arguments
///
/// * `series` - Polars Series of security returns
///
/// # Returns
///
/// * `f64` - Standard deviation
pub fn std_dev(series: &Series) -> f64 {
    let dev_vec = series.f64().unwrap().to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
    dev_vec.population_std_dev()
}

/// computes the downside deviation of a series of security returns
///
/// # Arguments
///
/// * `series` - Polars Series of security returns
///
/// # Returns
///
/// * `f64` - Downside deviation
pub fn downside_deviation(series: &Series) -> f64 {
    let downside_mask = &series.lt_eq(0.0).unwrap();
    let downside_returns = series.filter(downside_mask).unwrap();
    std_dev(&downside_returns)
}

/// Computes the z-score corresponding to the confidence level
///
/// # Arguments
///
/// * `confidence_level` - Confidence level in decimal (e.g. 0.95 for 95%)
///
/// # Returns
///
/// * `f64` - Z-score
pub fn z_score(confidence_level: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap(); // Mean=0, Standard Deviation=1 for standard normal distribution
    normal.inverse_cdf(confidence_level)
}


/// Computes the alpha and beta of a series of security returns
///
/// # Arguments
///
/// * `x_series` - Polars Series of security returns
/// * `y_series` - Polars Series of benchmark returns
///
/// # Returns
///
/// * `(f64, f64)` - Tuple of alpha and beta
pub fn ols_regression(x_series: &Series, y_series: &Series) -> (f64, f64) {
    // Convert Polars Series to Vec<f64>
    let x_data= x_series.f64().unwrap().to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
    let y_data = y_series.f64().unwrap().to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();

    // Create a matrix from x_data
    let x_matrix = DenseMatrix::from_column(&x_data);

    // Create a Linear Regression model
    let model = LinearRegression::fit(&x_matrix, &y_data, Default::default()).unwrap();

    // Get the intercept and slope
    let intercept = *model.intercept();
    let slope = *model.coefficients().get((0,0));

    (intercept, slope)
}

/// Computes the covariance matrix of a polars dataframe of security returns
///
/// # Arguments
///
/// * `df` - Polars DataFrame of security returns
///
/// # Returns
///
/// * `ndarray::Array2<f64>` - Covariance matrix
pub fn covariance_matrix(df: &DataFrame) -> Result<ndarray::Array2<f64>, Box<dyn Error>> {
    let num_columns = df.width();
    let mut covariance_matrix = ndarray::Array2::zeros((num_columns, num_columns));

    for i in 0..num_columns {
        for j in 0..num_columns {
            let series_i = df.select_at_idx(i).unwrap().f64()?.to_vec().iter()
                .map(|x| x.unwrap()).collect::<Vec<f64>>();
            let series_j = df.select_at_idx(j).unwrap().f64()?.to_vec().iter()
                .map(|x| x.unwrap()).collect::<Vec<f64>>();
            let cov = series_i.population_covariance(series_j);
            covariance_matrix[(i, j)] = cov;
        }
    }

    Ok(covariance_matrix)
}


/// Computes the correlation matrix of a polars dataframe of security returns
///
/// # Arguments
///
/// * `df` - Polars DataFrame of security returns
///
/// # Returns
///
/// * `ndarray::Array2<f64>` - Correlation matrix
pub fn correlation_matrix(df: &DataFrame) -> Result<ndarray::Array2<f64>, Box<dyn Error>> {
    let covariance_matrix = covariance_matrix(df)?;
    let num_columns = covariance_matrix.nrows();

    let mut correlation_matrix = ndarray::Array2::zeros((num_columns, num_columns));

    // Calculate standard deviations of each column
    let std_devs= (0..num_columns)
        .map(|i| {
            let series = df.select_at_idx(i).unwrap().f64()?.into_iter()
                .map(|x| x.unwrap()).collect::<Vec<f64>>();
            Ok(series.std_dev())
        })
        .collect::<Result<Vec<f64>, PolarsError>>()?;

    // Compute the correlation matrix
    for i in 0..num_columns {
        for j in 0..num_columns {
            let std_dev_i = std_devs[i];
            let std_dev_j = std_devs[j];
            if std_dev_i != 0.0 && std_dev_j != 0.0 {
                correlation_matrix[(i, j)] = covariance_matrix[(i, j)] / (std_dev_i * std_dev_j);
            } else {
                correlation_matrix[(i, j)] = 0.0;
            }
        }
    }

    Ok(correlation_matrix)
}

/// computes the maximum drawdown of a series of security returns
///
/// # Arguments
///
/// * `returns` - Polars Series of security returns
///
/// # Returns
///
/// * `(Vec<f64>, f64)` - Rolling drawdowns and maximum drawdown
pub fn maximum_drawdown(returns: &Series) -> (Vec<f64>, f64) {
    // Convert the Series into a Vec<f64>
    let returns = returns
        .f64()
        .unwrap()
        .to_vec()
        .iter()
        .map(|x| x.unwrap())
        .collect::<Vec<f64>>();

    // Step 1: Calculate cumulative returns
    let mut cumulative_returns = Vec::with_capacity(returns.len());
    let mut cumulative_sum = 0.0;
    for &return_value in &returns {
        cumulative_sum += return_value;
        cumulative_returns.push(cumulative_sum);
    }

    // Step 2: Calculate cumulative maximum of cumulative returns
    let mut cumulative_max = Vec::with_capacity(cumulative_returns.len());
    let mut current_max = cumulative_returns[0];
    for &cum_return in &cumulative_returns {
        if cum_return > current_max {
            current_max = cum_return;
        }
        cumulative_max.push(current_max);
    }

    // Step 3: Calculate drawdowns
    let mut rolling_drawdowns = Vec::new();
    let mut max_drawdown = 0.0;
    for (cum_max, cum_return) in cumulative_max.iter().zip(cumulative_returns.iter()) {
        let drawdown = cum_max - cum_return; // Difference between the peak and current value
        rolling_drawdowns.push(-drawdown); // Negative drawdown for underwater plot
        if drawdown > max_drawdown {
            max_drawdown = drawdown; // Update maximum drawdown
        }
    }

    // Return rolling drawdowns and the overall maximum drawdown
    (rolling_drawdowns, max_drawdown)
}


/// computes the value at risk of a series of security returns
///
/// # Arguments
///
/// * `returns` - Polars Series of security returns
/// * `confidence_level` - Confidence level in decimal (e.g. 0.95 for 95%)
///
/// # Returns
///
/// * `f64` - Value at risk
pub fn value_at_risk(returns: &Series, confidence_level: f64) -> f64 {
    let returns = returns.f64().unwrap().to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
    let mut sorted_returns = returns.clone();
    sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let index = ((1.0 - confidence_level) * (returns.len() as f64 - 1.0)) as usize;
    sorted_returns[index]
}

/// computes the expected shortfall of a series of security returns
///
/// # Arguments
///
/// * `returns` - Polars Series of security returns
/// * `confidence_level` - Confidence level in decimal (e.g. 0.95 for 95%)
///
/// # Returns
///
/// * `f64` - Expected shortfall
pub fn expected_shortfall(returns: &Series, confidence_level: f64) -> f64 {
    let var = value_at_risk(returns, confidence_level);
    let returns = returns.f64().unwrap().to_vec().iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
    let loss_returns = returns.iter().filter(|&x| x < &var).cloned().collect::<Vec<f64>>();
    let es = loss_returns.iter().sum::<f64>() / (loss_returns.len() as f64);
    es
}

/// Computes the mean return of a portfolio
///
/// # Arguments
///
/// * `weights` - Vector of portfolio weights
/// * `mean_returns` - Vector of mean returns
///
/// # Returns
///
/// * `f64` - Mean portfolio return
pub fn mean_portfolio_return(weights: &[f64], mean_returns: &Vec<f64>) -> f64 {
    let weights = Series::new("Weights".into(), weights);
    let mean_returns = Series::new("Mean Returns".into(), mean_returns);
    let weighted_returns = mean_returns.mul(weights).unwrap();
    weighted_returns.sum().unwrap()
}

/// Computes the standard deviation of a portfolio
///
/// # Arguments
///
/// * `weights` - Vector of portfolio weights
/// * `cov_matrix` - Covariance matrix of security returns
///
/// # Returns
///
/// * `f64` - Portfolio standard deviation
pub fn portfolio_std_dev(weights: &[f64], cov_matrix: &ndarray::Array2<f64>) -> f64 {
    let _len = weights.len();
    let weights = ndarray::Array1::from(weights.to_vec());
    let portfolio_variance = weights.dot(cov_matrix).dot(&weights.t());
    portfolio_variance.sqrt()
}

/// Computes the downside deviation of a portfolio
///
/// # Arguments
///
/// * `weights` - Vector of portfolio weights
/// * `portfolio_returns` - DataFrame of portfolio returns for each asset
///
/// # Returns
///
/// * `f64` - Portfolio downside deviation
pub fn portfolio_downside_dev(weights: &[f64], portfolio_returns: &DataFrame) -> f64 {
    let port_returns = daily_portfolio_returns(weights, portfolio_returns);
    let downside_returns = port_returns
        .f64()
        .unwrap()
        .into_iter()
        .filter_map(|opt| opt.map(|r| if r < 0.0 { r.powi(2) } else { 0.0 }))
        .collect::<Vec<f64>>();

    let n = port_returns.len() as f64; // Use total number of returns
    if n == 0.0 {
        return 0.0; // Handle empty returns
    }

    let mean_downside = downside_returns.iter().sum::<f64>() / n;
    mean_downside.sqrt()
}

/// Computes the daily/time_interval returns of a portfolio given the weights and asset returns
///
/// # Arguments
///
/// * `weights` - Vector of portfolio weights
/// * `returns` - Polars DataFrame of security returns
///
/// # Returns
///
/// * `Series` - Portfolio returns
pub fn daily_portfolio_returns(weights: &[f64], returns: &DataFrame) -> Series {
    let mut portfolio_returns = Series::new("Portfolio Returns".into(), vec![0.0; returns.height()]);
    for (i, weight) in weights.iter().enumerate() {
        let col_str = returns.get_column_names()[i];
        let security_returns = returns.column(col_str).unwrap().f64().unwrap().to_vec();
        let weighted_returns = security_returns.iter().map(|x| x.unwrap() * weight).collect::<Vec<f64>>();
        portfolio_returns = (portfolio_returns + Series::new("Portfolio Returns".into(), weighted_returns)).unwrap();
    }
    portfolio_returns
}

/// Computes the cumulative return of a series of security returns
///
/// # Arguments
///
/// * `returns` - Polars Series of security returns
///
/// # Returns
///
/// * `f64` - Cumulative return
pub fn cumulative_return(returns: &Series) -> f64 {
    // Calculate the cumulative return
    let cumulative_returns = returns.f64().unwrap().to_vec().iter().map(|x|
        1.0 + (x.unwrap()/100.0)).collect::<Vec<f64>>();
    let cumulative_return = (cumulative_returns.iter().product::<f64>() - 1.0) * 100.0;
    cumulative_return
}

/// Computes the daily/time_interval cumulative returns of a returns series
///
/// # Arguments
///
/// * `returns` - Polars Series of security returns
///
/// # Returns
///
/// * `Series` - Cumulative returns
pub fn cumulative_returns_list(returns: Vec<f64>) -> Vec<f64> {
    let mut cumulative_returns = Vec::new();
    let mut cumulative_return = 1.0;

    for return_value in returns {
        cumulative_return *= 1.0 + return_value/100.0;
        cumulative_returns.push(cumulative_return - 1.0);
    }

    cumulative_returns
}

/// Performs a non-zero linear interpolation on a vector of values
///
/// # Arguments
///
/// * `vec` - Vector of values
///
/// # Returns
///
/// * `Vec<f64>` - Vector of interpolated values
pub fn linear_interpolation(vec: Vec<f64>) -> Vec<f64> {
    let mut vec = vec.clone();
    let len = vec.len();

    for i in 0..len {
        if vec[i] == 0.0 {
            let mut left_index = i;
            let mut right_index = i;

            // Find the left and right non-zero values
            while left_index > 0 && vec[left_index] == 0.0 {
                left_index -= 1;
            }
            while right_index < len - 1 && vec[right_index] == 0.0 {
                right_index += 1;
            }

            // Perform linear interpolation
            if vec[left_index] != 0.0 && vec[right_index] != 0.0 {
                let left_value = vec[left_index];
                let right_value = vec[right_index];
                let interpolation_ratio = (i - left_index) as f64 / (right_index - left_index) as f64;
                vec[i] = left_value + (right_value - left_value) * interpolation_ratio;
            } else if vec[left_index] != 0.0 {
                // If only left value is non-zero, set the interpolated value to the left value
                vec[i] = vec[left_index];
            } else if vec[right_index] != 0.0 {
                // If only right value is non-zero, set the interpolated value to the right value
                vec[i] = vec[right_index];
            }
        }
    }
    vec
}



