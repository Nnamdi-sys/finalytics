use crate::server::search_symbols;
use dioxus::prelude::*;

#[component]
pub fn Symbol(symbol: Signal<String>, title: String) -> Element {
    let mut input_value = use_signal(|| symbol.read().clone()); // Tracks user input
    let mut suggestions = use_signal::<Vec<(String, String)>>(|| vec![]); // (symbol, name) pairs
    let mut is_dropdown_open = use_signal(|| false); // Controls dropdown visibility
    let mut has_interacted = use_signal(|| false); // Tracks user interaction
    let mut is_selected = use_signal(|| false); // Tracks if a suggestion was selected
    let mut search_generation = use_signal(|| 0u64); // Guards against stale async responses

    // Fetch suggestions from server when input changes
    use_effect(move || {
        if !*has_interacted.read() || *is_selected.read() {
            *suggestions.write() = vec![];
            *is_dropdown_open.write() = false;
            return;
        }
        let query = input_value.read().clone();
        if query.is_empty() {
            *suggestions.write() = vec![];
            *is_dropdown_open.write() = false;
            return;
        }

        // Increment generation to invalidate any in-flight requests
        *search_generation.write() += 1;
        let current_gen = *search_generation.read();

        spawn(async move {
            match search_symbols(query).await {
                Ok(results) => {
                    // Only apply results if this is still the latest search
                    if *search_generation.read() == current_gen {
                        *is_dropdown_open.write() = !results.is_empty();
                        *suggestions.write() = results;
                    }
                }
                Err(_) => {
                    if *search_generation.read() == current_gen {
                        *suggestions.write() = vec![];
                        *is_dropdown_open.write() = false;
                    }
                }
            }
        });
    });

    // Handle input change
    let oninput = move |event: FormEvent| {
        *has_interacted.write() = true; // Mark as interacted
        *is_selected.write() = false; // Reset selection on new input
        let new_value = event.value();
        input_value.set(new_value.clone());
        symbol.set(new_value); // Sync with parent signal
    };

    // Handle suggestion selection
    let mut select_suggestion = move |selected_symbol: String| {
        symbol.set(selected_symbol.clone());
        input_value.set(selected_symbol);
        *is_dropdown_open.write() = false;
        *is_selected.write() = true; // Mark as selected to suppress suggestions
        *has_interacted.write() = true; // Ensure interaction is marked
    };

    // Handle click outside to close dropdown
    let onclick_outside = move |_| {
        *is_dropdown_open.write() = false;
        *is_selected.write() = true; // Treat as final selection
    };

    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                flex: 1;
                min-width: 120px;
                position: relative;
            "#,
            label { r#for: "symbol", "{title}" }
            input {
                class: "form-control",
                id: "{title}" ,
                name: "{title}" ,
                r#type: "text",
                required: true,
                value: "{input_value}",
                oninput: oninput,
                autocomplete: "off", // Disable browser autocomplete
                onfocus: move |_| {
                    *has_interacted.write() = true; // Mark as interacted on focus
                    *is_selected.write() = false; // Allow suggestions on focus
                    *is_dropdown_open.write() = !suggestions.read().is_empty();
                },
            }
            if *is_dropdown_open.read() {
                div {
                    style: r#"
                        position: absolute;
                        top: 100%;
                        left: 0;
                        right: 0;
                        background-color: #fff;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                        max-height: 200px;
                        overflow-y: auto;
                        z-index: 1100;
                    "#,
                    for (sym, name) in suggestions.read().iter().cloned() {
                        div {
                            class: "dropdown-item",
                            style: r#"
                                padding: 8px 12px;
                                cursor: pointer;
                                white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                            "#,
                            onclick: move |_| select_suggestion(sym.clone()),
                            "{sym} - {name}"
                        }
                    }
                }
            }
            // Invisible overlay to capture clicks outside
            if *is_dropdown_open.read() {
                div {
                    style: r#"
                        position: fixed;
                        top: 0;
                        left: 0;
                        width: 100vw;
                        height: 100vh;
                        z-index: 1099;
                    "#,
                    onclick: onclick_outside,
                }
            }
        }
    }
}

