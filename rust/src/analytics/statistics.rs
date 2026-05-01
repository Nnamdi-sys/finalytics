use crate::data::yahoo::config::Interval;
use crate::error::{series_to_vec_f64, FinalyticsError};
use crate::models::portfolio::{is_period_boundary, RebalanceStrategy, ScheduleFrequency};
use crate::prelude::IntervalDays;
use chrono::{Datelike, Months, NaiveDate};
use polars::prelude::*;
use smartcore::linalg::basic::arrays::{Array, Array2};
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::linear_regression::LinearRegression;
use statrs::distribution::{ContinuousCDF, Normal};
use statrs::statistics::Statistics;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

// ---------------------------------------------------------------------------
// Rebalancing configuration
// ---------------------------------------------------------------------------

/// Bundles the target weights and strategy for the simulation loop.
///
/// Constructed by `Portfolio` from the optimization result (or allocation-derived
/// weights) and the user-supplied `RebalanceStrategy`.
#[derive(Debug, Clone)]
pub struct RebalanceConfig {
    /// Target weight vector the portfolio is rebalanced towards.
    pub target_weights: Vec<f64>,
    /// When / how rebalancing is triggered.
    pub strategy: RebalanceStrategy,
}

/// A single rebalance event recorded during the simulation.
#[derive(Debug, Clone)]
pub struct RebalanceEvent {
    /// Row index in the simulation (0-based).
    pub row: usize,
    /// Date string at which the rebalance occurred.
    pub date: String,
    /// One-way turnover: Σ |Δwᵢ| / 2, where Δwᵢ is the weight change per asset.
    pub turnover: f64,
}

// ---------------------------------------------------------------------------
// Transaction event tracking (enriched rebalance + cash flow events)
// ---------------------------------------------------------------------------

/// Describes what kind of portfolio event occurred on a given simulation row.
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionEventType {
    /// A rebalance was triggered (no external cash flow on this row).
    Rebalance,
    /// An external cash flow was applied (no rebalance on this row).
    CashFlow,
    /// Both an external cash flow and a rebalance occurred on the same row.
    RebalanceAndCashFlow,
}

impl std::fmt::Display for TransactionEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionEventType::Rebalance => write!(f, "Rebalance"),
            TransactionEventType::CashFlow => write!(f, "Cash Flow"),
            TransactionEventType::RebalanceAndCashFlow => write!(f, "Rebalance + Cash Flow"),
        }
    }
}

/// A single transaction event recorded during the simulation.
///
/// Captures per-asset detail for any row where a rebalance and/or external
/// cash flow occurred.  This is the enriched replacement used by the
/// transaction-history table; the legacy [`RebalanceEvent`] is still emitted
/// for backward compatibility.
#[derive(Debug, Clone)]
pub struct TransactionEvent {
    /// Row index in the simulation (0-based).
    pub row: usize,
    /// Date string at which the event occurred.
    pub date: String,
    /// What happened on this row.
    pub event_type: TransactionEventType,
    /// Total portfolio value *before* any cash flows or rebalancing on this row
    /// (i.e. after asset returns have been applied).
    pub portfolio_value_before: f64,
    /// Total portfolio value *after* all cash flows and rebalancing on this row.
    pub portfolio_value_after: f64,
    /// Per-asset dollar values before cash flows / rebalancing.
    pub asset_values_before: Vec<f64>,
    /// Per-asset dollar values after cash flows / rebalancing.
    pub asset_values_after: Vec<f64>,
    /// Per-asset trade amounts (`after − before`).
    /// Positive = buy / inflow, negative = sell / outflow.
    pub trade_amounts: Vec<f64>,
    /// One-way turnover for rebalance events (0.0 when no rebalance).
    pub turnover: f64,
    /// Total external cash flow on this row (positive = addition, negative = withdrawal).
    /// Zero when the event is a pure rebalance.
    pub cash_flow_amount: f64,
    /// Cumulative time-weighted return (%) from inception to this row.
    pub cumulative_twr: f64,
    /// Cumulative money-weighted return (annualised %) from inception to this
    /// row.  `None` when there are no external cash flows yet (MWR == TWR) or
    /// when the solver fails to converge.
    pub cumulative_mwr: Option<f64>,
}

// ---------------------------------------------------------------------------
// XIRR solver (money-weighted / internal rate of return)
// ---------------------------------------------------------------------------

/// A single dated cash flow for the XIRR calculation.
#[derive(Debug, Clone)]
pub struct DatedCashFlow {
    pub date: NaiveDate,
    pub amount: f64,
}

/// Compute the annualised internal rate of return (XIRR) for a series of
/// irregularly-spaced cash flows using Newton-Raphson iteration.
///
/// Cash-flow sign convention (standard XIRR):
/// * Negative = money leaving the investor (investment / contribution)
/// * Positive = money returning to the investor (withdrawal / terminal value)
///
/// Returns `None` if the solver fails to converge within the iteration limit.
pub fn xirr(cash_flows: &[DatedCashFlow]) -> Option<f64> {
    if cash_flows.len() < 2 {
        return None;
    }

    let t0 = cash_flows[0].date;

    // Year fractions relative to first cash flow
    let year_fracs: Vec<f64> = cash_flows
        .iter()
        .map(|cf| (cf.date - t0).num_days() as f64 / 365.0)
        .collect();

    // NPV(r) = Σ CF_i / (1+r)^t_i
    let npv = |r: f64| -> f64 {
        cash_flows
            .iter()
            .zip(year_fracs.iter())
            .map(|(cf, &t)| cf.amount / (1.0 + r).powf(t))
            .sum::<f64>()
    };

    // d(NPV)/dr = Σ -t_i * CF_i / (1+r)^(t_i+1)
    let dnpv = |r: f64| -> f64 {
        cash_flows
            .iter()
            .zip(year_fracs.iter())
            .map(|(cf, &t)| -t * cf.amount / (1.0 + r).powf(t + 1.0))
            .sum::<f64>()
    };

    let max_iter = 200;
    let tol = 1e-9;
    let mut r = 0.1_f64; // initial guess 10%

    for _ in 0..max_iter {
        let f = npv(r);
        let df = dnpv(r);

        if df.abs() < 1e-14 {
            // Derivative too small — try a perturbation
            r += 0.01;
            continue;
        }

        let step = f / df;
        r -= step;

        // Guard against divergence
        if r <= -1.0 {
            r = -0.99;
        }

        if step.abs() < tol {
            return Some(r);
        }
    }

    // Fallback: bisection on a wide bracket if Newton failed
    xirr_bisection(cash_flows, &year_fracs)
}

