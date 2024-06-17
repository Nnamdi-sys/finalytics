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
    pub use crate::data::config::Interval;
    pub use crate::analytics::optimization::ObjectiveFunction;


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

}


#[cfg(test)]

mod tests {
    use crate::prelude::*;
    #[tokio::test]
    async fn get_quote() {
        let ticker = TickerBuilder::new().ticker("AAPL")
            .start_date("2023-01-01")
            .end_date("2023-02-01")
            .interval(Interval::OneDay)
            .benchmark_symbol("^GSPC")
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .build();

        let quote = ticker.get_quote().await.unwrap();
        dbg!(quote);
    }
}
