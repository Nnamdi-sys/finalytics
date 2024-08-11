use bincode;
use dioxus::prelude::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
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

#[server]
pub async fn get_portfolio_chart(
    symbols: Vec<String>,
    benchmark_symbol: String,
    start_date: String,
    end_date: String,
    interval: String,
    confidence_level: f64,
    risk_free_rate: f64,
    objective_function: String,
    chart_num: usize,
) -> Result<String, ServerFnError> {
    let chart = match PortfolioBuilder::new()
        .ticker_symbols(symbols.iter().map(|x| x.as_str()).collect())
        .benchmark_symbol(&benchmark_symbol)
        .start_date(&start_date)
        .end_date(&end_date)
        .interval(Interval::from_str(&interval))
        .confidence_level(confidence_level)
        .risk_free_rate(risk_free_rate)
        .objective_function(ObjectiveFunction::from_str(&objective_function))
        .build().await {
        Ok(pc) => match chart_num {
            1 => pc.optimization_chart(Some(800), Some(1200)).map(|oc| oc.to_html()).unwrap_or("".to_string()),
            2 => pc.performance_chart(Some(800), Some(1200)).map(|pc| pc.to_html()).unwrap_or("".to_string()),
            3 => pc.performance_stats_table(Some(800), Some(1200)).map(|ps| ps.to_html()).unwrap_or("".to_string()),
            4 => pc.asset_returns_chart(Some(800), Some(1200)).map(|arc| arc.to_html()).unwrap_or("".to_string()),
            _ => "".to_string(),
        },
        Err(_) => "".to_string(),
    };

    let chart = extract_javascript(chart);

    Ok(chart)
}


#[server]
pub async fn get_ticker_chart(
    symbol: String,
    start_date: String,
    end_date: String,
    interval: String,
    benchmark_symbol: String,
    confidence_level: f64,
    risk_free_rate: f64,
    chart_num: usize,
) -> Result<String, ServerFnError> {
    let tc = TickerBuilder::new()
        .ticker(&symbol)
        .start_date(&start_date)
        .end_date(&end_date)
        .interval(Interval::from_str(&interval))
        .benchmark_symbol(&benchmark_symbol)
        .confidence_level(confidence_level)
        .risk_free_rate(risk_free_rate)
        .build();

    let chart = match chart_num {
            1 => tc.candlestick_chart(Some(800), Some(1200)).await.map(|cc| cc.to_html()).unwrap_or("".to_string()),
            2 => tc.summary_stats_table(Some(800), Some(1200)).await.map(|ss| ss.to_html()).unwrap_or("".to_string()),
            3 => tc.performance_chart(Some(800), Some(1200)).await.map(|pc| pc.to_html()).unwrap_or("".to_string()),
            4 => tc.performance_stats_table(Some(800), Some(1200)).await.map(|ps| ps.to_html()).unwrap_or("".to_string()),
            5 => tc.financials_tables(Some(800), Some(1200)).await.map(|ft| ft["Income Statement"].to_html()).unwrap_or("".to_string()),
            6 => tc.financials_tables(Some(800), Some(1200)).await.map(|ft| ft["Balance Sheet"].to_html()).unwrap_or("".to_string()),
            7 => tc.financials_tables(Some(800), Some(1200)).await.map(|ft| ft["Cashflow Statement"].to_html()).unwrap_or("".to_string()),
            8 => tc.financials_tables(Some(800), Some(1200)).await.map(|ft| ft["Financial Ratios"].to_html()).unwrap_or("".to_string()),
            9 => tc.options_charts(Some(800), Some(1200)).await.map(|vc| vc["Volatility Surface"].to_html()).unwrap_or("".to_string()),
            _ => "".to_string(),
        };


    let chart = extract_javascript(chart);

    Ok(chart)
}


#[server]
pub async fn highlight_code(code: String, lang: String) -> Result<String, ServerFnError> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ss.find_syntax_by_extension(&lang).unwrap();
    let code = highlighted_html_for_string(&code, &ss, syntax, &ts.themes["base16-ocean.dark"]).unwrap();
    Ok(code)
}

#[server]
pub async fn save_code_images() -> Result<(), ServerFnError> {

    let _ = TickerBuilder::new()
        .ticker("AAPL")
        .start_date("2020-01-01")
        .end_date("2024-01-01")
        .interval(Interval::OneDay)
        .benchmark_symbol("^GSPC")
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .build().performance_chart(Some(800), Some(1200)).await.unwrap()
        .to_png("./public/images/ticker_chart.png", 800, 600, 1.0);


    let _ = PortfolioBuilder::new()
        .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
        .benchmark_symbol("^GSPC")
        .start_date("2020-01-01")
        .end_date("2024-01-01")
        .interval(Interval::OneDay)
        .confidence_level(0.95)
        .risk_free_rate(0.02)
        .objective_function(ObjectiveFunction::MaxSharpe)
        .build().await.unwrap().optimization_chart(Some(800), Some(1200)).unwrap()
        .to_png("./public/images/portfolio_chart.png", 800, 600, 1.0);

    Ok(())

}


fn extract_javascript(html: String) -> String {
    let script = html.replace("<!doctype html>", "");
    let script = script.replace("\n<html lang=\"en\">\n\n<head>\n    <meta charset=\"utf-8\" />\n</head>\n\n<body>\n    <div>\n        <script src=\"https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js\"></script>\n        <script src=\"https://cdn.plot.ly/plotly-2.12.1.min.js\"></script>\n        <div id=\"plotly-html-element\" class=\"plotly-graph-div\" style=\"height:100%; width:100%;\"></div>\n\n        <script type=\"module\">\n            ", "");
    let script = script.replace("\n        </script>\n    </div>\n</body>\n\n</html>", "");
    script
}