/// Bisection fallback for XIRR when Newton-Raphson diverges.
fn xirr_bisection(cash_flows: &[DatedCashFlow], year_fracs: &[f64]) -> Option<f64> {
    let npv = |r: f64| -> f64 {
        cash_flows
            .iter()
            .zip(year_fracs.iter())
            .map(|(cf, &t)| cf.amount / (1.0 + r).powf(t))
            .sum::<f64>()
    };

    let mut lo = -0.99_f64;
    let mut hi = 10.0_f64;
    let max_iter = 300;
    let tol = 1e-9;

    let f_lo = npv(lo);
    let f_hi = npv(hi);

    // Need sign change
    if f_lo.signum() == f_hi.signum() {
        return None;
    }

    for _ in 0..max_iter {
        let mid = (lo + hi) / 2.0;
        let f_mid = npv(mid);
        if f_mid.abs() < tol || (hi - lo) < tol {
            return Some(mid);
        }
        if f_mid.signum() == f_lo.signum() {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    None
}

/// Compute the cumulative time-weighted return (%) from a series of
/// percentage returns up to (and including) the given row index.
fn cumulative_twr_at_row(portfolio_returns: &[f64], row: usize) -> f64 {
    let mut cum = 1.0_f64;
    for i in 0..=row {
        if i < portfolio_returns.len() {
            cum *= 1.0 + portfolio_returns[i];
        }
    }
    cum - 1.0
}

/// Checks whether a rebalance should fire on the current row.
///
/// * `row` — current simulation row (0-based)
/// * `dates` — full date string array
/// * `current_values` — current per-asset dollar values (post-return, pre-rebalance)
/// * `config` — rebalance configuration
/// * `last_rebalance_row` — row index of the most recent rebalance (or 0)
fn should_rebalance(
    row: usize,
    dates: &[String],
    current_values: &[f64],
    config: &RebalanceConfig,
    last_rebalance_row: usize,
) -> bool {
    if row == 0 {
        return false; // never rebalance on the very first row
    }

    let calendar_trigger = |freq: ScheduleFrequency| -> bool {
        // Compare current date with previous date to detect a period boundary
        is_period_boundary(&dates[row - 1], &dates[row], freq)
    };

    let threshold_trigger = |threshold: f64| -> bool {
        let total: f64 = current_values.iter().sum();
        if total <= 0.0 {
            return false;
        }
        for (i, &val) in current_values.iter().enumerate() {
            let current_w = val / total;
            let target_w = config.target_weights[i];
            if (current_w - target_w).abs() > threshold {
                return true;
            }
        }
        false
    };

    // Avoid rebalancing twice on the same row (shouldn't happen, but guard)
    if row == last_rebalance_row {
        return false;
    }

    match &config.strategy {
        RebalanceStrategy::Calendar(freq) => calendar_trigger(*freq),
        RebalanceStrategy::Threshold(t) => threshold_trigger(*t),
        RebalanceStrategy::CalendarOrThreshold(freq, t) => {
            calendar_trigger(*freq) || threshold_trigger(*t)
        }
    }
}

/// Apply a rebalance: redistribute `values` so that weights match `target_weights`
/// while keeping the total portfolio value constant.
///
/// Returns the one-way turnover (Σ |Δwᵢ| / 2).
fn apply_rebalance(values: &mut [f64], target_weights: &[f64]) -> f64 {
    let total: f64 = values.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let mut turnover = 0.0;
    for (i, val) in values.iter_mut().enumerate() {
        let old_w = *val / total;
        let new_w = target_weights[i];
        turnover += (new_w - old_w).abs();
        *val = total * new_w;
    }
    turnover / 2.0
}

/// Distribute a `Rebalance`-allocated cash flow across assets to move weights
/// closer to the target.
///
/// For **additions** (amount > 0): allocates new money proportionally to each
/// asset's weight deficit (`max(0, target_w − current_w)`). If no asset is
/// underweight, falls back to pro-rata by target weights.
///
/// For **withdrawals** (amount < 0): withdraws proportionally from each asset's
/// weight surplus (`max(0, current_w − target_w)`). If no asset is overweight,
/// falls back to pro-rata by target weights.
fn distribute_rebalance_cash_flow(values: &mut [f64], amount: f64, target_weights: &[f64]) {
    let total: f64 = values.iter().sum();
    if total <= 0.0 || amount.abs() < 1e-12 {
        return;
    }

    let num_assets = values.len();
    let current_weights: Vec<f64> = values.iter().map(|v| v / total).collect();

    if amount > 0.0 {
        // Addition: direct to underweight assets
        let deficits: Vec<f64> = (0..num_assets)
            .map(|i| (target_weights[i] - current_weights[i]).max(0.0))
            .collect();
        let deficit_sum: f64 = deficits.iter().sum();
        if deficit_sum > 1e-12 {
            for (i, val) in values.iter_mut().enumerate() {
                *val += amount * deficits[i] / deficit_sum;
            }
        } else {
            // No underweight assets — fall back to pro-rata
            for (i, val) in values.iter_mut().enumerate() {
                *val += amount * target_weights[i];
            }
        }
    } else {
        // Withdrawal (amount < 0): take from overweight assets
        let abs_amount = amount.abs();
        let surpluses: Vec<f64> = (0..num_assets)
            .map(|i| (current_weights[i] - target_weights[i]).max(0.0))
            .collect();
        let surplus_sum: f64 = surpluses.iter().sum();
        if surplus_sum > 1e-12 {
            for (i, val) in values.iter_mut().enumerate() {
                *val -= abs_amount * surpluses[i] / surplus_sum;
                if *val < 0.0 {
                    *val = 0.0;
                }
            }
        } else {
            // No overweight assets — fall back to pro-rata
            for (i, val) in values.iter_mut().enumerate() {
                *val -= abs_amount * target_weights[i];
                if *val < 0.0 {
                    *val = 0.0;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ReturnsFrequency enum
// ---------------------------------------------------------------------------

/// Supported aggregation frequencies for returns tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReturnsFrequency {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl std::fmt::Display for ReturnsFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ReturnsFrequency::Hourly => "Hourly",
            ReturnsFrequency::Daily => "Daily",
            ReturnsFrequency::Weekly => "Weekly",
            ReturnsFrequency::Monthly => "Monthly",
            ReturnsFrequency::Yearly => "Yearly",
        };
        write!(f, "{s}")
    }
}

impl ReturnsFrequency {
    /// Derive the native (base) frequency from an `IntervalDays`.
    pub fn from_interval(interval: &IntervalDays) -> Self {
        let mode = interval.mode;
        if mode < 1.0 {
            ReturnsFrequency::Hourly
        } else if mode < 4.0 {
            ReturnsFrequency::Daily
        } else if mode < 15.0 {
            ReturnsFrequency::Weekly
        } else if mode < 100.0 {
            ReturnsFrequency::Monthly
        } else {
            ReturnsFrequency::Yearly
        }
    }

    /// Derive the native (base) frequency from a Yahoo `Interval` enum.
    pub fn from_interval_enum(interval: Interval) -> Self {
        match interval {
            Interval::TwoMinutes
            | Interval::FiveMinutes
            | Interval::FifteenMinutes
            | Interval::ThirtyMinutes
            | Interval::SixtyMinutes
            | Interval::NinetyMinutes
            | Interval::OneHour => ReturnsFrequency::Hourly,
            Interval::OneDay => ReturnsFrequency::Daily,
            Interval::FiveDays | Interval::OneWeek => ReturnsFrequency::Weekly,
            Interval::OneMonth | Interval::ThreeMonths => ReturnsFrequency::Monthly,
        }
    }

    /// Returns all frequencies that are >= `self` in the hierarchy, in
    /// ascending order.
    pub fn available_frequencies(&self) -> Vec<ReturnsFrequency> {
        use ReturnsFrequency::*;
        let all = [Hourly, Daily, Weekly, Monthly, Yearly];
        all.iter().copied().filter(|f| f >= self).collect()
    }
}

// ---------------------------------------------------------------------------
// PerformancePeriod enum
// ---------------------------------------------------------------------------

/// Standard trailing-window periods for performance statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerformancePeriod {
    Full,
    SixMonths,
    OneYear,
    ThreeYears,
    FiveYears,
    TenYears,
}

impl std::fmt::Display for PerformancePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PerformancePeriod::Full => "Full",
            PerformancePeriod::SixMonths => "6M",
            PerformancePeriod::OneYear => "1Y",
            PerformancePeriod::ThreeYears => "3Y",
            PerformancePeriod::FiveYears => "5Y",
            PerformancePeriod::TenYears => "10Y",
        };
        write!(f, "{s}")
    }
}

