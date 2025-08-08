use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use chrono::{Local, Duration};
use std::collections::HashMap;
use crate::components::chart::ChartContainer;
use crate::components::symbols::Symbol;
use crate::components::utils::Loading;
use crate::components::table::TableContainer;
use crate::server::{get_screener_metadata, ScreenerMetadata};
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
            style: r#"
                display: flex;
                flex-direction: column;
                margin: 0;
                padding: 16px;
                background-color: #f5f5f5;
                gap: 16px;
                box-sizing: border-box;
                @media (max-width: 768px) {{
                    padding: 8px;
                    gap: 8px;
                }}
            "#,

            // Top Form Bar
            div {
                style: r#"
                    width: 100%;
                    background-color: #ffffff;
                    padding: 20px;
                    border-radius: 10px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    box-sizing: border-box;
                "#,
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

            // Dashboard Panel
            div {
                style: r#"
                    flex: 1;
                    width: 100%;
                    background: #ffffff;
                    border-radius: 8px;
                    padding: 16px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    overflow: hidden;
                    display: flex;
                    flex-direction: column;
                    box-sizing: border-box;
                "#,
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
                        1 => rsx! { TableContainer { html: content.clone(), title: "Screener Overview" } },
                        2 => rsx! { TableContainer { html: content.clone(), title: "Screener Metrics" } },
                        3 => rsx! { TableContainer { html: content.clone(), title: "Screener Performance" } },
                        4 => rsx! { ChartContainer { html: content.clone(), id: "plotly-html-element" } },
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

#[component]
pub fn ScreenerForm(
    quote_type: Signal<String>,
    filters: Signal<Vec<String>>,
    sort_field: Signal<String>,
    sort_descending: Signal<bool>,
    offset: Signal<usize>,
    size: Signal<usize>,
    active_tab: Signal<usize>,
    screener_data: Resource<String>,
) -> Element {
    let metadata = use_server_future(move || async move {
        get_screener_metadata().await.unwrap_or(ScreenerMetadata {
            exchange: vec![], region: vec![], sector: vec![],
            industry: vec![], peer_group: vec![], fund_family: vec![],
            fund_category: vec![], metrics: HashMap::new(),
        })
    })?.value().unwrap();

    let metrics = use_memo(move || {
        metadata.metrics.get(&quote_type()).cloned().unwrap_or_default()
    });

    let mut region        = use_signal(|| "us".to_string());
    let mut exchange      = use_signal(|| "Any".to_string());
    let mut sector        = use_signal(|| "Any".to_string());
    let mut industry      = use_signal(|| "Any".to_string());
    let mut peer_group    = use_signal(|| "Any".to_string());
    let mut fund_family   = use_signal(|| "Any".to_string());
    let mut fund_category = use_signal(|| "Any".to_string());

    let default_sort_field = match quote_type.read().as_str() {
        "EQUITY"|"CRYPTOCURRENCY" => ("intradaymarketcap","Market Cap"),
        "ETF"|"MUTUALFUND"        => ("fundnetassets","Fund Net Assets"),
        "INDEX"|"FUTURE"          => ("percentchange","Percent Change"),
        _                         => ("",""),
    };

    rsx! {
        div {
            style: r#"
                background-color: #f5f5f5;
                padding: 20px;
                border-radius: 10px;
                box-sizing: border-box;
            "#,
            form {
                style: r#"
                    display: flex;
                    flex-wrap: wrap;
                    gap: 16px;
                    align-items: flex-end;
                "#,
                onsubmit: move |e| {
                    screener_data.clear();
                    quote_type.set(e.values()["quote_type"].as_value());
                    let mut new_filters = vec![];
                    match quote_type.read().as_str() {
                        "EQUITY"|"INDEX" => {
                            if region.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["region","{region}"]}}"#)
                                );
                            }
                            if exchange.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["exchange","{exchange}"]}}"#)
                                );
                            }
                            if sector.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["sector","{sector}"]}}"#)
                                );
                            }
                            if industry.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["industry","{industry}"]}}"#)
                                );
                            }
                        }
                        "ETF"|"MUTUALFUND" => {
                            if region.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["region","{region}"]}}"#)
                                );
                            }
                            if exchange.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["exchange","{exchange}"]}}"#)
                                );
                            }
                            if peer_group.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["peer_group","{peer_group}"]}}"#)
                                );
                            }
                            if fund_family.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["fund_family","{fund_family}"]}}"#)
                                );
                            }
                            if fund_category.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["fund_category","{fund_category}"]}}"#)
                                );
                            }
                        }
                        "FUTURE" => {
                            if region.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["region","{region}"]}}"#)
                                );
                            }
                            if exchange.read().as_str()!="Any" {
                                new_filters.push(
                                  format!(r#"{{"operator":"eq","operands":["exchange","{exchange}"]}}"#)
                                );
                            }
                        }
                        "CRYPTOCURRENCY" => {
                            new_filters.push(
                              r#"{"operator":"eq","operands":["exchange","CCC"]}"#.to_string()
                            );
                            new_filters.push(
                              r#"{"operator":"eq","operands":["currency","USD"]}"#.to_string()
                            );
                        }
                        _ => {}
                    }
                    filters.set(new_filters);
                    sort_field.set(e.values()["sort_field"].as_value());
                    sort_descending.set(e.values()["sort_descending"].as_value()=="true");
                    offset.set(e.values()["offset"].as_value().parse::<usize>().unwrap_or(0));
                    size.set(e.values()["size"].as_value().parse::<usize>().unwrap_or(100));
                    active_tab.set(1);
                    screener_data.restart();
                },

                // Asset Class
                div { style: r#"
                        display: flex;
                        flex-direction: column;
                        min-width: 150px;
                    "#,
                    label { r#for:"quote_type","Asset Class" }
                    select {
                        class:"form-control",
                        id:"quote_type",
                        name:"quote_type",
                        required:true,
                        value:"{quote_type}",
                        onchange: move |e| {
                            quote_type.set(e.value().to_string());
                            filters.set(vec![]);
                            active_tab.set(1);
                            screener_data.clear();
                            screener_data.restart();
                        },
                        option { value:"EQUITY","Equity" }
                        option { value:"MUTUALFUND","Mutual Fund" }
                        option { value:"ETF","ETF" }
                        option { value:"INDEX","Index" }
                        option { value:"FUTURE","Future" }
                        option { value:"CRYPTOCURRENCY","Crypto" }
                    }
                }

                // Dynamic Filters based on Quote Type
                {
                    match quote_type.read().as_str() {
                        "EQUITY" | "INDEX" => rsx! {
                            // Region
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "region", "Region" }
                                select {
                                    class: "form-control",
                                    id: "region",
                                    name: "region",
                                    value: "{region}",
                                    onchange: move |e| {
                                            region.set(e.value());
                                        },
                                    option { value: "US", "United States" }
                                    {metadata.region.iter().map(|(value, name)| rsx! {
                                        option { value: "{value}", "{name}" }
                                    })}
                                }
                            }
                            // Exchange
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "exchange", "Exchange" }
                                select {
                                    class: "form-control",
                                    id: "exchange",
                                    name: "exchange",
                                    value: "{exchange}",
                                    onchange: move |e| {
                                            exchange.set(e.value());
                                        },
                                    option { value: "NMS", "NASDAQ" }
                                    option { value: "Any", "Any" }
                                    {metadata.exchange.iter().map(|(value, name)| rsx! {
                                        option { value: "{value}", "{name}" }
                                    })}
                                }
                            }
                            // Sector
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "sector", "Sector" }
                                select {
                                    class: "form-control",
                                    id: "sector",
                                    name: "sector",
                                    value: "{sector}",
                                    onchange: move |e| {
                                            sector.set(e.value());
                                        },
                                    option { value: "Any", "Any" }
                                    {metadata.sector.iter().map(|variant| rsx! {
                                        option { value: "{variant}", "{variant}" }
                                    })}
                                }
                            }
                            // Industry
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "industry", "Industry" }
                                select {
                                    class: "form-control",
                                    id: "industry",
                                    name: "industry",
                                    value: "{industry}",
                                    onchange: move |e| {
                                            industry.set(e.value());
                                        },
                                    option { value: "Any", "Any" }
                                    {metadata.industry.iter().map(|variant| rsx! {
                                        option { value: "{variant}", "{variant}" }
                                    })}
                                }
                            }
                        },
                        "ETF" | "MUTUALFUND" => rsx! {
                            // Region
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "region", "Region" }
                                select {
                                    class: "form-control",
                                    id: "region",
                                    name: "region",
                                    value: "{region}",
                                    onchange: move |e| {
                                            region.set(e.value());
                                        },
                                    option { value: "us", "United States" }
                                    {metadata.region.iter().map(|(value, name)| rsx! {
                                        option { value: "{value}", "{name}" }
                                    })}
                                }
                            }
                            // Exchange
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "exchange", "Exchange" }
                                select {
                                    class: "form-control",
                                    id: "exchange",
                                    name: "exchange",
                                    value: "{exchange}",
                                    onchange: move |e| {
                                            exchange.set(e.value());
                                        },
                                    option { value: "Any", "Any" }
                                    {metadata.exchange.iter().map(|(value, name)| rsx! {
                                        option { value: "{value}", "{name}" }
                                    })}
                                }
                            }
                            // Peer Group
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "peer_group", "Peer Group" }
                                select {
                                    class: "form-control",
                                    id: "peer_group",
                                    name: "peer_group",
                                    value: "{peer_group}",
                                    onchange: move |e| {
                                            peer_group.set(e.value());
                                        },
                                    option { value: "Any", "Any" }
                                    {metadata.peer_group.iter().map(|variant| rsx! {
                                        option { value: "{variant}", "{variant}" }
                                    })}
                                }
                            }
                            // Fund Family
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "fund_family", "Fund Family" }
                                select {
                                    class: "form-control",
                                    id: "fund_family",
                                    name: "fund_family",
                                    value: "{fund_family}",
                                    onchange: move |e| {
                                            fund_family.set(e.value());
                                        },
                                    option { value: "Any", "Any" }
                                    {metadata.fund_family.iter().map(|variant| rsx! {
                                        option { value: "{variant}", "{variant}" }
                                    })}
                                }
                            }
                            // Fund Category
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "fund_category", "Fund Category" }
                                select {
                                    class: "form-control",
                                    id: "fund_category",
                                    name: "fund_category",
                                    value: "{fund_category}",
                                    onchange: move |e| {
                                            fund_category.set(e.value());
                                        },
                                    option { value: "Any", "Any" }
                                    {metadata.fund_category.iter().map(|variant| rsx! {
                                        option { value: "{variant}", "{variant}" }
                                    })}
                                }
                            }
                        },
                        "FUTURE" => rsx! {
                            // Region
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "region", "Region" }
                                select {
                                    class: "form-control",
                                    id: "region",
                                    name: "region",
                                    value: "{region}",
                                    onchange: move |e| {
                                            region.set(e.value());
                                        },
                                    option { value: "us", "United States" }
                                    {metadata.region.iter().map(|(value, name)| rsx! {
                                        option { value: "{value}", "{name}" }
                                    })}
                                }
                            }
                            // Exchange
                            div { style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    min-width: 120px;
                                "#,
                                label { r#for: "exchange", "Exchange" }
                                select {
                                    class: "form-control",
                                    id: "exchange",
                                    name: "exchange",
                                    value: "{exchange}",
                                    onchange: move |e| {
                                            exchange.set(e.value());
                                        },
                                    option { value: "Any", "Any" }
                                    {metadata.exchange.iter().map(|(value, name)| rsx! {
                                        option { value: "{value}", "{name}" }
                                    })}
                                }
                            }
                        },
                        "CRYPTOCURRENCY" => rsx! {}, // No special fields
                        _ => rsx! {},
                    }
                }

                // Sort By
                div { style: r#"
                        display: flex;
                        flex-direction: column;
                        min-width: 140px;
                    "#,
                    label { r#for:"sort_field", "Sort By" }
                    select {
                        class:"form-control",
                        id:"sort_field",
                        name:"sort_field",
                        option { value:"{default_sort_field.0}","{default_sort_field.1}" }
                        {metrics.read().iter().map(|(key,metric)| rsx!{
                            option { value:"{key}","{metric.name}" }
                        })}
                    }
                }

                // Order
                div { style: r#"
                        display: flex;
                        flex-direction: column;
                        min-width: 120px;
                    "#,
                    label { r#for:"sort_descending", "Order" }
                    select {
                        class:"form-control",
                        id:"sort_descending",
                        name:"sort_descending",
                        value:"{sort_descending}",
                        option { value:"true", "Descending" }
                        option { value:"false", "Ascending" }
                    }
                }

                // Offset
                div { style: r#"
                        display: flex;
                        flex-direction: column;
                        min-width: 100px;
                    "#,
                    label { r#for:"offset", "Offset" }
                    input {
                        class:"form-control",
                        id:"offset",
                        name:"offset",
                        r#type:"number",
                        min:"0",
                        step:"50",
                        required:true,
                        value:"{offset.to_string()}"
                    }
                }

                // Size
                div { style: r#"
                        display: flex;
                        flex-direction: column;
                        min-width: 100px;
                    "#,
                    label { r#for:"size", "Size" }
                    input {
                        class:"form-control",
                        id:"size",
                        name:"size",
                        r#type:"number",
                        min:"5",
                        step:"5",
                        max:"50",
                        required:true,
                        value:"{size.to_string()}"
                    }
                }

                // Submit
                div { style: r#"
                        display: flex;
                        align-items: center;
                        padding-top: 8px;
                    "#,
                    button {
                        class:"btn btn-primary",
                        r#type:"submit",
                        formnovalidate:true,
                        "Run Screener"
                    }
                }
            }
        }
    }
}


