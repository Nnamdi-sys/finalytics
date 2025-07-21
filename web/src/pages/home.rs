use dioxus::prelude::*;
use std::collections::HashMap;
use crate::components::footer::Footer;
use crate::server::highlight_code;

static TICKER_IMAGE: Asset = asset!("/public/images/ticker.png");
static PORTFOLIO_IMAGE: Asset = asset!("/public/images/portfolio.png");

#[component]
pub fn Home() -> Element {

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

    let mut code_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut ticker_map: HashMap<String, String> = HashMap::new();
    ticker_map.insert("rs".to_string(), ticker_rs);
    ticker_map.insert("py".to_string(), ticker_py);
    let mut portfolio_map: HashMap<String, String> = HashMap::new();
    portfolio_map.insert("rs".to_string(), portfolio_rs);
    portfolio_map.insert("py".to_string(), portfolio_py);
    code_map.insert("ticker".to_string(), ticker_map);
    code_map.insert("portfolio".to_string(), portfolio_map);

    let mut image_links: HashMap<String, String> = HashMap::new();
    image_links.insert("ticker".to_string(), "https://finalytics.rs/ticker".to_string());
    image_links.insert("portfolio".to_string(), "https://finalytics.rs/portfolio".to_string());

    let mut code_links: HashMap<String, String> = HashMap::new();
    code_links.insert("rs".to_string(), "https://crates.io/crates/finalytics".to_string());
    code_links.insert("py".to_string(), "https://pypi.org/project/finalytics".to_string());


    let mut example_type_state = use_signal(|| "ticker".to_string());
    let mut language_state = use_signal(|| "rs".to_string());

    rsx! {
        // Header section
        div {
            style: "padding: 10px; margin-top: 10px; margin-bottom: 0px; \
                    font-family: 'Poppins', sans-serif; text-align: center;",
            
            // Main header with gradient text
            h1 {
                style: "font-size: 40px; font-weight: 700; margin-bottom: 20px; \
                        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3); \
                        background: linear-gradient(90deg, rgba(46,125,50,1) 0%, rgba(76,175,80,1) 50%, rgba(46,125,50,1) 100%); \
                        -webkit-background-clip: text; \
                        -webkit-text-fill-color: transparent; \
                        display: inline-block;",
                "Financial Analytics Powered by Rust"
            }
            
            // Download badges
            div {
                style: "text-align: center; margin-bottom: 20px;",
                a {
                    href: "https://crates.io/crates/finalytics",
                    target: "_blank",
                    style: "text-decoration: none; display: inline-flex; align-items: center; gap: 10px; margin: 0 10px;",
                    img {
                        style: "height: 20px;",
                        src: "https://img.shields.io/crates/v/finalytics",
                        alt: "Crates.io version"
                    }
                    img {
                        style: "height: 20px;",
                        src: "https://img.shields.io/crates/d/finalytics?color=orange",
                        alt: "Crates.io downloads"
                    }
                }
                a {
                    href: "https://pypi.org/project/finalytics",
                    target: "_blank",
                    style: "text-decoration: none; display: inline-flex; align-items: center; gap: 10px; margin: 0 10px;",
                    img {
                        style: "height: 20px;",
                        src: "https://img.shields.io/pypi/v/finalytics?color=blue",
                        alt: "PyPI version"
                    }
                    img {
                        style: "height: 20px;",
                        src: "https://static.pepy.tech/badge/finalytics",
                        alt: "PyPI downloads"
                    }
                }
            }
            
            // Description
            p {
                style: "font-size: 20px; color: #424242; line-height: 1.6; text-align: center; margin: 10px;",
                "Finalytics leverages Rust's high performance with a Python integration, offering robust tools for financial data analysis."
            }
        }
        
        // Main content section
        div {
            style: "padding: 10px; margin-top: 10px; margin-bottom: 0px;",
            
            // Flex container for responsive layout
            div {
                style: "display: flex; flex-direction: column; gap: 20px; \
                        @media (min-width: 768px); flex-direction: row;",
                
                // Code container
                div {
                    style: "flex: 1; border: 1px solid #ccc; padding: 10px; \
                            background-color: #2b303b; color: white; \
                            display: flex; flex-direction: column;",
                    
                    // Language tabs
                    div {
                        style: "display: flex; gap: 5px; margin-bottom: 10px;",
                        
                        // Rust button
                        button {
                            style: if *language_state.read() == "rs" {
                                "padding: 5px 10px; border: none; cursor: pointer; \
                                 font-weight: bold; color: #fff; font-size: 14px; \
                                 border-radius: 4px; background-color: #2E7D32;"
                            } else {
                                "padding: 5px 10px; border: none; cursor: pointer; \
                                 font-weight: bold; color: #fff; font-size: 14px; \
                                 border-radius: 4px; background-color: #333;"
                            },
                            onclick: move |_| language_state.set("rs".to_string()),
                            i { class: "devicon-rust-plain me-2" },
                            "Rust"
                        }
                        
                        // Python button
                        button {
                            style: if *language_state.read() == "py" {
                                "padding: 5px 10px; border: none; cursor: pointer; \
                                 font-weight: bold; color: #fff; font-size: 14px; \
                                 border-radius: 4px; background-color: #2E7D32;"
                            } else {
                                "padding: 5px 10px; border: none; cursor: pointer; \
                                 font-weight: bold; color: #fff; font-size: 14px; \
                                 border-radius: 4px; background-color: #333;"
                            },
                            onclick: move |_| language_state.set("py".to_string()),
                            i { class: "devicon-python-plain me-2" },
                            "Python"
                        }
                    }
                    
                    // Code content
                    div {
                        style: "overflow-x: auto;",
                        pre {
                            style: "margin: 0;",
                            a {
                                href: "{code_links[&*language_state.read()].clone()}",
                                target: "_blank",
                                style: "text-decoration: none; color: inherit;",
                                dangerous_inner_html: "{code_map[&*example_type_state.read()][&*language_state.read()]}"
                            }
                        }
                    }
                }
                
                // Image container
                div {
                    style: "flex: 1; text-align: center; display: flex; flex-direction: column;",
                    
                    // Example type tabs
                    div {
                        style: "display: flex; gap: 5px; margin-bottom: 10px;",
                        
                        // Ticker button
                        button {
                            style: if *example_type_state.read() == "ticker" {
                                "padding: 5px 10px; border: none; cursor: pointer; \
                                 font-weight: bold; color: #2E7D32; font-size: 16px; \
                                 background-color: #d0d0d0;"
                            } else {
                                "padding: 5px 10px; border: none; cursor: pointer; \
                                 font-weight: bold; color: #2E7D32; font-size: 16px; \
                                 background-color: #f0f0f0;"
                            },
                            onclick: move |_| example_type_state.set("ticker".to_string()),
                            i { class: "bi bi-graph-up me-2" },
                            "Ticker"
                        }
                        
                        // Portfolio button
                        button {
                            style: if *example_type_state.read() == "portfolio" {
                                "padding: 5px 10px; border: none; cursor: pointer; \
                                 font-weight: bold; color: #2E7D32; font-size: 16px; \
                                 background-color: #d0d0d0;"
                            } else {
                                "padding: 5px 10px; border: none; cursor: pointer; \
                                 font-weight: bold; color: #2E7D32; font-size: 16px; \
                                 background-color: #f0f0f0;"
                            },
                            onclick: move |_| example_type_state.set("portfolio".to_string()),
                            i { class: "bi bi-pie-chart me-2" },
                            "Portfolio"
                        }
                    }
                    
                    // Image
                    a {
                        href: "{image_links[&*example_type_state.read()].clone()}",
                        target: "_blank",
                        img {
                            src: if *example_type_state.read() == "ticker" { 
                                TICKER_IMAGE 
                            } else { 
                                PORTFOLIO_IMAGE 
                            },
                            style: "max-width: 100%; height: auto;",
                            alt: "Financial chart"
                        }
                    }
                }
            }
        }
        
        Footer {}
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