use dioxus::prelude::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::str::FromStr;
#[cfg(feature = "server")]
use finalytics::prelude::*;
#[cfg(feature = "server")]
use syntect::highlighting::ThemeSet;
#[cfg(feature = "server")]
use syntect::html::highlighted_html_for_string;
#[cfg(feature = "server")]
use syntect::parsing::SyntaxSet;

static EMBEDDED_DATALIST: &[u8] = include_bytes!("../datalist.bin");

pub static ALL_SYMBOLS_DATALIST: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let map: HashMap<String, String> = bincode::deserialize(EMBEDDED_DATALIST).unwrap();
    Mutex::new(map)
});

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
    active_tab : usize,
) -> Result<String, ServerFnError<String>> {
    let report_html = tokio::task::spawn_blocking(move || {
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
                .build()
                .await
                .map_err(|e| format!("PortfolioBuilder error: {e}"));

            match portfolio {
                Ok(portfolio) => {
                    let chart = match active_tab {
                        1 => portfolio.optimization_chart(None, None).map_err(|e| format!("Optimization Chart error: {e}")).unwrap().to_html(),
                        2 => portfolio.performance_chart(None, None).map_err(|e| format!("Performance Chart error: {e}")).unwrap().to_html(),
                        3 => portfolio.performance_stats_table().await.map_err(|e| format!("Performance Stats Table error: {e}")).unwrap().to_html().unwrap(),
                        4 => portfolio.returns_table().map_err(|e| format!("Returns Table error: {e}")).unwrap().to_html().unwrap(),
                        5 => portfolio.returns_chart(None, None).map_err(|e| format!("Returns Chart error: {e}")).unwrap().to_html(),
                        6 => portfolio.returns_matrix(None, None).map_err(|e| format!("Returns Matrix error: {e}")).unwrap().to_html(),
                        _ => "".to_string(),
                    };

                    Ok(chart)
                }
                Err(e) => Err(e),
            }
        })
    })
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(format!("Blocking task failed: {e}")))??;

    Ok(report_html)
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
    active_tab: usize
) -> Result<String, ServerFnError<String>> {
    let chart = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();

        rt.block_on(async {
            let chart = match &*report_type {
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
                    match active_tab {
                        1 => ticker.ohlcv_table().await.unwrap().to_html().unwrap(),
                        2 => ticker.candlestick_chart(None, None).await.unwrap().to_html(),
                        3 => ticker.performance_chart(None, None).await.unwrap().to_html(),
                        4 => ticker.performance_stats_table().await.unwrap().to_html().unwrap(),
                        _ => "".to_string(),
                    }
                },
                "financials" => {
                    let ticker = Ticker::builder()
                        .ticker(&symbol)
                        .build();
                    let frequency = StatementFrequency::from_str(&frequency).unwrap();
                    let financials = ticker.financials_tables(frequency).await.unwrap();
                    match active_tab {
                        1 => financials.income_statement.to_html().unwrap(),
                        2 => financials.balance_sheet.to_html().unwrap(),
                        3 => financials.cashflow_statement.to_html().unwrap(),
                        4 => financials.financial_ratios.to_html().unwrap(),
                        _ => unreachable!(),
                    }
                },
                "options" => {
                    let ticker = Ticker::builder()
                        .ticker(&symbol)
                        .risk_free_rate(risk_free_rate)
                        .build();
                    match active_tab {
                        1 | 2 => {
                            let tables = ticker.options_tables().await.unwrap();
                            match active_tab {
                                1 => tables.options_chain.to_html().unwrap(),
                                2 => tables.volatility_surface.to_html().unwrap(),
                                _ => unreachable!(),
                            }
                        }
                        3..=5 => {
                            let charts = ticker.options_charts(None, None).await.unwrap();
                            match active_tab {
                                3 => charts.volatility_smile.to_html(),
                                4 => charts.volatility_term_structure.to_html(),
                                5 => charts.volatility_surface.to_html(),
                                _ => unreachable!(),
                            }
                        }
                        _ => "".to_string(),
                    }
                },
                "news" => {
                    let ticker = Ticker::builder()
                        .ticker(&symbol)
                        .start_date(&start_date)
                        .end_date(&end_date)
                        .build();
                    match active_tab {
                        1 => ticker.news_sentiment_table().await.unwrap().to_html().unwrap(),
                        2 => ticker.news_sentiment_chart(None, None).await.unwrap().to_html(),
                        _ => "".to_string()
                    }
                },
                _ => "".to_string()
            };
            
            chart
        })
    }).await
        .map_err(|e| ServerFnError::<String>::ServerError(format!("Blocking task failed: {e}")))?;
    
    Ok(chart)
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

#[server]
pub async fn highlight_code(code: String, lang: String) -> Result<String, ServerFnError> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ss.find_syntax_by_extension(&lang).unwrap();
    let mut html = highlighted_html_for_string(&code, &ss, syntax, &ts.themes["base16-ocean.dark"])?;
    html = html.replace(
        "<pre style=\"",
        "<pre style=\"font-size: 1.2em; "
    );
    Ok(html)
}