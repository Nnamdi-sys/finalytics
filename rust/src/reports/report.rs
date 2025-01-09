use std::error::Error;
use crate::reports::table::DataTable;
use crate::prelude::{TableType, Portfolio, PortfolioCharts, StatementFrequency, Ticker, TickerCharts, TickerData, Tickers, TickersCharts};
use crate::reports::tabs::TabbedHtml;

#[derive(Debug, Clone, Copy)]
pub enum ReportType {
    Performance,
    Financials,
    Options,
    News
}

impl ReportType {
    pub fn from_str(report_type: &str) -> Self {
        match report_type {
            "performance" => ReportType::Performance,
            "financials" => ReportType::Financials,
            "options" => ReportType::Options,
            "news" => ReportType::News,
            _ => panic!("Invalid Report Type")
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            ReportType::Performance => "performance",
            ReportType::Financials => "financials",
            ReportType::Options => "options",
            ReportType::News => "news",
        }
    }
}

pub trait Report {
    fn report(&self, report_type: Option<ReportType>) -> impl std::future::Future<Output = Result<TabbedHtml, Box<dyn Error>>>;
}

impl Report for Ticker {
    async fn report(&self, report_type: Option<ReportType>) -> Result<TabbedHtml, Box<dyn Error>> {
        let report_type = report_type.unwrap_or(ReportType::Performance);
        let report = match report_type {
            ReportType::Performance => {
                let mut tabs: Vec<(String, String)> = Vec::new();
                let price_table = self.ohlcv_table().await?.to_html()?;
                tabs.push(("Price Data".to_string(), price_table));
                let candlestick_chart = self.candlestick_chart(None, None).await?
                    .to_html().replace("plotly-html-element", "candlestick_chart");
                tabs.push(("Candlestick Chart".to_string(), candlestick_chart));
                let performance_chart = self.performance_chart(None, None).await?
                    .to_html().replace("plotly-html-element", "performance_chart");
                tabs.push(("Performance Chart".to_string(), performance_chart));
                let performance_stats = self.performance_stats_table().await?.to_html()?;
                tabs.push(("Performance Stats".to_string(), performance_stats));
                let report = TabbedHtml::new(report_type, tabs);
                report
            }
            ReportType::Financials => {
                let annual_financials = self.financials_tables(StatementFrequency::Annual).await?;
                let quarterly_financials = self.financials_tables(StatementFrequency::Quarterly).await?;
                let tabs: Vec<(String, String)> = vec![
                    ("Quarterly Income Statement".to_string(), quarterly_financials.income_statement.to_html()?),
                    ("Annual Income Statement".to_string(), annual_financials.income_statement.to_html()?),
                    ("Quarterly Balance Sheet".to_string(), quarterly_financials.balance_sheet.to_html()?),
                    ("Annual Balance Sheet".to_string(), annual_financials.balance_sheet.to_html()?),
                    ("Quarterly Cash Flow Statement".to_string(), quarterly_financials.cashflow_statement.to_html()?),
                    ("Annual Cash Flow Statement".to_string(), annual_financials.cashflow_statement.to_html()?),
                    ("Quarterly Financial Ratios".to_string(), quarterly_financials.financial_ratios.to_html()?),
                    ("Annual Financial Ratios".to_string(), annual_financials.financial_ratios.to_html()?),
                ];
                let report = TabbedHtml::new(report_type, tabs);
                report
            }
            ReportType::Options => {
                let options_charts = self.options_charts(None, None).await?;
                let options_table = self.options_tables().await?;
                let tabs: Vec<(String, String)> = vec![
                    ("Options Chain".to_string(), options_table.options_chain.to_html()?),
                    ("Volatility Surface Data".to_string(), options_table.volatility_surface.to_html()?),
                    ("Volatility Smile".to_string(), options_charts.volatility_smile.to_html().replace("plotly-html-element", "volatility_smile")),
                    ("Volatility Term Structure".to_string(), options_charts.volatility_term_structure.to_html().replace("plotly-html-element", "volatility_term_structure")),
                    ("Volatility Surface Chart".to_string(), options_charts.volatility_surface.to_html().replace("plotly-html-element", "volatility_surface")),
                ];
                let report = TabbedHtml::new(report_type, tabs);
                report
            },
            ReportType::News => {
                let mut tabs: Vec<(String, String)> = Vec::new();
                let mut news = self.get_news().await?;
                let _ = news.drop_in_place("Title")?;
                news.rename("Link", "Title")?;
                let news_table = DataTable::new(news.into(), TableType::NewsSentiment).to_html()?;
                tabs.push(("News Sentiment Data".to_string(), news_table));
                let news_chart = self.news_sentiment_chart(None, None).await?
                    .to_html().replace("plotly-html-element", "news_chart");
                tabs.push(("News Sentiment Chart".to_string(), news_chart));
                let report = TabbedHtml::new(report_type, tabs);
                report
            }
        };
        Ok(report)
    }
}

impl Report for Portfolio {
    async fn report(&self, report_type: Option<ReportType>) -> Result<TabbedHtml, Box<dyn Error>> {
        let report_type = report_type.unwrap_or(ReportType::Performance);
        let report = match report_type {
            ReportType::Performance => {
                let mut tabs: Vec<(String, String)> = Vec::new();
                let optimization_chart = self.optimization_chart(None, None)?
                    .to_html().replace("plotly-html-element", "optimization_chart");
                tabs.push(("Optimization Chart".to_string(), optimization_chart));
                let performance_chart = self.performance_chart(None, None)?
                    .to_html().replace("plotly-html-element", "performance_chart");
                tabs.push(("Performance Chart".to_string(), performance_chart));
                let performance_stats = self.performance_stats_table().await?.to_html()?;
                tabs.push(("Performance Stats".to_string(), performance_stats));
                let returns_table = self.returns_table()?.to_html()?;
                tabs.push(("Returns Data".to_string(), returns_table));
                let returns_chart = self.returns_chart(None, None)?
                    .to_html().replace("plotly-html-element", "returns_chart");
                tabs.push(("Returns Chart".to_string(), returns_chart));
                let returns_matrix = self.returns_matrix(None, None)?
                    .to_html().replace("plotly-html-element", "returns_matrix");
                tabs.push(("Returns Matrix".to_string(), returns_matrix));
                let report = TabbedHtml::new(report_type, tabs);
                report
            }
            _ => unimplemented!("Only Performance Report is supported for Portfolio")
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
                let returns_table = self.returns_table().await?.to_html()?;
                tabs.push(("Returns Data".to_string(), returns_table));
                let performance_stats = self.performance_stats_table().await?.to_html()?;
                tabs.push(("Performance Stats".to_string(), performance_stats));
                let returns_chart = self.returns_chart(None, None).await?
                    .to_html().replace("plotly-html-element", "returns_chart");
                tabs.push(("Returns Chart".to_string(), returns_chart));
                let returns_matrix = self.returns_matrix(None, None).await?
                    .to_html().replace("plotly-html-element", "returns_matrix");
                tabs.push(("Returns Matrix".to_string(), returns_matrix));
                let report = TabbedHtml::new(report_type, tabs);
                report
            }
            _ => unimplemented!("Only Performance Report is supported for Tickers")
        };
        Ok(report)
    }
}