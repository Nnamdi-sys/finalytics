use dioxus::prelude::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::str::FromStr;
#[cfg(feature = "server")]
use finalytics::prelude::*;

static EMBEDDED_DATALIST: &[u8] = include_bytes!("../datalist.bin");

pub static ALL_SYMBOLS_DATALIST: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let map: HashMap<String, String> = bincode::deserialize(EMBEDDED_DATALIST).unwrap();
    Mutex::new(map)
});

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PortfolioTabs {
    pub optimization_chart: Option<String>,
    pub performance_chart: String,
    pub performance_stats_table: String,
    pub returns_table: String,
    pub returns_chart: String,
    pub returns_matrix: String,
}

#[allow(clippy::too_many_arguments)]
#[server]
pub async fn get_portfolio_charts(
    symbols: Vec<String>,
    benchmark_symbol: String,
    start_date: String,
    end_date: String,
    interval: String,
    confidence_level: f64,
    risk_free_rate: f64,
    objective_function: String,
    constraints: Option<Vec<(f64, f64)>>,
    weights: Option<Vec<f64>>,
) -> Result<PortfolioTabs, ServerFnError<String>> {
    let report_html = tokio::task::spawn_blocking(move || {
        let constraints = Constraints {
            asset_weights: constraints,
            categorical_weights: None
        };
        let rt = tokio::runtime::Handle::current();

        rt.block_on(async {
            let portfolio = Portfolio::builder()
                .ticker_symbols(symbols.iter().map(|x| x.as_str()).collect())
                .benchmark_symbol(&benchmark_symbol)
                .start_date(&start_date)
                .end_date(&end_date)
                .interval(Interval::from_str(&interval).unwrap())
                .confidence_level(confidence_level)
                .risk_free_rate(risk_free_rate)
                .objective_function(ObjectiveFunction::from_str(&objective_function).unwrap())
                .constraints(Some(constraints.clone()))
                .weights(weights.clone())
                .build()
                .await
                .map_err(|e| format!("PortfolioBuilder error: {e}"))?;

            let optimization_chart = if constraints.asset_weights.is_some() || weights.is_none() {
                Some(
                    portfolio
                        .optimization_chart(None, None)
                        .map_err(|e| format!("Optimization Chart error: {e}"))?
                        .to_html(),
                )
            } else {
                None
            };

            let performance_chart = portfolio
                .performance_chart(None, None)
                .map_err(|e| format!("Performance Chart error: {e}"))?
                .to_html();

            let performance_stats_table = portfolio
                .performance_stats_table()
                .await
                .map_err(|e| format!("Performance Stats Table error: {e}"))?
                .to_html()
                .unwrap();

            let returns_table = portfolio
                .returns_table()
                .map_err(|e| format!("Returns Table error: {e}"))?
                .to_html()
                .unwrap();

            let returns_chart = portfolio
                .returns_chart(None, None)
                .map_err(|e| format!("Returns Chart error: {e}"))?
                .to_html();

            let returns_matrix = portfolio
                .returns_matrix(None, None)
                .map_err(|e| format!("Returns Matrix error: {e}"))?
                .to_html();

            Ok(PortfolioTabs {
                optimization_chart,
                performance_chart,
                performance_stats_table,
                returns_table,
                returns_chart,
                returns_matrix,
            })
        })
    })
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(format!("Blocking task failed: {e}")))?;

    report_html
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerformanceTabs {
    pub ohlcv_table: String,
    pub candlestick_chart: String,
    pub performance_chart: String,
    pub performance_stats_table: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FinancialsTabs {
    pub income_statement: String,
    pub balance_sheet: String,
    pub cashflow_statement: String,
    pub financial_ratios: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptionsTabs {
    pub options_chain: String,
    pub volatility_surface_table: String,
    pub volatility_smile: String,
    pub volatility_term_structure: String,
    pub volatility_surface_chart: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewsTabs {
    pub news_sentiment_table: String,
    pub news_sentiment_chart: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TickerTabs {
    Performance(PerformanceTabs),
    Financials(FinancialsTabs),
    Options(OptionsTabs),
    News(NewsTabs),
}

#[allow(clippy::too_many_arguments)]
#[server]
pub async fn get_ticker_charts(
    symbol: String,
    start_date: String,
    end_date: String,
    interval: String,
    benchmark_symbol: String,
    confidence_level: f64,
    risk_free_rate: f64,
    report_type: String,
    frequency: String,
) -> Result<TickerTabs, ServerFnError<String>> {
    let charts = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();

        rt.block_on(async {
            match &*report_type {
                "performance" => {
                    let ticker = Ticker::builder()
                        .ticker(&symbol)
                        .start_date(&start_date)
                        .end_date(&end_date)
                        .interval(Interval::from_str(&interval).unwrap())
                        .benchmark_symbol(&benchmark_symbol)
                        .confidence_level(confidence_level)
                        .risk_free_rate(risk_free_rate)
                        .build();
                    let ohlcv_table = ticker.ohlcv_table().await.unwrap().to_html().unwrap();
                    let candlestick_chart = ticker.candlestick_chart(None, None).await.unwrap().to_html();
                    let performance_chart = ticker.performance_chart(None, None).await.unwrap().to_html();
                    let performance_stats_table = ticker.performance_stats_table().await.unwrap().to_html().unwrap();
                    Ok(TickerTabs::Performance(PerformanceTabs {
                        ohlcv_table,
                        candlestick_chart,
                        performance_chart,
                        performance_stats_table,
                    }))
                },
                "financials" => {
                    let ticker = Ticker::builder()
                        .ticker(&symbol)
                        .build();
                    let frequency = StatementFrequency::from_str(&frequency).unwrap();
                    let financials = ticker.financials_tables(frequency, None).await.unwrap();
                    let income_statement = financials.income_statement.to_html().unwrap();
                    let balance_sheet = financials.balance_sheet.to_html().unwrap();
                    let cashflow_statement = financials.cashflow_statement.to_html().unwrap();
                    let financial_ratios = financials.financial_ratios.to_html().unwrap();
                    Ok(TickerTabs::Financials(FinancialsTabs {
                        income_statement,
                        balance_sheet,
                        cashflow_statement,
                        financial_ratios,
                    }))
                },
                "options" => {
                    let ticker = Ticker::builder()
                        .ticker(&symbol)
                        .risk_free_rate(risk_free_rate)
                        .build();
                    let tables = ticker.options_tables().await.unwrap();
                    let charts = ticker.options_charts(None, None).await.unwrap();
                    let options_chain = tables.options_chain.to_html().unwrap();
                    let volatility_surface_table = tables.volatility_surface.to_html().unwrap();
                    let volatility_smile = charts.volatility_smile.to_html();
                    let volatility_term_structure = charts.volatility_term_structure.to_html();
                    let volatility_surface_chart = charts.volatility_surface.to_html();
                    Ok(TickerTabs::Options(OptionsTabs {
                        options_chain,
                        volatility_surface_table,
                        volatility_smile,
                        volatility_term_structure,
                        volatility_surface_chart,
                    }))
                },
                "news" => {
                    let ticker = Ticker::builder()
                        .ticker(&symbol)
                        .start_date(&start_date)
                        .end_date(&end_date)
                        .build();
                    let news_sentiment_table = ticker.news_sentiment_table().await.unwrap().to_html().unwrap();
                    let news_sentiment_chart = ticker.news_sentiment_chart(None, None).await.unwrap().to_html();
                    Ok(TickerTabs::News(NewsTabs {
                        news_sentiment_table,
                        news_sentiment_chart,
                    }))
                },
                _ => Err(ServerFnError::ServerError("Invalid report type".to_string())),
            }
        })
    })
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(format!("Blocking task failed: {e}")))?;

    charts
}

#[server]
pub async fn get_screener_data(
    quote_type: String,
    filters: Vec<String>,
    sort_field: String,
    sort_descending: bool,
    offset: usize,
    size: usize,
    active_tab: usize
) -> Result<String, ServerFnError<String>> {
    let screener = tokio::task::spawn_blocking(move || {
        let quote_type = QuoteType::from_str(&quote_type).unwrap();
        let filters = filters.into_iter()
            .map(ScreenerFilter::Custom)
            .collect::<Vec<_>>();
        let sort_field = if !sort_field.is_empty() {
            let quote_type = match quote_type {
                QuoteType::Equity => ScreenerMetric::Equity(EquityScreener::from_str(&sort_field).unwrap()),
                QuoteType::MutualFund => ScreenerMetric::MutualFund(MutualFundScreener::from_str(&sort_field).unwrap()),
                QuoteType::Etf => ScreenerMetric::Etf(EtfScreener::from_str(&sort_field).unwrap()),
                QuoteType::Index => ScreenerMetric::Index(IndexScreener::from_str(&sort_field).unwrap()),
                QuoteType::Future => ScreenerMetric::Future(FutureScreener::from_str(&sort_field).unwrap()),
                QuoteType::Crypto => ScreenerMetric::Crypto(CryptoScreener::from_str(&sort_field).unwrap()),
            };
            Some(quote_type)
        } else {
            None
        };
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let screener = ScreenerBuilder {
                quote_type: Some(quote_type),
                filters,
                sort_field,
                sort_descending,
                offset,
                size,
            }.build().await.unwrap();

            match active_tab {
                1 => screener.overview().to_html().unwrap(),
                2 => screener.metrics().await.unwrap().to_html().unwrap(),
                _ => unreachable!()
            }
        })
    })
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(format!("Blocking task failed: {e}")))?;

    Ok(screener)
}


