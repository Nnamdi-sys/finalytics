use std::str::FromStr;
use ndarray::Array2;
use rayon::prelude::*;
use polars::frame::DataFrame;
use nlopt::{Algorithm, Nlopt, Target};
use serde::Deserialize;
use crate::analytics::statistics::{mean_portfolio_return, portfolio_std_dev, maximum_drawdown, value_at_risk,
                                   expected_shortfall, daily_portfolio_returns, portfolio_downside_dev};

/// Portfolio Optimization Result Struct
#[derive(Debug, Clone)]
pub struct OptResult {
    pub optimal_weights: Vec<f64>,
    pub category_weights: Vec<(String, String, f64)>,
    pub efficient_frontier: Vec<Vec<f64>>,
}

/// Objective functions for the optimization
#[derive(Debug, Clone, Copy)]
pub enum ObjectiveFunction {
    MaxSharpe,
    MaxSortino,
    MaxReturn,
    MinVol,
    MinVar,
    MinCVaR,
    MinDrawdown,
}

impl FromStr for ObjectiveFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "max_sharpe" => Ok(ObjectiveFunction::MaxSharpe),
            "max_sortino" => Ok(ObjectiveFunction::MaxSortino),
            "min_vol" => Ok(ObjectiveFunction::MinVol),
            "max_return" => Ok(ObjectiveFunction::MaxReturn),
            "min_drawdown" => Ok(ObjectiveFunction::MinDrawdown),
            "min_var" => Ok(ObjectiveFunction::MinVar),
            "min_cvar" => Ok(ObjectiveFunction::MinCVaR),
            _ => Err(format!("Unsupported objective function: {s}")),
        }
    }
}

