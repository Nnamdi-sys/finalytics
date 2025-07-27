use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use polars::frame::DataFrame;
use polars::prelude::Column;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub symbol: String,
    #[serde(rename = "longName")]
    pub name: String,
    #[serde(rename = "quoteType")]
    pub asset_class: String,
    #[serde(rename = "fullExchangeName")]
    pub exchange: String,
    #[serde(default)]
    #[serde(rename = "financialCurrency")]
    pub currency: String,
    #[serde(rename = "regularMarketTime")]
    pub timestamp: i64,
    #[serde(rename = "regularMarketPrice")]
    pub price: f64,
    #[serde(rename = "regularMarketOpen")]
    pub open: f64,
    #[serde(rename = "regularMarketDayHigh")]
    pub high: f64,
    #[serde(rename = "regularMarketDayLow")]
    pub low: f64,
    #[serde(rename = "regularMarketPreviousClose")]
    pub close : f64,
    #[serde(rename = "regularMarketVolume")]
    pub volume: f64,
    #[serde(default)]
    pub bid : f64,
    #[serde(default)]
    pub ask : f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerSummaryStats {
    #[serde(rename = "defaultKeyStatistics")]
    pub default_key_statistics: Option<KeyStatistics>,
    #[serde(rename = "financialData")]
    pub financial_data: Option<FinancialData>,
    #[serde(rename = "summaryDetail")]
    pub summary_detail: Option<SummaryDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedValue {
    #[serde(rename = "fmt")]
    pub formatted: Option<String>,
    #[serde(rename = "raw")]
    pub raw: Option<f64>,
    #[serde(rename = "longFmt")]
    pub long_formatted: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyStatistics {
    // Fiscal Year
    #[serde(rename = "lastFiscalYearEnd")]
    pub fiscal_year_end: Option<FormattedValue>,
    #[serde(rename = "mostRecentQuarter")]
    pub most_recent_quarter: Option<FormattedValue>,
    #[serde(rename = "nextFiscalYearEnd")]
    pub next_fiscal_year_end: Option<FormattedValue>,

    // Valuation Measures
    #[serde(rename = "52WeekChange")]
    pub week_change_52: Option<FormattedValue>,
    #[serde(rename = "SandP52WeekChange")]
    pub sandp_52_week_change: Option<FormattedValue>,
    #[serde(rename = "beta")]
    pub beta: Option<FormattedValue>,
    #[serde(rename = "bookValue")]
    pub book_value: Option<FormattedValue>,
    #[serde(rename = "enterpriseToEbitda")]
    pub enterprise_to_ebitda: Option<FormattedValue>,
    #[serde(rename = "enterpriseToRevenue")]
    pub enterprise_to_revenue: Option<FormattedValue>,
    #[serde(rename = "enterpriseValue")]
    pub enterprise_value: Option<FormattedValue>,
    #[serde(rename = "forwardEps")]
    pub forward_eps: Option<FormattedValue>,
    #[serde(rename = "forwardPE")]
    pub forward_pe: Option<FormattedValue>,
    #[serde(rename = "priceToBook")]
    pub price_to_book: Option<FormattedValue>,


    // Share Statistics
    #[serde(rename = "floatShares")]
    pub float_shares: Option<FormattedValue>,
    #[serde(rename = "heldPercentInsiders")]
    pub held_percent_insiders: Option<FormattedValue>,
    #[serde(rename = "heldPercentInstitutions")]
    pub held_percent_institutions: Option<FormattedValue>,
    #[serde(rename = "sharesOutstanding")]
    pub shares_outstanding: Option<FormattedValue>,
    #[serde(rename = "sharesShort")]
    pub shares_short: Option<FormattedValue>,
    #[serde(rename = "sharesShortPreviousMonthDate")]
    pub shares_short_prev_month_date: Option<FormattedValue>,
    #[serde(rename = "sharesShortPriorMonth")]
    pub shares_short_prior_month: Option<FormattedValue>,
    #[serde(rename = "shortPercentOfFloat")]
    pub short_percent_of_float: Option<FormattedValue>,
    #[serde(rename = "shortRatio")]
    pub short_ratio: Option<FormattedValue>,

    // Dividends & Splits
    #[serde(rename = "lastDividendDate")]
    pub last_dividend_date: Option<FormattedValue>,
    #[serde(rename = "lastDividendValue")]
    pub last_dividend_value: Option<FormattedValue>,
    #[serde(rename = "lastSplitDate")]
    pub last_split_date: Option<FormattedValue>,
    #[serde(rename = "lastSplitFactor")]
    pub last_split_factor: Option<String>,
    
    // Fund Info
    #[serde(rename = "annualHoldingsTurnover")]
    pub annual_holdings_turnover: Option<FormattedValue>,
    #[serde(rename = "annualReportExpenseRatio")]
    pub annual_expense_ratio: Option<FormattedValue>,
    #[serde(rename = "beta3Year")]
    pub beta_3year: Option<FormattedValue>,
    #[serde(rename = "category")]
    pub category: Option<String>,
    #[serde(rename = "fundFamily")]
    pub fund_family: Option<String>,
    #[serde(rename = "fundInceptionDate")]
    pub fund_inception_date: Option<FormattedValue>,
    #[serde(rename = "morningStarOverallRating")]
    pub morning_star_overall_rating: Option<FormattedValue>,
    #[serde(rename = "morningStarRiskRating")]
    pub morning_star_risk_rating: Option<FormattedValue>,
    #[serde(rename = "totalAssets")]
    pub total_assets: Option<FormattedValue>,
    #[serde(rename = "yield")]
    pub yields: Option<FormattedValue>,
    #[serde(rename = "ytdReturn")]
    pub ytd_return: Option<FormattedValue>,
    #[serde(rename = "threeYearAverageReturn")]
    pub three_year_avg_return: Option<FormattedValue>,
    #[serde(rename = "fiveYearAverageReturn")]
    pub five_year_avg_return: Option<FormattedValue>,

    // Other fields
    #[serde(rename = "dateShortInterest")]
    pub date_short_interest: Option<FormattedValue>,
    #[serde(rename = "earningsQuarterlyGrowth")]
    pub earnings_quarterly_growth: Option<FormattedValue>,
    #[serde(rename = "netIncomeToCommon")]
    pub net_income_to_common: Option<FormattedValue>,
    #[serde(rename = "trailingEps")]
    pub trailing_eps: Option<FormattedValue>,
    #[serde(rename = "maxAge")]
    pub max_age: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialData {
    // Income Statement
    #[serde(rename = "totalRevenue")]
    pub total_revenue: Option<FormattedValue>,
    #[serde(rename = "revenuePerShare")]
    pub revenue_per_share: Option<FormattedValue>,
    #[serde(rename = "revenueGrowth")]
    pub revenue_growth: Option<FormattedValue>,
    #[serde(rename = "grossProfits")]
    pub gross_profits: Option<FormattedValue>,
    #[serde(rename = "grossMargins")]
    pub gross_margins: Option<FormattedValue>,
    #[serde(rename = "ebitda")]
    pub ebitda: Option<FormattedValue>,
    #[serde(rename = "ebitdaMargins")]
    pub ebitda_margins: Option<FormattedValue>,
    #[serde(rename = "operatingMargins")]
    pub operating_margins: Option<FormattedValue>,
    #[serde(rename = "netIncomeToCommon")]
    pub net_income_to_common: Option<FormattedValue>,
    #[serde(rename = "earningsGrowth")]
    pub earnings_growth: Option<FormattedValue>,
    #[serde(rename = "profitMargins")]
    pub profit_margins: Option<FormattedValue>,
    #[serde(rename = "returnOnAssets")]
    pub return_on_assets: Option<FormattedValue>,
    #[serde(rename = "returnOnEquity")]
    pub return_on_equity: Option<FormattedValue>,
    

    // Balance Sheet
    #[serde(rename = "totalCash")]
    pub total_cash: Option<FormattedValue>,
    #[serde(rename = "totalDebt")]
    pub total_debt: Option<FormattedValue>,
    #[serde(rename = "totalCashPerShare")]
    pub total_cash_per_share: Option<FormattedValue>,
    #[serde(rename = "debtToEquity")]
    pub debt_to_equity: Option<FormattedValue>,
    #[serde(rename = "currentRatio")]
    pub current_ratio: Option<FormattedValue>,
    #[serde(rename = "quickRatio")]
    pub quick_ratio: Option<FormattedValue>,

    // Cash Flow
    #[serde(rename = "operatingCashflow")]
    pub operating_cash_flow: Option<FormattedValue>,
    #[serde(rename = "freeCashflow")]
    pub free_cash_flow: Option<FormattedValue>,

    // Analyst Ratings
    #[serde(rename = "recommendationMean")]
    pub recommendation_mean: Option<FormattedValue>,
    #[serde(rename = "recommendationKey")]
    pub recommendation_key: Option<String>,
    #[serde(rename = "numberOfAnalystOpinions")]
    pub analyst_opinions_count: Option<FormattedValue>,

    // Other financial metrics
    #[serde(rename = "currentPrice")]
    pub current_price: Option<FormattedValue>,
    #[serde(rename = "targetHighPrice")]
    pub target_high_price: Option<FormattedValue>,
    #[serde(rename = "targetLowPrice")]
    pub target_low_price: Option<FormattedValue>,
    #[serde(rename = "targetMeanPrice")]
    pub target_mean_price: Option<FormattedValue>,
    #[serde(rename = "targetMedianPrice")]
    pub target_median_price: Option<FormattedValue>,
    #[serde(rename = "maxAge")]
    pub max_age: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryDetail {
    // Pricing
    #[serde(rename = "open")]
    pub open: Option<FormattedValue>,
    #[serde(rename = "dayHigh")]
    pub day_high: Option<FormattedValue>,
    #[serde(rename = "dayLow")]
    pub day_low: Option<FormattedValue>,
    #[serde(rename = "previousClose")]
    pub previous_close: Option<FormattedValue>,
    #[serde(rename = "bid")]
    pub bid: Option<FormattedValue>,
    #[serde(rename = "ask")]
    pub ask: Option<FormattedValue>,

    // Volume
    #[serde(rename = "volume")]
    pub volume: Option<FormattedValue>,
    #[serde(rename = "averageVolume")]
    pub avg_volume: Option<FormattedValue>,
    #[serde(rename = "averageVolume10days")]
    pub avg_volume_10_day: Option<FormattedValue>,

    // Dividends
    #[serde(rename = "dividendRate")]
    pub dividend_rate: Option<FormattedValue>,
    #[serde(rename = "dividendYield")]
    pub dividend_yield: Option<FormattedValue>,
    #[serde(rename = "exDividendDate")]
    pub ex_dividend_date: Option<FormattedValue>,
    #[serde(rename = "payoutRatio")]
    pub payout_ratio: Option<FormattedValue>,

    // Averages
    #[serde(rename = "fiftyDayAverage")]
    pub fifty_day_avg: Option<FormattedValue>,
    #[serde(rename = "twoHundredDayAverage")]
    pub two_hundred_day_avg: Option<FormattedValue>,

    // Market Info
    #[serde(rename = "marketCap")]
    pub market_cap: Option<FormattedValue>,
    #[serde(rename = "priceToSalesTrailing12Months")]
    pub price_to_sales_ttm: Option<FormattedValue>,
    #[serde(rename = "yield")]
    pub yields: Option<FormattedValue>,
    #[serde(rename = "ytdReturn")]
    pub ytd_return: Option<FormattedValue>,
    
    // Crypto Info
    #[serde(rename = "circulatingSupply")]
    pub circulating_supply: Option<FormattedValue>,
    #[serde(rename = "coinMarketCapLink")]
    pub coin_market_cap_link: Option<String>,

    // Other fields
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(rename = "tradeable")]
    pub tradable: Option<bool>,
    #[serde(rename = "maxAge")]
    pub max_age: Option<f64>,
}

impl TickerSummaryStats {
    pub fn to_dataframe(&self) -> Result<DataFrame, Box<dyn Error>> {

        let (dks, fd, sd) = (
            self.default_key_statistics.as_ref(),
            self.financial_data.as_ref(),
            self.summary_detail.as_ref(),
        );

        let fields: Vec<String> = vec![
            // Pricing Information
            "Currency".into(),
            "Current Price".into(),
            "Day Range".into(),
            "52 Week Change".into(),
            "Previous Close".into(),
            "Open".into(),
            "Bid/Ask".into(),
            "Volume".into(),
            "50 Day Avg".into(),
            "200 Day Avg".into(),

            // Profitability
            "Total Revenue".into(),
            "Revenue Per Share".into(),
            "Revenue Growth".into(),
            "Gross Profit".into(),
            "EBITDA".into(),
            "EBITDA Margin".into(),
            "Operating Margin".into(),
            "Net Profit Margin".into(),
            "Earnings Growth".into(),
            "Return on Equity".into(),
            "Return on Assets".into(),

            // Cash Flow & Debt
            "Total Cash".into(),
            "Total Debt".into(),
            "Debt to Equity".into(),
            "Current Ratio".into(),
            "Quick Ratio".into(),
            "Operating Cash Flow".into(),
            "Free Cash Flow".into(),

            // Valuation
            "Market Cap".into(),
            "Enterprise Value".into(),
            "Enterprise to EBITDA".into(),
            "Enterprise to Revenue".into(),
            "Forward EPS".into(),
            "P/E (Forward)".into(),
            "Price to Book".into(),
            "Beta (5Y Monthly)".into(),

            // Dividends & Yield
            "Dividend Yield".into(),
            "Dividend Rate".into(),
            "Payout Ratio".into(),
            "Ex-Dividend Date".into(),
            "Last Split".into(),

            // Share Statistics
            "Shares Outstanding".into(),
            "Float Shares".into(),
            "Short Interest".into(),
            "Short % of Float".into(),
            "% Held by Institutions".into(),

            // Analyst Ratings
            "Analyst Consensus".into(),
            "Average Target".into(),
            "High/Low Target".into(),
            "Analyst Count".into(),
            
            // Fund Info
            "Total Assets".into(),
            "Fund Family".into(),
            "Fund Inception Date".into(),
            "Category".into(),
            "Yield".into(),
            "YTD Return".into(),
            "3Y Avg Return".into(),
            "5Y Avg Return".into(),
            "Beta (3Y)".into(),
            "Morningstar Overall Rating".into(),
            "Morningstar Risk Rating".into(),
            "Annual Holdings Turnover".into(),
            "Annual Expense Ratio".into(),
            
            // Crypto Info
            "Circulating Supply".into(),
            "CoinMarketCap Link".into(),
        ];

        let values: Vec<String> = vec![
            // Pricing
            sd.and_then(|s| s.currency.as_ref()).unwrap_or(&"USD".to_string()).into(),
            fd.and_then(|d| d.current_price.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            self.format_day_range(sd),
            dks.and_then(|d| d.week_change_52.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            sd.and_then(|s| s.previous_close.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            sd.and_then(|s| s.open.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            self.format_bid_ask(sd),
            sd.and_then(|s| s.volume.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            sd.and_then(|s| s.fifty_day_avg.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            sd.and_then(|s| s.two_hundred_day_avg.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),

            // Profitability
            fd.and_then(|f| f.total_revenue.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.revenue_per_share.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.revenue_growth.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.gross_profits.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.ebitda.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.ebitda_margins.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|d| d.operating_margins.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|d| d.profit_margins.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|d| d.earnings_growth.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|d| d.return_on_equity.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|d| d.return_on_assets.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),

            // Cash/Debt
            fd.and_then(|f| f.total_cash.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.total_debt.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.debt_to_equity.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.current_ratio.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.quick_ratio.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.operating_cash_flow.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            fd.and_then(|f| f.free_cash_flow.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),

            // Valuation
            sd.and_then(|s| s.market_cap.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.enterprise_value.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.enterprise_to_ebitda.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.enterprise_to_revenue.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.forward_eps.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.forward_pe.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.price_to_book.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.beta.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),

            // Dividends
            sd.and_then(|s| s.dividend_yield.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            sd.and_then(|s| s.dividend_rate.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            sd.and_then(|s| s.payout_ratio.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            sd.and_then(|s| s.ex_dividend_date.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            self.format_last_split(dks),

            // Shares
            dks.and_then(|d| d.shares_outstanding.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.float_shares.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.shares_short.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.short_percent_of_float.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.held_percent_institutions.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),

            // Analyst
            fd.map(|f| f.recommendation_key.as_deref().unwrap_or("")).unwrap_or("").into(),
            self.format_target_prices(fd),
            self.format_high_low_target(fd),
            fd.and_then(|f| f.analyst_opinions_count.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            
            // Fund Info
            dks.and_then(|d| d.annual_holdings_turnover.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.annual_expense_ratio.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.beta_3year.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.category.as_ref())
                .map_or("", |v| v.as_str()).into(),
            dks.and_then(|d| d.fund_family.as_ref())
                .map_or("", |v| v.as_str()).into(),
            dks.and_then(|d| d.fund_inception_date.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.morning_star_overall_rating.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.morning_star_risk_rating.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.total_assets.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.yields.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or(
                    sd.and_then(|s| s.yields.as_ref())
                        .map_or("", |v| v.formatted.as_deref().unwrap_or("")),
                )).into(),
            dks.and_then(|d| d.ytd_return.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or(
                    sd.and_then(|s| s.ytd_return.as_ref())
                        .map_or("", |v| v.formatted.as_deref().unwrap_or("")),
                )).into(),
            dks.and_then(|d| d.three_year_avg_return.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            dks.and_then(|d| d.five_year_avg_return.as_ref())
                .map_or("", |v| v.formatted.as_deref().unwrap_or("")).into(),
            
            // Crypto Info
            sd.and_then(|s| s.circulating_supply.as_ref())
                .map_or("", |v| v.long_formatted.as_deref().unwrap_or("")).into(),
            sd.and_then(|s| s.coin_market_cap_link.as_ref())
                .map_or("", |v| v.as_str()).into(),
        ];

        let df = DataFrame::new(vec![
            Column::new("Metric".into(), fields),
            Column::new("Value".into(), values),
        ])?;

        Ok(df)
    }

    // Helper functions for complex formatting
    fn format_day_range(&self, sd: Option<&SummaryDetail>) -> String {
        let high = sd.and_then(|s| s.day_high.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        let low = sd.and_then(|s| s.day_low.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        format!("{high} - {low}")
    }

    fn format_bid_ask(&self, sd: Option<&SummaryDetail>) -> String {
        let bid = sd.and_then(|s| s.bid.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        let ask = sd.and_then(|s| s.ask.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        format!("{bid} / {ask}")
    }

    fn format_last_split(&self, dks: Option<&KeyStatistics>) -> String {
        let date = dks.and_then(|d| d.last_split_date.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        let factor = dks.and_then(|d| d.last_split_factor.as_deref())
            .unwrap_or("");
        format!("{date} ({factor})")
    }

    fn format_target_prices(&self, fd: Option<&FinancialData>) -> String {
        let median = fd.and_then(|f| f.target_median_price.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        let mean = fd.and_then(|f| f.target_mean_price.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        format!("Median: {median}, Mean: {mean}")
    }

    fn format_high_low_target(&self, fd: Option<&FinancialData>) -> String {
        let high = fd.and_then(|f| f.target_high_price.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        let low = fd.and_then(|f| f.target_low_price.as_ref())
            .and_then(|v| v.formatted.as_deref())
            .unwrap_or("");
        format!("{high} - {low}")
    }
}



#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct OptionContract {
    pub contractSymbol: String,
    pub strike: f64,
    pub currency: String,
    pub lastPrice: f64,
    #[serde(default)]
    pub change: f64,
    #[serde(default)]
    pub percentChange: f64,
    #[serde(default)]
    pub openInterest: f64,
    #[serde(default)]
    pub bid: f64,
    #[serde(default)]
    pub ask: f64,
    pub contractSize: String,
    pub expiration: i64,
    pub lastTradeDate: i64,
    pub impliedVolatility: f64,
    pub inTheMoney: bool,
}

#[derive(Debug)]
pub struct Options {
    pub ticker_price: f64,
    pub expiration_dates: Vec<String>,
    pub ttms: Vec<f64>,
    pub strikes: Vec<f64>,
    pub chain: DataFrame
}

#[derive(Debug, Deserialize)]
pub struct FundamentalsResponse {
    pub timeseries: TimeSeries,
}

#[derive(Debug, Deserialize)]
pub struct TimeSeries {
    pub result: Vec<HashMap<String, Value>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Object {
    pub asOfDate: String,
    pub reportedValue: Figure,
}

#[derive(Debug, Deserialize)]
pub struct Figure {
    pub raw: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Interval {
    TwoMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    SixtyMinutes,
    NinetyMinutes,
    OneHour,
    OneDay,
    FiveDays,
    OneWeek,
    OneMonth,
    ThreeMonths,
}

// Implement Display to convert Interval to &str (and get .to_string() for free)
impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Interval::TwoMinutes => "2m",
            Interval::FiveMinutes => "5m",
            Interval::FifteenMinutes => "15m",
            Interval::ThirtyMinutes => "30m",
            Interval::SixtyMinutes => "60m",
            Interval::NinetyMinutes => "90m",
            Interval::OneHour => "1h",
            Interval::OneDay => "1d",
            Interval::FiveDays => "5d",
            Interval::OneWeek => "1wk",
            Interval::OneMonth => "1mo",
            Interval::ThreeMonths => "3mo",
        };
        write!(f, "{s}")
    }
}

// Implement FromStr to convert &str to Interval
impl FromStr for Interval {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let interval = match s {
            "2m" => Interval::TwoMinutes,
            "5m" => Interval::FiveMinutes,
            "15m" => Interval::FifteenMinutes,
            "30m" => Interval::ThirtyMinutes,
            "60m" => Interval::SixtyMinutes,
            "90m" => Interval::NinetyMinutes,
            "1h" => Interval::OneHour,
            "1d" => Interval::OneDay,
            "5d" => Interval::FiveDays,
            "1wk" => Interval::OneWeek,
            "1mo" => Interval::OneMonth,
            "3mo" => Interval::ThreeMonths,
            _ => return Err(()),
        };
        Ok(interval)
    }
}

impl Interval {
    pub fn to_days(&self) -> f64 {
        match self {
            Interval::TwoMinutes => 2.0 / 24.0 * 60.0,
            Interval::FiveMinutes => 5.0 / 24.0 * 60.0,
            Interval::FifteenMinutes => 15.0 / 24.0 * 60.0,
            Interval::ThirtyMinutes => 30.0 / 24.0 * 60.0,
            Interval::SixtyMinutes => 60.0 / 24.0 * 60.0,
            Interval::OneHour => 60.0 / 24.0 * 60.0,
            Interval::NinetyMinutes => 90.0 / 24.0 * 60.0,
            Interval::OneDay => 1.0,
            Interval::FiveDays => 5.0,
            Interval::OneWeek => 5.0,
            Interval::OneMonth => 20.0,
            Interval::ThreeMonths => 60.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StatementType {
    IncomeStatement,
    BalanceSheet,
    CashFlowStatement,
    FinancialRatios,
}

impl fmt::Display for StatementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StatementType::IncomeStatement => "income-statement",
            StatementType::BalanceSheet => "balance-sheet",
            StatementType::CashFlowStatement => "cash-flow",
            StatementType::FinancialRatios => "financial-ratios",
        };
        write!(f, "{s}")
    }
}

impl FromStr for StatementType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "income-statement" => Ok(StatementType::IncomeStatement),
            "balance-sheet" => Ok(StatementType::BalanceSheet),
            "cash-flow" => Ok(StatementType::CashFlowStatement),
            "financial-ratios" => Ok(StatementType::FinancialRatios),
            _ => Err(format!("StatementType '{s}' not implemented")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StatementFrequency {
    Annual,
    Quarterly
}

impl fmt::Display for StatementFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StatementFrequency::Annual => "annual",
            StatementFrequency::Quarterly => "quarterly",
        };
        write!(f, "{s}")
    }
}

impl FromStr for StatementFrequency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "annual" => Ok(StatementFrequency::Annual),
            "quarterly" => Ok(StatementFrequency::Quarterly),
            _ => Err(()), 
        }
    }
}

pub struct Fundamentals;

impl Fundamentals {
    pub fn get_income_statement_items(&self, frequency: StatementFrequency) -> String {
        let income_vec =  vec![
            "TotalRevenue", "ExciseTaxes", "OperatingRevenue", "GrossProfit", "CostOfRevenue",
            "SalariesAndWages", "RentAndLandingFees", "InsuranceAndClaims", "OtherGandA",
            "GeneralAndAdministrativeExpense", "SellingAndMarketingExpense", "SellingGeneralAndAdministration",
            "ResearchAndDevelopment", "DepreciationIncomeStatement", "AmortizationOfIntangiblesIncomeStatement",
            "DepreciationAndAmortizationInIncomeStatement", "Amortization", "DepreciationAmortizationDepletionIncomeStatement",
            "DepletionIncomeStatement", "ProvisionForDoubtfulAccounts", "OtherTaxes", "OtherOperatingExpenses",
            "OperatingExpense", "InterestIncomeNonOperating", "InterestExpenseNonOperating", "InterestExpense",
            "NetNonOperatingInterestIncomeExpense", "GainOnSaleOfSecurity", "EarningsFromEquityInterest",
            "SecuritiesAmortization", "RestructuringAndMergernAcquisition", "ImpairmentOfCapitalAssets",
            "WriteOff", "OtherSpecialCharges", "GainOnSaleOfBusiness", "GainOnSaleOfPPE", "SpecialIncomeCharges",
            "OtherNonOperatingIncomeExpenses", "OtherIncomeExpense", "PretaxIncome", "TaxProvision",
            "EarningsFromEquityInterestNetOfTax", "NetIncomeContinuousOperations", "NetIncomeDiscontinuousOperations",
            "NetIncomeExtraordinary", "NetIncomeFromTaxLossCarryforward", "NetIncomeIncludingNoncontrollingInterests",
            "MinorityInterests", "NetIncome", "PreferredStockDividends", "OtherunderPreferredStockDividend",
            "NetIncomeCommonStockholders", "AverageDilutionEarnings", "DilutedNIAvailtoComStockholders",
            "BasicContinuousOperations", "BasicDiscontinuousOperations", "BasicExtraordinary", "BasicAccountingChange",
            "TaxLossCarryforwardBasicEPS", "BasicEPSOtherGainsLosses", "BasicEPS", "DilutedContinuousOperations",
            "DilutedDiscontinuousOperations", "DilutedExtraordinary", "DilutedAccountingChange",
            "TaxLossCarryforwardDilutedEPS", "DilutedEPSOtherGainsLosses", "DilutedEPS", "BasicAverageShares",
            "DilutedAverageShares", "DividendPerShare", "TotalOperatingIncomeAsReported", "ReportedNormalizedBasicEPS",
            "ReportedNormalizedDilutedEPS", "RentExpenseSupplemental", "TotalExpenses",
            "NetIncomeFromContinuingAndDiscontinuedOperation", "NormalizedIncome", "ContinuingAndDiscontinuedBasicEPS",
            "ContinuingAndDiscontinuedDilutedEPS", "InterestIncome", "InterestExpense", "NetInterestIncome", "EBIT", "EBITDA",
            "ReconciledCostOfRevenue", "ReconciledDepreciation", "NetIncomeFromContinuingOperationNetMinorityInterest",
            "TotalUnusualItemsExcludingGoodwill", "TotalUnusualItems", "NormalizedBasicEPS", "NormalizedDilutedEPS",
            "NormalizedEBITDA", "TaxRateForCalcs", "TaxEffectOfUnusualItems",
        ];

        let out_str = income_vec.iter().map(|x| format!("{frequency}{x}")).collect::<Vec<String>>();
        out_str.join(",")

    }

    pub fn get_balance_sheet_items(&self, frequency: StatementFrequency) -> String {
        let balance_vec = vec![
            "TreasurySharesNumber", "PreferredSharesNumber", "OrdinarySharesNumber", "ShareIssued", "NetDebt",
            "TotalDebt", "TangibleBookValue", "InvestedCapital", "WorkingCapital", "NetTangibleAssets",
            "CapitalLeaseObligations", "CommonStockEquity", "PreferredStockEquity", "TotalCapitalization",
            "TotalEquityGrossMinorityInterest", "MinorityInterest", "StockholdersEquity",
            "OtherEquityInterest", "GainsLossesNotAffectingRetainedEarnings", "OtherEquityAdjustments",
            "FixedAssetsRevaluationReserve", "ForeignCurrencyTranslationAdjustments",
            "MinimumPensionLiabilities", "UnrealizedGainLoss", "TreasuryStock", "RetainedEarnings",
            "AdditionalPaidInCapital", "CapitalStock", "OtherCapitalStock", "CommonStock", "PreferredStock",
            "TotalPartnershipCapital", "GeneralPartnershipCapital", "LimitedPartnershipCapital",
            "TotalLiabilitiesNetMinorityInterest", "TotalNonCurrentLiabilitiesNetMinorityInterest",
            "OtherNonCurrentLiabilities", "LiabilitiesHeldforSaleNonCurrent", "RestrictedCommonStock",
            "PreferredSecuritiesOutsideStockEquity", "DerivativeProductLiabilities", "EmployeeBenefits",
            "NonCurrentPensionAndOtherPostretirementBenefitPlans", "NonCurrentAccruedExpenses",
            "DuetoRelatedPartiesNonCurrent", "TradeandOtherPayablesNonCurrent",
            "NonCurrentDeferredLiabilities", "NonCurrentDeferredRevenue",
            "NonCurrentDeferredTaxesLiabilities", "LongTermDebtAndCapitalLeaseObligation",
            "LongTermCapitalLeaseObligation", "LongTermDebt", "LongTermProvisions", "CurrentLiabilities",
            "OtherCurrentLiabilities", "CurrentDeferredLiabilities", "CurrentDeferredRevenue",
            "CurrentDeferredTaxesLiabilities", "CurrentDebtAndCapitalLeaseObligation",
            "CurrentCapitalLeaseObligation", "CurrentDebt", "OtherCurrentBorrowings", "LineOfCredit",
            "CommercialPaper", "CurrentNotesPayable", "PensionandOtherPostRetirementBenefitPlansCurrent",
            "CurrentProvisions", "PayablesAndAccruedExpenses", "CurrentAccruedExpenses", "InterestPayable",
            "Payables", "OtherPayable", "DuetoRelatedPartiesCurrent", "DividendsPayable", "TotalTaxPayable",
            "IncomeTaxPayable", "AccountsPayable", "TotalAssets", "TotalNonCurrentAssets",
            "OtherNonCurrentAssets", "DefinedPensionBenefit", "NonCurrentPrepaidAssets",
            "NonCurrentDeferredAssets", "NonCurrentDeferredTaxesAssets", "DuefromRelatedPartiesNonCurrent",
            "NonCurrentNoteReceivables", "NonCurrentAccountsReceivable", "FinancialAssets",
            "InvestmentsAndAdvances", "OtherInvestments", "InvestmentinFinancialAssets",
            "HeldToMaturitySecurities", "AvailableForSaleSecurities",
            "FinancialAssetsDesignatedasFairValueThroughProfitorLossTotal", "TradingSecurities",
            "LongTermEquityInvestment", "InvestmentsinJointVenturesatCost",
            "InvestmentsInOtherVenturesUnderEquityMethod", "InvestmentsinAssociatesatCost",
            "InvestmentsinSubsidiariesatCost", "InvestmentProperties", "GoodwillAndOtherIntangibleAssets",
            "OtherIntangibleAssets", "Goodwill", "NetPPE", "AccumulatedDepreciation", "GrossPPE", "Leases",
            "ConstructionInProgress", "OtherProperties", "MachineryFurnitureEquipment",
            "BuildingsAndImprovements", "LandAndImprovements", "Properties", "CurrentAssets",
            "OtherCurrentAssets", "HedgingAssetsCurrent", "AssetsHeldForSaleCurrent", "CurrentDeferredAssets",
            "CurrentDeferredTaxesAssets", "RestrictedCash", "PrepaidAssets", "Inventory",
            "InventoriesAdjustmentsAllowances", "OtherInventories", "FinishedGoods", "WorkInProcess",
            "RawMaterials", "Receivables", "ReceivablesAdjustmentsAllowances", "OtherReceivables",
            "DuefromRelatedPartiesCurrent", "TaxesReceivable", "AccruedInterestReceivable", "NotesReceivable",
            "LoansReceivable", "AccountsReceivable", "AllowanceForDoubtfulAccountsReceivable",
            "GrossAccountsReceivable", "CashCashEquivalentsAndShortTermInvestments",
            "OtherShortTermInvestments", "CashAndCashEquivalents", "CashEquivalents", "CashFinancial",
        ];

        let out_str = balance_vec.iter().map(|x| format!("{frequency}{x}")).collect::<Vec<String>>();
        out_str.join(",")

    }

    pub fn get_cash_flow_items(&self, frequency: StatementFrequency) -> String {
        let cash_vec = vec![
            "ForeignSales", "DomesticSales", "AdjustedGeographySegmentData", "FreeCashFlow",
            "RepurchaseOfCapitalStock", "RepaymentOfDebt", "IssuanceOfDebt", "IssuanceOfCapitalStock",
            "CapitalExpenditure", "InterestPaidSupplementalData", "IncomeTaxPaidSupplementalData",
            "EndCashPosition", "OtherCashAdjustmentOutsideChangeinCash", "BeginningCashPosition",
            "EffectOfExchangeRateChanges", "ChangesInCash", "OtherCashAdjustmentInsideChangeinCash",
            "CashFlowFromDiscontinuedOperation", "FinancingCashFlow", "CashFromDiscontinuedFinancingActivities",
            "CashFlowFromContinuingFinancingActivities", "NetOtherFinancingCharges", "InterestPaidCFF",
            "ProceedsFromStockOptionExercised", "CashDividendsPaid", "PreferredStockDividendPaid",
            "CommonStockDividendPaid", "NetPreferredStockIssuance", "PreferredStockPayments",
            "PreferredStockIssuance", "NetCommonStockIssuance", "CommonStockPayments", "CommonStockIssuance",
            "NetIssuancePaymentsOfDebt", "NetShortTermDebtIssuance", "ShortTermDebtPayments",
            "ShortTermDebtIssuance", "NetLongTermDebtIssuance", "LongTermDebtPayments", "LongTermDebtIssuance",
            "InvestingCashFlow", "CashFromDiscontinuedInvestingActivities",
            "CashFlowFromContinuingInvestingActivities", "NetOtherInvestingChanges", "InterestReceivedCFI",
            "DividendsReceivedCFI", "NetInvestmentPurchaseAndSale", "SaleOfInvestment", "PurchaseOfInvestment",
            "NetInvestmentPropertiesPurchaseAndSale", "SaleOfInvestmentProperties",
            "PurchaseOfInvestmentProperties", "NetBusinessPurchaseAndSale", "SaleOfBusiness",
            "PurchaseOfBusiness", "NetIntangiblesPurchaseAndSale", "SaleOfIntangibles", "PurchaseOfIntangibles",
            "NetPPEPurchaseAndSale", "SaleOfPPE", "PurchaseOfPPE", "CapitalExpenditureReported",
            "OperatingCashFlow", "CashFromDiscontinuedOperatingActivities",
            "CashFlowFromContinuingOperatingActivities", "TaxesRefundPaid", "InterestReceivedCFO",
            "InterestPaidCFO", "DividendReceivedCFO", "DividendPaidCFO", "ChangeInWorkingCapital",
            "ChangeInOtherWorkingCapital", "ChangeInOtherCurrentLiabilities", "ChangeInOtherCurrentAssets",
            "ChangeInPayablesAndAccruedExpense", "ChangeInAccruedExpense", "ChangeInInterestPayable",
            "ChangeInPayable", "ChangeInDividendPayable", "ChangeInAccountPayable", "ChangeInTaxPayable",
            "ChangeInIncomeTaxPayable", "ChangeInPrepaidAssets", "ChangeInInventory", "ChangeInReceivables",
            "ChangesInAccountReceivables", "OtherNonCashItems", "ExcessTaxBenefitFromStockBasedCompensation",
            "StockBasedCompensation", "UnrealizedGainLossOnInvestmentSecurities", "ProvisionandWriteOffofAssets",
            "AssetImpairmentCharge", "AmortizationOfSecurities", "DeferredTax", "DeferredIncomeTax",
            "DepreciationAmortizationDepletion", "Depletion", "DepreciationAndAmortization",
            "AmortizationCashFlow", "AmortizationOfIntangibles", "Depreciation", "OperatingGainsLosses",
            "PensionAndEmployeeBenefitExpense", "EarningsLossesFromEquityInvestments",
            "GainLossOnInvestmentSecurities", "NetForeignCurrencyExchangeGainLoss", "GainLossOnSaleOfPPE",
            "GainLossOnSaleOfBusiness", "NetIncomeFromContinuingOperations",
            "CashFlowsfromusedinOperatingActivitiesDirect", "TaxesRefundPaidDirect", "InterestReceivedDirect",
            "InterestPaidDirect", "DividendsReceivedDirect", "DividendsPaidDirect", "ClassesofCashPayments",
            "OtherCashPaymentsfromOperatingActivities", "PaymentsonBehalfofEmployees",
            "PaymentstoSuppliersforGoodsandServices", "ClassesofCashReceiptsfromOperatingActivities",
            "OtherCashReceiptsfromOperatingActivities", "ReceiptsfromGovernmentGrants", "ReceiptsfromCustomers",
        ];

        let out_str = cash_vec.iter().map(|x| format!("{frequency}{x}")).collect::<Vec<String>>();
        out_str.join(",")

    }
}