#[cfg(feature = "server")]
use finalytics::data::yahoo::screeners::screeners::FieldMetadata;

#[cfg(not(feature = "server"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldMetadata {
    pub name: String,
    pub description: String,
    pub data_type: String,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScreenerMetadata {
    pub exchange: Vec<(String, String)>,
    pub region: Vec<(String, String)>,
    pub sector: Vec<String>,
    pub industry: Vec<String>,
    pub peer_group: Vec<String>,
    pub fund_family: Vec<String>,
    pub fund_category: Vec<String>,
    pub metrics: HashMap<String, HashMap<String, FieldMetadata>>,
}


#[server]
pub async fn get_screener_metadata() -> Result<ScreenerMetadata, ServerFnError> {
    let metadata = ScreenerMetadata {
        exchange: Exchange::iter().map(|s| (s.to_string(), s.full_name().to_string())).collect(),
        region: Region::iter().map(|s| (s.to_string(), s.full_name().to_string())).collect(),
        sector: Sector::VARIANTS.iter().map(|&s| s.to_string()).collect(),
        industry: Industry::VARIANTS.iter().map(|&s| s.to_string()).collect(),
        peer_group: PeerGroup::VARIANTS.iter().map(|&s| s.to_string()).collect(),
        fund_family: FundFamily::VARIANTS.iter().map(|&s| s.to_string()).collect(),
        fund_category: FundCategory::VARIANTS.iter().map(|&s| s.to_string()).collect(),
        metrics: HashMap::from([
            ("EQUITY".to_string(), EquityScreener::metrics().clone()),
            ("MUTUALFUND".to_string(), MutualFundScreener::metrics().clone()),
            ("ETF".to_string(), EtfScreener::metrics().clone()),
            ("INDEX".to_string(), IndexScreener::metrics().clone()),
            ("FUTURE".to_string(), FutureScreener::metrics().clone()),
            ("CRYPTOCURRENCY".to_string(), CryptoScreener::metrics().clone()),
        ]),
    };
    Ok(metadata)
}

