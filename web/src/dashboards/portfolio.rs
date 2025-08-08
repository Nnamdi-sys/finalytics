use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::components::display::PortfolioDisplay;
use crate::server::{get_portfolio_charts, PortfolioTabs};
use crate::components::symbols::{Symbol, Symbols};
use crate::components::weights::WeightAllocationInputs;

#[component]
pub fn Portfolio() -> Element {
    let symbols = use_signal(|| "AAPL,MSFT,NVDA,BTC-USD".to_string());
    let benchmark_symbol = use_signal(|| "^GSPC".to_string());
    let mut start_date = use_signal(|| "2023-01-01".to_string());
    let mut end_date = use_signal(|| "2024-12-31".to_string());
    let mut interval = use_signal(|| "1d".to_string());
    let mut confidence_level = use_signal(|| 0.95);
    let mut risk_free_rate = use_signal(|| 0.02);
    let mut objective_function = use_signal(|| "max_sharpe".to_string());
    let mut weight_mode = use_signal::<Option<String>>(|| None);
    let mut allocations = use_signal::<Option<Vec<f64>>>(|| None);
    let mut weight_ranges = use_signal::<Option<Vec<(f64, f64)>>>(|| None);
    let mut allocation_error = use_signal(|| String::new());
    let mut weights_error = use_signal(|| String::new());

    info!("Portfolio: symbols: {:?}", symbols());
    info!("Portfolio: benchmark: {:?}", benchmark_symbol());
    info!("Portfolio: start: {:?}", start_date());
    info!("Portfolio: end: {:?}", end_date());
    info!("Portfolio: interval: {:?}", interval());
    info!("Portfolio: confidence: {:?}", confidence_level());
    info!("Portfolio: risk_free: {:?}", risk_free_rate());
    info!("Portfolio: objective: {:?}", objective_function());
    info!("Portfolio: weight_mode: {:?}", weight_mode());
    info!("Portfolio: allocations: {:?}", allocations());
    info!("Portfolio: weight_ranges: {:?}", weight_ranges());

    let mut charts = use_server_future(move || async move {
        match get_portfolio_charts(
            symbols().split(',').map(str::to_string).collect(),
            benchmark_symbol(),
            start_date(),
            end_date(),
            interval(),
            confidence_level(),
            risk_free_rate(),
            objective_function(),
            weight_ranges(),
            allocations(),
        )
            .await
        {
            Ok(charts) => charts,
            Err(e) => PortfolioTabs {
                optimization_chart: Some(format!("Error: {e}")),
                performance_chart: format!("Error: {e}"),
                performance_stats_table: format!("Error: {e}"),
                returns_table: format!("Error: {e}"),
                returns_chart: format!("Error: {e}"),
                returns_matrix: format!("Error: {e}"),
            },
        }
    })?;

    // Sync allocations and weight_ranges with symbols count
    use_effect(move || {
        let symbol_count = symbols().split(',').filter(|s| !s.trim().is_empty()).count();
        match *weight_mode.read() {
            Some(ref mode) if mode == "allocation" => {
                if allocations.read().as_ref().map_or(true, |a| a.len() != symbol_count) {
                    allocations.set(Some(vec![1.0 / symbol_count as f64; symbol_count]));
                }
            }
            Some(ref mode) if mode == "weights" => {
                if weight_ranges.read().as_ref().map_or(true, |w| w.len() != symbol_count) {
                    weight_ranges.set(Some(vec![(0.1, 0.5); symbol_count]));
                }
            }
            _ => {
                allocations.set(None);
                weight_ranges.set(None);
            }
        }
    });

    // Validate inputs on form submission
    let mut validate_inputs = move |allocs: Option<&Vec<f64>>, ranges: Option<&Vec<(f64, f64)>>| {
        allocation_error.set(String::new());
        weights_error.set(String::new());

        match *weight_mode.read() {
            Some(ref mode) if mode == "allocation" => {
                if let Some(allocs) = allocs {
                    let sum: f64 = allocs.iter().sum();
                    if (sum - 1.0).abs() > 0.0001 {
                        allocation_error.set("Sum of allocations must equal 1.".to_string());
                        return false;
                    }
                    for &alloc in allocs.iter() {
                        if !(0.0..=1.0).contains(&alloc) {
                            allocation_error.set("Allocations must be between 0 and 1.".to_string());
                            return false;
                        }
                    }
                } else {
                    allocation_error.set("Allocations not provided.".to_string());
                    return false;
                }
            }
            Some(ref mode) if mode == "weights" => {
                if let Some(ranges) = ranges {
                    for (i, &(min, max)) in ranges.iter().enumerate() {
                        if !(0.0..=1.0).contains(&min) || !(0.0..=1.0).contains(&max) {
                            weights_error.set(format!("Weight range for symbol {} must be between 0 and 1.", i + 1));
                            return false;
                        }
                        if min > max {
                            weights_error.set(format!("Min weight must be <= max weight for symbol {}.", i + 1));
                            return false;
                        }
                    }
                } else {
                    weights_error.set("Weight ranges not provided.".to_string());
                    return false;
                }
            }
            _ => {}
        }
        true
    };

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

            // Form Bar
            div {
                style: r#"
                    width: 100%;
                    background-color: #ffffff;
                    padding: 20px;
                    border-radius: 10px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    box-sizing: border-box;
                "#,
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
                            let new_weight_mode = e.values().get("weight_mode").map(|v| v.as_value()).filter(|v| !v.is_empty());
                            weight_mode.set(new_weight_mode.clone());

                            let new_allocs = if new_weight_mode.as_deref() == Some("allocation") {
                                Some(
                                    symbols()
                                        .split(',')
                                        .filter(|s| !s.trim().is_empty())
                                        .enumerate()
                                        .map(|(i, _)| {
                                            e.values()
                                                .get(&format!("allocation_{i}"))
                                                .map(|v| v.as_value().parse::<f64>().unwrap_or(0.0))
                                                .unwrap_or(0.0)
                                        })
                                        .collect::<Vec<f64>>()
                                )
                            } else {
                                None
                            };

                            let new_ranges = if new_weight_mode.as_deref() == Some("weights") {
                                Some(
                                    symbols()
                                        .split(',')
                                        .filter(|s| !s.trim().is_empty())
                                        .enumerate()
                                        .map(|(i, _)| {
                                            let min = e.values()
                                                .get(&format!("min_weight_{i}"))
                                                .map(|v| v.as_value().parse::<f64>().unwrap_or(0.1))
                                                .unwrap_or(0.1);
                                            let max = e.values()
                                                .get(&format!("max_weight_{i}"))
                                                .map(|v| v.as_value().parse::<f64>().unwrap_or(0.5))
                                                .unwrap_or(0.5);
                                            (min, max)
                                        })
                                        .collect::<Vec<(f64, f64)>>()
                                )
                            } else {
                                None
                            };

                            if validate_inputs(new_allocs.as_ref(), new_ranges.as_ref()) {
                                charts.clear();
                                start_date.set(e.values()["start_date"].as_value());
                                end_date.set(e.values()["end_date"].as_value());
                                interval.set(e.values()["interval"].as_value());
                                confidence_level.set(
                                    e.values()["confidence_level"]
                                        .as_value()
                                        .parse::<f64>()
                                        .unwrap_or(0.95)
                                );
                                risk_free_rate.set(
                                    e.values()["risk_free_rate"]
                                        .as_value()
                                        .parse::<f64>()
                                        .unwrap_or(0.02)
                                );
                                objective_function.set(e.values()["objective_function"].as_value());
                                allocations.set(new_allocs);
                                weight_ranges.set(new_ranges);
                                charts.restart();
                            }
                        },

                        Symbols { symbols: symbols }
                        Symbol { symbol: benchmark_symbol, title: "Benchmark Symbol" }
                        div {
                            style: r#"display: flex; gap: 8px;"#,
                            div {
                                style: r#"flex: 1; display: flex; flex-direction: column;"#,
                                label { r#for: "start_date", "Start Date" }
                                input {
                                    class: "form-control",
                                    id: "start_date",
                                    name: "start_date",
                                    r#type: "date",
                                    required: true,
                                    value: "{start_date}"
                                }
                            }
                            div {
                                style: r#"flex: 1; display: flex; flex-direction: column;"#,
                                label { r#for: "end_date", "End Date" }
                                input {
                                    class: "form-control",
                                    id: "end_date",
                                    name: "end_date",
                                    r#type: "date",
                                    required: true,
                                    value: "{end_date}"
                                }
                            }
                        }
                        div {
                            style: r#"display: flex; flex-direction: column; min-width: 100px;"#,
                            label { r#for: "interval", "Interval" }
                            select {
                                class: "form-control",
                                id: "interval",
                                name: "interval",
                                required: true,
                                value: "{interval}",
                                option { value: "1d", "Daily" }
                                option { value: "1wk", "Weekly" }
                                option { value: "1mo", "Monthly" }
                                option { value: "3mo", "Quarterly" }
                            }
                        }
                        div {
                            style: r#"display: flex; gap: 8px;"#,
                            div {
                                style: r#"flex: 1; display: flex; flex-direction: column;"#,
                                label { r#for: "confidence_level", "Confidence Level" }
                                input {
                                    class: "form-control",
                                    id: "confidence_level",
                                    name: "confidence_level",
                                    r#type: "number",
                                    step: "0.01",
                                    required: true,
                                    value: "{confidence_level}"
                                }
                            }
                            div {
                                style: r#"flex: 1; display: flex; flex-direction: column;"#,
                                label { r#for: "risk_free_rate", "Risk Free Rate" }
                                input {
                                    class: "form-control",
                                    id: "risk_free_rate",
                                    name: "risk_free_rate",
                                    r#type: "number",
                                    step: "0.01",
                                    required: true,
                                    value: "{risk_free_rate}"
                                }
                            }
                        }
                        div {
                            style: r#"display: flex; flex-direction: column; min-width: 150px;"#,
                            label { r#for: "objective_function", "Objective Function" }
                            select {
                                class: "form-control",
                                id: "objective_function",
                                name: "objective_function",
                                required: true,
                                value: "{objective_function}",
                                option { value: "max_sharpe", "Max Sharpe" }
                                option { value: "min_vol", "Min Volatility" }
                                option { value: "max_return", "Max Return" }
                                option { value: "min_var", "Min VaR" }
                                option { value: "min_cvar", "Min CVaR" }
                                option { value: "min_drawdown", "Min Drawdown" }
                            }
                        }
                        div {
                            style: r#"display: flex; flex-direction: column; min-width: 150px;"#,
                            label { r#for: "weight_mode", "Weight Mode" }
                            select {
                                class: "form-control",
                                id: "weight_mode",
                                name: "weight_mode",
                                value: "{weight_mode.read().as_deref().unwrap_or(\"\")}",
                                onchange: move |e| {
                                    let value = e.value();
                                    weight_mode.set(if value.is_empty() { None } else { Some(value) });
                                },
                                option { value: "", "None" }
                                option { value: "allocation", "Allocation" }
                                option { value: "weights", "Weights" }
                            }
                        }
                        WeightAllocationInputs {
                            symbols: symbols,
                            weight_mode: weight_mode,
                            allocations: allocations,
                            weight_ranges: weight_ranges,
                            allocation_error: allocation_error,
                            weights_error: weights_error
                        }
                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            "Generate Report"
                        }
                    }
                }
            }

            // Render PortfolioTabs Component
            div {
                style: r#"display: flex; gap: 16px;"#,
                PortfolioDisplay { charts: charts, weight_mode: weight_mode }
            }
        }
    }
}