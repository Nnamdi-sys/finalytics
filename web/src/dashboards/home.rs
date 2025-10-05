use crate::components::chart::ChartContainer;
use crate::components::code::CodeContainer;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use std::collections::HashMap;

static TICKER_HTML: &str = include_str!("../../public/html/ticker.html");
static PORTFOLIO_HTML: &str = include_str!("../../public/html/portfolio.html");

#[component]
pub fn Home() -> Element {
    // Assemble maps
    let mut code_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    code_map.insert("ticker".to_string(), {
        let mut m = HashMap::new();
        m.insert("rs".to_string(), get_code_examples("ticker_rs".to_string()));
        m.insert("py".to_string(), get_code_examples("ticker_py".to_string()));
        m.insert("go".to_string(), get_code_examples("ticker_go".to_string()));
        m.insert("js".to_string(), get_code_examples("ticker_js".to_string()));
        m
    });
    code_map.insert("portfolio".to_string(), {
        let mut m = HashMap::new();
        m.insert(
            "rs".to_string(),
            get_code_examples("portfolio_rs".to_string()),
        );
        m.insert(
            "py".to_string(),
            get_code_examples("portfolio_py".to_string()),
        );
        m.insert(
            "go".to_string(),
            get_code_examples("portfolio_go".to_string()),
        );
        m.insert(
            "js".to_string(),
            get_code_examples("portfolio_js".to_string()),
        );
        m
    });

    // UI state
    let mut example_type = use_signal(|| "ticker".to_string());
    let mut language = use_signal(|| "rs".to_string());

    info!("Example Type: {:?}", example_type());
    info!("Language: {:?}", language());

    rsx! {
        // Header section
        div {
            style: r#"
                padding: 10px;
                margin-top: 10px;
                margin-bottom: 0px;
                font-family: 'Poppins', sans-serif;
                text-align: center;
            "#,
            h1 {
                style: r#"
                    font-size: 40px;
                    font-weight: 700;
                    margin-bottom: 20px;
                    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
                    background: linear-gradient(
                        90deg,
                        rgba(46,125,50,1) 0%,
                        rgba(76,175,80,1) 50%,
                        rgba(46,125,50,1) 100%
                    );
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    display: inline-block;
                "#,
                "Financial Analytics Powered by Rust"
            }
            div {
                style: r#"
                    text-align: center;
                    margin-bottom: 20px;
                "#,
                a {
                    href: "https://crates.io/crates/finalytics",
                    target: "_blank",
                    style: r#"
                        text-decoration: none;
                        display: inline-flex;
                        align-items: center;
                        gap: 2px;
                        margin: 0 10px;
                    "#,
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/crates/v/finalytics", alt: "Crates.io version" }
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/crates/d/finalytics?color=orange", alt: "Crates.io downloads" }
                }
                a {
                    href: "https://pypi.org/project/finalytics",
                    target: "_blank",
                    style: r#"
                        text-decoration: none;
                        display: inline-flex;
                        align-items: center;
                        gap: 2px;
                        margin: 0 10px;
                    "#,
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/pypi/v/finalytics?color=blue", alt: "PyPI version" }
                    img { style: r#"height: 20px;"#, src: "https://static.pepy.tech/badge/finalytics", alt: "PyPI downloads" }
                }
                a {
                    href: "https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics",
                    target: "_blank",
                    style: r#"
                        text-decoration: none;
                        display: inline-flex;
                        align-items: center;
                        gap: 10px;
                        margin: 0 10px;
                    "#,
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/badge/go-reference-blue?logo=go", alt: "Go Reference" }
                }
                a {
                    href: "https://www.npmjs.com/package/finalytics",
                    target: "_blank",
                    style: r#"
                        text-decoration: none;
                        display: inline-flex;
                        align-items: center;
                        gap: 10px;
                        margin: 0 10px;
                    "#,
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/npm/v/finalytics?color=green", alt: "NPM version" }
                }
                a {
                    href: "https://github.com/Nnamdi-sys/finalytics",
                    target: "_blank",
                    style: r#"
                        text-decoration: none;
                        display: inline-flex;
                        align-items: center;
                        gap: 10px;
                        margin: 0 10px;
                    "#,
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/github/stars/Nnamdi-sys/finalytics?style=social", alt: "GitHub stars" }
                }
            }
            p {
                style: r#"
                    font-size: 20px;
                    color: #424242;
                    line-height: 1.6;
                    text-align: center;
                    margin: 10px;
                "#,
                "Finalytics is a high-performance Rust library for security analysis and portfolio optimization, with bindings for Python, Node.js and Go."
            }
        }

        // Examples container
        div {
            style: r#"
                display: flex;
                flex-wrap: wrap;
                gap: 20px;
                padding: 10px;
                margin: 10px;
            "#,

            // Code card with unified dark background
            div {
                style: r#"
                    flex: 1 1 calc(50% - 20px);
                    min-width: 300px;
                    background: #2b303b;
                    border-radius: 8px;
                    box-shadow: 0 2px 6px rgba(0,0,0,0.1);
                    display: flex;
                    flex-direction: column;
                "#,
                div {
                    style: r#"
                        display: flex;
                        gap: 8px;
                        margin-bottom: 10px;
                        background: #2b303b;
                        padding: 10px;
                    "#,
                    button {
                        style: if language() == "rs" {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #fff;
                                background-color: #2E7D32;
                            "#
                        } else {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #fff;
                                background-color: #333;
                            "#
                        },
                        onclick: move |_| language.set("rs".to_string()),
                        i { class: "devicon-rust-plain me-2" },
                        "Rust"
                    }
                    button {
                        style: if language() == "py" {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #fff;
                                background-color: #2E7D32;
                            "#
                        } else {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #fff;
                                background-color: #333;
                            "#
                        },
                        onclick: move |_| language.set("py".to_string()),
                        i { class: "devicon-python-plain me-2" },
                        "Python"
                    }
                    button {
                        style: if language() == "go" {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #fff;
                                background-color: #2E7D32;
                            "#
                        } else {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #fff;
                                background-color: #333;
                            "#
                        },
                        onclick: move |_| language.set("go".to_string()),
                        i { class: "devicon-go-plain me-2" },
                        "Go"
                    }
                    button {
                        style: if language() == "js" {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #fff;
                                background-color: #2E7D32;
                            "#
                        } else {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #fff;
                                background-color: #333;
                            "#
                        },
                        onclick: move |_| language.set("js".to_string()),
                        i { class: "devicon-javascript-plain me-2" },
                        "Node.js"
                    }
                }
                div {
                    style: r#"
                        flex: 1;
                        background: #2b303b;
                        color: white;
                        overflow-x: auto;
                        padding: 10px;
                        display: flex;
                        flex-direction: column;
                    "#,
                    {
                        let key = format!("code-{}-{}", example_type(), language());
                        match (example_type().as_str(), language().as_str()) {
                            ("ticker", "rs") => rsx! {
                                CodeContainer {
                                    key: "{key}",
                                    code: code_map["ticker"]["rs"].clone(),
                                    language: "rs".to_string(),
                                    id: "code-rs"
                                }
                            },
                            ("ticker", "py") => rsx! {
                                CodeContainer {
                                    key: "{key}",
                                    code: code_map["ticker"]["py"].clone(),
                                    language: "py".to_string(),
                                    id: "code-py"
                                }
                            },
                            ("ticker", "go") => rsx! {
                                CodeContainer {
                                    key: "{key}",
                                    code: code_map["ticker"]["go"].clone(),
                                    language: "go".to_string(),
                                    id: "code-go"
                                }
                            },
                            ("ticker", "js") => rsx! {
                                CodeContainer {
                                    key: "{key}",
                                    code: code_map["ticker"]["js"].clone(),
                                    language: "js".to_string(),
                                    id: "code-js"
                                }
                            },
                            ("portfolio", "rs") => rsx! {
                                CodeContainer {
                                    key: "{key}",
                                    code: code_map["portfolio"]["rs"].clone(),
                                    language: "rs".to_string(),
                                    id: "code-rs"
                                }
                            },
                            ("portfolio", "py") => rsx! {
                                CodeContainer {
                                    key: "{key}",
                                    code: code_map["portfolio"]["py"].clone(),
                                    language: "py".to_string(),
                                    id: "code-py"
                                }
                            },
                            ("portfolio", "go") => rsx! {
                                CodeContainer {
                                    key: "{key}",
                                    code: code_map["portfolio"]["go"].clone(),
                                    language: "go".to_string(),
                                    id: "code-go"
                                }
                            },
                            ("portfolio", "js") => rsx! {
                                CodeContainer {
                                    key: "{key}",
                                    code: code_map["portfolio"]["js"].clone(),
                                    language: "js".to_string(),
                                    id: "code-js"
                                }
                            },
                            _ => rsx! { div { "Error: Invalid code selection" } },
                        }
                    }
                }
            }

            // Chart card with tabs
            div {
                style: r#"
                    flex: 1 1 calc(50% - 20px);
                    min-width: 300px;
                    background: white;
                    border-radius: 8px;
                    box-shadow: 0 2px 6px rgba(0,0,0,0.1);
                    display: flex;
                    flex-direction: column;
                "#,
                div {
                    style: r#"
                        display: flex;
                        gap: 8px;
                        margin-bottom: 10px;
                    "#,
                    button {
                        style: if example_type() == "ticker" {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #2E7D32;
                                background-color: #d0d0d0;
                            "#
                        } else {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #2E7D32;
                                background-color: #f0f0f0;
                            "#
                        },
                        onclick: move |_| example_type.set("ticker".to_string()),
                        i { class: "bi bi-graph-up me-2" },
                        "Ticker"
                    }
                    button {
                        style: if example_type() == "portfolio" {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #2E7D32;
                                background-color: #d0d0d0;
                            "#
                        } else {
                            r#"
                                padding: 5px 10px;
                                border: none;
                                cursor: pointer;
                                font-weight: bold;
                                color: #2E7D32;
                                background-color: #f0f0f0;
                            "#
                        },
                        onclick: move |_| example_type.set("portfolio".to_string()),
                        i { class: "bi bi-pie-chart me-2" },
                        "Portfolio"
                    }
                }
                div {
                    style: r#"
                        flex: 1;
                        text-align: center;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        padding: 10px;
                        max-height: 800px; /* Match CodeContainer height */
                        overflow-y: auto; /* Enable scrolling */
                    "#,
                    div {
                        style: r#"
                            width: 100%;
                            height: 100%; /* Fill parent container */
                            object-fit: contain;
                            border-radius: 4px;
                            display: block; /* Ensure link fills container */
                        "#,
                        {
                            let key = format!("chart-{}", example_type());
                            match example_type().as_str() {
                                "ticker" => rsx! {
                                    ChartContainer {
                                        key: "{key}",
                                        html: TICKER_HTML.to_string(),
                                        id: "chart-ticker"
                                    }
                                },
                                "portfolio" => rsx! {
                                    ChartContainer {
                                        key: "{key}",
                                        html: PORTFOLIO_HTML.to_string(),
                                        id: "chart-portfolio"
                                    }
                                },
                                _ => rsx! { div { "Error: Invalid chart selection" } },
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn get_code_examples(category: String) -> String {
    let ticker_rs = r###"
    use std::error::Error;
    use finalytics::prelude::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {

        let ticker = Ticker::builder()
                           .ticker("AAPL")
                           .start_date("2023-01-01")
                           .end_date("2024-12-31")
                           .interval(Interval::OneDay)
                           .benchmark_symbol("^GSPC")
                           .confidence_level(0.95)
                           .risk_free_rate(0.02)
                           .build();

        ticker.report(Some(ReportType::Performance)).await?.show()?;

        Ok(())
    }
        "###
    .to_string();

    let ticker_py = r###"
    from finalytics import Ticker

    ticker = Ticker(symbol="AAPL",
                    start_date="2023-01-01",
                    end_date="2024-12-31",
                    interval="1d",
                    benchmark="^GSPC",
                    confidence_level=0.95,
                    risk_free_rate=0.02)

    ticker.report("performance").show()
        "###
    .to_string();

    let portfolio_rs = r###"
    use std::error::Error;
    use finalytics::prelude::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {

        let ticker_symbols = vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"];

        let portfolio = Portfolio::builder()
                           .ticker_symbols(ticker_symbols)
                           .benchmark_symbol("^GSPC")
                           .start_date("2023-01-01")
                           .end_date("2024-12-31")
                           .interval(Interval::OneDay)
                           .confidence_level(0.95)
                           .risk_free_rate(0.02)
                           .objective_function(ObjectiveFunction::MaxSharpe)
                           .build().await?;

        portfolio.report(Some(ReportType::Performance)).await?.show()?;

        Ok(())
    }
        "###
    .to_string();

    let portfolio_py = r###"
    from finalytics import Portfolio

    symbols = ["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"]

    portfolio = Portfolio(symbols=symbols,
                        benchmark_symbol="^GSPC",
                        start_date="2023-01-01",
                        end_date="2024-12-31",
                        interval="1d",
                        confidence_level=0.95,
                        risk_free_rate=0.02,
                        objective_function="max_sharpe")

    portfolio.report("performance").show()
        "###
    .to_string();

    let ticker_go = r###"
    import (
        "github.com/Nnamdi-sys/finalytics/go/finalytics"
    )

    ticker, err := finalytics.NewTickerBuilder().
        Symbol("AAPL").
        StartDate("2023-01-01").
        EndDate("2024-12-31").
        Interval("1d").
        BenchmarkSymbol("^GSPC").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        Build()
    if err != nil {
        panic(err)
    }

    defer ticker.Free()

    report, err := ticker.Report("performance")
    if err == nil {
        report.Show()
    }

    "###
    .to_string();

    let ticker_js = r###"
    import { TickerBuilder } from 'finalytics';

    const ticker = await new TickerBuilder()
        .symbol('AAPL')
        .startDate('2023-01-01')
        .endDate('2024-12-31')
        .interval('1d')
        .benchmarkSymbol('^GSPC')
        .confidenceLevel(0.95)
        .riskFreeRate(0.02)
        .build();

    const report = await ticker.report('performance');
    await report.show();

    ticker.free()

    "###
    .to_string();

    let portfolio_go = r###"
    import (
        "github.com/Nnamdi-sys/finalytics/go/finalytics"
    )

    symbols := []string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}

    portfolio, err := finalytics.NewPortfolioBuilder().
        Symbols(symbols).
        BenchmarkSymbol("^GSPC").
        StartDate("2023-01-01").
        EndDate("2024-12-31").
        Interval("1d").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        ObjectiveFunction("max_sharpe").
        Build()
    if err != nil {
        panic(err)
    }

    defer portfolio.Free()

    report, err := portfolio.Report("performance")
    if err == nil {
        report.Show()
    }

    "###
    .to_string();

    let portfolio_js = r###"
    import { PortfolioBuilder } from 'finalytics';

    const symbols = ['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'];

    const portfolio = await new PortfolioBuilder()
        .symbols(symbols)
        .benchmarkSymbol('^GSPC')
        .startDate('2023-01-01')
        .endDate('2024-12-31')
        .interval('1d')
        .confidenceLevel(0.95)
        .riskFreeRate(0.02)
        .objectiveFunction('max_sharpe')
        .build();

    const report = await portfolio.report('performance');
    await report.show();

    portfolio.free()
    "###
    .to_string();

    let code = match category.as_str() {
        "ticker_rs" => ticker_rs,
        "ticker_py" => ticker_py,
        "ticker_go" => ticker_go,
        "ticker_js" => ticker_js,
        "portfolio_rs" => portfolio_rs,
        "portfolio_py" => portfolio_py,
        "portfolio_go" => portfolio_go,
        "portfolio_js" => portfolio_js,
        _ => "".to_string(),
    };

    code
}
