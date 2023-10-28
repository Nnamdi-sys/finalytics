#[cfg(test)]
mod tests {
    use finalytics::analytics::fundamentals::Financials;
    use finalytics::analytics::optimization::ObjectiveFunction;
    use finalytics::analytics::performance::{PortfolioPerformanceStats, TickerPerformanceStats};
    use finalytics::charts::portfolio::PortfolioCharts;
    use finalytics::charts::ticker::TickerCharts;
    use finalytics::data::keys::{AssetClass, Category, Exchange};
    use finalytics::data::ticker::{Interval, Ticker};
    use finalytics::database::db::{get_symbols_count, get_symbols};
    #[cfg(feature = "kaleido")]
    use finalytics::utils::chart_utils::PlotImage;
    #[cfg(feature = "kaleido")]
    use finalytics::utils::chart_utils::ImgFormat;

    #[tokio::test]
    async fn test_ticker_functions() {
        // Ticker-related tests
        let ticker = Ticker::new("AAPL").await.unwrap();

        let quote = ticker.get_quote().await;
        assert!(quote.is_ok());

        let stats = ticker.get_ticker_stats().await;
        assert!(stats.is_ok());

        let options = ticker.get_options().await;
        assert!(options.is_ok());

        let chart = ticker.get_chart("2023-08-01",
                                     "2023-09-20", Interval::OneHour).await;
        assert!(chart.is_ok());

        let news = ticker.get_news("2023-01-01",
                                   "2023-01-07", false).await;
        assert!(news.is_ok());
    }


    #[tokio::test]
    async fn test_financials_functions() {
        // Financials-related tests
        let financials = Financials::new("AAPL").await.unwrap();

        let income_statement = financials.format_income_statement();
        assert!(income_statement.is_ok());

        let balance_sheet = financials.format_balance_sheet();
        assert!(balance_sheet.is_ok());

        let cash_flow = financials.format_cashflow_statement();
        dbg!(cash_flow.unwrap());
        //assert!(cash_flow.is_ok());

        let ratios = financials.compute_ratios();
        assert!(ratios.is_ok());
    }

    #[tokio::test]
    async fn test_performance_functions() {
        // Performance-related tests
        let ticker_perf = TickerPerformanceStats::new(
            "AAPL", "^GSPC", "2022-01-01", "2022-12-31",
            Interval::OneDay, 0.95, 0.02)
            .await.unwrap()
            .compute_stats();
        assert!(ticker_perf.is_ok());

        let portfolio_perf = PortfolioPerformanceStats::new(
            Vec::from(["AAPL".to_string(), "GOOG".to_string(), "NVDA".to_string(), "^TNX".to_string()]),
            "^GSPC", "2021-01-01", "2023-01-01", Interval::OneDay,
            0.95, 0.02, 1000,
            ObjectiveFunction::MaxSharpe)
            .await.unwrap()
            .compute_stats();
        assert!(portfolio_perf.is_ok());

        // Add similar assertions for other performance functions
    }

    #[tokio::test]
    async fn test_charts_functions() {
        // Ticker charts-related tests
        let ticker_charts = TickerCharts::new("AAPL", "2019-01-01", "2023-01-01",
                                              Interval::OneDay, "^GSPC", 0.95,
                                              0.02).unwrap();

        let candlestick_chart = ticker_charts.candlestick_chart().await;
        #[cfg(feature = "kaleido")]
        candlestick_chart.unwrap().save_image("candlestick.png", ImgFormat::PNG, 1000, 1000, 1.0);

        let performance_chart = ticker_charts.performance_chart().await;
        assert!(performance_chart.is_ok());

        let summary_stats = ticker_charts.summary_stats_table().await;
        assert!(summary_stats.is_ok());

        let performance_stats = ticker_charts.performance_stats_table().await;
        assert!(performance_stats.is_ok());

        let vol_charts = ticker_charts.options_volatility_charts().await;
        assert!(vol_charts.is_ok());

        let financial_statements = ticker_charts.financial_statements().await;
        assert!(financial_statements.is_ok());

        // Portfolio charts-related tests
        let portfolio_charts = PortfolioCharts::new(
            Vec::from(["NVDA".to_string(), "BRK-A".to_string(), "AAPL".to_string(), "^TNX".to_string()]),
            "^GSPC", "2017-01-01", "2023-01-01", Interval::OneDay,
            0.95, 0.02, 1000, ObjectiveFunction::MaxSharpe).await.unwrap();

        let optimization_chart = portfolio_charts.optimization_chart();
        #[cfg(feature = "kaleido")]
        optimization_chart.unwrap().save_image("optimization.png", ImgFormat::PNG, 1000, 1000, 1.0);

        let performance_chart = portfolio_charts.performance_chart();
        assert!(performance_chart.is_ok());

        let asset_returns_chart = portfolio_charts.asset_returns_chart();
        assert!(asset_returns_chart.is_ok());

        let performance_stats_table = portfolio_charts.performance_stats_table();
        assert!(performance_stats_table.is_ok());
    }
    #[tokio::test]
    async fn test_db_functions() {
        // Database-related tests
        let res1 = get_symbols(AssetClass::All, Category::All, Exchange::All).unwrap();
        assert!(res1.len() >= 200000);

        let res2 = get_symbols_count().unwrap() as usize;
        assert_eq!(res1.len(), res2);
    }
}
