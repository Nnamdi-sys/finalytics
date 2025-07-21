use dioxus::prelude::*;
use crate::components::chart::ChartContainer;
use crate::components::table::TableContainer;
use crate::components::utils::Loading;

#[component]
pub fn PortfolioDashboard(
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
                        "Optimization Chart"
                    }
                    button {
                        class: if *active_tab.read() == 2 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(2);
                            chart.clear();
                            chart.restart();
                        },
                        "Performance Chart"
                    }
                    button {
                        class: if *active_tab.read() == 3 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(3);
                            chart.clear();
                            chart.restart();
                        },
                        "Performance Stats"
                    }
                    button {
                        class: if *active_tab.read() == 4 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(4);
                            chart.clear();
                            chart.restart();
                        },
                        "Returns Data"
                    }
                    button {
                        class: if *active_tab.read() == 5 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(5);
                            chart.clear();
                            chart.restart();
                        },
                        "Returns Chart"
                    }
                    button {
                        class: if *active_tab.read() == 6 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(6);
                            chart.clear();
                            chart.restart();
                        },
                        "Returns Matrix"
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
                            1 | 2 => rsx! { ChartContainer { html: content.clone() } },
                            3 | 4 => rsx! { TableContainer { html: content.clone() } },
                            5 | 6 => rsx! { ChartContainer { html: content.clone() } },
                            _ => rsx! { },
                        },
                        _ => rsx! {
                            Loading {}
                        }
                    }
                }
            }
        }
    }
}