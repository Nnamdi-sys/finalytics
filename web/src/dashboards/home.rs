use dioxus::prelude::*;
use std::collections::HashMap;
use crate::server::highlight_code;

static TICKER_IMAGE: Asset = asset!("/public/images/ticker.png");
static PORTFOLIO_IMAGE: Asset = asset!("/public/images/portfolio.png");

#[component]
pub fn Home() -> Element {
    // Fetch and highlight code examples
    let ticker_rs = use_server_future(|| async {
        let code = get_code_examples("ticker_rs".to_string());
        highlight_code(code, "rs".to_string()).await.unwrap()
    })?.value().read().clone().unwrap();

    let ticker_py = use_server_future(|| async {
        let code = get_code_examples("ticker_py".to_string());
        highlight_code(code, "py".to_string()).await.unwrap()
    })?.value().read().clone().unwrap();

    let portfolio_rs = use_server_future(|| async {
        let code = get_code_examples("portfolio_rs".to_string());
        highlight_code(code, "rs".to_string()).await.unwrap()
    })?.value().read().clone().unwrap();

    let portfolio_py = use_server_future(|| async {
        let code = get_code_examples("portfolio_py".to_string());
        highlight_code(code, "py".to_string()).await.unwrap()
    })?.value().read().clone().unwrap();

    // Assemble maps
    let mut code_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    code_map.insert("ticker".to_string(), {
        let mut m = HashMap::new();
        m.insert("rs".to_string(), ticker_rs.clone());
        m.insert("py".to_string(), ticker_py.clone());
        m
    });
    code_map.insert("portfolio".to_string(), {
        let mut m = HashMap::new();
        m.insert("rs".to_string(), portfolio_rs.clone());
        m.insert("py".to_string(), portfolio_py.clone());
        m
    });

    let mut image_links: HashMap<String, String> = HashMap::new();
    image_links.insert("ticker".to_string(), "https://finalytics.rs/ticker".to_string());
    image_links.insert("portfolio".to_string(), "https://finalytics.rs/portfolio".to_string());

    let mut code_links: HashMap<String, String> = HashMap::new();
    code_links.insert("rs".to_string(), "https://crates.io/crates/finalytics".to_string());
    code_links.insert("py".to_string(), "https://pypi.org/project/finalytics".to_string());

    // UI state
    let mut example_type = use_signal(|| "ticker".to_string());
    let mut language = use_signal(|| "rs".to_string());

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
                        gap: 10px;
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
                        gap: 10px;
                        margin: 0 10px;
                    "#,
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/pypi/v/finalytics?color=blue", alt: "PyPI version" }
                    img { style: r#"height: 20px;"#, src: "https://static.pepy.tech/badge/finalytics", alt: "PyPI downloads" }
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
                "Finalytics leverages Rust's high performance with a Python integration, offering robust tools for financial data analysis."
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
                // Tabs bar sits on same dark background
                div {
                    style: r#"
                        display: flex;
                        gap: 8px;
                        margin-bottom: 10px;
                        background: #2b303b;
                        padding: 10px;
                    "#,
                    // Rust tab
                    button {
                        style: if *language.read() == "rs" {
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
                    // Python tab
                    button {
                        style: if *language.read() == "py" {
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
                }
                // Code panel
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
                    pre {
                        style: r#"
                            margin: 0;
                            flex: 1;
                        "#,
                        dangerous_inner_html: "{code_map[&*example_type.read()][&*language.read()]}"
                    }
                    a {
                        href: "{code_links[&*language.read()].clone()}",
                        target: "_blank",
                        style: r#"
                            margin-top: 10px;
                            align-self: flex-end;
                            text-decoration: none;
                            color: #2E7D32;
                        "#,
                    }
                }
            }

            // Image card with tabs
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
                        style: if *example_type.read() == "ticker" {
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
                        style: if *example_type.read() == "portfolio" {
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
                    "#,
                    a {
                        href: "{image_links[&*example_type.read()].clone()}",
                        target: "_blank",
                        img {
                            style: r#"
                                width: 100%;
                                height: auto;
                                object-fit: contain;
                                border-radius: 4px;
                            "#,
                            src: if *example_type.read() == "ticker" { TICKER_IMAGE } else { PORTFOLIO_IMAGE },
                            alt: "Example Screenshot"
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

        // Construct Ticker Object
        let ticker = Ticker::builder()
                           .ticker("AAPL")
                           .start_date("2023-01-01")
                           .end_date("2024-12-31")
                           .interval(Interval::OneDay)
                           .benchmark_symbol("^GSPC")
                           .confidence_level(0.95)
                           .risk_free_rate(0.02)
                           .build();

        // Display Ticker Performance Chart
        ticker.performance_chart(800, 1200).await?.show();

        Ok(())
    }
        "###.to_string();

    let ticker_py = r###"
    from finalytics import Ticker

    // Construct Ticker Object
    ticker = Ticker(symbol="AAPL",
                    start_date="2023-01-01",
                    end_date="2024-12-31",
                    interval="1d",
                    benchmark="^GSPC",
                    confidence_level=0.95,
                    risk_free_rate=0.02)

    // Display Ticker Performance Chart
    ticker.performance_chart(height=800, width=1200).show()
        "###.to_string();


    let portfolio_rs = r###"
    use std::error::Error;
    use finalytics::prelude::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {
    
        // Define list of ticker symbols
        let ticker_symbols = vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"];

        // Construct Portfolio Object
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

        // Display Portfolio Optimization Chart
        portfolio.optimization_chart(800, 1200)?.show();

        Ok(())
    }
        "###.to_string();


    let portfolio_py = r###"
    from finalytics import Portfolio
    
    // Define list of ticker symbols
    symbols = ["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"]

    // Construct Portfolio Object
    portfolio = Portfolio(symbols=symbols,
                        benchmark_symbol="^GSPC",
                        start_date="2023-01-01",
                        end_date="2024-12-31",
                        interval="1d",
                        confidence_level=0.95,
                        risk_free_rate=0.02,
                        objective_function="max_sharpe")

    // Display Portfolio Optimization Chart
    portfolio.optimization_chart(height=800, width=1200).show()
        "###.to_string();

    let code = match category.as_str() {
        "ticker_rs" => ticker_rs,
        "ticker_py" => ticker_py,
        "portfolio_rs" => portfolio_rs,
        "portfolio_py" => portfolio_py,
        _ => "".to_string()
    };

    code
}