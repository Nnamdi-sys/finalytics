use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::server::{get_ticker_charts, NewsTabs, TickerTabs};
use crate::components::symbols::Symbol;
use crate::components::display::NewsDisplay;
use chrono;

#[component]
pub fn News() -> Element {
    let mut symbol = use_signal(|| "AAPL".to_string());
    let default_start = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(30))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let default_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let mut start_date = use_signal(|| default_start);
    let mut end_date = use_signal(|| default_end);

    info!("symbol: {:?}", symbol());
    info!("start: {:?}", start_date());
    info!("end: {:?}", end_date());

    let mut charts = use_server_future(move || async move {
        match get_ticker_charts(
            symbol(),
            start_date(),
            end_date(),
            String::new(),
            String::new(),
            f64::default(),
            f64::default(),
            String::from("news"),
            String::new(),
        )
            .await
        {
            Ok(TickerTabs::News(tabs)) => tabs,
            Ok(_) => NewsTabs {
                news_sentiment_table: String::from("Invalid report type"),
                news_sentiment_chart: String::from("Invalid report type"),
            },
            Err(e) => NewsTabs {
                news_sentiment_table: format!("Error: {e}"),
                news_sentiment_chart: format!("Error: {e}"),
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
                            start_date.set(e.values()["start_date"].as_value());
                            end_date.set(e.values()["end_date"].as_value());
                            charts.restart();
                        },

                        // Symbol input
                        Symbol { symbol: symbol, title: "Symbol" }

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

                        // Submit
                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            "Generate Report"
                        }
                    }
                }
            }

            // NewsDisplay
            NewsDisplay { charts: charts }
        }
    }
}