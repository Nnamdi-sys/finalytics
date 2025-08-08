use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::server::{get_ticker_charts, PerformanceTabs, TickerTabs};
use crate::components::symbols::Symbol;
use crate::components::display::PerformanceDisplay;

#[component]
pub fn Performance() -> Element {
    let symbol = use_signal(|| "AAPL".to_string());
    let benchmark_symbol = use_signal(|| "^GSPC".to_string());
    let mut start_date = use_signal(|| "2023-01-01".to_string());
    let mut end_date = use_signal(|| "2024-12-31".to_string());
    let mut interval = use_signal(|| "1d".to_string());
    let mut confidence_level = use_signal(|| 0.95);
    let mut risk_free_rate = use_signal(|| 0.02);

    info!("symbol: {:?}", symbol());
    info!("benchmark: {:?}", benchmark_symbol());
    info!("start: {:?}", start_date());
    info!("end: {:?}", end_date());
    info!("interval: {:?}", interval());
    info!("confidence: {:?}", confidence_level());
    info!("risk_free: {:?}", risk_free_rate());

    let mut charts = use_server_future(move || async move {
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
        )
            .await
        {
            Ok(TickerTabs::Performance(tabs)) => tabs,
            Ok(_) => PerformanceTabs {
                ohlcv_table: String::from("Invalid report type"),
                candlestick_chart: String::from("Invalid report type"),
                performance_chart: String::from("Invalid report type"),
                performance_stats_table: String::from("Invalid report type"),
            },
            Err(e) => PerformanceTabs {
                ohlcv_table: format!("Error: {e}"),
                candlestick_chart: format!("Error: {e}"),
                performance_chart: format!("Error: {e}"),
                performance_stats_table: format!("Error: {e}"),
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
                            charts.restart();
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

            // PerformanceDisplay
            PerformanceDisplay { charts: charts }
        }
    }
}