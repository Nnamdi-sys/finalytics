use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::dashboards::ticker::TickerDashboard;
use crate::forms::ticker::TickerForm;
use crate::server::get_ticker_charts;



#[component]
pub fn Ticker() -> Element {

    let symbol = use_signal(|| "AAPL".to_string());
    let benchmark_symbol = use_signal(|| "^GSPC".to_string());
    let start_date = use_signal(|| "2023-01-01".to_string());
    let end_date = use_signal(|| "2024-12-31".to_string());
    let interval = use_signal(|| "1d".to_string());
    let confidence_level = use_signal(|| 0.95);
    let risk_free_rate = use_signal(|| 0.02);
    let active_tab = use_signal(|| 1);
    let report_type = use_signal(|| "performance".to_string());

    info!("{:?}", symbol());
    info!("{:?}", benchmark_symbol());
    info!("{:?}", start_date());
    info!("{:?}", end_date());
    info!("{:?}", interval());
    info!("{:?}", confidence_level());
    info!("{:?}", risk_free_rate());
    info!("{:?}", report_type());
    info!("{:?}", active_tab());


    let chart = use_server_future(move ||{ async move{
       let chart = match get_ticker_charts(
           symbol(),
           start_date(),
           end_date(),
           interval(),
           benchmark_symbol(),
           confidence_level(),
           risk_free_rate(),
           report_type(),
           active_tab(),
       ).await {
           Ok(chart) => chart,
           Err(e) => format!("Error: {e}"),
       };
        chart
   }
   })?;


    rsx! {
        div {
            style: "display: flex; height: 100vh; margin: 0; padding: 20px; background-color: #f5f5f5; gap: 20px;",
            
            // Form Panel (20% width)
            div {
                style: "width: 20%; min-width: 250px; position: relative;",
                TickerForm {
                    symbol: symbol,
                    benchmark_symbol: benchmark_symbol,
                    start_date: start_date,
                    end_date: end_date,
                    interval: interval,
                    confidence_level: confidence_level,
                    risk_free_rate: risk_free_rate,
                    report_type: report_type,
                    active_tab: active_tab,
                    chart: chart,
                }
            }
            
            // Dashboard Panel (80% width)
            div {
                style: "width: 80%; overflow: auto; background: white; border-radius: 8px; padding: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                TickerDashboard {
                    active_tab: active_tab,
                    report_type: report_type,
                    chart: chart,
                }
            }
        }
    }
}



