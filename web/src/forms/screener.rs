use dioxus::prelude::*;
use std::collections::HashMap;
use crate::server::{get_screener_metadata, ScreenerMetadata};

#[component]
pub fn ScreenerForm (
    quote_type: Signal<String>,
    filters: Signal<Vec<String>>,
    sort_field: Signal<String>,
    sort_descending: Signal<bool>,
    offset: Signal<usize>,
    size: Signal<usize>,
    active_tab: Signal<usize>,
    screener_data: Resource<String>,
) -> Element {
    // Fetch metadata from server
    let metadata = use_server_future(move || async move {
        get_screener_metadata().await.unwrap_or(ScreenerMetadata {
            exchange: vec![],
            region: vec![],
            sector: vec![],
            industry: vec![],
            peer_group: vec![],
            fund_family: vec![],
            fund_category: vec![],
            metrics: HashMap::new(),
        })
    })?.value().unwrap();

    // Get metrics for the current quote_type
    let metrics = use_memo(move || {
        metadata.metrics.get(&quote_type()).cloned().unwrap_or_default()
    });

    let mut region = use_signal(|| "us".to_string());
    let mut exchange = use_signal(|| "Any".to_string());
    let mut sector = use_signal(|| "Any".to_string());
    let mut industry = use_signal(|| "Any".to_string());
    let mut peer_group = use_signal(|| "Any".to_string());
    let mut fund_family = use_signal(|| "Any".to_string());
    let mut fund_category = use_signal(|| "Any".to_string());

    let default_sort_field = match quote_type.read().as_str() {
        "EQUITY" | "CRYPTOCURRENCY" => ("intradaymarketcap", "Market Cap (Intraday)"),
        "ETF" | "MUTUALFUND" => ("fundnetassets", "Fund Net Assets"),
        "INDEX" | "FUTURE" => ("percentchange", "Percent Change"),
        _ => ("", ""),
    };

    rsx! {
        div {
            style: "background-color: #f5f5f5; padding: 20px; border-radius: 10px; width: 300px; margin-right: 20px; margin-top: 5px;",

            form {
                style: "width: 100%;",
                onsubmit: move |e| {
                    screener_data.clear();
                    quote_type.set(e.values()["quote_type"].as_value());
                    let mut new_filters = vec![];
                    match quote_type.read().as_str() {
                        "EQUITY" | "INDEX" => {
                            if region.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["region", "{region}"]}}"#));
                            }
                            if exchange.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["exchange", "{exchange}"]}}"#));
                            }
                            if sector.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["sector", "{sector}"]}}"#));
                            }
                            if industry.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["industry", "{industry}"]}}"#));
                            }
                        }
                        "ETF" | "MUTUALFUND" => {
                            if region.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["region", "{region}"]}}"#));
                            }
                            if exchange.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["exchange", "{exchange}"]}}"#));
                            }
                            if peer_group.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["peer_group", "{peer_group}"]}}"#));
                            }
                            if fund_family.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["fund_family", "{fund_family}"]}}"#));
                            }
                            if fund_category.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["fund_category", "{fund_category}"]}}"#));
                            }
                        }
                        "FUTURE" => {
                            if region.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["region", "{region}"]}}"#));
                            }
                            if exchange.read().as_str() != "Any" {
                                new_filters.push(format!(r#"{{"operator": "eq", "operands": ["exchange", "{exchange}"]}}"#));
                            }
                        }
                        "CRYPTOCURRENCY" => {
                            new_filters.push(r#"{"operator": "eq", "operands": ["exchange", "CCC"]}"#.to_string());
                            new_filters.push(r#"{"operator": "eq", "operands": ["currency", "USD"]}"#.to_string());
                        }
                        _ => {}
                    }
                    filters.set(new_filters);
                    sort_field.set(e.values()["sort_field"].as_value());
                    sort_descending.set(e.values()["sort_descending"].as_value() == "true");
                    offset.set(e.values()["offset"].as_value().parse::<usize>().unwrap_or(0));
                    size.set(e.values()["size"].as_value().parse::<usize>().unwrap_or(100));
                    active_tab.set(1);
                    screener_data.restart();
                },

                // Quote Type
                div { style: "margin-bottom: 15px;",
                    label { r#for: "quote_type", "Asset Class" }
                    select {
                        class: "form-control",
                        id: "quote_type",
                        name: "quote_type",
                        required: true,
                        value: "{quote_type}",
                        onchange: move |e| {
                            quote_type.set(e.value().to_string());
                            filters.set(vec![]);
                            active_tab.set(1);
                            screener_data.clear();
                            screener_data.restart();
                        },
                        option { value: "EQUITY", "Equity" }
                        option { value: "MUTUALFUND", "Mutual Fund" }
                        option { value: "ETF", "ETF" }
                        option { value: "INDEX", "Index" }
                        option { value: "FUTURE", "Future" }
                        option { value: "CRYPTOCURRENCY", "Crypto" }
                    }
                }

                // Dynamic Filters based on Quote Type
                {
                    match quote_type.read().as_str() {
                        "EQUITY" | "INDEX" => rsx! {
                            // Region
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                            div { style: "margin-bottom: 15px;",
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
                div { style: "margin-bottom: 15px;",
                    label { r#for: "sort_field", "Sort By" }
                    select {
                        class: "form-control",
                        id: "sort_field",
                        name: "sort_field",
                        option { value: "{default_sort_field.0}", "{default_sort_field.1}" }
                        {metrics.read().iter().map(|(key, metric)| rsx! {
                            option { value: "{key}", "{metric.name}" }
                        })}
                    }
                }

                // Sort Descending
                div { style: "margin-bottom: 15px;",
                    label { r#for: "sort_descending", "Sort Order" }
                    select {
                        class: "form-control",
                        id: "sort_descending",
                        name: "sort_descending",
                        value: "{sort_descending}",
                        option { value: "true", "Descending" }
                        option { value: "false", "Ascending" }
                    }
                }

                // Offset
                div { style: "margin-bottom: 15px;",
                    label { r#for: "offset", "Offset" }
                    input {
                        class: "form-control",
                        id: "offset",
                        name: "offset",
                        r#type: "number",
                        min: "0",
                        step: "50",
                        required: true,
                        value: "{offset.to_string()}",
                    }
                }

                // Size
                div { style: "margin-bottom: 15px;",
                    label { r#for: "size", "Size" }
                    input {
                        class: "form-control",
                        id: "size",
                        name: "size",
                        r#type: "number",
                        min: "5",
                        step: "5",
                        max: "50",
                        required: true,
                        value: "{size.to_string()}",
                    }
                }

                // Submit Button
                button {
                    class: "btn btn-primary",
                    r#type: "submit",
                    formnovalidate: true,
                    "Run Screener"
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
                    benchmark_symbol.set(e.values()["benchmark_symbol"].as_value());
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
                div {
                    style: "flex: 1; min-width: 200px;",
                    label { r#for: "benchmark_symbol", "Benchmark" }
                    input {
                        class: "form-control",
                        id: "benchmark_symbol",
                        name: "benchmark_symbol",
                        r#type: "text",
                        list: "all_symbols",
                        required: true,
                        value: "{benchmark_symbol}",
                    }
                }


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
