use dioxus::prelude::*;
use crate::components::chart::ChartContainer;
use crate::components::utils::Loading;
use crate::components::table::TableContainer;
use crate::forms::screener::ScreenerTickersForm;

#[component]
pub fn ScreenerDashboard(
    active_tab: Signal<usize>,
    screener_data: Resource<String>,
    benchmark_symbol: Signal<String>,
    start_date: Signal<String>,
    end_date: Signal<String>,
    risk_free_rate: Signal<f64>,
    objective_function: Signal<String>,
) -> Element {

    rsx!{
        div {
            class: "tab-content",

            nav {
                div {
                    class: "nav nav-tabs",
                    style: "margin-bottom: 20px;",
                    button {
                        class: if *active_tab.read() == 1 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(1);
                            screener_data.clear();
                            screener_data.restart();
                        },
                        "Overview"
                    }
                    button {
                        class: if *active_tab.read() == 2 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(2);
                            screener_data.clear();
                            screener_data.restart();
                        },
                        "Metrics"
                    }
                    button {
                        class: if *active_tab.read() == 3 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(3);
                            screener_data.clear();
                            screener_data.restart();
                        },
                        "Performance"
                    }
                    button {
                        class: if *active_tab.read() == 4 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(4);
                            screener_data.clear();
                            screener_data.restart();
                        },
                        "Optimization"
                    }
                }
            }

            // Tab content area
            div {
                class: "tab-content",
                style: "flex: 1; overflow: auto;",
                match *active_tab.read() {
                    1 | 2 => rsx! {
                        ScreenerDisplay { 
                            active_tab,
                            screener_data 
                        }
                    },
                    3 | 4 => rsx! {
                        div {
                            style: "display: flex; flex-direction: column; gap: 20px;",
                            ScreenerTickersForm {
                                benchmark_symbol,
                                start_date,
                                end_date,
                                risk_free_rate,
                                objective_function,
                                screener_data,
                                active_tab,
                            }
                            ScreenerDisplay { 
                                active_tab,
                                screener_data 
                            }
                        }
                    },
                    _ => rsx! {}
                }
            }
        }
    }
}


#[component]
pub fn ScreenerDisplay(
    active_tab: Signal<usize>,
    screener_data: Resource<String>,
) -> Element {
    rsx! {
        div {
            class: "tab-pane fade show active",
            style: "height: 100%;",
            match &*screener_data.value().read_unchecked() {
                Some(content) =>  {
                    match *active_tab.read() {
                        1..=3 => rsx! { TableContainer { html: content.clone() } },
                        4 => rsx! { ChartContainer { html: content.clone() } },
                        _ => rsx! {}
                    }
                },
                _ => rsx! {
                    Loading {}
                }
            }
        }
    }
}