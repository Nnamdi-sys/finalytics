use dioxus::prelude::*;
use crate::components::table::TableContainer;
use crate::components::utils::Loading;

#[component]
pub fn FinancialsDashboard(
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
                        "Quarterly Income Statement"
                    }
                    button {
                        class: if *active_tab.read() == 2 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(2);
                            chart.clear();
                            chart.restart();
                        },
                        "Annual Income Statement"
                    }
                    button {
                        class: if *active_tab.read() == 3 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(3);
                            chart.clear();
                            chart.restart();
                        },
                        "Quarterly Balance Sheet"
                    }
                    button {
                        class: if *active_tab.read() == 4 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(4);
                            chart.clear();
                            chart.restart();
                        },
                        "Annual Balance Sheet"
                    }
                    button {
                        class: if *active_tab.read() == 5 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(5);
                            chart.clear();
                            chart.restart();
                        },
                        "Quarterly Cash Flow Statement"
                    }
                    button {
                        class: if *active_tab.read() == 6 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(6);
                            chart.clear();
                            chart.restart();
                        },
                        "Annual Cash Flow Statement"
                    }
                    button {
                        class: if *active_tab.read() == 7 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(7);
                            chart.clear();
                            chart.restart();
                        },
                        "Quarterly Financial Ratios"
                    }
                    button {
                        class: if *active_tab.read() == 8 { "nav-link active" } else { "nav-link" },
                        onclick: move |_| {
                            active_tab.set(8);
                            chart.clear();
                            chart.restart();
                        },
                        "Annual Financial Ratios"
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
                        Some(content) =>  {
                            rsx! { TableContainer { html: content.clone() } }
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