#[component]
pub fn Symbols(symbols: Signal<String>) -> Element {
    let mut input_value = use_signal(|| String::new()); // Tracks current input
    let mut selected_symbols = use_signal(|| {
        if symbols.read().is_empty() {
            vec![]
        } else {
            symbols
                .read()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
        }
    }); // Internal Vec of selected symbols
    let mut suggestions = use_signal::<Vec<(String, String)>>(|| vec![]); // (symbol, name) pairs
    let mut is_dropdown_open = use_signal(|| false);
    let mut has_interacted = use_signal(|| false);
    let mut is_selected = use_signal(|| false);
    let mut search_generation = use_signal(|| 0u64); // Guards against stale async responses

    // Sync selected_symbols with symbols signal
    use_effect(move || {
        let new_value = selected_symbols.read().join(",");
        symbols.set(new_value);
    });

    // Fetch suggestions from server when input changes
    use_effect(move || {
        if !*has_interacted.read() || *is_selected.read() {
            *suggestions.write() = vec![];
            *is_dropdown_open.write() = false;
            return;
        }
        let query = input_value.read().clone();
        if query.is_empty() {
            *suggestions.write() = vec![];
            *is_dropdown_open.write() = false;
            return;
        }

        // Increment generation to invalidate any in-flight requests
        *search_generation.write() += 1;
        let current_gen = *search_generation.read();
        let current_symbols = selected_symbols.read().clone();

        spawn(async move {
            match search_symbols(query).await {
                Ok(results) => {
                    // Only apply results if this is still the latest search
                    if *search_generation.read() == current_gen {
                        // Exclude already-selected symbols from suggestions
                        let filtered: Vec<(String, String)> = results
                            .into_iter()
                            .filter(|(sym, _)| !current_symbols.contains(sym))
                            .collect();
                        *is_dropdown_open.write() = !filtered.is_empty();
                        *suggestions.write() = filtered;
                    }
                }
                Err(_) => {
                    if *search_generation.read() == current_gen {
                        *suggestions.write() = vec![];
                        *is_dropdown_open.write() = false;
                    }
                }
            }
        });
    });

    // Handle input change
    let oninput = move |event: FormEvent| {
        *has_interacted.write() = true;
        *is_selected.write() = false;
        input_value.set(event.value());
    };

    // Handle suggestion selection
    let mut select_suggestion = move |selected_symbol: String| {
        selected_symbols.write().push(selected_symbol.clone());
        input_value.set(String::new()); // Clear input
        *is_dropdown_open.write() = false;
        *is_selected.write() = true;
        *has_interacted.write() = true;
    };

    // Handle Enter key to select top suggestion
    let onkeydown = move |event: KeyboardEvent| {
        if event.code() == Code::Enter && !suggestions.read().is_empty() {
            event.prevent_default();
            select_suggestion(suggestions.read()[0].0.clone());
        }
    };

    // Handle remove symbol
    let mut remove_symbol = move |symbol_to_remove: String| {
        selected_symbols
            .write()
            .retain(|sym| sym != &symbol_to_remove);
    };

    // Handle click outside to close dropdown
    let onclick_outside = move |_| {
        *is_dropdown_open.write() = false;
        *is_selected.write() = true;
    };

    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                flex: 1;
                min-width: 100%;
                position: relative;
            "#,
            label { r#for: "symbols", "Symbols" }
            // Display selected symbols as cancellable tags
            div {
                style: r#"
                    display: flex;
                    flex-wrap: wrap;
                    gap: 8px;
                    margin-bottom: 8px;
                "#,
                for sym in selected_symbols.read().iter().cloned() {
                    span {
                        class: "badge bg-primary",
                        style: r#"
                            display: inline-flex;
                            align-items: center;
                            padding: 4px 8px;
                            font-size: 14px;
                        "#,
                        "{sym}"
                        i {
                            class: "bi bi-x ms-1",
                            style: r#"
                                cursor: pointer;
                                font-size: 16px;
                            "#,
                            onclick: move |_| remove_symbol(sym.clone()),
                        }
                    }
                }
            }
            input {
                class: "form-control",
                id: "symbols",
                name: "symbols",
                r#type: "text",
                value: "{input_value}",
                placeholder: "Search for symbols here...",
                oninput: oninput,
                onkeydown: onkeydown,
                autocomplete: "off",
                onfocus: move |_| {
                    *has_interacted.write() = true;
                    *is_selected.write() = false;
                    *is_dropdown_open.write() = !suggestions.read().is_empty();
                },
            }
            if *is_dropdown_open.read() {
                div {
                    style: r#"
                        position: absolute;
                        top: 100%;
                        left: 0;
                        right: 0;
                        background-color: #fff;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                        max-height: 200px;
                        overflow-y: auto;
                        z-index: 1100;
                    "#,
                    for (sym, name) in suggestions.read().iter().cloned() {
                        div {
                            class: "dropdown-item",
                            style: r#"
                                padding: 8px 12px;
                                cursor: pointer;
                                white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                            "#,
                            onclick: move |_| select_suggestion(sym.clone()),
                            "{sym} - {name}"
                        }
                    }
                }
            }
            if *is_dropdown_open.read() {
                div {
                    style: r#"
                        position: fixed;
                        top: 0;
                        left: 0;
                        width: 100vw;
                        height: 100vh;
                        z-index: 1099;
                    "#,
                    onclick: onclick_outside,
                }
            }
        }
    }
}
