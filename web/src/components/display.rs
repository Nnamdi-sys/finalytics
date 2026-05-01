use crate::components::chart::ChartContainer;
use crate::components::table::TableContainer;
use crate::components::utils::Loading;
use crate::server::OptionsTabs;
use crate::server::PerformanceTabs;
use crate::server::{FinancialsTabs, NewsTabs, PortfolioTabs};
use dioxus::document::eval;
use dioxus::prelude::*;

#[component]
pub fn PortfolioDisplay(
    charts: Resource<PortfolioTabs>,
    weight_mode: Signal<Option<String>>,
) -> Element {
    rsx! {
        script {
            r#"
                function scrollToElement(id) {{
                    const element = document.getElementById(id);
                    if (element) {{
                        element.scrollIntoView({{ behavior: 'smooth' }});
                    }}
                }}
            "#
        }

        // Wrap everything in a vertical flex layout
        div {
            class: "display-wrapper",

            // Horizontal Sticky Nav Bar
            nav {
                class: "display-nav",
                div {
                    class: "display-nav-buttons",

                    if weight_mode.read().is_none() || weight_mode.read().as_deref() == Some("weights") {
                        button {
                            class: "nav-link",
                            onclick: move |_| {
                                eval(r#"window.scrollToElement('optimization_chart');"#);
                            },
                            i { class: "fas fa-chart-line", style: "font-size: 16px;" }
                            span { "Optimization Chart" }
                        }
                    }

                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('performance_chart');"#);
                        },
                        i { class: "fas fa-chart-bar", style: "font-size: 16px;" }
                        span { "Performance Chart" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('portfolio_value_chart');"#);
                        },
                        i { class: "fas fa-chart-line", style: "font-size: 16px;" }
                        span { "Portfolio Value" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('performance_stats_table');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Performance Stats" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('returns_table');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Returns Data" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('returns_chart');"#);
                        },
                        i { class: "fas fa-chart-area", style: "font-size: 16px;" }
                        span { "Returns Chart" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('returns_matrix');"#);
                        },
                        i { class: "fas fa-th", style: "font-size: 16px;" }
                        span { "Returns Matrix" }
                    }
                }
            }

            // Scrollable Content Below the Nav
            div {
                class: "display-content",

                match &*charts.value().read_unchecked() {
                    Some(charts) => rsx! {
                        // Optimization Chart
                        if let Some(html) = &charts.optimization_chart {
                            if weight_mode.read().is_none() || weight_mode.read().as_deref() == Some("weights") {
                                div {
                                    id: "optimization_chart",
                                    class: "display-chart-container",
                                    ChartContainer { html: html.clone(), id: "optimization_chart" }
                                }
                            }
                        }

                        // Performance Chart
                        div {
                            id: "performance_chart",
                            class: "display-chart-container",
                            ChartContainer { html: charts.performance_chart.clone(), id: "performance_chart" }
                        }

                        // Portfolio Value Chart
                        div {
                            id: "portfolio_value_chart",
                            class: "display-chart-container",
                            ChartContainer { html: charts.portfolio_value_chart.clone(), id: "portfolio_value_chart" }
                        }

                        // Performance Stats Table
                        div {
                            id: "performance_stats_table",
                            class: "display-table-container",
                            TableContainer { html: charts.performance_stats_table.clone(), title: "Performance Stats" }
                        }

                        // Returns Table
                        div {
                            id: "returns_table",
                            class: "display-table-container",
                            TableContainer { html: charts.returns_table.clone(), title: "Returns Data" }
                        }

                        // Returns Chart
                        div {
                            id: "returns_chart",
                            class: "display-chart-container",
                            ChartContainer { html: charts.returns_chart.clone(), id: "returns_chart" }
                        }

                        // Returns Matrix
                        div {
                            id: "returns_matrix",
                            class: "display-chart-container",
                            ChartContainer { html: charts.returns_matrix.clone(), id: "returns_matrix" }
                        }
                    },
                    _ => rsx! { Loading {} }
                }
            }
        }

        // Shared responsive styles
        DisplayStyles {}
    }
}

