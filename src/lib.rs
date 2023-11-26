pub mod data;
pub mod analytics;
pub mod charts;
pub mod utils;
pub mod database;


pub use crate::data::ticker::Ticker;
pub use crate::analytics::technicals::TechnicalIndicators;
pub use crate::analytics::fundamentals::Financials;
pub use crate::analytics::stochastics::BlackScholesModel;
pub use crate::analytics::performance::TickerPerformanceStats;
pub use crate::analytics::performance::PortfolioPerformanceStats;
pub use crate::charts::ticker::TickerCharts;
pub use crate::charts::portfolio::PortfolioCharts;
pub use crate::database::db::get_symbols;
pub use crate::analytics::sentiment::scrape_news;
pub use crate::analytics::stochastics::implied_volatility_bisection;

#[cfg(test)]

mod tests {
    use crate::data::keys::{AssetClass, Category, Exchange};
    use crate::database::db::get_symbols_count;
    use super::*;
    #[tokio::test]
    async fn check_symbols_count() {
        // Database-related tests
        let res1 = get_symbols(AssetClass::All, Category::All, Exchange::All).unwrap();
        assert!(res1.len() >= 200000);

        let res2 = get_symbols_count().unwrap() as usize;
        assert_eq!(res1.len(), res2);
    }
}
