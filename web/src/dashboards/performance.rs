use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::server::get_ticker_charts;
use crate::components::utils::Loading;
use crate::components::symbols::Symbol;
use crate::components::chart::ChartContainer;
use crate::components::table::TableContainer;

#[component]
pub fn Performance() -> Element {
    let symbol = use_signal(|| "AAPL".to_string());
    let benchmark_symbol = use_signal(|| "^GSPC".to_string());
    let mut start_date = use_signal(|| "2023-01-01".to_string());
    let mut end_date = use_signal(|| "2024-12-31".to_string());
    let mut interval = use_signal(|| "1d".to_string());
    let mut confidence_level = use_signal(|| 0.95);
    let mut risk_free_rate = use_signal(|| 0.02);
    let mut active_tab = use_signal(|| 1);

    info!("symbol: {:?}", symbol());
    info!("benchmark: {:?}", benchmark_symbol());
    info!("start: {:?}", start_date());
    info!("end: {:?}", end_date());
    info!("interval: {:?}", interval());
    info!("confidence: {:?}", confidence_level());
    info!("risk_free: {:?}", risk_free_rate());
    info!("active_tab: {:?}", active_tab());

    let mut chart = use_server_future(move || async move {
        match get_ticker_charts(
            symbol(),
            start_date(),
            end_date(),
            interval(),
            benchmark_symbol(),
            confidence_level(),
            risk_free_rate(),
            String::from("performance"),
            String::new(),
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

            // Form bar at top
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
                            active_tab.set(1);
                            chart.restart();
                        },

                        // Symbol input
                        Symbol { symbol: symbol, title: "Symbol" }

                        // Benchmark input
                        Symbol { symbol: benchmark_symbol, title: "Benchmark Symbol" }

                        // Date range
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

                        // Interval
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

                        // Confidence & risk-free
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

                        // Submit
                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            "Generate Report"
                        }
                    }
                }
            }

            // Dashboard below form
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
                            onclick: move |_| { active_tab.set(1); chart.clear(); chart.restart(); },
                            "Price Data"
                        }
                        button {
                            class: if *active_tab.read() == 2 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(2); chart.clear(); chart.restart(); },
                            "Candlestick Chart"
                        }
                        button {
                            class: if *active_tab.read() == 3 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(3); chart.clear(); chart.restart(); },
                            "Performance Chart"
                        }
                        button {
                            class: if *active_tab.read() == 4 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(4); chart.clear(); chart.restart(); },
                            "Performance Stats Table"
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
                                1 | 4 => rsx! { TableContainer { html: content.clone() } },
                                2 | 3 => rsx! { ChartContainer { html: content.clone() } },
                                _ => rsx! {},
                            },
                            _ => rsx! { Loading {} }
                        }
                    }
                }
            }
        }
    }
}
