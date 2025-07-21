use dioxus::prelude::*;
use crate::dashboards::news::NewsDashboard;
use crate::dashboards::options::OptionsDashboard;
use crate::dashboards::financials::FinancialsDashboard;
use crate::dashboards::performance::PerformanceDashboard;

#[component]
pub fn TickerDashboard(
    active_tab: Signal<usize>,
    report_type: Signal<String>,
    chart: Resource<String>
) -> Element {

    rsx!{
        match &**report_type.read() {
            "performance" => rsx!{
                PerformanceDashboard {
                    active_tab: active_tab,
                    chart: chart,
                }
            },
            "financials" => rsx! {
                FinancialsDashboard {
                    active_tab: active_tab,
                    chart: chart,
                }
            },
            "options" => rsx! {
                OptionsDashboard {
                    active_tab: active_tab,
                    chart: chart,
                }
            },
            "news" => rsx! {
                NewsDashboard {
                    active_tab: active_tab,
                    chart: chart,
                }
            },
            _ => rsx! {}
        }
    }
}