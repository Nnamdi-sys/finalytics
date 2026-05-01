use ndarray::{Array1, Array2};
use nlopt::{Algorithm, Nlopt, Target};
use polars::frame::DataFrame;
use rayon::prelude::*;
use serde::Deserialize;
use std::cmp::Ordering;
use std::str::FromStr;

// ===========================================================================
// Data structures
// ===========================================================================

/// Portfolio Optimization Result Struct (internal)
#[derive(Debug, Clone)]
pub struct OptResult {
    pub optimal_weights: Vec<f64>,
    pub category_weights: Vec<(String, String, f64)>,
    /// Mean-variance efficient frontier points `[return, risk]`.
    /// Populated only for frontier-type objectives (MaxSharpe, MaxSortino).
    pub efficient_frontier: Vec<Vec<f64>>,
    /// Component risk contribution of each asset under the optimal weights.
    pub risk_contributions: Vec<f64>,
    /// Name of the algorithm that produced the solution.
    pub optimization_method: String,
}

/// Objective functions for portfolio optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveFunction {
    MaxSharpe,
    MaxSortino,
    MaxReturn,
    MinVol,
    MinVar,
    MinCVaR,
    MinDrawdown,
    /// Equal Risk Contribution – each asset contributes equally to portfolio risk.
    RiskParity,
    /// Maximize the diversification ratio: (Σ wᵢσᵢ) / σ_portfolio.
    MaxDiversification,
    /// Hierarchical Risk Parity (López de Prado, 2016).
    /// Uses hierarchical clustering – no numerical optimiser needed.
    HierarchicalRiskParity,
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
            "risk_parity" => Ok(ObjectiveFunction::RiskParity),
            "max_diversification" => Ok(ObjectiveFunction::MaxDiversification),
            "hrp" | "hierarchical_risk_parity" => Ok(ObjectiveFunction::HierarchicalRiskParity),
            _ => Err(format!("Unsupported objective function: {s}")),
        }
    }
}

impl ObjectiveFunction {
    /// Returns `true` when the mean-variance efficient frontier is a meaningful
    /// visualisation for this objective.
    pub fn uses_frontier(&self) -> bool {
        matches!(
            self,
            ObjectiveFunction::MaxSharpe
                | ObjectiveFunction::MaxSortino
                | ObjectiveFunction::MaxReturn
                | ObjectiveFunction::MinVol
        )
    }

