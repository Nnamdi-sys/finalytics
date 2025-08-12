use std::str::FromStr;
use ndarray::Array2;
use rayon::prelude::*;
use polars::frame::DataFrame;
use nlopt::{Algorithm, Nlopt, Target};
use crate::analytics::statistics::{
    mean_portfolio_return, portfolio_std_dev, maximum_drawdown,
    value_at_risk, expected_shortfall, daily_portfolio_returns};

/// Portfolio Optimization Result Struct
#[derive(Debug, Clone)]
pub struct OptResult {
    pub optimal_weights: Vec<f64>,
    pub efficient_frontier: Vec<Vec<f64>>,
}

/// Objective functions for the optimization
#[derive(Debug, Clone, Copy)]
pub enum ObjectiveFunction {
    MaxSharpe,
    MinVol,
    MaxReturn,
    MinDrawdown,
    MinVar,
    MinCVaR,
}

impl FromStr for ObjectiveFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "max_sharpe" => Ok(ObjectiveFunction::MaxSharpe),
            "min_vol" => Ok(ObjectiveFunction::MinVol),
            "max_return" => Ok(ObjectiveFunction::MaxReturn),
            "min_drawdown" => Ok(ObjectiveFunction::MinDrawdown),
            "min_var" => Ok(ObjectiveFunction::MinVar),
            "min_cvar" => Ok(ObjectiveFunction::MinCVaR),
            _ => Err(format!("Unsupported objective function: {s}")),
        }
    }
}

/// Computes the optimal portfolio weights for a given set of assets based on a given objective function
/// and subject to a constraint for weights to sum to one and respect bounds (including negative)
///
/// # Arguments
///
/// * `mean_returns` - Vector of mean returns for each asset
/// * `cov_matrix` - Covariance matrix of asset returns
/// * `portfolio_returns` - DataFrame of portfolio returns for each asset
/// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
/// * `confidence_level` - Confidence level for VaR and CVaR in decimal (e.g 0.95 for 95%)
/// * `objective` - Objective function to optimize (e.g. ObjectiveFunction::MaxSharpe)
/// * `constraints` - Vector of (lower_bound, upper_bound) for each asset, allowing negative lb
///
/// # Returns
///
/// * `Result<OptResult, String>` - Returns Ok(OptResult) or Err if optimization fails critically
pub fn portfolio_optimization(
    mean_returns: &Vec<f64>,
    cov_matrix: &Array2<f64>,
    portfolio_returns: &DataFrame,
    risk_free_rate: f64,
    confidence_level: f64,
    objective: ObjectiveFunction,
    constraints: Vec<(f64, f64)>,
) -> OptResult {
    let num_assets = mean_returns.len();
    let (lower_bounds, upper_bounds): (Vec<f64>, Vec<f64>) = constraints.iter().cloned().unzip();

    // Initialize NLopt with COBYLA for robustness
    let mut opt = Nlopt::new(
        Algorithm::Cobyla,
        num_assets,
        |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
            let _return = mean_portfolio_return(x, mean_returns);
            let std_dev = portfolio_std_dev(x, cov_matrix);
            match objective {
                ObjectiveFunction::MaxSharpe => {
                    -((_return - risk_free_rate) / std_dev)
                },
                ObjectiveFunction::MinVol => std_dev,
                ObjectiveFunction::MaxReturn => -_return,
                ObjectiveFunction::MinDrawdown => {
                    let returns = daily_portfolio_returns(x, portfolio_returns);
                    maximum_drawdown(&returns).1
                },
                ObjectiveFunction::MinVar => {
                    let returns = daily_portfolio_returns(x, portfolio_returns);
                    -value_at_risk(&returns, confidence_level)
                },
                ObjectiveFunction::MinCVaR => {
                    let returns = daily_portfolio_returns(x, portfolio_returns);
                    -expected_shortfall(&returns, confidence_level)
                }
            }
        },
        Target::Minimize,
        (),
    );

    // Set bounds
    let _ = opt.set_lower_bounds(&lower_bounds);
    let _ =opt.set_upper_bounds(&upper_bounds);
    let _ =opt.add_equality_constraint(
        |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| x.iter().sum::<f64>() - 1.0,
        (),
        1e-6,
    );
    let _ =opt.set_xtol_rel(1e-3);
    let _ =opt.set_ftol_rel(1e-3);
    let _ = opt.set_maxeval(2000);

    // Initial guess: equal weights
    let initial_params: Vec<f64> = vec![1.0 / num_assets as f64; num_assets];
    let mut optimal_weights = initial_params.clone();

    // Optimize
    let _ = opt.optimize(&mut optimal_weights);

    // Normalize weights
    let sum_w: f64 = optimal_weights.iter().sum();
    if (sum_w - 1.0).abs() > 1e-6 {
        optimal_weights.iter_mut().for_each(|w| *w /= sum_w);
    }

    // Compute feasible return range
    let min_return = mean_returns.iter().zip(lower_bounds.iter()).map(|(r, lb)| r * lb).sum::<f64>();
    let max_return = mean_returns.iter().zip(upper_bounds.iter()).map(|(r, ub)| r * ub).sum::<f64>();
    let min_asset_return = min_return.max(-1.0);
    let max_asset_return = max_return;

    let num_ef_points = 200;
    let target_returns = quadspace(min_asset_return, max_asset_return, num_ef_points);

    let efficient_frontier: Vec<Vec<f64>> = target_returns.par_iter().filter_map(|&target| {
        let mut ef_opt = Nlopt::new(
            Algorithm::Cobyla,
            num_assets,
            |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                portfolio_std_dev(x, cov_matrix)
            },
            Target::Minimize,
            (),
        );

        let _ = ef_opt.set_lower_bounds(&lower_bounds);
        let _ = ef_opt.set_upper_bounds(&upper_bounds);
        let _ = ef_opt.add_equality_constraint(
            |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| x.iter().sum::<f64>() - 1.0,
            (),
            1e-6,
        );
        let _ = ef_opt.add_equality_constraint(
            |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                mean_portfolio_return(x, mean_returns) - target
            },
            (),
            1e-6,
        );
        let _ =ef_opt.set_xtol_rel(1e-3);
        let _ = ef_opt.set_ftol_rel(1e-3);
        let _ = ef_opt.set_maxeval(500);

        let mut ef_weights = initial_params.clone();
        let _ = ef_opt.optimize(&mut ef_weights);

        let sum_w: f64 = ef_weights.iter().sum();
        if (sum_w - 1.0).abs() > 1e-6 {
            ef_weights.iter_mut().for_each(|w| *w /= sum_w);
        }

        let actual_return = mean_portfolio_return(&ef_weights, mean_returns);
        let actual_std_dev = portfolio_std_dev(&ef_weights, cov_matrix);

        Some(vec![actual_return, actual_std_dev])
    }).collect();

    //let efficient_frontier = efficient_frontier_points(ef_points);

    OptResult {
        optimal_weights,
        efficient_frontier,
    }
}

/// Helper to generate quadratically spaced points, with more points near start
fn quadspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n <= 1 {
        return vec![start];
    }
    let mut points = Vec::with_capacity(n);
    for i in 0..n {
        let t = (i as f64) / (n as f64 - 1.0);
        let t_quad = t * t; // Quadratic spacing: more points near start
        points.push(start + (end - start) * t_quad);
    }
    points
}