impl PerformancePeriod {
    /// Number of months this period spans. `None` for `Full`.
    pub fn months(&self) -> Option<u32> {
        match self {
            PerformancePeriod::Full => None,
            PerformancePeriod::SixMonths => Some(6),
            PerformancePeriod::OneYear => Some(12),
            PerformancePeriod::ThreeYears => Some(36),
            PerformancePeriod::FiveYears => Some(60),
            PerformancePeriod::TenYears => Some(120),
        }
    }

    /// All sub-periods in canonical order (excludes Full).
    pub fn sub_periods() -> &'static [PerformancePeriod] {
        &[
            PerformancePeriod::SixMonths,
            PerformancePeriod::OneYear,
            PerformancePeriod::ThreeYears,
            PerformancePeriod::FiveYears,
            PerformancePeriod::TenYears,
        ]
    }
}

// ---------------------------------------------------------------------------
// Resampling helpers (public)
// ---------------------------------------------------------------------------

/// Map a date-string to a grouping key for the given target frequency.
/// Expects dates in `YYYY-MM-DD ...` or `YYYY-MM-DD` format.
pub fn date_to_group_key(date: &str, freq: ReturnsFrequency) -> String {
    let date_part = &date[..date.len().min(10)];

    match freq {
        ReturnsFrequency::Hourly => {
            if date.len() >= 13 {
                date[..13].to_string()
            } else {
                date_part.to_string()
            }
        }
        ReturnsFrequency::Daily => date_part.to_string(),
        ReturnsFrequency::Weekly => {
            if let Some(nd) = parse_naive_date(date_part) {
                let iso = nd.iso_week();
                format!("{}-W{:02}", iso.year(), iso.week())
            } else {
                date_part.to_string()
            }
        }
        ReturnsFrequency::Monthly => {
            if date_part.len() >= 7 {
                date_part[..7].to_string()
            } else {
                date_part.to_string()
            }
        }
        ReturnsFrequency::Yearly => {
            if date_part.len() >= 4 {
                date_part[..4].to_string()
            } else {
                date_part.to_string()
            }
        }
    }
}

/// Parse `YYYY-MM-DD` (or longer) into a `chrono::NaiveDate`.
pub fn parse_naive_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(&s[..s.len().min(10)], "%Y-%m-%d").ok()
}

/// Identify group boundaries. Returns a vec of `(group_key, start_row, end_row_exclusive)`.
pub fn compute_groups(dates: &[String], freq: ReturnsFrequency) -> Vec<(String, usize, usize)> {
    if dates.is_empty() {
        return Vec::new();
    }
    let mut groups: Vec<(String, usize, usize)> = Vec::new();
    let mut current_key = date_to_group_key(&dates[0], freq);
    let mut start = 0_usize;

    for (i, d) in dates.iter().enumerate().skip(1) {
        let key = date_to_group_key(d, freq);
        if key != current_key {
            groups.push((current_key, start, i));
            current_key = key;
            start = i;
        }
    }
    groups.push((current_key, start, dates.len()));
    groups
}

/// Resample a percentage-returns DataFrame by compounding within each group.
///
/// `returns_df` has one column per asset (no timestamp column).
/// Returns `(group_labels, resampled_df)` where `resampled_df` has the same
/// column names and one row per group.
pub fn resample_returns_pct(
    dates: &[String],
    returns_df: &DataFrame,
    freq: ReturnsFrequency,
) -> Result<(Vec<String>, DataFrame), Box<dyn Error>> {
    let groups = compute_groups(dates, freq);
    let col_names: Vec<String> = returns_df
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut labels: Vec<String> = Vec::with_capacity(groups.len());
    let mut col_vecs: Vec<Vec<f64>> = vec![Vec::with_capacity(groups.len()); col_names.len()];

    for (key, start, end) in &groups {
        labels.push(key.clone());
        for (ci, name) in col_names.iter().enumerate() {
            let series = returns_df.column(name)?.f64()?;
            let mut compounded = 1.0_f64;
            for row in *start..*end {
                let r = series.get(row).unwrap_or(0.0);
                compounded *= 1.0 + r;
            }
            col_vecs[ci].push(compounded - 1.0);
        }
    }

    let columns: Vec<Column> = col_names
        .iter()
        .zip(col_vecs.into_iter())
        .map(|(name, vals)| Column::new(name.as_str().into(), vals))
        .collect();

    let df = DataFrame::new(columns)?;
    Ok((labels, df))
}