    /// Returns `true` when a risk-contribution chart is the preferred
    /// visualisation for this objective.
    pub fn uses_risk_contribution_chart(&self) -> bool {
        !self.uses_frontier()
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

// ===========================================================================
// Fast helpers – no DataFrame / Series allocations in the hot loop
// ===========================================================================

/// Pre-extract a returns DataFrame into a row-major `Vec<Vec<f64>>` so that
/// the objective function never touches Polars.
/// `returns_matrix[t][i]` = percentage return of asset `i` at time `t`.
fn extract_returns_matrix(df: &DataFrame) -> Vec<Vec<f64>> {
    let n = df.height();
    let cols: Vec<Vec<f64>> = df
        .get_columns()
        .iter()
        .map(|c| match c.f64() {
            Ok(ca) => ca.into_no_null_iter().collect::<Vec<f64>>(),
            Err(_) => {
                eprintln!(
                    "WARNING: column '{}' is not Float64 ({:?}), returning zeros",
                    c.name(),
                    c.dtype()
                );
                vec![0.0; c.len()]
            }
        })
        .collect();
    let num_assets = cols.len();
    let mut matrix = Vec::with_capacity(n);
    for t in 0..n {
        let mut row = Vec::with_capacity(num_assets);
        for col in &cols {
            row.push(col[t]);
        }
        matrix.push(row);
    }
    matrix
}

/// Compute simple weighted portfolio returns from the pre-extracted matrix.
/// `port_ret[t] = Σᵢ wᵢ · r_{t,i}` (percentage returns).
#[inline]
fn fast_weighted_returns(weights: &[f64], returns_matrix: &[Vec<f64>]) -> Vec<f64> {
    returns_matrix
        .iter()
        .map(|row| row.iter().zip(weights).map(|(r, w)| r * w).sum())
        .collect()
}

/// Downside deviation computed relative to a minimum acceptable return (target).
#[inline]
fn fast_downside_dev(port_returns: &[f64], target: f64) -> f64 {
    let n = port_returns.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let sum_sq: f64 = port_returns
        .iter()
        .map(|&r| {
            let diff = r - target;
            if diff < 0.0 {
                diff * diff
            } else {
                0.0
            }
        })
        .sum();
    (sum_sq / n).sqrt()
}

/// Maximum drawdown from percentage returns (returned as a positive percentage value).
#[inline]
fn fast_maximum_drawdown(port_returns: &[f64]) -> f64 {
    let mut cum = 1.0;
    let mut peak = 1.0;
    let mut max_dd = 0.0_f64;
    for &r in port_returns {
        cum *= 1.0 + r;
        if cum > peak {
            peak = cum;
        }
        let dd = (peak - cum) / peak;
        if dd > max_dd {
            max_dd = dd;
        }
    }
    max_dd
}

/// Historical Value-at-Risk from percentage returns (returned as a negative percentage value).
#[inline]
fn fast_value_at_risk(port_returns: &[f64], confidence_level: f64) -> f64 {
    let mut sorted = port_returns.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let index = ((1.0 - confidence_level) * (sorted.len() as f64 - 1.0)) as usize;
    sorted[index]
}

/// Historical Expected Shortfall (CVaR) from percentage returns.
#[inline]
fn fast_expected_shortfall(port_returns: &[f64], confidence_level: f64) -> f64 {
    let var = fast_value_at_risk(port_returns, confidence_level);
    let losses: Vec<f64> = port_returns.iter().filter(|&&x| x < var).copied().collect();
    if losses.is_empty() {
        var
    } else {
        losses.iter().sum::<f64>() / losses.len() as f64
    }
}

// ===========================================================================
// Risk decomposition
// ===========================================================================

/// Component risk contribution of each asset.
/// `CRC_i = w_i · (Σw)_i / σ_p` so that `Σ CRC_i = σ_p`.
pub fn risk_contributions(weights: &[f64], cov_matrix: &Array2<f64>) -> Vec<f64> {
    let n = weights.len();
    let w = Array1::from(weights.to_vec());
    let sigma_w = cov_matrix.dot(&w);
    let port_var = w.dot(&sigma_w);
    let port_vol = port_var.sqrt();
    if port_vol < 1e-14 {
        return vec![0.0; n];
    }
    (0..n).map(|i| weights[i] * sigma_w[i] / port_vol).collect()
}

/// Percentage risk contribution: `PRC_i = CRC_i / σ_p` (sums to 1.0).
pub fn pct_risk_contributions(weights: &[f64], cov_matrix: &Array2<f64>) -> Vec<f64> {
    let crc = risk_contributions(weights, cov_matrix);
    let total: f64 = crc.iter().sum();
    if total.abs() < 1e-14 {
        vec![1.0 / weights.len() as f64; weights.len()]
    } else {
        crc.iter().map(|c| c / total).collect()
    }
}

// ===========================================================================
// Constraint helpers shared across optimisations
// ===========================================================================

/// Resolve lower / upper bounds from constraints, defaulting to [0, 1].
fn resolve_bounds(num_assets: usize, constraints: &Constraints) -> (Vec<f64>, Vec<f64>) {
    let (mut lb, mut ub) = (vec![0.0; num_assets], vec![1.0; num_assets]);
    if let Some(bounds) = &constraints.asset_weights {
        let (new_lb, new_ub): (Vec<f64>, Vec<f64>) = bounds.iter().cloned().unzip();
        for i in 0..num_assets {
            lb[i] = f64::max(lb[i], new_lb.get(i).copied().unwrap_or(0.0));
            ub[i] = f64::min(ub[i], new_ub.get(i).copied().unwrap_or(1.0));
        }
    }
    (lb, ub)
}

/// Add the sum-to-one equality constraint to an NLopt handle.
fn add_sum_to_one<F: nlopt::ObjFn<T>, T>(opt: &mut Nlopt<F, T>) {
    let _ = opt.add_equality_constraint(
        |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| x.iter().sum::<f64>() - 1.0,
        (),
        1e-8,
    );
}

/// Add categorical (group) inequality constraints to an NLopt handle.
fn add_categorical_constraints<F: nlopt::ObjFn<T>, T>(
    opt: &mut Nlopt<F, T>,
    constraints: &Constraints,
) {
    if let Some(category_types) = &constraints.categorical_weights {
        for cat_weights in category_types {
            for (category, lb, ub) in cat_weights.weight_per_category.clone() {
                let indices: Vec<usize> = cat_weights
                    .category_per_symbol
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == category)
                    .map(|(i, _)| i)
                    .collect();
                if !indices.is_empty() {
                    // sum(w_i for i in group) <= ub
                    let idx = indices.clone();
                    let _ = opt.add_inequality_constraint(
                        move |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                            let sum: f64 = idx.iter().map(|&i| x[i]).sum();
                            sum - ub
                        },
                        (),
                        1e-8,
                    );
                    // lb <= sum(w_i for i in group)  →  lb - sum <= 0
                    let idx = indices.clone();
                    let _ = opt.add_inequality_constraint(
                        move |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                            let sum: f64 = idx.iter().map(|&i| x[i]).sum();
                            lb - sum
                        },
                        (),
                        1e-8,
                    );
                }
            }
        }
    }
}

