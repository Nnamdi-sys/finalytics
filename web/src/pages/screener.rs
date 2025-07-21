use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use chrono::{Local, Duration};
use crate::dashboards::screener::ScreenerDashboard;
use crate::forms::screener::ScreenerForm;
use crate::server::{get_screener_data, get_screener_performance, get_screener_portfolio, get_screener_symbols};

#[component]
pub fn Screener() -> Element {
    // Initialize Signals with default values
    let quote_type = use_signal(|| "EQUITY".to_string());
    let filters = use_signal(|| vec![
        r#"{"operator": "eq", "operands": ["region", "us"]}"#.to_string(),
        r#"{"operator": "eq", "operands": ["exchange", "NMS"]}"#.to_string()
    ]);
    let sort_field = use_signal(|| "intradaymarketcap".to_string());
    let sort_descending = use_signal(|| true);
    let offset = use_signal(|| 0);
    let size = use_signal(|| 50);
    let active_tab = use_signal(|| 1);
    let benchmark_symbol = use_signal(|| "^GSPC".to_string());
    let start_date = use_signal(|| (Local::now() - Duration::days(365)).format("%Y-%m-%d").to_string());
    let end_date = use_signal(|| Local::now().format("%Y-%m-%d").to_string());
    let risk_free_rate = use_signal(|| 0.02);
    let objective_function = use_signal(|| "max_sharpe".to_string());


    // Log Signal values for debugging
    info!("quote_type: {:?}", quote_type());
    info!("filters: {:?}", filters());
    info!("sort_field: {:?}", sort_field());
    info!("sort_descending: {:?}", sort_descending());
    info!("offset: {:?}", offset());
    info!("size: {:?}", size());
    info!("active_tab: {:?}", active_tab());
    info!("benchmark_symbol: {:?}", benchmark_symbol());
    info!("start_date: {:?}", start_date());
    info!("end_date: {:?}", end_date());
    info!("risk_free_rate: {:?}", risk_free_rate());
    info!("objective_function: {:?}", objective_function());

    // Fetch screener data using server function
    let screener_data = use_server_future(move || async move {
        let quote_type = quote_type.read().to_string();
        let filters = filters.read().clone();
        let active_tab = active_tab.read().to_owned();
        let data = match active_tab {
            1 | 2 => {
                match get_screener_data(
                    quote_type,
                    filters,
                    sort_field(),
                    sort_descending(),
                    offset(),
                    size(),
                    active_tab,
                ).await {
                    Ok(data) => data,
                    Err(e) => format!("Error: {e}"),
                }
            }
            3 => {
                let symbols = get_screener_symbols(
                    quote_type,
                    filters,
                    sort_field(),
                    sort_descending(),
                    offset(),
                    size()
                ).await;

                if let Ok(symbols) = symbols {
                    match get_screener_performance(
                        symbols,
                        start_date(),
                        end_date(),
                        benchmark_symbol(),
                        risk_free_rate(),
                    ).await {
                        Ok(data) => data,
                        Err(e) => format!("Error: {e}"),
                    }
                } else {
                    "".to_string()
                }
            }
            4 => {
                let symbols = get_screener_symbols(
                    quote_type,
                    filters,
                    sort_field(),
                    sort_descending(),
                    offset(),
                    size()
                ).await;

                if let Ok(symbols) = symbols {
                    match get_screener_portfolio(
                        symbols,
                        start_date(),
                        end_date(),
                        benchmark_symbol(),
                        risk_free_rate(),
                        objective_function(),
                    ).await {
                        Ok(data) => data,
                        Err(e) => format!("Error: {e}"),
                    }
                } else {
                    "".to_string()
                }
            }
            _ => "".to_string(),
        };
        data
    })?;

    rsx! {
        div {
            style: "display: flex; height: 100vh; margin: 0; padding: 20px; background-color: #f5f5f5; gap: 20px;",

            // Form Panel (20% width)
            div {
                style: "width: 20%; min-width: 250px; position: relative;",
                ScreenerForm {
                    quote_type,
                    filters,
                    sort_field,
                    sort_descending,
                    offset,
                    size,
                    active_tab,
                    screener_data,
                }
            }

            // Dashboard Panel (80% width)
            div {
                style: "width: 80%; overflow: auto; background: white; border-radius: 8px; padding: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                ScreenerDashboard {
                    active_tab,
                    screener_data,
                    benchmark_symbol,
                    start_date,
                    end_date,
                    risk_free_rate,
                    objective_function,
                }
            }
        }
    }
}