/// Constraints for the Objective Functions
///
/// # Fields
///
/// * `asset_weights` - Optional lower and upper bound asset weight constraint for each ticker symbol in order
///   - `Vec<(f64, f64)>` - `Vec<(lower_bound, upper_bound)>`
/// * `categorical_weights` - Optional weights for multiple categorical constraints such as Asset Class, Sector, Industry, Region
#[derive(Debug, Clone)]
pub struct Constraints {
    pub asset_weights: Option<Vec<(f64, f64)>>,
    pub categorical_weights: Option<Vec<CategoricalWeights>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoricalWeights {
    pub name: String,
    pub category_per_symbol: Vec<String>,
    pub weight_per_category: Vec<(String, f64, f64)>,
}

/// Computes the optimal portfolio weights for a given set of assets based on a given objective function
/// and subject to a constraint for weights to sum to one and respect bounds (including negative)
///
/// # Arguments
///
/// * `mean_returns` - Vector of mean returns for each asset
/// * `cov_matrix` - Covariance matrix of asset returns
/// * `portfolio_returns` - DataFrame of portfolio returns for each asset
/// * `interval` - Interval for return calculations (e.g., daily, weekly)
/// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
/// * `confidence_level` - Confidence level for VaR and CVaR in decimal (e.g 0.95 for 95%)
/// * `objective` - Objective function to optimize (e.g. ObjectiveFunction::MaxSharpe)
/// * `constraints` - Constraints struct with optional asset and categorical constraints
///
/// # Returns
///
/// * `OptResult`
#[allow(clippy::too_many_arguments)]
pub fn portfolio_optimization(
    mean_returns: &Vec<f64>,
    cov_matrix: &Array2<f64>,
    portfolio_returns: &DataFrame,
    risk_free_rate: f64,
    confidence_level: f64,
    objective: ObjectiveFunction,
    constraints: Constraints,
) -> OptResult {
    let num_assets = mean_returns.len();

    // Determine bounds, default to [0.0, 1.0]
    let (mut lower_bounds, mut upper_bounds) = (vec![0.0; num_assets], vec![1.0; num_assets]);
    if let Some(bounds) = &constraints.asset_weights {
        let (new_lb, new_ub): (Vec<f64>, Vec<f64>) = bounds.iter().cloned().unzip();
        for i in 0..num_assets {
            lower_bounds[i] = f64::max(lower_bounds[i], new_lb.get(i).copied().unwrap_or(0.0));
            upper_bounds[i] = f64::min(upper_bounds[i], new_ub.get(i).copied().unwrap_or(1.0));
        }
    }

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
                ObjectiveFunction::MaxSortino => {
                    let downside_dev = portfolio_downside_dev(x, portfolio_returns);
                    if downside_dev == 0.0 {
                        -f64::INFINITY // Handle zero downside deviation
                    } else {
                        -((_return - risk_free_rate) / downside_dev)
                    }
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
    let _ = opt.set_upper_bounds(&upper_bounds);
    let _ = opt.add_equality_constraint(
        |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| x.iter().sum::<f64>() - 1.0,
        (),
        1e-6,
    );

    // Apply categorical constraints
    if let Some(category_types) = &constraints.categorical_weights {
        for cat_weights in category_types {
            // Asset weight constraints
            for (category, lb, ub) in cat_weights.weight_per_category.clone() {
                let indices: Vec<usize> = cat_weights
                    .category_per_symbol
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == category)
                    .map(|(i, _)| i)
                    .collect();
                if !indices.is_empty() {
                    let indices_clone = indices.clone();
                    let _ = opt.add_inequality_constraint(
                        move |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                            let sum: f64 = indices_clone.iter().map(|&i| x[i]).sum();
                            sum - ub
                        },
                        (),
                        1e-6,
                    );
                    let indices_clone = indices.clone();
                    let _ = opt.add_inequality_constraint(
                        move |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                            let sum: f64 = indices_clone.iter().map(|&i| x[i]).sum();
                            lb - sum
                        },
                        (),
                        1e-6,
                    );
                }
            }
        }
    }

    let _ = opt.set_xtol_rel(1e-3);
    let _ = opt.set_ftol_rel(1e-3);
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

    // Compute category asset weights
    let category_weights = {
        let mut cat_asset_weights = Vec::new();
        if let Some(category_types) = &constraints.categorical_weights {
            for cat_weights in category_types {
                for (category, _, _) in cat_weights.weight_per_category.clone() {
                    let indices: Vec<usize> = cat_weights
                        .category_per_symbol
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| **c == category)
                        .map(|(i, _)| i)
                        .collect();
                    if !indices.is_empty() {
                        let sum: f64 = indices.iter().map(|&i| optimal_weights[i]).sum();
                        cat_asset_weights.push((cat_weights.name.clone(), category.clone(), sum));
                    }
                }
            }
        }
        cat_asset_weights
    };

    // Compute efficient frontier with constraints
    let efficient_frontier = efficient_frontier_points(
        mean_returns,
        cov_matrix,
        &constraints,
        200,
    );

    OptResult {
        optimal_weights,
        category_weights,
        efficient_frontier,
    }
}

