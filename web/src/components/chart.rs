use dioxus::prelude::*;
use regex::Regex;

#[component]
pub fn ChartContainer(html: String) -> Element {
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

    rsx! {
        div {
            class: "tab-pane fade show active",
            style: "padding: 5px;",
            div {
                script {
                    src: "https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"
                }
                script {
                    src: "https://cdn.plot.ly/plotly-2.12.1.min.js"
                }
                div {
                    id: "plotly-html-element",
                    class: "plotly-graph-div",
                    style: "position:relative; top:0; left:0; width:100%; height:80vh;"
                }
            }
        }
    }
}
