use dioxus::prelude::*;
use regex::Regex;

#[component]
pub fn ChartContainer(html: String, id: String) -> Element {
    let html = html.replace("plotly-html-element", id.as_str());
    let regex = Regex::new(r#"<script\s+type="module">\s*(?s)(.*?)</script>"#);
    let script = if let Ok(regex) = regex {
        regex
            .captures(&html)
            .and_then(|caps| caps.get(1)
                .map(|m| m.as_str().trim().replace(r#","config":{"fillFrame":true,"responsive":true}"#, "")))
            .unwrap_or_default()
    } else {
        String::new()
    };

    use_effect(move || {
        document::eval(&script);
    });

    // Determine height style based on id
    let container_style = if id == "chart-ticker" || id == "chart-portfolio" {
        r#"
            padding: 5px;
            width: 100%;
            height: 70vh; /* Match CodeContainer height */
            overflow-y: auto; /* Enable scrolling */
        "#
    } else {
        r#"
            padding: 5px;
            width: 100%;
            height: 100vh; /* Default height for other uses */
        "#
    };

    rsx! {
        div {
            class: "tab-pane fade show active",
            style: "{container_style}",
            script {
                src: "https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"
            }
            script {
                src: "https://cdn.plot.ly/plotly-2.12.1.min.js"
            }
            div {
                id: "{id}",
                class: "plotly-graph-div",
                style: r#"
                    position: relative;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%; /* Fill parent container */
                "#
            }
        }
    }
}