use dioxus::prelude::*;

#[component]
pub fn CodeContainer(code: String, language: String, id: String) -> Element {
    // Map language to Prism.js class (e.g., "rs" -> "language-rust", "py" -> "language-python")
    let prism_language = match language.as_str() {
        "rs" => "language-rust",
        "py" => "language-python",
        "go" => "language-go",
        "js" => "language-javascript",
        _ => "language-text",
    };

    rsx! {
        div {
            style: r#"
                position: relative;
                width: 100%;
                max-height: 800px; /* Fixed max height for responsiveness */
                overflow-y: auto;
                background: #2b303b;
                border-radius: 8px;
                padding: 10px;
                font-family: 'Fira Code', monospace;
            "#,
            pre {
                class: "line-numbers", /* Optional: Remove if line numbers are not needed */
                style: r#"
                    margin: 0;
                    padding: 10px;
                    font-size: clamp(0.8rem, 2.5vw, 0.9rem); /* Responsive font size */
                    line-height: 1.5;
                    background: transparent;
                "#,
                code {
                    key: "{id}", // Ensure re-rendering when id changes
                    id: "{id}",
                    class: "{prism_language}",
                    "{code}"
                }
            },
            // Load Prism.js and CSS
            script {
                src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"
            },
            script {
                src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-rust.min.js"
            },
            script {
                src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-python.min.js"
            },
            script {
                src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-go.min.js"
            },
            script {
                src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-javascript.min.js"
            },
            link {
                rel: "stylesheet",
                href: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-okaidia.min.css"
            }
        }
    }
}