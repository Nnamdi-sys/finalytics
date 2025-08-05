use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::server::get_ticker_charts;
use crate::components::utils::Loading;
use crate::components::symbols::Symbol;
use crate::components::chart::ChartContainer;
use crate::components::table::TableContainer;

#[component]
pub fn Options() -> Element {
    let mut symbol = use_signal(|| "AAPL".to_string());
    let mut risk_free_rate = use_signal(|| 0.02);
    let mut active_tab = use_signal(|| 1);

    info!("symbol: {:?}", symbol());
    info!("risk_free: {:?}", risk_free_rate());
    info!("active_tab: {:?}", active_tab());

    let mut chart = use_server_future(move || async move {
        match get_ticker_charts(
            symbol(),
            String::new(),
            String::new(),
            String::new(), 
            String::new(),
            f64::default(),
            risk_free_rate(),
            String::from("options"),
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
                            symbol.set(e.values()["symbol"].as_value());
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

                        // Risk Free Rate
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
                            "Options Chain"
                        }
                        button {
                            class: if *active_tab.read() == 2 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(2); chart.clear(); chart.restart(); },
                            "Volatility Surface Data"
                        }
                        button {
                            class: if *active_tab.read() == 3 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(3); chart.clear(); chart.restart(); },
                            "Volatility Smile Chart"
                        }
                        button {
                            class: if *active_tab.read() == 4 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(4); chart.clear(); chart.restart(); },
                            "Volatility Skew Chart"
                        }
                        button {
                            class: if *active_tab.read() == 5 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(5); chart.clear(); chart.restart(); },
                            "Volatility Surface Chart"
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
                                1 | 2 => rsx! { TableContainer { html: content.clone() } },
                                3..=5 => rsx! { ChartContainer { html: content.clone() } },
                                _ => rsx! { },
                            },
                            _ => rsx! { Loading {} }
                        }
                    }
                }
            }
        }
    }
}

