use dioxus::prelude::*;
use dioxus::logger::tracing::info;

#[component]
pub fn WeightAllocationInputs(
    symbols: Signal<String>,
    weight_mode: Signal<Option<String>>,
    allocations: Signal<Option<Vec<f64>>>,
    weight_ranges: Signal<Option<Vec<(f64, f64)>>>,
    allocation_error: Signal<String>,
    weights_error: Signal<String>,
) -> Element {
    // Compute allocation and weight values
    let allocation_values: Vec<String> = symbols()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .enumerate()
        .map(|(i, _)| {
            allocations
                .read()
                .as_ref()
                .and_then(|a| a.get(i).map(|v| v.to_string()))
                .unwrap_or("0.0".to_string())
        })
        .collect();

    let min_weight_values: Vec<String> = symbols()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .enumerate()
        .map(|(i, _)| {
            weight_ranges
                .read()
                .as_ref()
                .and_then(|w| w.get(i).map(|&(min, _)| min.to_string()))
                .unwrap_or("0.1".to_string())
        })
        .collect();

    let max_weight_values: Vec<String> = symbols()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .enumerate()
        .map(|(i, _)| {
            weight_ranges
                .read()
                .as_ref()
                .and_then(|w| w.get(i).map(|&(_, max)| max.to_string()))
                .unwrap_or("0.5".to_string())
        })
        .collect();

    info!("WeightAllocationInputs: symbols: {:?}", symbols());
    info!("WeightAllocationInputs: weight_mode: {:?}", weight_mode());
    info!("WeightAllocationInputs: allocation_values: {:?}", allocation_values);
    info!("WeightAllocationInputs: min_weight_values: {:?}", min_weight_values);
    info!("WeightAllocationInputs: max_weight_values: {:?}", max_weight_values);

    rsx! {
        if let Some(mode) = weight_mode.read().as_deref() {
            div {
                style: r#"
                    display: flex;
                    flex-direction: column;
                    width: 100%;
                    gap: 8px;
                "#,
                if mode == "allocation" {
                    div {
                        style: r#"color: red;"#,
                        "{allocation_error}"
                    }
                    for (i , symbol) in symbols().split(',').filter(|s| !s.trim().is_empty()).enumerate() {
                        div {
                            style: r#"
                                display: flex;
                                flex-direction: column;
                                min-width: 120px;
                            "#,
                            label {
                                r#for: format!("allocation_{}", i),
                                "{symbol.trim()} Allocation"
                            }
                            input {
                                class: "form-control",
                                id: format!("allocation_{}", i),
                                name: format!("allocation_{}", i),
                                r#type: "number",
                                step: "0.01",
                                min: "0",
                                max: "1",
                                required: true,
                                value: "{allocation_values.get(i).cloned().unwrap_or(\"0.0\".to_string())}"
                            }
                        }
                    }
                } else if mode == "weights" {
                    div {
                        style: r#"color: red;"#,
                        "{weights_error}"
                    }
                    for (i , symbol) in symbols().split(',').filter(|s| !s.trim().is_empty()).enumerate() {
                        div {
                            style: r#"
                                display: flex;
                                gap: 8px;
                                width: 100%;
                            "#,
                            div {
                                style: r#"
                                    flex: 1;
                                    display: flex;
                                    flex-direction: column;
                                "#,
                                label {
                                    r#for: format!("min_weight_{}", i),
                                    "{symbol.trim()} Min Weight"
                                }
                                input {
                                    class: "form-control",
                                    id: format!("min_weight_{}", i),
                                    name: format!("min_weight_{}", i),
                                    r#type: "number",
                                    step: "0.01",
                                    min: "0",
                                    max: "1",
                                    required: true,
                                    value: "{min_weight_values.get(i).cloned().unwrap_or(\"0.1\".to_string())}"
                                }
                            }
                            div {
                                style: r#"
                                    flex: 1;
                                    display: flex;
                                    flex-direction: column;
                                "#,
                                label {
                                    r#for: format!("max_weight_{}", i),
                                    "{symbol.trim()} Max Weight"
                                }
                                input {
                                    class: "form-control",
                                    id: format!("max_weight_{}", i),
                                    name: format!("max_weight_{}", i),
                                    r#type: "number",
                                    step: "0.01",
                                    min: "0",
                                    max: "1",
                                    required: true,
                                    value: "{max_weight_values.get(i).cloned().unwrap_or(\"0.5\".to_string())}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}