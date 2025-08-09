use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::server::{get_ticker_charts, FinancialsTabs, TickerTabs};
use crate::components::symbols::Symbol;
use crate::components::display::FinancialsDisplay;

#[component]
pub fn Financials() -> Element {
    let symbol = use_signal(|| "AAPL".to_string());
    let mut frequency = use_signal(|| "quarterly".to_string());
    let mut fetch_data = use_signal(|| false);

    info!("symbol: {:?}", symbol());
    info!("frequency: {:?}", frequency());
    info!("fetch_data: {:?}", fetch_data());

    let mut charts = use_server_future(move || async move {
        if !fetch_data() {
            return FinancialsTabs {
                income_statement: String::new(),
                balance_sheet: String::new(),
                cashflow_statement: String::new(),
                financial_ratios: String::new(),
            };
        }
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
        )
            .await
        {
            Ok(TickerTabs::Financials(tabs)) => tabs,
            Ok(_) => FinancialsTabs {
                income_statement: String::from("Invalid report type"),
                balance_sheet: String::from("Invalid report type"),
                cashflow_statement: String::from("Invalid report type"),
                financial_ratios: String::from("Invalid report type"),
            },
            Err(e) => FinancialsTabs {
                income_statement: format!("Error: {e}"),
                balance_sheet: format!("Error: {e}"),
                cashflow_statement: format!("Error: {e}"),
                financial_ratios: format!("Error: {e}"),
            },
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
                            charts.clear();
                            frequency.set(e.values()["frequency"].as_value());
                            fetch_data.set(true);
                            charts.restart();
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

            // FinancialsDisplay
            FinancialsDisplay { charts: charts }
        }
    }
}