#[component]
pub fn PerformanceDisplay(charts: Resource<PerformanceTabs>) -> Element {
    rsx! {
        script {
            r#"
                function scrollToElement(id) {{
                    const element = document.getElementById(id);
                    if (element) {{
                        element.scrollIntoView({{ behavior: 'smooth' }});
                    }}
                }}
            "#
        }

        // Wrap everything in a vertical flex layout
        div {
            class: "display-wrapper",

            // Horizontal Sticky Nav Bar
            nav {
                class: "display-nav",
                div {
                    class: "display-nav-buttons",
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('ohlcv_table');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Price Data" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('candlestick_chart');"#);
                        },
                        i { class: "fas fa-chart-bar", style: "font-size: 16px;" }
                        span { "Candlestick Chart" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('performance_chart');"#);
                        },
                        i { class: "fas fa-chart-line", style: "font-size: 16px;" }
                        span { "Performance Chart" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('performance_stats_table');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Performance Stats" }
                    }
                }
            }

            // Scrollable Content Below the Nav
            div {
                class: "display-content",
                match &*charts.value().read_unchecked() {
                    Some(charts) => rsx! {
                        // OHLCV Table
                        div {
                            id: "ohlcv_table",
                            class: "display-table-container",
                            TableContainer { html: charts.ohlcv_table.clone(), title: "Price Data" }
                        }
                        // Candlestick Chart
                        div {
                            id: "candlestick_chart",
                            class: "display-chart-container",
                            ChartContainer { html: charts.candlestick_chart.clone(), id: "candlestick_chart" }
                        }
                        // Performance Chart
                        div {
                            id: "performance_chart",
                            class: "display-chart-container",
                            ChartContainer { html: charts.performance_chart.clone(), id: "performance_chart" }
                        }
                        // Performance Stats Table
                        div {
                            id: "performance_stats_table",
                            class: "display-table-container",
                            TableContainer { html: charts.performance_stats_table.clone(), title: "Performance Stats" }
                        }
                    },
                    _ => rsx! { Loading {} }
                }
            }
        }

        // Shared responsive styles
        DisplayStyles {}
    }
}

#[component]
pub fn OptionsDisplay(charts: Resource<OptionsTabs>) -> Element {
    rsx! {
        script {
            r#"
                function scrollToElement(id) {{
                    const element = document.getElementById(id);
                    if (element) {{
                        element.scrollIntoView({{ behavior: 'smooth' }});
                    }}
                }}
            "#
        }

        // Wrap everything in a vertical flex layout
        div {
            class: "display-wrapper",

            // Horizontal Sticky Nav Bar
            nav {
                class: "display-nav",
                div {
                    class: "display-nav-buttons",
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('options_chain');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Options Chain" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('volatility_surface_table');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Volatility Surface Data" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('volatility_smile');"#);
                        },
                        i { class: "fas fa-chart-line", style: "font-size: 16px;" }
                        span { "Volatility Smile Chart" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('volatility_term_structure');"#);
                        },
                        i { class: "fas fa-chart-line", style: "font-size: 16px;" }
                        span { "Volatility Skew Chart" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('volatility_surface_chart');"#);
                        },
                        i { class: "fas fa-chart-area", style: "font-size: 16px;" }
                        span { "Volatility Surface Chart" }
                    }
                }
            }

            // Scrollable Content Below the Nav
            div {
                class: "display-content",
                match &*charts.value().read_unchecked() {
                    Some(charts) => rsx! {
                        // Options Chain Table
                        div {
                            id: "options_chain",
                            class: "display-table-container",
                            TableContainer { html: charts.options_chain.clone(), title: "Options Chain" }
                        }
                        // Volatility Surface Table
                        div {
                            id: "volatility_surface_table",
                            class: "display-table-container",
                            TableContainer { html: charts.volatility_surface_table.clone(), title: "Volatility Surface Data" }
                        }
                        // Volatility Smile Chart
                        div {
                            id: "volatility_smile",
                            class: "display-chart-container",
                            ChartContainer { html: charts.volatility_smile.clone(), id: "volatility_smile" }
                        }
                        // Volatility Term Structure Chart
                        div {
                            id: "volatility_term_structure",
                            class: "display-chart-container",
                            ChartContainer { html: charts.volatility_term_structure.clone(), id: "volatility_term_structure" }
                        }
                        // Volatility Surface Chart
                        div {
                            id: "volatility_surface_chart",
                            class: "display-chart-container",
                            ChartContainer { html: charts.volatility_surface_chart.clone(), id: "volatility_surface_chart" }
                        }
                    },
                    _ => rsx! { Loading {} }
                }
            }
        }

        // Shared responsive styles
        DisplayStyles {}
    }
}