/// Computes efficient frontier points for given target returns with constraints
fn efficient_frontier_points(
    mean_returns: &Vec<f64>,
    cov_matrix: &Array2<f64>,
    constraints: &Constraints,
    num_ef_points: usize,
) -> Vec<Vec<f64>> {
    let num_assets = mean_returns.len();
    let initial_params: Vec<f64> = vec![1.0 / num_assets as f64; num_assets];

    // Determine bounds, default to [0.0, 1.0]
    let (mut lower_bounds, mut upper_bounds) = (vec![0.0; num_assets], vec![1.0; num_assets]);
    if let Some(bounds) = &constraints.asset_weights {
        let (new_lb, new_ub): (Vec<f64>, Vec<f64>) = bounds.iter().cloned().unzip();
        for i in 0..num_assets {
            lower_bounds[i] = f64::max(lower_bounds[i], new_lb.get(i).copied().unwrap_or(0.0));
            upper_bounds[i] = f64::min(upper_bounds[i], new_ub.get(i).copied().unwrap_or(1.0));
        }
    }

    // Compute target returns for efficient frontier
    let min_return = mean_returns.iter().zip(lower_bounds.iter()).map(|(r, lb)| r * lb).sum::<f64>();
    let max_return = mean_returns.iter().zip(upper_bounds.iter()).map(|(r, ub)| r * ub).sum::<f64>();
    let min_asset_return = min_return.max(-1.0);
    let max_asset_return = max_return;
    let target_returns = quad_space(min_asset_return, max_asset_return, num_ef_points);

    target_returns.par_iter().filter_map(|&target| {
        let mut ef_opt = Nlopt::new(
            Algorithm::Cobyla,
            num_assets,
            |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                portfolio_std_dev(x, cov_matrix)
            },
            Target::Minimize,
            (),
        );

        // Set bounds
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

        // Apply categorical constraints
        if let Some(category_types) = &constraints.categorical_weights {
            for cat_weights in category_types {
                // Asset weight constraints
                for (category, lb, ub) in cat_weights.weight_per_category.clone() {
                    let indices: Vec<usize> = cat_weights
                        .category_per_symbol
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| **c == category)
                        .map(|(i, _)| i)
                        .collect();
                    if !indices.is_empty() {
                        let indices_clone = indices.clone();
                        let _ = ef_opt.add_inequality_constraint(
                            move |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                                let sum: f64 = indices_clone.iter().map(|&i| x[i]).sum();
                                sum - ub
                            },
                            (),
                            1e-6,
                        );
                        let indices_clone = indices.clone();
                        let _ = ef_opt.add_inequality_constraint(
                            move |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                                let sum: f64 = indices_clone.iter().map(|&i| x[i]).sum();
                                lb - sum
                            },
                            (),
                            1e-6,
                        );
                    }
                }
            }
        }

        let _ = ef_opt.set_xtol_rel(1e-3);
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
    }).collect()
}

/// Helper to generate quadratically spaced points, with more points near start
fn quad_space(start: f64, end: f64, n: usize) -> Vec<f64> {
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

pub fn filter_constraints(
    constraints: Option<Constraints>,
    ticker_symbols: Vec<String>,
    fetched_symbols: Vec<String>,
) -> Constraints {
    match constraints {
        Some(c) => {
            // Filter asset weights
            let filtered_asset_weights = c.asset_weights.map(|weights| {
                ticker_symbols
                    .iter()
                    .zip(weights)
                    .filter(|(symbol, _)| fetched_symbols.contains(&symbol.to_string()))
                    .map(|(_, w)| w)
                    .collect()
            });

            // Filter categorical weights
            let filtered_categorical_weights = c.categorical_weights.map(|cats| {
                cats.into_iter()
                    .map(|cat| {
                        let filtered_categories = ticker_symbols
                            .iter()
                            .zip(cat.category_per_symbol)
                            .filter(|(symbol, _)| fetched_symbols.contains(&symbol.to_string()))
                            .map(|(_, category)| category)
                            .collect::<Vec<_>>();

                        // weight_per_category likely doesn’t need filtering unless
                        // some categories disappear entirely — in which case:
                        let filtered_weight_per_category = cat
                            .weight_per_category
                            .into_iter()
                            .filter(|(category_name, _, _)| {
                                filtered_categories.contains(category_name)
                            })
                            .collect::<Vec<_>>();

                        CategoricalWeights {
                            name: cat.name,
                            category_per_symbol: filtered_categories,
                            weight_per_category: filtered_weight_per_category,
                        }
                    })
                    .collect()
            });

            Constraints {
                asset_weights: filtered_asset_weights,
                categorical_weights: filtered_categorical_weights,
            }
        }
        None => Constraints {
            asset_weights: Some(vec![(0.0, 1.0); fetched_symbols.len()]),
            categorical_weights: None,
        },
    }
}