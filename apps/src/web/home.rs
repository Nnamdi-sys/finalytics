use dioxus::prelude::*;
use std::collections::HashMap;
use crate::web::footer::Footer;
use crate::web::server::highlight_code;
//se crate::server::server::save_code_images;


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

    /*let _ = use_server_future(move || async {
            save_code_images().await.unwrap()
        })?;*/

    let mut code_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut ticker_map: HashMap<String, String> = HashMap::new();
    ticker_map.insert("rs".to_string(), ticker_rs);
    ticker_map.insert("py".to_string(), ticker_py);
    let mut portfolio_map: HashMap<String, String> = HashMap::new();
    portfolio_map.insert("rs".to_string(), portfolio_rs);
    portfolio_map.insert("py".to_string(), portfolio_py);
    code_map.insert("ticker".to_string(), ticker_map);
    code_map.insert("portfolio".to_string(), portfolio_map);

    let mut image_map: HashMap<String, String> = HashMap::new();
    image_map.insert("ticker".to_string(), "images/ticker_chart.png".to_string());
    image_map.insert("portfolio".to_string(), "images/portfolio_chart.png".to_string());

    let mut image_links: HashMap<String, String> = HashMap::new();
    image_links.insert("ticker".to_string(), "https://finalytics.rs/ticker".to_string());
    image_links.insert("portfolio".to_string(), "https://finalytics.rs/portfolio".to_string());

    let mut code_links: HashMap<String, String> = HashMap::new();
    code_links.insert("rs".to_string(), "https://crates.io/crates/finalytics".to_string());
    code_links.insert("py".to_string(), "https://pypi.org/project/finalytics".to_string());


    let mut code_map_state = use_signal(|| "ticker".to_string());
    let mut image_map_state = use_signal(|| "ticker".to_string());
    let mut language_state = use_signal(|| "rs".to_string());


    rsx! {

        div {
            style: "padding: 10px; margin-top: 10px; margin-bottom: 0px; font-family: 'Poppins', sans-serif;", // Adding font-family for a beautiful font style

            h1 {
                class: "text-center",
                style: "font-size: 36px; font-weight: bold; color: #2E7D32; margin-bottom: 20px; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);",
                "A Rust Library for Financial Data Analysis"
            }

            p {
                class: "text-center",
                style: "font-size: 18px; color: #424242; line-height: 1.6; text-align: justify;",
                "Finalytics is a Rust library designed for retrieving financial data and performing various financial analysis tasks, including fundamental analysis, technical analysis, sentiment analysis, options pricing, and portfolio optimization. It also provides a simple and easy-to-use API in Python."
            }
        }


        div {
            style: "padding: 10px; margin-top: 10px; margin-bottom: 0px;",

            ul {
                class: "nav nav-tabs",
                li {
                    class: "nav-item",
                    button {
                        class: if &**code_map_state.read() == "ticker" { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            code_map_state.set("ticker".to_string());
                            image_map_state.set("ticker".to_string());
                        },
                        "Security Analysis"
                    }
                }
                li {
                    class: "nav-item",
                    button {
                        class: if &**code_map_state.read() == "portfolio" { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            code_map_state.set("portfolio".to_string());
                            image_map_state.set("portfolio".to_string());
                        },
                        "Portfolio Optimization"
                    }
                }
            }

            div {
                class: "tab-content",
                div {
                    class: "tab-pane fade show active",
                    ul {
                        class: "nav nav-tabs",
                        li {
                            class: "nav-item",
                            button {
                                class: if &**language_state.read() == "rs" { "nav-link active" } else { "nav-link" },
                                onclick: move |_| {
                                    language_state.set("rs".to_string());
                                },
                                "Rust"
                            }
                        }
                        li {
                            class: "nav-item",
                            button {
                                class: if &**language_state.read() == "py" { "nav-link active" } else { "nav-link" },
                                onclick: move |_| {
                                    language_state.set("py".to_string());
                                },
                                "Python"
                            }
                        }
                    }
                    div {
                        class: "tab-content",
                        div {
                            class: "tab-pane fade show active",
                            div {
                                class: "row", // Create a row to contain the tab content and chart side by side
                                div {
                                    class: "col-md-6 d-flex flex-column justify-content-center", // Set the column width for the code content
                                    padding: "10px",
                                    a {
                                        href: "{code_links[&*language_state.read()].clone()}",
                                        target: "_blank",
                                        style: "text-decoration: none;",
                                        dangerous_inner_html: "{code_map[&*code_map_state.read()][&*language_state.read()].clone()}"
                                    }
                                }
                                div {
                                    class: "col-md-6 d-flex flex-column justify-content-center", // Set the column width for the chart
                                    padding: "10px",
                                    a {
                                        href: "{image_links[&*image_map_state.read()].clone()}",
                                        target: "_blank",
                                        img {
                                            src: "{image_map[&*image_map_state.read()].clone()}",
                                            style: "max-width: 100%; height: auto;", // Added style for responsive image
                                        }
                                    }
                                }
                            }
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
        let ticker = TickerBuilder::new()
                                   .ticker("AAPL")?
                                   .start_date("2023-01-01")
                                   .end_date("2023-02-01")
                                   .interval(Interval::OneDay)
                                   .benchmark_symbol("^GSPC")
                                   .confidence_level(0.95)
                                   .risk_free_rate(0.02)
                                   .build()?;

        // Display Ticker Performance Chart
        let _ = tc.performance_chart().await?.show();

        Ok(())
    }
        "###.to_string();

    let ticker_py = r###"
    from finalytics import Ticker

    // Construct Ticker Object
    ticker = Ticker(symbol="AAPL")

    // Display Ticker Performance Chart
    ticker.display_performance_chart(
                                    start="2019-01-01",
                                    end="2023-12-31",
                                    interval="1d",
                                    benchmark="^GSPC",
                                    confidence_level=0.95,
                                    risk_free_rate=0.02,
                                    display_format="html"
                                    )
        "###.to_string();


    let portfolio_rs = r###"
    use std::error::Error;
    use finalytics::prelude::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {

        // Construct Portfolio Object
        let portfolio = PortfolioBuilder::new()
                                        .ticker_symbols(vec!["AAPL", "MSFT", "NVDA", "BTC-USD"])
                                        .benchmark_symbol("^GSPC")
                                        .start_date("2017-01-01")
                                        .end_date("2023-01-01")
                                        .interval(Interval::OneDay)
                                        .confidence_level(0.95)
                                        .risk_free_rate(0.02)
                                        .max_iterations(1000)
                                        .objective_function(ObjectiveFunction::MaxSharpe)
                                        .build().await?;

        // Display Portfolio Optimization Chart
        let _ = pc.optimization_chart()?.show();

        Ok(())
    }
        "###.to_string();


    let portfolio_py = r###"
    from finalytics import Portfolio

    // Construct Portfolio Object
    portfolio = Portfolio(symbols=["AAPL", "MSFT", "NVDA", "BTC-USD"],
                            benchmark_symbol="^GSPC",
                            start_date="2019-01-01",
                            end_date="2023-12-31",
                            interval="1d",
                            confidence_level=0.95,
                            risk_free_rate=0.02,
                            max_iterations=1000,
                            objective_function="max_sharpe")

    // Display Portfolio Optimization Chart
    portfolio.display_optimization_chart(display_format="html")
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