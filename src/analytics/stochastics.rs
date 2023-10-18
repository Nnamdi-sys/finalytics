use statrs::distribution::{ContinuousCDF, Normal};
use statrs::distribution::Continuous;


#[derive(Debug, Copy, Clone)]
pub enum OptionType {
    Call,
    Put,
}

impl OptionType {
    pub fn to_string(&self) -> String {
        match self {
            OptionType::Call => "Call".to_string(),
            OptionType::Put => "Put".to_string(),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct BlackScholesModel {
    pub s: f64,
    pub k: f64,
    pub t: f64,
    pub r: f64,
    pub v: f64,
    pub option_type: OptionType,
    pub option_price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub rho: f64,
    pub vega: f64,
}

impl BlackScholesModel {
    /// Computes the Black-Scholes Model Option Price and Greeks for a given option
    ///
    /// # Arguments
    ///
    /// * `s` - Spot price
    /// * `k` - Strike price
    /// * `t` - Time to maturity (in years)
    /// * `r` - Risk-free interest rate in decimal (e.g 0.02 for 2%)
    /// * `v` - Implied volatility in decimal (e.g 0.30 for 30%)
    /// * `option_type` - Option type enum (e.g OptionType::Call)
    ///
    /// # Returns
    ///
    /// * `BlackScholesModel` struct
    ///
    /// # Example
    ///
    /// ```
    /// use finalytics::analytics::stochastics::{BlackScholesModel, OptionType};
    ///
    /// fn main() {
    ///     let result = BlackScholesModel::compute(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call);
    ///     println!("{:?}", result);
    /// }
    /// ```
    pub fn compute(
        s: f64,
        k: f64,
        t: f64,
        r: f64,
        v: f64,
        option_type: OptionType) -> Self {
        let d1 = (s.ln() - k.ln() + (r + (v * v) / 2.0) * t) / (v * t.sqrt());
        let d2 = d1 - v * t.sqrt();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let option_price = match option_type {
            OptionType::Call => {
                let normal = Normal::new(0.0, 1.0).unwrap();
                let cdf_d1 = normal.cdf(d1);
                let cdf_d2 = normal.cdf(d2);
                s * cdf_d1 - k * (-r * t).exp() * cdf_d2
            }
            OptionType::Put => {
                let normal = Normal::new(0.0, 1.0).unwrap();
                let cdf_minus_d1 = normal.cdf(-d1);
                let cdf_minus_d2 = normal.cdf(-d2);
                k * (-r * t).exp() * cdf_minus_d2 - s * cdf_minus_d1
            }
        };
        let delta = match option_type {
            OptionType::Call => {
                normal.cdf(d1)
            }
            OptionType::Put => {
                -normal.cdf(-d1)
            }
        };
        let gamma = normal.pdf(d1) / (s * v * t.sqrt());
        let theta = match option_type {
            OptionType::Call => {
                -((s * v * normal.pdf(d1)) / (2.0 * t.sqrt()))
                    - r * k * (-r * t).exp() * normal.cdf(d2)
            }
            OptionType::Put => {
                -((s * v * normal.pdf(d1)) / (2.0 * t.sqrt()))
                    + r * k * (-r * t).exp() * normal.cdf(-d2)
            }
        };
        let rho = match option_type {
            OptionType::Call => {
                k * t * (-r * t).exp() * normal.cdf(d2)
            }
            OptionType::Put => {
                -k * t * (-r * t).exp() * normal.cdf(-d2)
            }
        };
        let vega = match option_type {
            OptionType::Call => {
                s * t.sqrt() * normal.pdf(d1)
            }
            OptionType::Put => {
                s * t.sqrt() * normal.pdf(-d1)
            }
        };
        Self {
            s,
            k,
            t,
            r,
            v,
            option_type,
            option_price,
            delta,
            gamma,
            theta,
            rho,
            vega,
        }
    }
}

/// Computes the implied volatility for an option using the bisection method
///
/// # Arguments
///
/// * `option_price` - Option price
/// * `s` - Spot price
/// * `k` - Strike price
/// * `t` - Time to maturity (in years)
/// * `r` - Risk-free interest rate in decimal (e.g 0.02 for 2%)
/// * `option_type` - Option type enum (e.g OptionType::Call)
///
/// # Returns
///
/// * `f64` Implied volatility in decimal
///
/// # Example
///
/// ```
/// use finalytics::analytics::stochastics::{implied_volatility_bisection, OptionType};
///
/// fn main() {
///     let result = implied_volatility_bisection(10.0, 100.0, 100.0, 1.0, 0.05, OptionType::Call);
///     println!("{:?}", result);
/// }
/// ```
pub fn implied_volatility_bisection(
    option_price: f64,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    option_type: OptionType,
) -> f64 {
    let mut low = 0.01;  // Lower bound for volatility
    let mut high = 1.0;   // Upper bound for volatility

    let mut mid= 0.0;
    let mut price:f64;
    let max_iterations = 1000;
    let tolerance = 0.001;
    let mut i = 0;

    while i < max_iterations {
        mid = (low + high) / 2.0;
        price = BlackScholesModel::compute(s, k, t, r, mid, option_type).option_price;

        if price > option_price {
            high = mid;
        } else {
            low = mid;
        }

        if (price - option_price).abs() < tolerance {
            return mid;
        }

        i += 1;
    }

    mid
}