/// Resample a percentage-returns Series (e.g. portfolio aggregate) by compounding.
/// Returns `(group_labels, resampled_values)`.
pub fn resample_series_pct(
    dates: &[String],
    series: &Series,
    freq: ReturnsFrequency,
) -> Result<(Vec<String>, Vec<f64>), Box<dyn Error>> {
    let groups = compute_groups(dates, freq);
    let f64_ca = series.f64()?;
    let mut labels = Vec::with_capacity(groups.len());
    let mut result = Vec::with_capacity(groups.len());
    for (key, start, end) in &groups {
        labels.push(key.clone());
        let mut compounded = 1.0_f64;
        for row in *start..*end {
            let r = f64_ca.get(row).unwrap_or(0.0);
            compounded *= 1.0 + r;
        }
        result.push(compounded - 1.0);
    }
    Ok((labels, result))
}

/// Resample dollar-value rows by taking the last value in each group.
///
/// `asset_values` is `[num_rows][num_assets]`. `portfolio_values` is `[num_rows]`.
/// Returns `(group_labels, sampled_asset_values, sampled_portfolio_values)`.
pub fn resample_values_last(
    dates: &[String],
    asset_values: &[Vec<f64>],
    portfolio_values: &[f64],
    freq: ReturnsFrequency,
) -> (Vec<String>, Vec<Vec<f64>>, Vec<f64>) {
    let groups = compute_groups(dates, freq);
    let mut labels = Vec::with_capacity(groups.len());
    let mut sampled_assets = Vec::with_capacity(groups.len());
    let mut sampled_portfolio = Vec::with_capacity(groups.len());

    for (key, _start, end) in &groups {
        let last = end - 1;
        labels.push(key.clone());
        if last < asset_values.len() {
            sampled_assets.push(asset_values[last].clone());
        } else {
            sampled_assets.push(Vec::new());
        }
        if last < portfolio_values.len() {
            sampled_portfolio.push(portfolio_values[last]);
        } else {
            sampled_portfolio.push(0.0);
        }
    }

    (labels, sampled_assets, sampled_portfolio)
}

// ---------------------------------------------------------------------------
// Periodic performance stats helpers
// ---------------------------------------------------------------------------

/// Determine which trailing-window periods are applicable given the data range.
///
/// Always includes `Full`. For each sub-period, checks whether the data range
/// covers at least that many months.
pub fn applicable_periods(start_date: &str, end_date: &str) -> Vec<PerformancePeriod> {
    let mut periods = vec![PerformancePeriod::Full];

    let start = match parse_naive_date(start_date) {
        Some(d) => d,
        None => return periods,
    };
    let end = match parse_naive_date(end_date) {
        Some(d) => d,
        None => return periods,
    };

    for &p in PerformancePeriod::sub_periods() {
        if let Some(months) = p.months() {
            // Compute the target start date by subtracting `months` from end
            if let Some(target_start) = end.checked_sub_months(Months::new(months)) {
                if target_start >= start {
                    periods.push(p);
                }
            }
        }
    }

    periods
}

/// Find the row index in `dates` that is the first row on or after `target`.
/// Returns `None` if no such row exists.
pub(crate) fn find_start_index(dates: &[String], target: NaiveDate) -> Option<usize> {
    for (i, d) in dates.iter().enumerate() {
        if let Some(nd) = parse_naive_date(d) {
            if nd >= target {
                return Some(i);
            }
        }
    }
    None
}

/// Compute `PerformanceStats` for each applicable trailing period.
///
/// `returns` and `benchmark_returns` must have the same length as `dates`.
/// Returns a vec of `(PerformancePeriod, PerformanceStats)` in canonical order.
pub fn compute_periodic_stats(
    dates: &[String],
    returns: &Series,
    benchmark_returns: Option<&Series>,
    risk_free_rate: f64,
    confidence_level: f64,
    interval: IntervalDays,
) -> Result<Vec<(PerformancePeriod, PerformanceStats)>, Box<dyn Error>> {
    if dates.is_empty() {
        return Ok(Vec::new());
    }

    let start_date = &dates[0];
    let end_date = &dates[dates.len() - 1];
    let end_nd = parse_naive_date(end_date).ok_or("Cannot parse end date")?;
    let periods = applicable_periods(start_date, end_date);

    let mut result = Vec::with_capacity(periods.len());

    for period in &periods {
        let start_idx = match period {
            PerformancePeriod::Full => 0,
            _ => {
                let months = period.months().unwrap();
                let target_start = end_nd
                    .checked_sub_months(Months::new(months))
                    .ok_or("Date arithmetic overflow")?;
                find_start_index(dates, target_start).unwrap_or(0)
            }
        };

        let slice_len = dates.len() - start_idx;
        if slice_len < 2 {
            continue;
        }

        let ret_slice = returns.slice(start_idx as i64, slice_len);
        let bench_slice = benchmark_returns.map(|b| b.slice(start_idx as i64, slice_len));

        let stats = PerformanceStats::compute_stats(
            ret_slice,
            bench_slice,
            risk_free_rate,
            confidence_level,
            interval,
        )?;

        result.push((*period, stats));
    }

    Ok(result)
}

/// Compute periodic stats for a DataFrame of per-asset returns.
///
/// Returns a `HashMap<PerformancePeriod, Vec<PerformanceStats>>` where each
/// `Vec` has one `PerformanceStats` per column in `returns_df` (same order).
pub fn compute_periodic_stats_per_asset(
    dates: &[String],
    returns_df: &DataFrame,
    benchmark_returns: Option<&Series>,
    risk_free_rate: f64,
    confidence_level: f64,
    interval: IntervalDays,
) -> Result<HashMap<PerformancePeriod, Vec<PerformanceStats>>, Box<dyn Error>> {
    if dates.is_empty() {
        return Ok(HashMap::new());
    }

    let start_date = &dates[0];
    let end_date = &dates[dates.len() - 1];
    let end_nd = parse_naive_date(end_date).ok_or("Cannot parse end date")?;
    let periods = applicable_periods(start_date, end_date);

    let col_names: Vec<String> = returns_df
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut result: HashMap<PerformancePeriod, Vec<PerformanceStats>> = HashMap::new();

    for period in &periods {
        let start_idx = match period {
            PerformancePeriod::Full => 0,
            _ => {
                let months = period.months().unwrap();
                let target_start = end_nd
                    .checked_sub_months(Months::new(months))
                    .ok_or("Date arithmetic overflow")?;
                find_start_index(dates, target_start).unwrap_or(0)
            }
        };

        let slice_len = dates.len() - start_idx;
        if slice_len < 2 {
            continue;
        }

        let bench_slice = benchmark_returns.map(|b| b.slice(start_idx as i64, slice_len));
        let mut asset_stats = Vec::with_capacity(col_names.len());

        for name in &col_names {
            let col_series = returns_df.column(name)?.as_series().unwrap();
            let ret_slice = col_series.slice(start_idx as i64, slice_len);
            let stats = PerformanceStats::compute_stats(
                ret_slice,
                bench_slice.clone(),
                risk_free_rate,
                confidence_level,
                interval,
            )?;
            asset_stats.push(stats);
        }

        result.insert(*period, asset_stats);
    }

    Ok(result)
}

