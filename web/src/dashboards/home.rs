use crate::components::chart::ChartContainer;
use crate::components::code::CodeContainer;
use dioxus::prelude::*;
use std::collections::HashMap;

static TICKER_HTML: &str = include_str!("../../public/html/ticker.html");
static PORTFOLIO_HTML: &str = include_str!("../../public/html/portfolio.html");

#[component]
pub fn Home() -> Element {
    // Build code maps for ticker and portfolio examples
    let ticker_codes: HashMap<String, String> = HashMap::from([
        ("rs".to_string(), get_code_examples("ticker_rs".to_string())),
        ("py".to_string(), get_code_examples("ticker_py".to_string())),
        ("go".to_string(), get_code_examples("ticker_go".to_string())),
        ("js".to_string(), get_code_examples("ticker_js".to_string())),
    ]);
    let portfolio_codes: HashMap<String, String> = HashMap::from([
        (
            "rs".to_string(),
            get_code_examples("portfolio_rs".to_string()),
        ),
        (
            "py".to_string(),
            get_code_examples("portfolio_py".to_string()),
        ),
        (
            "go".to_string(),
            get_code_examples("portfolio_go".to_string()),
        ),
        (
            "js".to_string(),
            get_code_examples("portfolio_js".to_string()),
        ),
    ]);

    // UI state
    let mut example_type = use_signal(|| "ticker".to_string());
    let active_lang = use_signal(|| "rs".to_string());

    rsx! {
        // Header section
        div {
            class: "home-header",
            h1 {
                class: "home-title",
                "Financial Analytics Powered by Rust"
            }
            div {
                class: "home-badges",
                a {
                    href: "https://crates.io/crates/finalytics",
                    target: "_blank",
                    class: "home-badge-link",
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/crates/v/finalytics", alt: "Crates.io version" }
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/crates/d/finalytics?color=orange", alt: "Crates.io downloads" }
                }
                a {
                    href: "https://pypi.org/project/finalytics",
                    target: "_blank",
                    class: "home-badge-link",
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/pypi/v/finalytics?color=blue", alt: "PyPI version" }
                    img { style: r#"height: 20px;"#, src: "https://static.pepy.tech/badge/finalytics", alt: "PyPI downloads" }
                }
                a {
                    href: "https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics",
                    target: "_blank",
                    class: "home-badge-link",
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/badge/go-reference-blue?logo=go", alt: "Go Reference" }
                }
                a {
                    href: "https://www.npmjs.com/package/finalytics",
                    target: "_blank",
                    class: "home-badge-link",
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/npm/v/finalytics?color=green", alt: "NPM version" }
                }
                a {
                    href: "https://github.com/Nnamdi-sys/finalytics",
                    target: "_blank",
                    class: "home-badge-link",
                    img { style: r#"height: 20px;"#, src: "https://img.shields.io/github/stars/Nnamdi-sys/finalytics?style=social", alt: "GitHub stars" }
                }
            }
            p {
                class: "home-description",
                "Finalytics is a high-performance Rust library for security analysis and portfolio optimization, with bindings for Python, Node.js and Go."
            }
        }

        // Examples container
        div {
            class: "home-examples",

            // Code container — language tabs are built into the component header
            div {
                class: "home-card home-card--code",
                {
                    let codes = if example_type() == "portfolio" {
                        portfolio_codes.clone()
                    } else {
                        ticker_codes.clone()
                    };
                    let key = format!("code-{}", example_type());
                    rsx! {
                        CodeContainer {
                            key: "{key}",
                            codes: codes,
                            id: "code-example",
                            active_lang: active_lang
                        }
                    }
                }
            }

            // Chart card with tabs
            div {
                class: "home-card home-card--chart",
                div {
                    class: "home-example-buttons",
                    button {
                        class: if example_type() == "ticker" { "home-example-btn home-example-btn--active" } else { "home-example-btn" },
                        onclick: move |_| example_type.set("ticker".to_string()),
                        i { class: "bi bi-graph-up me-2" },
                        span { "Ticker" }
                    }
                    button {
                        class: if example_type() == "portfolio" { "home-example-btn home-example-btn--active" } else { "home-example-btn" },
                        onclick: move |_| example_type.set("portfolio".to_string()),
                        i { class: "bi bi-pie-chart me-2" },
                        span { "Portfolio" }
                    }
                }
                div {
                    class: "home-chart-body",
                    div {
                        class: "home-chart-inner",
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

        // Home page responsive styles
        HomeStyles {}
    }
}

/// Shared responsive CSS for the home page.
#[component]
fn HomeStyles() -> Element {
    rsx! {
        style { r#"
            /* ========== Header ========== */
            .home-header {{
                padding: 10px;
                margin-top: 10px;
                margin-bottom: 0;
                font-family: 'Poppins', sans-serif;
                text-align: center;
            }}

            .home-title {{
                font-size: clamp(22px, 5vw, 40px);
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
                line-height: 1.3;
            }}

            .home-badges {{
                text-align: center;
                margin-bottom: 20px;
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
                gap: 8px 16px;
            }}

            .home-badge-link {{
                text-decoration: none;
                display: inline-flex;
                align-items: center;
                gap: 2px;
            }}

            .home-description {{
                font-size: clamp(14px, 3vw, 20px);
                color: #424242;
                line-height: 1.6;
                text-align: center;
                margin: 10px;
            }}

            /* ========== Examples Container ========== */
            .home-examples {{
                display: flex;
                flex-wrap: wrap;
                gap: 20px;
                padding: 10px;
                margin: 10px;
                box-sizing: border-box;
            }}

            /* ========== Cards (shared) ========== */
            .home-card {{
                flex: 1 1 calc(50% - 20px);
                min-width: 280px;
                border-radius: 8px;
                box-shadow: 0 2px 6px rgba(0,0,0,0.1);
                display: flex;
                flex-direction: column;
                box-sizing: border-box;
                overflow: hidden;
            }}

            .home-card--code {{
                background: #282c34;
                border: none;
                box-shadow: none;
                overflow: visible;
            }}

            .home-card--chart {{
                background: white;
            }}

            /* ========== Example Type Buttons (chart card) ========== */
            .home-example-buttons {{
                display: flex;
                flex-wrap: wrap;
                gap: 8px;
                margin-bottom: 10px;
                padding: 10px 10px 0 10px;
            }}

            .home-example-btn {{
                padding: 5px 10px;
                border: none;
                cursor: pointer;
                font-weight: bold;
                color: #2E7D32;
                background-color: #f0f0f0;
                display: inline-flex;
                align-items: center;
                gap: 4px;
                border-radius: 4px;
                font-size: 14px;
                white-space: nowrap;
            }}

            .home-example-btn--active {{
                background-color: #d0d0d0;
            }}

            /* ========== Chart Body ========== */
            .home-chart-body {{
                flex: 1;
                text-align: center;
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 10px;
                max-height: 800px;
                overflow-y: auto;
                min-height: 0;
            }}

            .home-chart-inner {{
                width: 100%;
                height: 100%;
                object-fit: contain;
                border-radius: 4px;
                display: block;
            }}

            /* ========== Tablet (<=768px) ========== */
            @media (max-width: 768px) {{
                .home-header {{
                    padding: 8px;
                    margin-top: 0;
                }}

                .home-title {{
                    margin-bottom: 12px;
                }}

                .home-badges {{
                    gap: 6px 10px;
                    margin-bottom: 12px;
                }}

                .home-badge-link img {{
                    height: 18px !important;
                }}

                .home-description {{
                    margin: 6px;
                }}

                .home-examples {{
                    gap: 14px;
                    padding: 6px;
                    margin: 6px;
                }}

                /* Stack cards vertically on tablet */
                .home-card {{
                    flex: 1 1 100%;
                    min-width: 0;
                }}

                .home-example-buttons {{
                    gap: 6px;
                    padding: 8px 8px 0 8px;
                }}

                .home-example-btn {{
                    padding: 4px 8px;
                    font-size: 12px;
                }}

                .home-chart-body {{
                    max-height: 60vh;
                    padding: 6px;
                }}
            }}

            /* ========== Small Phones (<=480px) ========== */
            @media (max-width: 480px) {{
                .home-header {{
                    padding: 6px;
                    margin-top: 0;
                }}

                .home-title {{
                    margin-bottom: 8px;
                }}

                .home-badges {{
                    gap: 4px 6px;
                    margin-bottom: 8px;
                }}

                .home-badge-link img {{
                    height: 16px !important;
                }}

                .home-description {{
                    margin: 4px;
                    line-height: 1.4;
                }}

                .home-examples {{
                    gap: 10px;
                    padding: 4px;
                    margin: 4px;
                }}

                .home-example-buttons {{
                    gap: 4px;
                    padding: 6px 6px 0 6px;
                }}

                .home-example-btn {{
                    padding: 4px 6px;
                    font-size: 11px;
                    gap: 2px;
                }}

                .home-chart-body {{
                    max-height: 55vh;
                    padding: 4px;
                }}
            }}
        "# }
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

        ticker.report(Some(ReportType::Performance))
        .await?.show()?;

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

        let ticker_symbols = vec!["NVDA", "GOOG", "AAPL",
        "MSFT", "BTC-USD"];

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

        portfolio.report(Some(ReportType::Performance))
        .await?.show()?;

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