#[server]
pub async fn get_screener_symbols(
    quote_type: String,
    filters: Vec<String>,
    sort_field: String,
    sort_descending: bool,
    offset: usize,
    size: usize,
) -> Result<Vec<String>, ServerFnError<String>> {
    let screener = tokio::task::spawn_blocking(move || {
        let quote_type = QuoteType::from_str(&quote_type).unwrap();
        let filters = filters.into_iter()
            .map(ScreenerFilter::Custom)
            .collect::<Vec<_>>();
        let sort_field = if !sort_field.is_empty() {
            let quote_type = match quote_type {
                QuoteType::Equity => ScreenerMetric::Equity(EquityScreener::from_str(&sort_field).unwrap()),
                QuoteType::MutualFund => ScreenerMetric::MutualFund(MutualFundScreener::from_str(&sort_field).unwrap()),
                QuoteType::Etf => ScreenerMetric::Etf(EtfScreener::from_str(&sort_field).unwrap()),
                QuoteType::Index => ScreenerMetric::Index(IndexScreener::from_str(&sort_field).unwrap()),
                QuoteType::Future => ScreenerMetric::Future(FutureScreener::from_str(&sort_field).unwrap()),
                QuoteType::Crypto => ScreenerMetric::Crypto(CryptoScreener::from_str(&sort_field).unwrap()),
            };
            Some(quote_type)
        } else {
            None
        };
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let screener = ScreenerBuilder {
                quote_type: Some(quote_type),
                filters,
                sort_field,
                sort_descending,
                offset,
                size,
            }.build().await.unwrap();

            screener.symbols
        })
    })
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(format!("Blocking task failed: {e}")))?;

    Ok(screener)
}


#[server]
pub async fn get_screener_performance(
    symbols: Vec<String>,
    start_date: String,
    end_date: String,
    benchmark_symbol: String,
    risk_free_rate: f64
) -> Result<String, ServerFnError<String>> {
    let chart = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();

        rt.block_on(async {
            let symbols: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
            let tickers = Tickers::builder()
                .tickers(symbols)
                .start_date(&start_date)
                .end_date(&end_date)
                .interval(Interval::OneDay)
                .benchmark_symbol(&benchmark_symbol)
                .confidence_level(0.95)
                .risk_free_rate(risk_free_rate)
                .build();
            let stats = tickers.performance_stats_table().await.unwrap().to_html().unwrap();
            stats
        })
    }).await
        .map_err(|e| ServerFnError::<String>::ServerError(format!("Blocking task failed: {e}")))?;

    Ok(chart)
}

#[server]
pub async fn get_screener_portfolio(
    symbols: Vec<String>,
    start_date: String,
    end_date: String,
    benchmark_symbol: String,
    risk_free_rate: f64,
    objective_function: String,
) -> Result<String, ServerFnError<String>> {
    let chart = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();

        rt.block_on(async {
            let symbols: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
            let portfolio = Portfolio::builder()
                .ticker_symbols(symbols)
                .start_date(&start_date)
                .end_date(&end_date)
                .interval(Interval::from_str("1d").unwrap())
                .benchmark_symbol(&benchmark_symbol)
                .confidence_level(0.95)
                .risk_free_rate(risk_free_rate)
                .objective_function(ObjectiveFunction::from_str(&objective_function).unwrap())
                .build()
                .await.unwrap();
            portfolio.optimization_chart(None, None).unwrap().to_html()
        })
    }).await
        .map_err(|e| ServerFnError::<String>::ServerError(format!("Blocking task failed: {e}")))?;

    Ok(chart)
}