/// Normalise weights so they sum to 1, with a warning threshold.
fn normalise_weights(weights: &mut Vec<f64>) {
    let sum: f64 = weights.iter().sum();
    if sum.abs() < 1e-14 {
        // Completely degenerate – fall back to equal
        let n = weights.len();
        *weights = vec![1.0 / n as f64; n];
        return;
    }
    if (sum - 1.0).abs() > 1e-6 {
        weights.iter_mut().for_each(|w| *w /= sum);
    }
}

/// Generate `n_starts` diverse feasible initial points for multi-start optimisation.
fn generate_starts(
    num_assets: usize,
    lower_bounds: &[f64],
    upper_bounds: &[f64],
    n_starts: usize,
) -> Vec<Vec<f64>> {
    let mut starts = Vec::with_capacity(n_starts);

    // Start 0: equal weight (clamped to bounds)
    let eq = vec![1.0 / num_assets as f64; num_assets];
    starts.push(clamp_and_normalise(&eq, lower_bounds, upper_bounds));

    // Additional random-ish feasible starts via deterministic perturbation
    for s in 1..n_starts {
        let mut w: Vec<f64> = (0..num_assets)
            .map(|i| {
                let mid = (lower_bounds[i] + upper_bounds[i]) / 2.0;
                let phase = (s as f64 * 0.618033988749 + i as f64 * 0.414213562373).fract();
                lower_bounds[i]
                    + (upper_bounds[i] - lower_bounds[i]) * phase.max(0.0).min(1.0) * 0.5
                    + mid * 0.5
            })
            .collect();
        let sum: f64 = w.iter().sum();
        if sum > 0.0 {
            w.iter_mut().for_each(|x| *x /= sum);
        }
        starts.push(clamp_and_normalise(&w, lower_bounds, upper_bounds));
    }
    starts
}

fn clamp_and_normalise(w: &[f64], lb: &[f64], ub: &[f64]) -> Vec<f64> {
    let mut out: Vec<f64> = w
        .iter()
        .enumerate()
        .map(|(i, &v)| v.max(lb[i]).min(ub[i]))
        .collect();
    let sum: f64 = out.iter().sum();
    if sum > 0.0 {
        out.iter_mut().for_each(|x| *x /= sum);
    }
    out
}

// ===========================================================================
// Compute category weights helper
// ===========================================================================