// ---------------------------------------------------------------------------
// PerformanceStats
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct PerformanceStats {
    pub daily_return: f64,
    pub daily_volatility: f64,
    pub cumulative_return: f64,
    pub annualized_return: f64,
    pub annualized_volatility: f64,
    pub alpha: Option<f64>,
    pub beta: Option<f64>,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub active_return: Option<f64>,
    pub active_risk: Option<f64>,
    pub information_ratio: Option<f64>,
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
        benchmark_returns: Option<Series>,
        risk_free_rate: f64,
        confidence_level: f64,
        interval: IntervalDays,
    ) -> Result<PerformanceStats, Box<dyn Error>> {
        let _len = returns.len();
        let days = interval.mode;
        let annual_days = 365.0 / interval.average;
        let per_period_rfr = (1.0 + risk_free_rate).powf(1.0 / annual_days) - 1.0;
        let cumulative_return = cumulative_return(&returns)?;
        let daily_return = returns.mean().ok_or("Error calculating mean return")? / days;
        let daily_volatility = std_dev(&returns)?;
        let annualized_return = (1.0 + daily_return).powf(annual_days) - 1.0;
        let annualized_volatility = daily_volatility * annual_days.sqrt();
        let sharpe_ratio = (annualized_return - risk_free_rate) / annualized_volatility;
        let sortino_ratio = (annualized_return - risk_free_rate)
            / (downside_deviation(&returns, per_period_rfr)? * annual_days.sqrt());
        let (_, maximum_drawdown) = maximum_drawdown(&returns)?;
        let calmar_ratio = annualized_return / maximum_drawdown;
        let value_at_risk = value_at_risk(&returns, confidence_level)?;
        let expected_shortfall = expected_shortfall(&returns, confidence_level)?;

        // Benchmark-dependent metrics
        let (alpha, beta, active_return, active_risk, information_ratio) =
            if let Some(ref bench) = benchmark_returns {
                let (a, b) = ols_regression(bench, &returns)?;
                let annualized_alpha = a * annual_days;
                let excess_returns = (returns.clone() - bench.clone())?;
                let daily_active_return = excess_returns
                    .mean()
                    .ok_or("Error calculating active return")?;
                let ar = daily_active_return * annual_days;
                let arisk = std_dev(&excess_returns)? * annual_days.sqrt();
                let ir = ar / arisk;
                (
                    Some(annualized_alpha),
                    Some(b),
                    Some(ar),
                    Some(arisk),
                    Some(ir),
                )
            } else {
                (None, None, None, None, None)
            };

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

// ---------------------------------------------------------------------------
// Basic statistical helpers
// ---------------------------------------------------------------------------

/// computes the standard deviation of a series of security returns
///
/// # Arguments
///
/// * `series` - Polars Series of security returns
///
/// # Returns
///
/// * `f64` - Standard deviation
pub fn std_dev(series: &Series) -> Result<f64, FinalyticsError> {
    let dev_vec = series_to_vec_f64(series, "std_dev_input")?;
    Ok(dev_vec.std_dev())
}

/// Computes the downside deviation of a series of security returns relative
/// to a minimum acceptable return (MAR / target).
///
/// # Arguments
///
/// * `series` - Polars Series of security returns
/// * `target` - Minimum acceptable return per period (e.g. per-period risk-free rate)
///
/// # Returns
///
/// * `f64` - Downside deviation
pub fn downside_deviation(series: &Series, target: f64) -> Result<f64, FinalyticsError> {
    let returns = series_to_vec_f64(series, "downside_deviation_input")?;
    let n = returns.len() as f64;
    if n == 0.0 {
        return Ok(0.0);
    }
    let sum_squared = returns
        .iter()
        .map(|&r| {
            let diff = r - target;
            if diff < 0.0 {
                diff.powi(2)
            } else {
                0.0
            }
        })
        .sum::<f64>();
    Ok((sum_squared / n).sqrt())
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
pub fn ols_regression(x_series: &Series, y_series: &Series) -> Result<(f64, f64), FinalyticsError> {
    let x_data = series_to_vec_f64(x_series, "ols_x")?;
    let y_data = series_to_vec_f64(y_series, "ols_y")?;

    if x_data.len() < 2 {
        return Err(FinalyticsError::InsufficientData {
            required: 2,
            actual: x_data.len(),
            context: "OLS regression requires at least 2 data points".into(),
        });
    }

    // Create a matrix from x_data
    let x_matrix = DenseMatrix::from_column(&x_data);

    // Create a Linear Regression model
    let model = LinearRegression::fit(&x_matrix, &y_data, Default::default())
        .map_err(|e| FinalyticsError::External(format!("OLS regression failed: {e}").into()))?;

    // Get the intercept and slope
    let intercept = *model.intercept();
    let slope = *model.coefficients().get((0, 0));

    Ok((intercept, slope))
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

// ---------------------------------------------------------------------------
// Covariance shrinkage
// ---------------------------------------------------------------------------

/// Controls whether and how the sample covariance matrix is shrunk before use
/// in portfolio optimisation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShrinkageMethod {
    /// No shrinkage — use the raw sample covariance matrix.
    None,
    /// Ledoit-Wolf (2004) shrinkage toward the constant-correlation target.
    /// The optimal shrinkage intensity δ is computed analytically from the data.
    LedoitWolf,
}

/// Result of a covariance shrinkage operation.
#[derive(Debug, Clone)]
pub struct ShrunkCovariance {
    /// The (possibly shrunk) covariance matrix.
    pub matrix: ndarray::Array2<f64>,
    /// The shrinkage intensity that was applied (0.0 when `ShrinkageMethod::None`).
    pub shrinkage_intensity: f64,
}

/// Computes the Ledoit-Wolf shrunk covariance matrix from a returns DataFrame.
///
/// The shrinkage target **F** is the *constant-correlation* matrix: it keeps
/// each asset's sample variance but replaces every pairwise correlation with
/// the average sample correlation.
///
/// The optimal shrinkage intensity **δ** is computed analytically following
/// Ledoit & Wolf, "A well-conditioned estimator for large-dimensional
/// covariance matrices", *Journal of Multivariate Analysis* 88 (2004).
///
/// # Arguments
///
/// * `df` – Polars DataFrame whose columns are asset return series (decimal).
/// * `method` – Which shrinkage method to apply (`None` returns the sample
///   covariance unchanged).
///
/// # Returns
///
/// A `ShrunkCovariance` containing the matrix and the intensity δ.
pub fn shrink_covariance(
    df: &DataFrame,
    method: ShrinkageMethod,
) -> Result<ShrunkCovariance, FinalyticsError> {
    let sample = covariance_matrix(df)?;

    match method {
        ShrinkageMethod::None => Ok(ShrunkCovariance {
            matrix: sample,
            shrinkage_intensity: 0.0,
        }),
        ShrinkageMethod::LedoitWolf => shrink_covariance_ledoit_wolf(df, sample),
    }
}

/// Internal implementation of the Ledoit-Wolf constant-correlation shrinkage.
fn shrink_covariance_ledoit_wolf(
    df: &DataFrame,
    sample: ndarray::Array2<f64>,
) -> Result<ShrunkCovariance, FinalyticsError> {
    let p = sample.nrows(); // number of assets
    let n = df.height(); // number of observations

    if p < 2 || n < 2 {
        // Not enough data to shrink — return the sample matrix as-is.
        return Ok(ShrunkCovariance {
            matrix: sample,
            shrinkage_intensity: 0.0,
        });
    }

    // --- Step 1: Extract the data matrix X (n × p), de-meaned ----------------
    let mut x = ndarray::Array2::<f64>::zeros((n, p));
    let mut means = vec![0.0_f64; p];

    for j in 0..p {
        // SAFETY: j is in 0..p which equals df.width()
        let col_ref = df.select_at_idx(j).unwrap();
        let col = series_to_vec_f64(col_ref.as_series().unwrap(), col_ref.name().as_str())?;
        let mean = col.iter().sum::<f64>() / n as f64;
        means[j] = mean;
        for (i, &val) in col.iter().enumerate() {
            x[(i, j)] = val - mean;
        }
    }

    // --- Step 2: Build the constant-correlation target F ---------------------
    // F_{ii} = s_{ii}  (same diagonal as sample)
    // F_{ij} = rho_bar * sqrt(s_{ii} * s_{jj})   for i ≠ j
    //
    // where rho_bar is the average of all sample correlations r_{ij}, i < j.

    let std_devs: Vec<f64> = (0..p).map(|i| sample[(i, i)].sqrt()).collect();

    let mut sum_corr = 0.0_f64;
    let mut count_corr = 0_usize;
    for i in 0..p {
        for j in (i + 1)..p {
            let denom = std_devs[i] * std_devs[j];
            if denom > 1e-14 {
                sum_corr += sample[(i, j)] / denom;
                count_corr += 1;
            }
        }
    }
    let rho_bar = if count_corr > 0 {
        sum_corr / count_corr as f64
    } else {
        0.0
    };

    let mut target = ndarray::Array2::<f64>::zeros((p, p));
    for i in 0..p {
        target[(i, i)] = sample[(i, i)];
        for j in (i + 1)..p {
            let f_ij = rho_bar * std_devs[i] * std_devs[j];
            target[(i, j)] = f_ij;
            target[(j, i)] = f_ij;
        }
    }

    // --- Step 3: Compute optimal shrinkage intensity δ -----------------------
    //
    // Following the Ledoit-Wolf (2004) analytical formula:
    //
    //   δ* = min(1, max(0, κ / T))
    //
    // where T = n and κ = (π_hat - γ_hat) / ρ_hat  (see paper for definitions).
    //
    // π_hat  = Σ_{i,j} Asy.Var(√n · s_{ij})    (sum of asymptotic variances
    //          of the sample covariance entries, estimated from the data)
    // γ_hat  = Σ_{i,j} (f_{ij} - s_{ij})²       (squared Frobenius distance
    //          between target and sample)
    //
    // The simplified estimator uses:
    //   π_hat = (1/n²) Σ_t Σ_{i,j} [ x_{ti}² x_{tj}² ] - Σ_{i,j} s_{ij}²

    // Compute π_hat: sum over all (i,j) of the sample variance of
    //   z_{t,ij} = x_{ti} · x_{tj}
    // Var(z_{ij}) ≈ (1/n) Σ_t z_{t,ij}² - s_{ij}²

    let nf = n as f64;
    let mut pi_hat = 0.0_f64;
    for i in 0..p {
        for j in 0..p {
            let s_ij = sample[(i, j)];
            let mut sum_sq = 0.0_f64;
            for t in 0..n {
                let z = x[(t, i)] * x[(t, j)];
                sum_sq += z * z;
            }
            // Asy. variance of s_{ij} ≈ (1/n) Σ z² - s²
            pi_hat += sum_sq / nf - s_ij * s_ij;
        }
    }

    // γ_hat: squared Frobenius norm ‖F - S‖²
    let mut gamma_hat = 0.0_f64;
    for i in 0..p {
        for j in 0..p {
            let diff = target[(i, j)] - sample[(i, j)];
            gamma_hat += diff * diff;
        }
    }

    // Compute δ — clamped to [0, 1]
    let delta = if gamma_hat.abs() < 1e-14 {
        // Target ≈ sample → no shrinkage needed
        0.0
    } else {
        let kappa = (pi_hat / nf) / gamma_hat;
        kappa.clamp(0.0, 1.0)
    };

    // --- Step 4: Blend -------------------------------------------------------
    let shrunk = &sample * (1.0 - delta) + &target * delta;

    Ok(ShrunkCovariance {
        matrix: shrunk,
        shrinkage_intensity: delta,
    })
}

pub fn covariance_matrix(df: &DataFrame) -> Result<ndarray::Array2<f64>, FinalyticsError> {
    let num_columns = df.width();
    // Pre-extract all columns once to avoid repeated unwrap/iteration
    let columns: Vec<Vec<f64>> = (0..num_columns)
        .map(|i| {
            // SAFETY: i is in 0..df.width(), so select_at_idx is always valid
            let col = df.select_at_idx(i).unwrap();
            series_to_vec_f64(col.as_series().unwrap(), col.name().as_str())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut covariance_matrix = ndarray::Array2::zeros((num_columns, num_columns));
    for i in 0..num_columns {
        for j in 0..num_columns {
            let cov = columns[i].clone().covariance(columns[j].clone());
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
pub fn correlation_matrix(df: &DataFrame) -> Result<ndarray::Array2<f64>, FinalyticsError> {
    let covariance_matrix = covariance_matrix(df)?;
    let num_columns = covariance_matrix.nrows();

    let mut correlation_matrix = ndarray::Array2::zeros((num_columns, num_columns));

    // Calculate standard deviations of each column
    let std_devs: Vec<f64> = (0..num_columns)
        .map(|i| {
            // SAFETY: i is in 0..num_columns which equals df.width()
            let col = df.select_at_idx(i).unwrap();
            let vec = series_to_vec_f64(col.as_series().unwrap(), col.name().as_str())?;
            Ok(vec.std_dev())
        })
        .collect::<Result<Vec<f64>, FinalyticsError>>()?;

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
pub fn maximum_drawdown(returns: &Series) -> Result<(Vec<f64>, f64), FinalyticsError> {
    let returns = series_to_vec_f64(returns, "maximum_drawdown_input")?;

    // Step 1: Calculate cumulative returns (geometric compounding)
    let mut cumulative_returns = Vec::with_capacity(returns.len());
    let mut cumulative_return = 1.0;
    for &return_value in &returns {
        cumulative_return *= 1.0 + return_value;
        cumulative_returns.push(cumulative_return);
    }

    // Step 2: Calculate cumulative maximum of cumulative returns
    let mut current_max = cumulative_returns[0];
    let mut rolling_drawdowns = Vec::with_capacity(returns.len());
    let mut max_drawdown = 0.0;

    for &cum_return in &cumulative_returns {
        if cum_return > current_max {
            current_max = cum_return;
        }
        let drawdown = (current_max - cum_return) / current_max;
        rolling_drawdowns.push(-drawdown);
        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
    }

    Ok((rolling_drawdowns, max_drawdown))
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
pub fn value_at_risk(returns: &Series, confidence_level: f64) -> Result<f64, FinalyticsError> {
    let returns = series_to_vec_f64(returns, "value_at_risk_input")?;
    let n = returns.len();
    if n == 0 {
        return Ok(0.0);
    }
    let mut sorted_returns = returns.clone();
    sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let pos = (1.0 - confidence_level) * (n as f64 - 1.0);
    let lower = pos.floor() as usize;
    let upper = (lower + 1).min(n - 1);
    let frac = pos - lower as f64;
    Ok(sorted_returns[lower] + frac * (sorted_returns[upper] - sorted_returns[lower]))
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
pub fn expected_shortfall(returns: &Series, confidence_level: f64) -> Result<f64, FinalyticsError> {
    let var = value_at_risk(returns, confidence_level)?;
    let returns = series_to_vec_f64(returns, "expected_shortfall_input")?;
    let loss_returns = returns
        .iter()
        .filter(|&x| *x <= var)
        .cloned()
        .collect::<Vec<f64>>();
    if loss_returns.is_empty() {
        Ok(var)
    } else {
        Ok(loss_returns.iter().sum::<f64>() / (loss_returns.len() as f64))
    }
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
    weights
        .iter()
        .zip(mean_returns.iter())
        .map(|(w, r)| w * r)
        .sum()
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
pub fn portfolio_downside_dev(
    weights: &[f64],
    portfolio_returns: &DataFrame,
) -> Result<f64, FinalyticsError> {
    let num_assets = weights.len();
    let no_cash_flows = vec![vec![0.0; num_assets]; portfolio_returns.height()];
    let empty_dates: Vec<String> = Vec::new();
    let empty_rebal_flows: Vec<f64> = Vec::new();
    let result = daily_portfolio_returns(
        weights,
        portfolio_returns,
        &no_cash_flows,
        None,
        &empty_dates,
        &empty_rebal_flows,
    )?;
    let port_returns = result.portfolio_returns;
    let returns_vec = series_to_vec_f64(&port_returns, "portfolio_downside_dev")?;

    let n = returns_vec.len() as f64;
    if n == 0.0 {
        return Ok(0.0);
    }

    let sum_squared: f64 = returns_vec
        .iter()
        .map(|&r| if r < 0.0 { r.powi(2) } else { 0.0 })
        .sum();

    Ok((sum_squared / n).sqrt())
}

/// Result of computing daily portfolio returns, including value tracking data.
#[derive(Debug, Clone)]
pub struct PortfolioReturnsResult {
    /// Percentage returns per period.
    pub portfolio_returns: Series,
    /// Total portfolio value at the end of each period.
    pub portfolio_values: Vec<f64>,
    /// Per-asset values at the end of each period: `[num_rows][num_assets]`.
    pub asset_values: Vec<Vec<f64>>,
    /// Per-asset values at the very end of the period.
    pub ending_values: Vec<f64>,
    /// Rebalance events that occurred during the simulation (legacy).
    pub rebalance_events: Vec<RebalanceEvent>,
    /// Enriched transaction events (rebalances + cash flows) with per-asset
    /// detail, cumulative TWR, and cumulative MWR.
    pub transaction_events: Vec<TransactionEvent>,
}

/// Computes the daily/time_interval returns of a buy-and-hold portfolio where weights drift
/// naturally based on cumulative asset performance (no rebalancing).
/// Per-asset cash flows (additions/withdrawals) are applied at the end of each period.
///
/// # Arguments
///
/// * `starting_values` - Vector of initial per-asset values (fractional weights or dollar amounts).
///   Percentage returns are scale-invariant; dollar tracking uses these values directly.
/// * `returns` - Polars DataFrame of security returns
/// * `asset_cash_flows` - Slice of per-asset cash flow vectors (same length as returns).
///   Each inner vector has one entry per asset: positive for additions, negative for withdrawals,
///   0.0 for no cash flow.
///
/// # Returns
///
/// * `PortfolioReturnsResult` - Portfolio returns and value tracking data
pub fn daily_portfolio_returns(
    starting_values: &[f64],
    returns: &DataFrame,
    asset_cash_flows: &[Vec<f64>],
    rebalance_config: Option<&RebalanceConfig>,
    dates: &[String],
    rebalance_cash_flows: &[f64],
) -> Result<PortfolioReturnsResult, FinalyticsError> {
    let n = returns.height();
    let num_assets = starting_values.len();
    let mut current_values: Vec<f64> = starting_values.to_vec();
    let mut portfolio_returns: Vec<f64> = Vec::with_capacity(n);
    let mut portfolio_values = Vec::with_capacity(n);
    let mut asset_values = Vec::with_capacity(n);
    let mut rebalance_events: Vec<RebalanceEvent> = Vec::new();
    let mut transaction_events: Vec<TransactionEvent> = Vec::new();
    let mut last_rebalance_row: usize = 0;

    // Track cumulative external cash flows for XIRR computation.
    // XIRR convention: negative = money invested, positive = money returned.
    let initial_investment: f64 = starting_values.iter().sum();
    let start_date = if !dates.is_empty() {
        parse_naive_date(&dates[0])
    } else {
        None
    };
    let mut xirr_cash_flows: Vec<DatedCashFlow> = Vec::new();
    if let Some(d) = start_date {
        xirr_cash_flows.push(DatedCashFlow {
            date: d,
            amount: -initial_investment, // investor puts money in
        });
    }

    for row in 0..n {
        let portfolio_value: f64 = current_values.iter().sum();
        let mut new_values = Vec::with_capacity(num_assets);
        let mut daily_return = 0.0;

        for (i, value) in current_values.iter().enumerate() {
            let col_str = returns.get_column_names()[i];
            let asset_return = returns
                .column(col_str)
                .map_err(|_| FinalyticsError::ColumnNotFound {
                    name: col_str.to_string(),
                })?
                .f64()
                .map_err(|_| FinalyticsError::DtypeMismatch {
                    column: col_str.to_string(),
                    expected: "Float64".into(),
                    actual: format!("{:?}", returns.column(col_str).unwrap().dtype()),
                })?
                .get(row)
                .ok_or_else(|| FinalyticsError::NullValues {
                    column: col_str.to_string(),
                    null_count: 1,
                })?;
            let current_weight = value / portfolio_value;
            daily_return += current_weight * asset_return;
            new_values.push(value * (1.0 + asset_return));
        }

        portfolio_returns.push(daily_return);

        // Snapshot values after returns but before any cash flows / rebalancing
        let values_after_returns = new_values.clone();

        // ---- Apply per-asset cash flows (ProRata / Custom / explicit) ----
        let row_flows = &asset_cash_flows[row];
        for (i, value) in new_values.iter_mut().enumerate() {
            *value += row_flows[i];
        }

        // ---- Apply Rebalance-allocated cash flows ----
        let rebal_cf = if row < rebalance_cash_flows.len() {
            rebalance_cash_flows[row]
        } else {
            0.0
        };
        if let Some(cfg) = rebalance_config {
            if rebal_cf.abs() > 1e-12 {
                distribute_rebalance_cash_flow(&mut new_values, rebal_cf, &cfg.target_weights);
            }
        } else if rebal_cf.abs() > 1e-12 {
            // No rebalance config but there are rebalance cash flows —
            // fall back to equal distribution
            let per_asset = rebal_cf / num_assets as f64;
            for val in new_values.iter_mut() {
                *val += per_asset;
            }
        }

        // Total external cash flow on this row
        let total_external_cf: f64 = row_flows.iter().sum::<f64>() + rebal_cf;
        let had_cash_flow = total_external_cf.abs() > 1e-12;

        // Record external cash flow for XIRR (negative = addition from investor)
        if had_cash_flow {
            if let Some(d) = dates.get(row).and_then(|s| parse_naive_date(s)) {
                xirr_cash_flows.push(DatedCashFlow {
                    date: d,
                    amount: -total_external_cf, // addition → investor outflow (neg)
                });
            }
        }

        // ---- Check for rebalancing trigger ----
        let mut row_turnover = 0.0;
        let mut had_rebalance = false;
        if let Some(cfg) = rebalance_config {
            if should_rebalance(row, dates, &new_values, cfg, last_rebalance_row) {
                row_turnover = apply_rebalance(&mut new_values, &cfg.target_weights);
                let date_str = if row < dates.len() {
                    dates[row].clone()
                } else {
                    format!("row-{row}")
                };
                rebalance_events.push(RebalanceEvent {
                    row,
                    date: date_str,
                    turnover: row_turnover,
                });
                last_rebalance_row = row;
                had_rebalance = true;
            }
        }

        // ---- Emit enriched TransactionEvent if anything happened ----
        if had_rebalance || had_cash_flow {
            let date_str = if row < dates.len() {
                dates[row].clone()
            } else {
                format!("row-{row}")
            };

            let event_type = match (had_rebalance, had_cash_flow) {
                (true, true) => TransactionEventType::RebalanceAndCashFlow,
                (true, false) => TransactionEventType::Rebalance,
                (false, true) => TransactionEventType::CashFlow,
                (false, false) => unreachable!(),
            };

            let pv_before: f64 = values_after_returns.iter().sum();
            let pv_after: f64 = new_values.iter().sum();

            let trade_amounts: Vec<f64> = new_values
                .iter()
                .zip(values_after_returns.iter())
                .map(|(a, b)| a - b)
                .collect();

            let cum_twr = cumulative_twr_at_row(&portfolio_returns, row);

            // Compute cumulative MWR (XIRR) from inception to this row
            let cum_mwr = if start_date.is_some() {
                if let Some(d) = dates.get(row).and_then(|s| parse_naive_date(s)) {
                    let pv_now: f64 = new_values.iter().sum();
                    // Build temporary cash-flow list: all prior flows + terminal value now
                    let mut cf_for_xirr = xirr_cash_flows.clone();
                    cf_for_xirr.push(DatedCashFlow {
                        date: d,
                        amount: pv_now, // investor gets this back
                    });
                    // Only compute if time span > 0
                    if cf_for_xirr.len() >= 2
                        && cf_for_xirr.last().unwrap().date > cf_for_xirr[0].date
                    {
                        xirr(&cf_for_xirr)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            transaction_events.push(TransactionEvent {
                row,
                date: date_str,
                event_type,
                portfolio_value_before: pv_before,
                portfolio_value_after: pv_after,
                asset_values_before: values_after_returns,
                asset_values_after: new_values.clone(),
                trade_amounts,
                turnover: row_turnover,
                cash_flow_amount: total_external_cf,
                cumulative_twr: cum_twr,
                cumulative_mwr: cum_mwr,
            });
        }

        let total_value: f64 = new_values.iter().sum();
        portfolio_values.push(total_value);
        asset_values.push(new_values.clone());

        current_values = new_values;
    }

    let ending_values = current_values;

    Ok(PortfolioReturnsResult {
        portfolio_returns: Series::new("Portfolio Returns".into(), portfolio_returns),
        portfolio_values,
        asset_values,
        ending_values,
        rebalance_events,
        transaction_events,
    })
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
pub fn cumulative_return(returns: &Series) -> Result<f64, FinalyticsError> {
    let returns_vec = series_to_vec_f64(returns, "cumulative_return_input")?;
    let product: f64 = returns_vec.iter().map(|&r| 1.0 + r).product();
    Ok(product - 1.0)
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
    let mut cumulative_returns = Vec::with_capacity(returns.len());
    let mut cumulative_return = 1.0;

    for return_value in returns {
        cumulative_return *= 1.0 + return_value;
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
                let interpolation_ratio =
                    (i - left_index) as f64 / (right_index - left_index) as f64;
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
