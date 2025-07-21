use dioxus::prelude::*;
use crate::server::ALL_SYMBOLS_DATALIST;

#[component]
pub fn PortfolioForm(
    symbols: Signal<String>,
    benchmark_symbol: Signal<String>,
    start_date: Signal<String>,
    end_date: Signal<String>,
    interval: Signal<String>,
    confidence_level: Signal<f64>,
    risk_free_rate: Signal<f64>,
    objective_function: Signal<String>,
    active_tab: Signal<usize>,
    chart: Resource<String>
) -> Element {
    rsx! {
        div {
            style: "background-color: #f5f5f5; padding: 20px; border-radius: 10px; width: 300px; margin-right: 20px; margin-top: 5px;",

            form {
                style: "width: 100%;",
                onsubmit: move |e| {
                    chart.clear();
                    symbols.set(e.values()["symbols"].as_value());
                    benchmark_symbol.set(e.values()["benchmark_symbol"].as_value());
                    start_date.set(e.values()["start_date"].as_value());
                    end_date.set(e.values()["end_date"].as_value());
                    interval.set(e.values()["interval"].as_value());
                    confidence_level.set(e.values()["confidence_level"].as_value().parse::<f64>().unwrap());
                    risk_free_rate.set(e.values()["risk_free_rate"].as_value().parse::<f64>().unwrap());
                    objective_function.set(e.values()["objective_function"].as_value());
                    active_tab.set(1);
                    chart.restart();
                },

                datalist {
                    id: "all_symbols",
                    {ALL_SYMBOLS_DATALIST.lock().unwrap().iter().map(|(symbol, name)| rsx! {
                        option { label: "{name}", value: "{symbol}" }
                    })}
                }

                div {
                    // Symbols
                    div { style: "margin-bottom: 15px;",
                        label { r#for: "symbols", "Symbols" }
                        input {
                            class: "form-control",
                            id: "symbols",
                            name: "symbols",
                            r#type: "email",
                            required: true,
                            multiple: "multiple",
                            list: "all_symbols",
                            value: "{symbols}",
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
                    // Objective Function
                    div { style: "margin-bottom: 15px;",
                        label { r#for: "objective_function", "Objective Function" }
                        select {
                            class: "form-control",
                            id: "objective_function",
                            name: "objective_function",
                            required: true,
                            value: "{objective_function}",
                            option { value: "max_sharpe", "Maximize Sharpe Ratio" }
                            option { value: "min_vol", "Minimize Volatility" }
                            option { value: "max_return", "Maximize Return" }
                            option { value: "min_var", "Minimize Value at Risk" }
                            option { value: "min_cvar", "Minimize Conditional Value at Risk" }
                            option { value: "min_drawdown", "Minimize Drawdown" }
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
}