fn compute_category_weights(
    optimal_weights: &[f64],
    constraints: &Constraints,
) -> Vec<(String, String, f64)> {
    let mut cat_asset_weights = Vec::new();
    if let Some(category_types) = &constraints.categorical_weights {
        for cat_weights in category_types {
            for (category, _, _) in &cat_weights.weight_per_category {
                let indices: Vec<usize> = cat_weights
                    .category_per_symbol
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == *category)
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
}

// ===========================================================================
// Main optimisation entry point
// ===========================================================================

/// Computes the optimal portfolio weights for a given set of assets based on a given objective function
/// and subject to a constraint for weights to sum to one and respect bounds.
///
/// # Arguments
///
/// * `mean_returns` - Vector of mean returns for each asset
/// * `cov_matrix` - Covariance matrix of asset returns
/// * `portfolio_returns` - DataFrame of portfolio returns for each asset
/// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
/// * `confidence_level` - Confidence level for VaR and CVaR in decimal (e.g 0.95 for 95%)
/// * `objective` - Objective function to optimize
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
    // Dispatch to the appropriate solver
    match objective {
        ObjectiveFunction::HierarchicalRiskParity => {
            optimise_hrp(mean_returns, cov_matrix, portfolio_returns, &constraints)
        }
        _ => optimise_nlopt(
            mean_returns,
            cov_matrix,
            portfolio_returns,
            risk_free_rate,
            confidence_level,
            objective,
            &constraints,
        ),
    }
}

// ===========================================================================
// NLopt-based optimisation (all objectives except HRP)
// ===========================================================================

fn optimise_nlopt(
    mean_returns: &[f64],
    cov_matrix: &Array2<f64>,
    portfolio_returns: &DataFrame,
    risk_free_rate: f64,
    confidence_level: f64,
    objective: ObjectiveFunction,
    constraints: &Constraints,
) -> OptResult {
    let num_assets = mean_returns.len();
    let (lower_bounds, upper_bounds) = resolve_bounds(num_assets, constraints);

    // Pre-extract returns matrix once – avoids DataFrame access in the hot loop
    let returns_matrix = extract_returns_matrix(portfolio_returns);

    // COBYLA for all objectives – it is derivative-free, handles non-smooth
    // objectives gracefully, and is the most robust NLopt algorithm for the
    // diverse objective landscape here. At typical portfolio sizes (<50 assets)
    // the speed difference vs gradient-based solvers is negligible.
    let algorithm = Algorithm::Cobyla;

    // Determine number of starts: more for non-convex objectives
    let n_starts = match objective {
        ObjectiveFunction::MinVol | ObjectiveFunction::MaxReturn => 1,
        ObjectiveFunction::MaxSharpe
        | ObjectiveFunction::RiskParity
        | ObjectiveFunction::MaxDiversification => 2,
        _ => 3, // Non-convex: MaxSortino, MinDrawdown, MinVar, MinCVaR
    };

    let starts = generate_starts(num_assets, &lower_bounds, &upper_bounds, n_starts);

    // Extract per-asset volatilities for diversification ratio
    let asset_vols: Vec<f64> = (0..num_assets).map(|i| cov_matrix[(i, i)].sqrt()).collect();

    // Multi-start optimisation: run from each start, keep the best
    let results: Vec<(Vec<f64>, f64)> = starts
        .iter()
        .filter_map(|init| {
            run_single_optimisation(
                init,
                mean_returns,
                cov_matrix,
                &returns_matrix,
                &asset_vols,
                risk_free_rate,
                confidence_level,
                objective,
                constraints,
                &lower_bounds,
                &upper_bounds,
                algorithm,
            )
        })
        .collect();

    let mut optimal_weights = if results.is_empty() {
        // Ultimate fallback: equal weights
        vec![1.0 / num_assets as f64; num_assets]
    } else {
        results
            .into_iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
            .unwrap_or_else(|| {
                // Fallback: equal weights with zero objective value
                (vec![1.0 / num_assets as f64; num_assets], 0.0)
            })
            .0
    };

    normalise_weights(&mut optimal_weights);

    // Compute risk contributions
    let rc = risk_contributions(&optimal_weights, cov_matrix);

    // Compute category weights
    let category_weights = compute_category_weights(&optimal_weights, constraints);

    // Compute efficient frontier only for frontier-type objectives
    let efficient_frontier = if objective.uses_frontier() {
        efficient_frontier_points(mean_returns, cov_matrix, constraints, 80)
    } else {
        Vec::new()
    };

    OptResult {
        optimal_weights,
        category_weights,
        efficient_frontier,
        risk_contributions: rc,
        optimization_method: "COBYLA".to_string(),
    }
}

/// Run a single NLopt optimisation from a given starting point.
/// Returns `Some((weights, objective_value))` on success, `None` on failure.
#[allow(clippy::too_many_arguments)]
fn run_single_optimisation(
    initial: &[f64],
    mean_returns: &[f64],
    cov_matrix: &Array2<f64>,
    returns_matrix: &[Vec<f64>],
    asset_vols: &[f64],
    risk_free_rate: f64,
    confidence_level: f64,
    objective: ObjectiveFunction,
    constraints: &Constraints,
    lower_bounds: &[f64],
    upper_bounds: &[f64],
    algorithm: Algorithm,
) -> Option<(Vec<f64>, f64)> {
    let num_assets = mean_returns.len();

    let mut opt = Nlopt::new(
        algorithm,
        num_assets,
        |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
            evaluate_objective(
                x,
                mean_returns,
                cov_matrix,
                returns_matrix,
                asset_vols,
                risk_free_rate,
                confidence_level,
                objective,
            )
        },
        Target::Minimize,
        (),
    );

    let _ = opt.set_lower_bounds(lower_bounds);
    let _ = opt.set_upper_bounds(upper_bounds);
    add_sum_to_one(&mut opt);
    add_categorical_constraints(&mut opt, constraints);

    let _ = opt.set_xtol_rel(1e-6);
    let _ = opt.set_ftol_rel(1e-6);
    let max_eval = match objective {
        ObjectiveFunction::MinDrawdown | ObjectiveFunction::MinVar | ObjectiveFunction::MinCVaR => {
            3000
        }
        _ => 2000,
    };
    let _ = opt.set_maxeval(max_eval as u32);

    let mut weights = initial.to_vec();
    match opt.optimize(&mut weights) {
        Ok((_status, val)) => {
            // Verify the solution is feasible
            let sum: f64 = weights.iter().sum();
            if (sum - 1.0).abs() > 0.05 {
                // Grossly infeasible – reject
                None
            } else {
                normalise_weights(&mut weights);
                Some((weights, val))
            }
        }
        Err(_) => {
            // Check if we still got a reasonable solution despite the error
            let sum: f64 = weights.iter().sum();
            if (sum - 1.0).abs() < 0.05 {
                normalise_weights(&mut weights);
                let val = evaluate_objective(
                    &weights,
                    mean_returns,
                    cov_matrix,
                    returns_matrix,
                    asset_vols,
                    risk_free_rate,
                    confidence_level,
                    objective,
                );
                if val.is_finite() {
                    Some((weights, val))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

/// Evaluate the objective function value for a given weight vector.
/// All objectives are mapped to a minimisation target.
#[allow(clippy::too_many_arguments)]
fn evaluate_objective(
    x: &[f64],
    mean_returns: &[f64],
    cov_matrix: &Array2<f64>,
    returns_matrix: &[Vec<f64>],
    asset_vols: &[f64],
    risk_free_rate: f64,
    confidence_level: f64,
    objective: ObjectiveFunction,
) -> f64 {
    match objective {
        ObjectiveFunction::MaxSharpe => {
            let ret = fast_mean_return(x, mean_returns);
            let vol = fast_std_dev(x, cov_matrix);
            if vol < 1e-14 {
                1e10 // Avoid division by zero – return large penalty
            } else {
                -((ret - risk_free_rate) / vol)
            }
        }
        ObjectiveFunction::MaxSortino => {
            let ret = fast_mean_return(x, mean_returns);
            let port_rets = fast_weighted_returns(x, returns_matrix);
            let dd = fast_downside_dev(&port_rets, risk_free_rate);
            if dd < 1e-14 {
                -1e10 // Large negative = very good Sortino (capped to avoid optimizer issues)
            } else {
                -((ret - risk_free_rate) / dd)
            }
        }
        ObjectiveFunction::MaxReturn => {
            let ret = fast_mean_return(x, mean_returns);
            -ret
        }
        ObjectiveFunction::MinVol => fast_std_dev(x, cov_matrix),
        ObjectiveFunction::MinDrawdown => {
            let port_rets = fast_weighted_returns(x, returns_matrix);
            fast_maximum_drawdown(&port_rets)
        }
        ObjectiveFunction::MinVar => {
            let port_rets = fast_weighted_returns(x, returns_matrix);
            // VaR is negative; minimising -VaR = maximising VaR (pushing it toward zero)
            -fast_value_at_risk(&port_rets, confidence_level)
        }
        ObjectiveFunction::MinCVaR => {
            let port_rets = fast_weighted_returns(x, returns_matrix);
            -fast_expected_shortfall(&port_rets, confidence_level)
        }
        ObjectiveFunction::RiskParity => {
            // Minimise the sum of squared differences in risk contributions
            risk_parity_objective(x, cov_matrix)
        }
        ObjectiveFunction::MaxDiversification => {
            // Maximise diversification ratio = Σ(wᵢσᵢ) / σ_p
            let weighted_vol_sum: f64 = x.iter().zip(asset_vols).map(|(w, v)| w * v).sum();
            let port_vol = fast_std_dev(x, cov_matrix);
            if port_vol < 1e-14 {
                1e10
            } else {
                -(weighted_vol_sum / port_vol)
            }
        }
        ObjectiveFunction::HierarchicalRiskParity => {
            // Should not be called – HRP uses its own path
            0.0
        }
    }
}

/// Fast mean portfolio return (simple dot product, no Series allocation).
#[inline]
fn fast_mean_return(weights: &[f64], mean_returns: &[f64]) -> f64 {
    weights.iter().zip(mean_returns).map(|(w, r)| w * r).sum()
}

/// Fast portfolio standard deviation via covariance matrix.
#[inline]
fn fast_std_dev(weights: &[f64], cov_matrix: &Array2<f64>) -> f64 {
    let w = Array1::from(weights.to_vec());
    let var = w.dot(&cov_matrix.dot(&w));
    if var < 0.0 {
        0.0
    } else {
        var.sqrt()
    }
}

/// Risk parity objective: minimise Σᵢ Σⱼ (wᵢ(Σw)ᵢ − wⱼ(Σw)ⱼ)²
fn risk_parity_objective(weights: &[f64], cov_matrix: &Array2<f64>) -> f64 {
    let n = weights.len();
    let w = Array1::from(weights.to_vec());
    let sigma_w = cov_matrix.dot(&w);
    // risk_contrib[i] = w_i * (Σw)_i
    let rc: Vec<f64> = (0..n).map(|i| weights[i] * sigma_w[i]).collect();
    let mut obj = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            let diff = rc[i] - rc[j];
            obj += diff * diff;
        }
    }
    obj
}

// ===========================================================================
// Hierarchical Risk Parity (López de Prado, 2016)
// ===========================================================================

fn optimise_hrp(
    mean_returns: &[f64],
    cov_matrix: &Array2<f64>,
    _portfolio_returns: &DataFrame,
    constraints: &Constraints,
) -> OptResult {
    let num_assets = mean_returns.len();

    // Step 1: correlation matrix from covariance matrix
    let corr = cov_to_corr(cov_matrix);

    // Step 2: distance matrix  d_ij = sqrt(0.5 * (1 - ρ_ij))
    let dist = corr_to_distance(&corr);

    // Step 3: single-linkage agglomerative clustering
    let linkage = single_linkage_clustering(&dist);

    // Step 4: quasi-diagonalisation – get the leaf ordering from the dendrogram
    let order = quasi_diagonalize(&linkage, num_assets);

    // Step 5: recursive bisection to allocate weights
    let mut weights = vec![1.0; num_assets];
    recursive_bisection(&mut weights, &order, cov_matrix);

    // Apply constraint clamping (best-effort for HRP since it's not optimiser-based)
    let (lb, ub) = resolve_bounds(num_assets, constraints);
    for i in 0..num_assets {
        weights[i] = weights[i].max(lb[i]).min(ub[i]);
    }
    normalise_weights(&mut weights);

    let rc = risk_contributions(&weights, cov_matrix);
    let category_weights = compute_category_weights(&weights, constraints);

    OptResult {
        optimal_weights: weights,
        category_weights,
        efficient_frontier: Vec::new(),
        risk_contributions: rc,
        optimization_method: "HRP".to_string(),
    }
}

/// Convert covariance matrix to correlation matrix.
fn cov_to_corr(cov: &Array2<f64>) -> Array2<f64> {
    let n = cov.nrows();
    let mut corr = Array2::zeros((n, n));
    let std_devs: Vec<f64> = (0..n).map(|i| cov[(i, i)].sqrt()).collect();
    for i in 0..n {
        for j in 0..n {
            if std_devs[i] > 0.0 && std_devs[j] > 0.0 {
                corr[(i, j)] = cov[(i, j)] / (std_devs[i] * std_devs[j]);
            }
        }
    }
    corr
}

/// Correlation → distance: d_ij = sqrt(0.5 * (1 − ρ_ij))
fn corr_to_distance(corr: &Array2<f64>) -> Array2<f64> {
    let n = corr.nrows();
    let mut dist = Array2::zeros((n, n));
    for i in 0..n {
        for j in 0..n {
            dist[(i, j)] = (0.5 * (1.0 - corr[(i, j)])).max(0.0).sqrt();
        }
    }
    dist
}

/// Linkage row: [idx_a, idx_b, distance, size]
type LinkageRow = (usize, usize, f64, usize);

/// Single-linkage agglomerative clustering.
/// Returns linkage matrix as Vec of (cluster_a, cluster_b, distance, new_size).
fn single_linkage_clustering(dist: &Array2<f64>) -> Vec<LinkageRow> {
    let n = dist.nrows();
    let mut active: Vec<bool> = vec![true; n];
    // Mutable distance matrix (condensed representation would be faster but this is fine for
    // the typical asset counts <200)
    let mut d = dist.clone();
    let mut cluster_size: Vec<usize> = vec![1; 2 * n]; // up to n-1 merges
    let mut linkage: Vec<LinkageRow> = Vec::with_capacity(n - 1);

    for step in 0..(n - 1) {
        // Find the closest pair among active clusters
        let mut best_i = 0;
        let mut best_j = 1;
        let mut best_d = f64::MAX;
        for i in 0..n + step {
            if i >= active.len() || !active[i] {
                continue;
            }
            for j in (i + 1)..(n + step) {
                if j >= active.len() || !active[j] {
                    continue;
                }
                let dij = if i < n && j < n {
                    d[(i, j)]
                } else {
                    // Distance between merged clusters stored in expanded form
                    f64::MAX // Will be set below
                };
                if dij < best_d {
                    best_d = dij;
                    best_i = i;
                    best_j = j;
                }
            }
        }

        let new_id = n + step;
        let new_size = cluster_size[best_i] + cluster_size[best_j];
        cluster_size.push(new_size);
        cluster_size[new_id] = new_size;
        linkage.push((best_i, best_j, best_d, new_size));

        active.push(true);
        active[best_i] = false;
        active[best_j] = false;

        // Update distance matrix: expand d to accommodate new cluster
        // For single linkage: d(new, k) = min(d(i, k), d(j, k))
        let old_rows = d.nrows();
        let mut new_d = Array2::from_elem((old_rows + 1, old_rows + 1), f64::MAX);
        for r in 0..old_rows {
            for c in 0..old_rows {
                new_d[(r, c)] = d[(r, c)];
            }
        }
        for k in 0..(old_rows + 1) {
            if k == new_id || !active.get(k).copied().unwrap_or(false) {
                new_d[(new_id, k)] = f64::MAX;
                new_d[(k, new_id)] = f64::MAX;
                continue;
            }
            let d_ik = if best_i < old_rows && k < old_rows {
                d[(best_i, k)]
            } else {
                new_d[(best_i, k)]
            };
            let d_jk = if best_j < old_rows && k < old_rows {
                d[(best_j, k)]
            } else {
                new_d[(best_j, k)]
            };
            let min_d = d_ik.min(d_jk);
            new_d[(new_id, k)] = min_d;
            new_d[(k, new_id)] = min_d;
        }
        new_d[(new_id, new_id)] = 0.0;
        d = new_d;
    }

    linkage
}

/// Quasi-diagonalize: traverse the dendrogram to produce leaf ordering.
fn quasi_diagonalize(linkage: &[LinkageRow], n: usize) -> Vec<usize> {
    if linkage.is_empty() {
        return (0..n).collect();
    }
    let root = n + linkage.len() - 1;
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if node < n {
            order.push(node);
        } else {
            let link_idx = node - n;
            if link_idx < linkage.len() {
                let (a, b, _, _) = linkage[link_idx];
                // Push in reverse so that left subtree is processed first
                stack.push(b);
                stack.push(a);
            }
        }
    }
    order
}

/// Recursive bisection: allocate weights by inverse-variance at each split.
fn recursive_bisection(weights: &mut Vec<f64>, order: &[usize], cov_matrix: &Array2<f64>) {
    if order.len() <= 1 {
        return;
    }

    let mid = order.len() / 2;
    let left = &order[..mid];
    let right = &order[mid..];

    let var_left = cluster_variance(left, cov_matrix);
    let var_right = cluster_variance(right, cov_matrix);

    let alloc = if var_left + var_right > 0.0 {
        1.0 - var_left / (var_left + var_right)
    } else {
        0.5
    };

    for &i in left {
        weights[i] *= alloc;
    }
    for &i in right {
        weights[i] *= 1.0 - alloc;
    }

    recursive_bisection(weights, left, cov_matrix);
    recursive_bisection(weights, right, cov_matrix);
}

/// Variance of a cluster of assets using the inverse-variance portfolio within the cluster.
fn cluster_variance(indices: &[usize], cov_matrix: &Array2<f64>) -> f64 {
    if indices.len() == 1 {
        return cov_matrix[(indices[0], indices[0])];
    }
    // Inverse-variance weights within the cluster
    let inv_var: Vec<f64> = indices
        .iter()
        .map(|&i| {
            let v = cov_matrix[(i, i)];
            if v > 0.0 {
                1.0 / v
            } else {
                1.0
            }
        })
        .collect();
    let sum_iv: f64 = inv_var.iter().sum();
    let w: Vec<f64> = inv_var.iter().map(|v| v / sum_iv).collect();

    // Cluster variance = w^T Σ_sub w
    let mut var = 0.0;
    for (wi_idx, &i) in indices.iter().enumerate() {
        for (wj_idx, &j) in indices.iter().enumerate() {
            var += w[wi_idx] * w[wj_idx] * cov_matrix[(i, j)];
        }
    }
    var
}

// ===========================================================================
// Efficient Frontier (mean-variance, COBYLA-based)
// ===========================================================================

/// Computes efficient frontier points for given target returns with constraints.
/// Uses COBYLA for robustness across all constraint configurations.
fn efficient_frontier_points(
    mean_returns: &[f64],
    cov_matrix: &Array2<f64>,
    constraints: &Constraints,
    num_ef_points: usize,
) -> Vec<Vec<f64>> {
    let num_assets = mean_returns.len();
    let (lower_bounds, upper_bounds) = resolve_bounds(num_assets, constraints);

    // Find the feasible return range
    let min_return: f64 = mean_returns
        .iter()
        .zip(lower_bounds.iter())
        .map(|(r, lb)| r * lb)
        .sum();
    let max_return: f64 = mean_returns
        .iter()
        .zip(upper_bounds.iter())
        .map(|(r, ub)| r * ub)
        .sum();
    let min_ret = min_return.max(-1.0);
    let max_ret = max_return;

    // Use linear spacing (not quadratic) — we want even coverage of the efficient half
    let targets = linspace(min_ret, max_ret, num_ef_points);

    let initial_params: Vec<f64> = vec![1.0 / num_assets as f64; num_assets];

    let mut points: Vec<Vec<f64>> = targets
        .par_iter()
        .filter_map(|&target| {
            let mut ef_opt = Nlopt::new(
                Algorithm::Cobyla,
                num_assets,
                |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                    fast_std_dev(x, cov_matrix)
                },
                Target::Minimize,
                (),
            );

            let _ = ef_opt.set_lower_bounds(&lower_bounds);
            let _ = ef_opt.set_upper_bounds(&upper_bounds);

            // Sum-to-one
            let _ = ef_opt.add_equality_constraint(
                |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                    x.iter().sum::<f64>() - 1.0
                },
                (),
                1e-8,
            );

            // Target return
            let _ = ef_opt.add_equality_constraint(
                |x: &[f64], _gradient: Option<&mut [f64]>, _param: &mut ()| {
                    fast_mean_return(x, mean_returns) - target
                },
                (),
                1e-8,
            );

            // Categorical constraints
            add_categorical_constraints(&mut ef_opt, constraints);

            let _ = ef_opt.set_xtol_rel(1e-5);
            let _ = ef_opt.set_ftol_rel(1e-5);
            let _ = ef_opt.set_maxeval(500);

            let mut ef_weights = initial_params.clone();
            let _ = ef_opt.optimize(&mut ef_weights);

            // Accept if approximately feasible
            let sum_w: f64 = ef_weights.iter().sum();
            if (sum_w - 1.0).abs() > 0.05 {
                return None;
            }
            normalise_weights(&mut ef_weights);

            let actual_return = fast_mean_return(&ef_weights, mean_returns);
            let actual_std_dev = fast_std_dev(&ef_weights, cov_matrix);

            Some(vec![actual_return, actual_std_dev])
        })
        .collect();

    // Sort by risk and filter to keep only the efficient (upper) half of the frontier
    points.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap_or(Ordering::Equal));

    // Find the minimum-variance point
    if let Some(min_var_idx) = points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a[1].partial_cmp(&b[1]).unwrap_or(Ordering::Equal))
        .map(|(i, _)| i)
    {
        // Keep only points at or above the minimum-variance portfolio's return
        let min_var_return = points[min_var_idx][0];
        points.retain(|p| p[0] >= min_var_return - 1e-10);
    }

    // Deduplicate points that are very close together
    points.dedup_by(|a, b| (a[0] - b[0]).abs() < 1e-8 && (a[1] - b[1]).abs() < 1e-8);

    points
}

/// Linearly spaced points from start to end (inclusive).
fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n <= 1 {
        return vec![start];
    }
    (0..n)
        .map(|i| start + (end - start) * (i as f64) / (n as f64 - 1.0))
        .collect()
}

// ===========================================================================
// Constraint filtering
// ===========================================================================

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
