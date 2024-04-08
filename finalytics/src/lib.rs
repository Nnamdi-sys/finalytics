pub mod models;
pub mod analytics;
pub mod charts;
pub mod utils;
pub mod data;


pub mod prelude {

    // Structs
    pub use crate::models::ticker::Ticker;
    pub use crate::models::portfolio::Portfolio;

    // Enums
    pub use crate::data::ticker::Interval;
    pub use crate::analytics::optimization::ObjectiveFunction;
    pub use crate::data::keys::{AssetClass, Category, Exchange};


    // Builders
    pub use crate::models::ticker::TickerBuilder;
    pub use crate::models::portfolio::PortfolioBuilder;


    // Traits
    pub use crate::data::ticker::TickerData;
    pub use crate::charts::ticker::TickerCharts;
    pub use crate::charts::portfolio::PortfolioCharts;
    pub use crate::analytics::fundamentals::Financials;
    pub use crate::analytics::sentiment::NewsSentiment;
    pub use crate::analytics::performance::TickerPerformance;
    pub use crate::analytics::stochastics::VolatilitySurface;
    pub use crate::analytics::technicals::TechnicalIndicators;

    #[cfg(feature = "kaleido")]
    pub use crate::utils::chart_utils::PlotImage;

    // Functions
    pub use crate::data::db::{get_symbols, search_symbols, get_symbols_count};

}


#[cfg(test)]

mod tests {
    use crate::prelude::*;
    #[tokio::test]
    async fn check_symbols_count() {
        // Database-related tests
        let res1 = get_symbols(AssetClass::All, Category::All, Exchange::All).unwrap();
        assert!(res1.len() >= 200000);

        let res2 = get_symbols_count().unwrap() as usize;
        assert_eq!(res1.len(), res2);
    }
}
