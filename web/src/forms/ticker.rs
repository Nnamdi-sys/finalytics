use dioxus::prelude::*;
use crate::server::ALL_SYMBOLS_DATALIST;

#[component]
pub fn TickerForm(
    symbol: Signal<String>,
    benchmark_symbol: Signal<String>,
    start_date: Signal<String>,
    end_date: Signal<String>,
    interval: Signal<String>,
    confidence_level: Signal<f64>,
    risk_free_rate: Signal<f64>,
    report_type: Signal<String>,
    active_tab: Signal<usize>,
    chart: Resource<String>,
) -> Element {
    rsx! {
        div {
            style: "background-color: #f5f5f5; padding: 20px; border-radius: 10px; width: 300px; margin-right: 20px; margin-top: 5px;",

            form {
                style: "width: 100%;",
                onsubmit: move |e| {
                    chart.clear();
                    symbol.set(e.values()["symbol"].as_value());
                    benchmark_symbol.set(e.values()["benchmark_symbol"].as_value());
                    start_date.set(e.values()["start_date"].as_value());
                    end_date.set(e.values()["end_date"].as_value());
                    interval.set(e.values()["interval"].as_value());
                    confidence_level.set(e.values()["confidence_level"].as_value().parse::<f64>().unwrap());
                    risk_free_rate.set(e.values()["risk_free_rate"].as_value().parse::<f64>().unwrap());
                    report_type.set(e.values()["report_type"].as_value());
                    active_tab.set(1);
                    chart.restart();
                },

                datalist {
                    id: "all_symbols",
                    {ALL_SYMBOLS_DATALIST.lock().unwrap().iter().map(|(sym, name)| rsx! {
                        option { label: "{name}", value: "{sym}" }
                    })}
                }

                // Symbol
                div { style: "margin-bottom: 15px;",
                    label { r#for: "symbol", "Symbol" }
                    input {
                        class: "form-control",
                        id: "symbol",
                        name: "symbol",
                        r#type: "text",
                        required: true,
                        list: "all_symbols",
                        value: "{symbol}",
                    }
                }

                // Benchmark
                div { style: "margin-bottom: 15px;",
                    label { r#for: "benchmark_symbol", "Benchmark" }
                    input {
                        class: "form-control",
                        id: "benchmark_symbol",
                        name: "benchmark_symbol",
                        r#type: "text",
                        required: true,
                        list: "all_symbols",
                        value: "{benchmark_symbol}",
                    }
                }

                // Start Date
                div { style: "margin-bottom: 15px;",
                    label { r#for: "start_date", "Start Date" }
                    input {
                        class: "form-control",
                        id: "start_date",
                        name: "start_date",
                        r#type: "date",
                        required: true,
                        value: "{start_date}",
                    }
                }

                // End Date
                div { style: "margin-bottom: 15px;",
                    label { r#for: "end_date", "End Date" }
                    input {
                        class: "form-control",
                        id: "end_date",
                        name: "end_date",
                        r#type: "date",
                        required: true,
                        value: "{end_date}",
                    }
                }

                // Interval
                div { style: "margin-bottom: 15px;",
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

                // Confidence Level
                div { style: "margin-bottom: 15px;",
                    label { r#for: "confidence_level", "Confidence Level" }
                    input {
                        class: "form-control",
                        id: "confidence_level",
                        name: "confidence_level",
                        r#type: "text",
                        required: true,
                        value: "{confidence_level.to_string()}",
                    }
                }

                // Risk Free Rate
                div { style: "margin-bottom: 15px;",
                    label { r#for: "risk_free_rate", "Risk Free Rate" }
                    input {
                        class: "form-control",
                        id: "risk_free_rate",
                        name: "risk_free_rate",
                        r#type: "text",
                        required: true,
                        value: "{risk_free_rate.to_string()}",
                    }
                }
                
                // Report Type
                div { style: "margin-bottom: 15px;",
                    label { r#for: "report_type", "Report Type" }
                    select {
                        class: "form-control",
                        id: "report_type",
                        name: "report_type",
                        required: true,
                        value: "{report_type}",
                        option { value: "performance", "Performance" }
                        option { value: "financials", "Financials" }
                        option { value: "options", "Options" }
                        option { value: "news", "News" }
                    }
                }

                // Submit Button
                button {
                    class: "btn btn-primary",
                    r#type: "submit",
                    formnovalidate: true,
                    "Generate Report"
                }
            }
        }
    }
}