#[component]
pub fn NewsDisplay(charts: Resource<NewsTabs>) -> Element {
    rsx! {
        script {
            r#"
                function scrollToElement(id) {{
                    const element = document.getElementById(id);
                    if (element) {{
                        element.scrollIntoView({{ behavior: 'smooth' }});
                    }}
                }}
            "#
        }

        // Wrap everything in a vertical flex layout
        div {
            class: "display-wrapper",

            // Horizontal Sticky Nav Bar
            nav {
                class: "display-nav",
                div {
                    class: "display-nav-buttons",
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('news_sentiment_table');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "News Sentiment Data" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('news_sentiment_chart');"#);
                        },
                        i { class: "fas fa-chart-bar", style: "font-size: 16px;" }
                        span { "News Sentiment Chart" }
                    }
                }
            }

            // Scrollable Content Below the Nav
            div {
                class: "display-content",
                match &*charts.value().read_unchecked() {
                    Some(charts) => rsx! {
                        // News Sentiment Table
                        div {
                            id: "news_sentiment_table",
                            class: "display-table-container",
                            TableContainer { html: charts.news_sentiment_table.clone(), title: "News Sentiment Data" }
                        }
                        // News Sentiment Chart
                        div {
                            id: "news_sentiment_chart",
                            class: "display-chart-container",
                            ChartContainer { html: charts.news_sentiment_chart.clone(), id: "news_sentiment_chart" }
                        }
                    },
                    _ => rsx! { Loading {} }
                }
            }
        }

        // Shared responsive styles
        DisplayStyles {}
    }
}

#[component]
pub fn FinancialsDisplay(charts: Resource<FinancialsTabs>) -> Element {
    rsx! {
        script {
            r#"
                function scrollToElement(id) {{
                    const element = document.getElementById(id);
                    if (element) {{
                        element.scrollIntoView({{ behavior: 'smooth' }});
                    }}
                }}
            "#
        }

        // Wrap everything in a vertical flex layout
        div {
            class: "display-wrapper",

            // Horizontal Sticky Nav Bar
            nav {
                class: "display-nav",
                div {
                    class: "display-nav-buttons",
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('income_statement');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Income Statement" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('balance_sheet');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Balance Sheet" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('cashflow_statement');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Cash Flow Statement" }
                    }
                    button {
                        class: "nav-link",
                        onclick: move |_| {
                            eval(r#"window.scrollToElement('financial_ratios');"#);
                        },
                        i { class: "fas fa-table", style: "font-size: 16px;" }
                        span { "Financial Ratios" }
                    }
                }
            }

            // Scrollable Content Below the Nav
            div {
                class: "display-content",
                match &*charts.value().read_unchecked() {
                    Some(charts) => rsx! {
                        // Income Statement Table
                        div {
                            id: "income_statement",
                            class: "display-table-container",
                            TableContainer { html: charts.income_statement.clone(), title: "Income Statement" }
                        }
                        // Balance Sheet Table
                        div {
                            id: "balance_sheet",
                            class: "display-table-container",
                            TableContainer { html: charts.balance_sheet.clone(), title: "Balance Sheet" }
                        }
                        // Cash Flow Statement Table
                        div {
                            id: "cashflow_statement",
                            class: "display-table-container",
                            TableContainer { html: charts.cashflow_statement.clone(), title: "Cash Flow Statement" }
                        }
                        // Financial Ratios Table
                        div {
                            id: "financial_ratios",
                            class: "display-table-container",
                            TableContainer { html: charts.financial_ratios.clone(), title: "Financial Ratios" }
                        }
                    },
                    _ => rsx! { Loading {} }
                }
            }
        }

        // Shared responsive styles
        DisplayStyles {}
    }
}

