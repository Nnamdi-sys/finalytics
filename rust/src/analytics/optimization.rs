use std::str::FromStr;
use std::sync::{Arc, RwLock};
use polars::frame::DataFrame;
use optimization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};
use crate::analytics::statistics::{mean_portfolio_return, portfolio_std_dev, rand_weights, maximum_drawdown,
                                   value_at_risk, expected_shortfall, daily_portfolio_returns, efficient_frontier_points};

/// Portfolio Optimization Result Struct
#[derive(Debug, Clone)]
pub struct OptResult {
    pub optimal_weights: Vec<f64>,
    pub efficient_frontier: Vec<Vec<f64>>,
}

/// Objective functions for the optimization
///
/// MaxSharpe: Maximize the Sharpe Ratio
/// MinVol: Minimize the portfolio volatility
/// MaxReturn: Maximize the portfolio return
/// MinDrawdown: Minimize the maximum drawdown
/// MinVar: Minimize the portfolio VaR
/// MinCVaR: Minimize the portfolio CVaR
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
/// and subject to a constraint for weights to sum to one and be non-negative
///
/// # Arguments
///
/// * `mean_returns` - Vector of mean returns for each asset
/// * `cov_matrix` - Covariance matrix of asset returns
/// * `portfolio_returns` - DataFrame of portfolio returns for each asset
/// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
/// * `confidence_level` - Confidence level for VaR and CVaR in decimal (e.g 0.95 for 95%)
/// * `max_iterations` - Maximum number of iterations for the optimization (e.g. 1000)
/// * `objective` - Objective function to optimize (e.g. ObjectiveFunction::MaxSharpe)
///
/// # Returns
///
/// * `OptResult` struct
pub fn portfolio_optimization(
    mean_returns: &Vec<f64>,
    cov_matrix: &ndarray::Array2<f64>,
    portfolio_returns: &DataFrame,
    risk_free_rate: f64,
    confidence_level: f64,
    objective: ObjectiveFunction,
    constraints: Vec<(f64, f64)>,
) -> OptResult {
    // objective: max_sharpe, min_vol, max_return, min_drawdown, min_var, min_cvar
    let efficient_frontier: Arc<RwLock<Vec<Vec<f64>>>> = Arc::new(RwLock::new(Vec::new()));
    let efficient_frontier_clone = Arc::clone(&efficient_frontier);

    // We use the gradient descent method to minimize the objective function
    let function = NumericalDifferentiation::new(Func(|weights: &[f64]| {
        let weights = enforce_constraints(weights, &constraints);
        let _return = mean_portfolio_return(&weights.to_vec(), mean_returns);
        let std_dev = portfolio_std_dev(&weights.to_vec(), cov_matrix);
        if let Ok(mut guard) = efficient_frontier_clone.write() {
            guard.push(vec![_return, std_dev]);
        }
        match objective {
            ObjectiveFunction::MaxSharpe => {
                let sharpe = (_return - risk_free_rate) / std_dev;
                -sharpe
            },
            ObjectiveFunction::MinVol => {
                std_dev
            },
            ObjectiveFunction::MaxReturn => {
                -_return
            },
            ObjectiveFunction::MinDrawdown => {
                let returns = daily_portfolio_returns(&weights, portfolio_returns);
                let (_, drawdown) = maximum_drawdown(&returns);
                drawdown
            },
            ObjectiveFunction::MinVar => {
                let returns = daily_portfolio_returns(&weights, portfolio_returns);
                let var = value_at_risk(&returns, confidence_level);
                -var
            },
            ObjectiveFunction::MinCVaR => {
                let returns = daily_portfolio_returns(&weights, portfolio_returns);
                let es = expected_shortfall(&returns, confidence_level);
                -es
            }
        }
    }));

    // We use a simple gradient descent scheme
    let minimizer = GradientDescent::new();
    let minimizer = minimizer.max_iterations(Some(1000));
    let minimizer = minimizer.gradient_tolerance(1e-3);

    // Initial guess for portfolio weights
    let num_assets = mean_returns.len();
    let initial_weights = rand_weights(num_assets);

    // Perform the actual minimization
    let solution = minimizer.minimize(&function, initial_weights);

    // Enforce the constraints on the solution
    let constrained_solution = enforce_constraints(&solution.position, &constraints);
    let efficient_frontier = efficient_frontier_points(efficient_frontier.read().unwrap().clone());
    OptResult {
        optimal_weights: constrained_solution,
        efficient_frontier: efficient_frontier.clone(),
    }
}

fn enforce_constraints(weights: &[f64], constraints: &[(f64, f64)]) -> Vec<f64> {
    let mut constrained_weights: Vec<f64> = weights.to_vec();

    // Apply upper and lower bounds
    for (i, &(lb, ub)) in constraints.iter().enumerate() {
        if constrained_weights[i] < lb {
            constrained_weights[i] = lb;
        } else if constrained_weights[i] > ub {
            constrained_weights[i] = ub;
        }
    }

    // Normalize weights to ensure they sum to one
    let sum: f64 = constrained_weights.iter().sum();
    constrained_weights.iter_mut().for_each(|w| *w /= sum);

    constrained_weights
}