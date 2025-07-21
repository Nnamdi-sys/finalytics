use dioxus::prelude::*;
use crate::components::chart::ChartContainer;
use crate::components::table::TableContainer;
use crate::components::utils::Loading;

#[component]
pub fn PerformanceDashboard(
    active_tab: Signal<usize>,
    chart: Resource<String>
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
                            chart.clear();
                            chart.restart();
                        },
                        "Price Data"
                    }
                    button {
                        class: if *active_tab.read() == 2 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(2);
                            chart.clear();
                            chart.restart();
                        },
                        "Candlestick Chart"
                    }
                    button {
                        class: if *active_tab.read() == 3 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(3);
                            chart.clear();
                            chart.restart();
                        },
                        "Performance Chart"
                    }
                    button {
                        class: if *active_tab.read() == 4 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(4);
                            chart.clear();
                            chart.restart();
                        },
                        "Performance Stats Table"
                    }
                }
            }
            
            // Tab content area
            div {
                class: "tab-content",
                style: "flex: 1; overflow: hidden;",
                
                // Single content container for all tabs
                div {
                    class: "tab-pane fade show active",
                    style: "height: 100%;",
                    match &*chart.value().read_unchecked() {
                        Some(content) => match *active_tab.read() {
                            1 => rsx! { TableContainer { html: content.clone() } },
                            2 | 3 => rsx! { ChartContainer { html: content.clone() } },
                            4 => rsx! { TableContainer { html: content.clone() } },
                            _ => rsx! { },
                        },
                        _ => {
                            rsx! {
                                Loading {}
                            }
                        }
                    }
                }
            }
        }
    }
}