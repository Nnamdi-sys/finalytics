use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::server::{get_ticker_charts, OptionsTabs, TickerTabs};
use crate::components::symbols::Symbol;
use crate::components::display::OptionsDisplay;

#[component]
pub fn Options() -> Element {
    let symbol = use_signal(|| "AAPL".to_string());
    let mut risk_free_rate = use_signal(|| 0.02);
    let mut fetch_data = use_signal(|| false);

    info!("symbol: {:?}", symbol());
    info!("risk_free: {:?}", risk_free_rate());
    info!("fetch_data: {:?}", fetch_data());

    let mut charts = use_server_future(move || async move {
        if !fetch_data() {
            return OptionsTabs {
                options_chain: String::new(),
                volatility_surface_table: String::new(),
                volatility_smile: String::new(),
                volatility_term_structure: String::new(),
                volatility_surface_chart: String::new(),
            };
        }
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
        )
            .await
        {
            Ok(TickerTabs::Options(tabs)) => tabs,
            Ok(_) => OptionsTabs {
                options_chain: String::from("Invalid report type"),
                volatility_surface_table: String::from("Invalid report type"),
                volatility_smile: String::from("Invalid report type"),
                volatility_term_structure: String::from("Invalid report type"),
                volatility_surface_chart: String::from("Invalid report type"),
            },
            Err(e) => OptionsTabs {
                options_chain: format!("Error: {e}"),
                volatility_surface_table: format!("Error: {e}"),
                volatility_smile: format!("Error: {e}"),
                volatility_term_structure: format!("Error: {e}"),
                volatility_surface_chart: format!("Error: {e}"),
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
                            risk_free_rate.set(
                                e.values()["risk_free_rate"]
                                    .as_value()
                                    .parse::<f64>()
                                    .unwrap()
                            );
                            fetch_data.set(true);
                            charts.restart();
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

            // OptionsDisplay
            OptionsDisplay { charts: charts }
        }
    }
}