/// Shared responsive CSS for all display components.
/// Rendered as a `<style>` block — browsers de-duplicate identical style blocks,
/// so it is safe (and simple) to include this from every display component.
#[component]
fn DisplayStyles() -> Element {
    rsx! {
        style { r#"
            /* ========== Display Wrapper ========== */
            .display-wrapper {{
                display: flex;
                flex-direction: column;
                width: 100%;
            }}

            /* ========== Sticky Nav Bar ========== */
            .display-nav {{
                position: sticky;
                top: 0;
                z-index: 1000;
                background-color: #ffffff;
                padding: 12px 16px;
                border-bottom: 1px solid #e0e0e0;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
            }}

            .display-nav-buttons {{
                display: flex;
                flex-wrap: wrap;
                gap: 12px;
            }}

            .display-nav-buttons .nav-link {{
                display: inline-flex;
                align-items: center;
                gap: 6px;
                white-space: nowrap;
            }}

            /* ========== Content Area ========== */
            .display-content {{
                width: 100%;
                padding: 16px;
                display: flex;
                flex-direction: column;
                gap: 16px;
                background-color: #ffffff;
                box-sizing: border-box;
            }}

            /* ========== Chart Containers ========== */
            .display-chart-container {{
                width: 100%;
                height: 800px;
                border: 1px solid #e0e0e0;
                border-radius: 6px;
                padding: 8px;
                overflow: hidden;
                box-sizing: border-box;
            }}

            /* ========== Table Containers ========== */
            .display-table-container {{
                width: 100%;
                border: 1px solid #e0e0e0;
                border-radius: 6px;
                padding: 8px;
                overflow: auto;
                box-sizing: border-box;
            }}

            /* ========== Tablet (≤ 768px) ========== */
            @media (max-width: 768px) {{
                .display-nav {{
                    padding: 8px 10px;
                }}

                .display-nav-buttons {{
                    gap: 6px;
                }}

                .display-nav-buttons .nav-link {{
                    font-size: 13px;
                    padding: 6px 8px;
                    gap: 4px;
                }}

                .display-nav-buttons .nav-link i {{
                    font-size: 14px !important;
                }}

                /* On mobile, hide label text and show only icons to save space */
                .display-nav-buttons .nav-link span {{
                    display: none;
                }}

                .display-content {{
                    padding: 10px 6px;
                    gap: 12px;
                }}

                .display-chart-container {{
                    height: 450px;
                    padding: 4px;
                }}

                .display-table-container {{
                    padding: 4px;
                }}
            }}

            /* ========== Small Phones (≤ 480px) ========== */
            @media (max-width: 480px) {{
                .display-nav {{
                    padding: 6px 6px;
                }}

                .display-nav-buttons {{
                    gap: 4px;
                    justify-content: center;
                }}

                .display-nav-buttons .nav-link {{
                    font-size: 12px;
                    padding: 6px 6px;
                }}

                .display-content {{
                    padding: 8px 4px;
                    gap: 10px;
                }}

                .display-chart-container {{
                    height: 350px;
                    border-radius: 4px;
                }}

                .display-table-container {{
                    border-radius: 4px;
                }}
            }}
        "# }
    }
}
