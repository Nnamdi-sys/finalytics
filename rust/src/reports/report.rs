use crate::prelude::{
    Portfolio, PortfolioCharts, StatementFrequency, Ticker, TickerCharts, Tickers, TickersCharts,
};

use crate::reports::table::build_period_toggle;
use crate::reports::tabs::TabbedHtml;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum ReportType {
    Performance,
    Optimization,
    Financials,
    Options,
    News,
}

impl fmt::Display for ReportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ReportType::Performance => "performance",
            ReportType::Optimization => "optimization",
            ReportType::Financials => "financials",
            ReportType::Options => "options",
            ReportType::News => "news",
        };
        write!(f, "{s}")
    }
}

impl FromStr for ReportType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "performance" => Ok(ReportType::Performance),
            "optimization" => Ok(ReportType::Optimization),
            "financials" => Ok(ReportType::Financials),
            "options" => Ok(ReportType::Options),
            "news" => Ok(ReportType::News),
            _ => Err(format!("Invalid report type: {s}")),
        }
    }
}

pub trait Report {
    fn report(
        &self,
        report_type: Option<ReportType>,
    ) -> impl std::future::Future<Output = Result<TabbedHtml, Box<dyn Error>>>;
}

impl Report for Ticker {
    async fn report(&self, report_type: Option<ReportType>) -> Result<TabbedHtml, Box<dyn Error>> {
        let report_type = report_type.unwrap_or(ReportType::Performance);
        let report = match report_type {
            ReportType::Performance => {
                let mut tabs: Vec<(String, String)> = Vec::new();
                let price_table = self.ohlcv_table().await?.to_html()?;
                tabs.push(("Price Data".to_string(), price_table));
                let candlestick_chart = self
                    .candlestick_chart(None, None)
                    .await?
                    .to_html()
                    .replace("plotly-html-element", "candlestick_chart");
                tabs.push(("Candlestick Chart".to_string(), candlestick_chart));
                let performance_chart = self
                    .performance_chart(None, None)
                    .await?
                    .to_html()
                    .replace("plotly-html-element", "performance_chart");
                tabs.push(("Performance Chart".to_string(), performance_chart));
                let performance_stats = self.performance_stats_table().await?.to_html()?;
                tabs.push(("Performance Stats".to_string(), performance_stats));
                TabbedHtml::new(report_type, tabs)
            }
            ReportType::Financials => {
                let annual_financials = self
                    .financials_tables(StatementFrequency::Annual, None)
                    .await?;
                let quarterly_financials = self
                    .financials_tables(StatementFrequency::Quarterly, None)
                    .await?;

                let income_toggle = build_period_toggle(
                    &[
                        (
                            "Quarterly".to_string(),
                            quarterly_financials.income_statement.to_html()?,
                        ),
                        (
                            "Annual".to_string(),
                            annual_financials.income_statement.to_html()?,
                        ),
                    ],
                    "fin_income",
                );
                let balance_toggle = build_period_toggle(
                    &[
                        (
                            "Quarterly".to_string(),
                            quarterly_financials.balance_sheet.to_html()?,
                        ),
                        (
                            "Annual".to_string(),
                            annual_financials.balance_sheet.to_html()?,
                        ),
                    ],
                    "fin_balance",
                );
                let cashflow_toggle = build_period_toggle(
                    &[
                        (
                            "Quarterly".to_string(),
                            quarterly_financials.cashflow_statement.to_html()?,
                        ),
                        (
                            "Annual".to_string(),
                            annual_financials.cashflow_statement.to_html()?,
                        ),
                    ],
                    "fin_cashflow",
                );
                let ratios_toggle = build_period_toggle(
                    &[
                        (
                            "Quarterly".to_string(),
                            quarterly_financials.financial_ratios.to_html()?,
                        ),
                        (
                            "Annual".to_string(),
                            annual_financials.financial_ratios.to_html()?,
                        ),
                    ],
                    "fin_ratios",
                );

                let tabs: Vec<(String, String)> = vec![
                    ("Income Statement".to_string(), income_toggle),
                    ("Balance Sheet".to_string(), balance_toggle),
                    ("Cash Flow Statement".to_string(), cashflow_toggle),
                    ("Financial Ratios".to_string(), ratios_toggle),
                ];
                TabbedHtml::new(report_type, tabs)
            }
            ReportType::Options => {
                let options_charts = self.options_charts(None, None).await?;
                let options_table = self.options_tables().await?;
                let tabs: Vec<(String, String)> = vec![
                    (
                        "Options Chain".to_string(),
                        options_table.options_chain.to_html()?,
                    ),
                    (
                        "Volatility Surface Data".to_string(),
                        options_table.volatility_surface.to_html()?,
                    ),
                    (
                        "Volatility Smile".to_string(),
                        options_charts
                            .volatility_smile
                            .to_html()
                            .replace("plotly-html-element", "volatility_smile"),
                    ),
                    (
                        "Volatility Term Structure".to_string(),
                        options_charts
                            .volatility_term_structure
                            .to_html()
                            .replace("plotly-html-element", "volatility_term_structure"),
                    ),
                    (
                        "Volatility Surface Chart".to_string(),
                        options_charts
                            .volatility_surface
                            .to_html()
                            .replace("plotly-html-element", "volatility_surface"),
                    ),
                ];
                TabbedHtml::new(report_type, tabs)
            }
            ReportType::News => {
                let mut tabs: Vec<(String, String)> = Vec::new();
                let news_table = self.news_sentiment_table().await?.to_html()?;
                tabs.push(("News Sentiment Data".to_string(), news_table));
                let news_chart = self
                    .news_sentiment_chart(None, None)
                    .await?
                    .to_html()
                    .replace("plotly-html-element", "news_chart");
                tabs.push(("News Sentiment Chart".to_string(), news_chart));
                TabbedHtml::new(report_type, tabs)
            }
            _ => unimplemented!("Report type '{report_type}' is not supported for Ticker"),
        };
        Ok(report)
    }
}