#[component]
pub fn ScreenerTickersForm(
    benchmark_symbol: Signal<String>,
    start_date: Signal<String>,
    end_date: Signal<String>,
    risk_free_rate: Signal<f64>,
    objective_function: Signal<String>,
    screener_data: Resource<String>,
    active_tab: Signal<usize>,
) -> Element {
    rsx! {
        div {
            style: "background-color: #f5f5f5; padding: 20px; border-radius: 10px; margin-top: 5px; width: 100%; display: flex; flex-direction: column;",

            form {
                style: "width: 100%; display: flex; flex-wrap: wrap; gap: 20px; align-items: flex-end;",
                onsubmit: move |e| {
                    screener_data.clear();
                    start_date.set(e.values()["start_date"].as_value());
                    end_date.set(e.values()["end_date"].as_value());
                    risk_free_rate.set(
                        e.values()["risk_free_rate"]
                            .as_value()
                            .parse::<f64>()
                            .unwrap_or(0.0)
                    );
                    objective_function.set(e.values()["objective_function"].as_value());
                    screener_data.restart();
                },

                // Start Date
                div {
                    style: "flex: 1; min-width: 180px;",
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
                div {
                    style: "flex: 1; min-width: 180px;",
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


                // Benchmark
                Symbol { symbol: benchmark_symbol, title: "Benchmark Symbol" }


                // Risk-Free Rate
                div {
                    style: "flex: 1; min-width: 180px;",
                    label { r#for: "risk_free_rate", "Risk-Free Rate" }
                    input {
                        class: "form-control",
                        id: "risk_free_rate",
                        name: "risk_free_rate",
                        r#type: "text",
                        required: true,
                        value: "{risk_free_rate.to_string()}",
                    }
                }

                if *active_tab.read() == 4 {
                    // Objective Function
                    div { style: "flex: 1; min-width: 200px;",
                        label { r#for: "objective_function", "Objective" }
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
                }

                // Submit Button
                div {
                    style: "min-width: 180px;",
                    button {
                        class: "btn btn-primary",
                        r#type: "submit",
                        formnovalidate: true,
                        "Submit"
                    }
                }
            }
        }
    }
}
