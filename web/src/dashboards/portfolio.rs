use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::server::get_portfolio_charts;
use crate::components::utils::Loading;
use crate::components::symbols::{Symbol, Symbols};
use crate::components::chart::ChartContainer;
use crate::components::table::TableContainer;

#[component]
pub fn Portfolio() -> Element {
    let symbols = use_signal(|| "AAPL,MSFT,NVDA,BTC-USD".to_string());
    let mut benchmark_symbol = use_signal(|| "^GSPC".to_string());
    let mut start_date = use_signal(|| "2023-01-01".to_string());
    let mut end_date = use_signal(|| "2024-12-31".to_string());
    let mut interval = use_signal(|| "1d".to_string());
    let mut confidence_level = use_signal(|| 0.95);
    let mut risk_free_rate = use_signal(|| 0.02);
    let mut objective_function = use_signal(|| "max_sharpe".to_string());
    let mut active_tab = use_signal(|| 1);

    info!("symbols: {:?}", symbols());
    info!("benchmark: {:?}", benchmark_symbol());
    info!("start: {:?}", start_date());
    info!("end: {:?}", end_date());
    info!("interval: {:?}", interval());
    info!("confidence: {:?}", confidence_level());
    info!("risk_free: {:?}", risk_free_rate());
    info!("objective: {:?}", objective_function());
    info!("active_tab: {:?}", active_tab());

    let mut chart = use_server_future(move || async move {
        match get_portfolio_charts(
            symbols()
                .split(',')
                .map(str::to_string)
                .collect(),
            benchmark_symbol(),
            start_date(),
            end_date(),
            interval(),
            confidence_level(),
            risk_free_rate(),
            objective_function(),
            active_tab(),
        )
            .await
        {
            Ok(chart) => chart,
            Err(e) => format!("Error: {e}"),
        }
    })?;

    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                margin: 0;
                padding: 16px;
                background-color: #f5f5f5;
                gap: 16px;
                box-sizing: border-box;
                @media (max-width: 768px) {{
                    padding: 8px;
                    gap: 8px;
                }}
            "#,

            // Form Bar
            div {
                style: r#"
                    width: 100%;
                    background-color: #ffffff;
                    padding: 20px;
                    border-radius: 10px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    box-sizing: border-box;
                "#,
                div {
                    style: r#"
                        background-color: #f5f5f5;
                        padding: 20px;
                        border-radius: 10px;
                        box-sizing: border-box;
                    "#,
                    form {
                        style: r#"
                            display: flex;
                            flex-wrap: wrap;
                            gap: 16px;
                            align-items: flex-end;
                        "#,
                        onsubmit: move |e| {
                            chart.clear();
                            benchmark_symbol.set(e.values()["benchmark_symbol"].as_value());
                            start_date.set(e.values()["start_date"].as_value());
                            end_date.set(e.values()["end_date"].as_value());
                            interval.set(e.values()["interval"].as_value());
                            confidence_level.set(
                                e.values()["confidence_level"]
                                    .as_value()
                                    .parse::<f64>()
                                    .unwrap()
                            );
                            risk_free_rate.set(
                                e.values()["risk_free_rate"]
                                    .as_value()
                                    .parse::<f64>()
                                    .unwrap()
                            );
                            objective_function.set(
                                e.values()["objective_function"].as_value()
                            );
                            active_tab.set(1);
                            chart.restart();
                        },

                        // Symbols Input
                        Symbols { symbols: symbols }

                        // Benchmark input
                        Symbol { symbol: benchmark_symbol, title: "Benchmark Symbol" }

                        // Date Range
                        div {
                            style: r#"
                                display: flex;
                                gap: 8px;
                            "#,
                            div {
                                style: r#"
                                    flex: 1;
                                    display: flex;
                                    flex-direction: column;
                                "#,
                                label { r#for: "start_date", "Start Date" }
                                input {
                                    class: "form-control",
                                    id: "start_date",
                                    name: "start_date",
                                    r#type: "date",
                                    required: true,
                                    value: "{start_date}"
                                }
                            }
                            div {
                                style: r#"
                                    flex: 1;
                                    display: flex;
                                    flex-direction: column;
                                "#,
                                label { r#for: "end_date", "End Date" }
                                input {
                                    class: "form-control",
                                    id: "end_date",
                                    name: "end_date",
                                    r#type: "date",
                                    required: true,
                                    value: "{end_date}"
                                }
                            }
                        }

                        // Interval Select
                        div {
                            style: r#"
                                display: flex;
                                flex-direction: column;
                                min-width: 100px;
                            "#,
                            label { r#for: "interval", "Interval" }
                            select {
                                class: "form-control",
                                id: "interval",
                                name: "interval",
                                required: true,
                                value: "{interval}",
                                option { value: "1d", "Daily" }
                                option { value: "1wk", "Weekly" }
                                option { value: "1mo", "Monthly" }
                                option { value: "3mo", "Quarterly" }
                            }
                        }

                        // Confidence & Rate
                        div {
                            style: r#"
                                display: flex;
                                gap: 8px;
                            "#,
                            div {
                                style: r#"
                                    flex: 1;
                                    display: flex;
                                    flex-direction: column;
                                "#,
                                label { r#for: "confidence_level", "Confidence Level" }
                                input {
                                    class: "form-control",
                                    id: "confidence_level",
                                    name: "confidence_level",
                                    r#type: "number",
                                    step: "0.01",
                                    required: true,
                                    value: "{confidence_level}"
                                }
                            }
                            div {
                                style: r#"
                                    flex: 1;
                                    display: flex;
                                    flex-direction: column;
                                "#,
                                label { r#for: "risk_free_rate", "Risk Free Rate" }
                                input {
                                    class: "form-control",
                                    id: "risk_free_rate",
                                    name: "risk_free_rate",
                                    r#type: "number",
                                    step: "0.01",
                                    required: true,
                                    value: "{risk_free_rate}" }
                            }
                        }

                        // Objective Function
                        div {
                            style: r#"
                                display: flex;
                                flex-direction: column;
                                min-width: 150px;
                            "#,
                            label { r#for: "objective_function", "Objective Function" }
                            select {
                                class: "form-control",
                                id: "objective_function",
                                name: "objective_function",
                                required: true,
                                value: "{objective_function}",
                                option { value: "max_sharpe", "Max Sharpe" }
                                option { value: "min_vol", "Min Volatility" }
                                option { value: "max_return", "Max Return" }
                                option { value: "min_var", "Min VaR" }
                                option { value: "min_cvar", "Min CVaR" }
                                option { value: "min_drawdown", "Min Drawdown" }
                            }
                        }

                        // Submit
                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            "Generate Report"
                        }
                    }
                }
            }

            // Dashboard Panel
            div {
                style: r#"
                    flex: 1;
                    width: 100%;
                    background-color: #ffffff;
                    border-radius: 8px;
                    padding: 16px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    overflow: hidden;
                    display: flex;
                    flex-direction: column;
                    box-sizing: border-box;
                "#,
                // Nav tabs
                nav {
                    style: r#"margin-bottom: 16px;"#,
                    div {
                        class: "nav nav-tabs",
                        style: r#"gap: 8px;"#,
                        button {
                            class: if *active_tab.read() == 1 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| {
                                active_tab.set(1);
                                chart.clear();
                                chart.restart();
                            },
                            "Optimization Chart"
                        }
                        button {
                            class: if *active_tab.read() == 2 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| {
                                active_tab.set(2);
                                chart.clear();
                                chart.restart();
                            },
                            "Performance Chart"
                        }
                        button {
                            class: if *active_tab.read() == 3 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| {
                                active_tab.set(3);
                                chart.clear();
                                chart.restart();
                            },
                            "Performance Stats"
                        }
                        button {
                            class: if *active_tab.read() == 4 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| {
                                active_tab.set(4);
                                chart.clear();
                                chart.restart();
                            },
                            "Returns Data"
                        }
                        button {
                            class: if *active_tab.read() == 5 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| {
                                active_tab.set(5);
                                chart.clear();
                                chart.restart();
                            },
                            "Returns Chart"
                        }
                        button {
                            class: if *active_tab.read() == 6 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| {
                                active_tab.set(6);
                                chart.clear();
                                chart.restart();
                            },
                            "Returns Matrix"
                        }
                    }
                }
                // Content
                div {
                    style: r#"flex:1; overflow:auto;"#,
                    div {
                        class: "tab-content",
                        style: r#"height:100%;"#,
                        match &*chart.value().read_unchecked() {
                            Some(content) => match *active_tab.read() {
                                3 | 4 => rsx! { TableContainer { html: content.clone() } },
                                _   => rsx! { ChartContainer { html: content.clone() } },
                            },
                            _ => rsx! { Loading {} }
                        }
                    }
                }
            }
        }
    }
}