impl Report for Portfolio {
    async fn report(&self, report_type: Option<ReportType>) -> Result<TabbedHtml, Box<dyn Error>> {
        let report_type = report_type.unwrap_or(ReportType::Performance);
        let report = match report_type {
            ReportType::Optimization => {
                let mut tabs: Vec<(String, String)> = Vec::new();

                // Optimization Chart
                let optimization_chart = self
                    .optimization_chart(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "optimization_chart");
                tabs.push(("Optimization Chart".to_string(), optimization_chart));

                // Portfolio Growth (stacked area: allocation + value over time)
                let portfolio_growth_chart = self
                    .portfolio_value_chart(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "portfolio_growth_chart");
                tabs.push(("Portfolio Growth".to_string(), portfolio_growth_chart));

                // Performance Chart
                let performance_chart = self
                    .performance_chart(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "performance_chart");
                tabs.push(("Performance Chart".to_string(), performance_chart));

                // Performance Stats (period toggle)
                let performance_stats = self.performance_stats_table()?.to_html()?;
                tabs.push(("Performance Stats".to_string(), performance_stats));

                // Returns Data (2D toggle: mode × frequency)
                let combined_returns_table = self.returns_table()?.to_html()?;
                tabs.push(("Returns Data".to_string(), combined_returns_table));

                // Returns Chart (%)
                let returns_chart = self
                    .returns_chart(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "returns_chart");
                tabs.push(("Returns Chart".to_string(), returns_chart));

                // Returns Matrix
                let returns_matrix = self
                    .returns_matrix(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "returns_matrix");
                tabs.push(("Returns Matrix".to_string(), returns_matrix));

                // Transaction History (conditional — only when events exist)
                if let Some(txn_table) = self.transaction_history_table()? {
                    let txn_html = txn_table.to_html()?;
                    tabs.push(("Transaction History".to_string(), txn_html));
                }

                TabbedHtml::new(report_type, tabs)
            }
            ReportType::Performance => {
                let mut tabs: Vec<(String, String)> = Vec::new();

                // Portfolio Growth (stacked area: allocation + value over time)
                let portfolio_growth_chart = self
                    .portfolio_value_chart(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "portfolio_growth_chart");
                tabs.push(("Portfolio Growth".to_string(), portfolio_growth_chart));

                // Performance Chart
                let performance_chart = self
                    .performance_chart(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "performance_chart");
                tabs.push(("Performance Chart".to_string(), performance_chart));

                // Performance Stats (period toggle)
                let performance_stats = self.performance_stats_table()?.to_html()?;
                tabs.push(("Performance Stats".to_string(), performance_stats));

                // Returns Data (2D toggle: mode × frequency)
                let combined_returns_table = self.returns_table()?.to_html()?;
                tabs.push(("Returns Data".to_string(), combined_returns_table));

                // Returns Chart (%)
                let returns_chart = self
                    .returns_chart(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "returns_chart");
                tabs.push(("Returns Chart".to_string(), returns_chart));

                // Returns Matrix
                let returns_matrix = self
                    .returns_matrix(None, None)?
                    .to_html()
                    .replace("plotly-html-element", "returns_matrix");
                tabs.push(("Returns Matrix".to_string(), returns_matrix));

                // Transaction History (conditional — only when events exist)
                if let Some(txn_table) = self.transaction_history_table()? {
                    let txn_html = txn_table.to_html()?;
                    tabs.push(("Transaction History".to_string(), txn_html));
                }

                TabbedHtml::new(report_type, tabs)
            }
            _ => unimplemented!("Report type '{report_type}' is not supported for Portfolio"),
        };
        Ok(report)
    }
}

impl Report for Tickers {
    async fn report(&self, report_type: Option<ReportType>) -> Result<TabbedHtml, Box<dyn Error>> {
        let report_type = report_type.unwrap_or(ReportType::Performance);
        let report = match report_type {
            ReportType::Performance => {
                let mut tabs: Vec<(String, String)> = Vec::new();
                let price_table = self.ohlcv_table().await?.to_html()?;
                tabs.push(("Price Data".to_string(), price_table));

                // Returns Data (frequency toggle — built inside returns_table())
                let returns_data = self.returns_table().await?.to_html()?;
                tabs.push(("Returns Data".to_string(), returns_data));

                // Performance Stats (period toggle — built inside performance_stats_table())
                let performance_stats = self.performance_stats_table().await?.to_html()?;
                tabs.push(("Performance Stats".to_string(), performance_stats));

                let returns_chart = self
                    .returns_chart(None, None)
                    .await?
                    .to_html()
                    .replace("plotly-html-element", "returns_chart");
                tabs.push(("Returns Chart".to_string(), returns_chart));
                let returns_matrix = self
                    .returns_matrix(None, None)
                    .await?
                    .to_html()
                    .replace("plotly-html-element", "returns_matrix");
                tabs.push(("Returns Matrix".to_string(), returns_matrix));
                TabbedHtml::new(report_type, tabs)
            }
            _ => unimplemented!("Report type '{report_type}' is not supported for Tickers"),
        };
        Ok(report)
    }
}
