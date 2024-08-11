use dioxus::prelude::*;
use crate::server::{get_ticker_chart, ALL_SYMBOLS_DATALIST};


#[allow(unused_variables)]
#[component]
pub fn Ticker() -> Element {

    let mut symbol = use_signal(|| "MSFT".to_string());
    let mut benchmark_symbol = use_signal(|| "^GSPC".to_string());
    let mut start_date = use_signal(|| "2020-01-01".to_string());
    let mut end_date = use_signal(|| "2023-12-31".to_string());
    let mut interval = use_signal(|| "1d".to_string());
    let mut confidence_level = use_signal(|| 0.95);
    let mut risk_free_rate = use_signal(|| 0.04);
    let mut chart_num = use_signal(|| 0usize);

    log::info!("{:?}", &symbol.read());
    log::info!("{:?}", &benchmark_symbol.read());
    log::info!("{:?}", &start_date.read());
    log::info!("{:?}", &end_date.read());
    log::info!("{:?}", &interval.read());
    log::info!("{:?}", &confidence_level.read());
    log::info!("{:?}", &risk_free_rate.read());
    log::info!("{:?}", &chart_num.read());


    let _ = use_server_future(move ||{ async move{
                                       let chart = match get_ticker_chart(
                                           symbol(),
                                           start_date(),
                                           end_date(),
                                           interval(),
                                           benchmark_symbol(),
                                           confidence_level(),
                                           risk_free_rate(),
                                           chart_num(),
                                       ).await {
                                           Ok(chart) => chart,
                                           Err(e) => format!("Error: {}", e),
                                       };
                                        if &*chart != "" {
                                            let mut ev = eval(&*chart);
                                            let res = ev.recv().await.unwrap_or_default();
                                            println!("{:?}", &res);
                                        }
                                   }
                                   })?;


    rsx! {

        div {

            style: "margin: 0; padding: 0; background-color: #f5f5f5; display: flex; flex-direction: column; height: 100%;",

            div {

                style: "flex: 1; display: flex;",

                form {

                    style: "background-color: #f5f5f5; padding: 20px; border-radius: 10px; margin-right: 20px; flex: 0.5; margin-top: 100px;",


                    onsubmit: move |e| {
                        symbol.set(e.values()["symbol"].as_value());
                        benchmark_symbol.set(e.values()["benchmark_symbol"].as_value());
                        start_date.set(e.values()["start_date"].as_value());
                        end_date.set(e.values()["end_date"].as_value());
                        interval.set(e.values()["interval"].as_value());
                        confidence_level.set(e.values()["confidence_level"].as_value().parse::<f64>().unwrap());
                        risk_free_rate.set(e.values()["risk_free_rate"].as_value().parse::<f64>().unwrap());
                        if *chart_num.read() == 0 {
                            chart_num.set(1);
                        };
                    },

                    datalist {
                        id: "all_symbols",
                        {ALL_SYMBOLS_DATALIST.lock().unwrap().iter().map(|(symbol, name)| {
                            rsx! {
                                option {
                                    label: "{name}",
                                    value: "{symbol}",
                                }
                            }
                        })}
                    }

                    label { r#for: "symbol", "Symbol" }
                    br {}
                    input {
                        id: "symbol",
                        name: "symbol",
                        r#type: "text",
                        required: true,
                        list: "all_symbols",
                        value: "{symbol}",
                    }
                    br {}
                    br {}

                    label { r#for: "benchmark_symbol", "Benchmark" }
                    br {}
                    input {
                        id: "benchmark_symbol",
                        name: "benchmark_symbol",
                        r#type: "text",
                        required: true,
                        list: "all_symbols",
                        value: "{benchmark_symbol}",
                    }
                    br {}
                    br {}

                    label { r#for: "start_date", "Start Date" }
                    br {}
                    input {
                        id: "start_date",
                        name: "start_date",
                        r#type: "date",
                        required: true,
                        value: "{start_date}",
                    }
                    br {}
                    br {}

                    label { r#for: "end_date", "End Date" }
                    br {}
                    input {
                        id: "end_date",
                        name: "end_date",
                        r#type: "date",
                        required: true,
                        value: "{end_date}",
                    }
                    br {}
                    br {}

                    label { r#for: "interval", "Interval" }
                    br {}
                    select {
                        id: "interval",
                        name: "interval",
                        required: true,
                        value: "{interval}",
                        option {
                            value: "1d",
                            "Daily"
                        }
                        option {
                            value: "1wk",
                            "Weekly"
                        }
                        option {
                            value: "1mo",
                            "Monthly"
                        }
                        option {
                            value: "3mo",
                            "Quarterly"
                        }
                    }
                    br {}
                    br {}

                    label { r#for: "confidence_level", "Confidence Level" }
                    br {}
                    input {
                        id: "confidence_level",
                        name: "confidence_level",
                        r#type: "text",
                        required: true,
                        value: "{confidence_level.to_string()}",
                    }
                    br {}
                    br {}

                    label { r#for: "risk_free_rate", "Risk Free Rate" }
                    br {}
                    input {
                        id: "risk_free_rate",
                        name: "risk_free_rate",
                        r#type: "text",
                        required: true,
                        value: "{risk_free_rate.to_string()}",
                    }
                    br {}
                    br {}

                    button {
                        class: "btn btn-primary",
                        r#type: "submit",
                        formnovalidate: true,
                        "Generate Chart"
                         },
                }

                div {
                    class: "tab-content",

                    nav {
                        div {
                            class: "nav nav-tabs",
                            style: "margin-bottom: 20px;",
                            button {
                                class: if *chart_num.read() == 1 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(1),
                                "Candlestick Chart"
                            }
                            button {
                                class: if *chart_num.read() == 2 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(2),
                                "Summary Stats Table"
                            }
                            button {
                                class: if *chart_num.read() == 3 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(3),
                                "Performance Chart"
                            }
                            button {
                                class: if *chart_num.read() == 4 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(4),
                                "Performance Stats Table"
                            }
                            button {
                                class: if *chart_num.read() == 5 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(5),
                                "Income Statement"
                            }
                            button {
                                class: if *chart_num.read() == 6 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(6),
                                "Balance Sheet"
                            }
                            button {
                                class: if *chart_num.read() == 7 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(7),
                                "Cash Flow Statement"
                            }
                            button {
                                class: if *chart_num.read() == 8 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(8),
                                "Financial Ratios"
                            }
                            button {
                                class: if *chart_num.read() == 9 { "nav-link active" } else { "nav-link" },
                                onclick: move |_| chart_num.set(9),
                                "Volatility Surface"
                            }
                        }
                    }

                    div {
                        class: "tab-pane fade show active",
                        style: "padding: 5px;",
                        div {
                            style: "height:100%; width:100%;",
                            script {
                                src: "https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"
                            }
                            script {
                                src: "https://cdn.plot.ly/plotly-2.12.1.min.js"
                            }
                            div {
                                id: "plotly-html-element",
                                class: "plotly-graph-div",
                                style: "height:100%; width:100%;"
                            }

                        }
                    }

                }
            }
        }
    }
}


