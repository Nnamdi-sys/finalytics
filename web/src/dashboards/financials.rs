use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::server::get_ticker_charts;
use crate::components::utils::Loading;
use crate::components::symbols::Symbol;
use crate::components::table::TableContainer;

#[component]
pub fn Financials() -> Element {
    let mut symbol = use_signal(|| "AAPL".to_string());
    let mut frequency = use_signal(|| "quarterly".to_string());
    let mut active_tab = use_signal(|| 1);

    info!("symbol: {:?}", symbol());
    info!("frequency: {:?}", frequency());
    info!("active_tab: {:?}", active_tab());

    let mut chart = use_server_future(move || async move {
        match get_ticker_charts(
            symbol(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            f64::default(),
            f64::default(),
            String::from("financials"),
            frequency(),
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
                            frequency.set(e.values()["frequency"].as_value());
                            active_tab.set(1);
                            chart.restart();
                        },

                        // Symbol input
                        Symbol { symbol: symbol, title: "Symbol" }

                        // Frequency
                        div {
                            style: r#"
                                flex: 1;
                                display: flex;
                                flex-direction: column;
                            "#,
                            label { r#for: "frequency", "Frequency" }
                            select {
                                class: "form-control",
                                id: "frequency",
                                name: "frequency",
                                required: true,
                                value: "{frequency}",
                                option { value: "annual", "Annual" }
                                option { value: "quarterly", "Quarterly" }
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
                            "Income Statement"
                        }
                        button {
                            class: if *active_tab.read() == 2 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(2); chart.clear(); chart.restart(); },
                            "Balance Sheet"
                        }
                        button {
                            class: if *active_tab.read() == 3 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(3); chart.clear(); chart.restart(); },
                            "Cash Flow Statement"
                        }
                        button {
                            class: if *active_tab.read() == 4 { "nav-link active" } else { "nav-link" },
                            onclick: move |_| { active_tab.set(4); chart.clear(); chart.restart(); },
                            "Financial Ratios"
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
                            Some(content) =>  {
                                rsx! { TableContainer { html: content.clone() } }
                            },
                            _ => rsx! { Loading {} }
                        }
                    }
                }
            }
        }
    }
}