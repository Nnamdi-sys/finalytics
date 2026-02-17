use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn CodeContainer(codes: HashMap<String, String>, id: String) -> Element {
    // Ordered list of languages to display (only those present in the map)
    let available_langs: Vec<(&str, &str, &str)> = vec![
        ("rs", "Rust", "devicon-rust-plain"),
        ("py", "Python", "devicon-python-plain"),
        ("go", "Go", "devicon-go-plain"),
        ("js", "JavaScript", "devicon-javascript-plain"),
    ];

    let tabs: Vec<(String, String, String)> = available_langs
        .iter()
        .filter(|(key, _, _)| codes.contains_key(*key))
        .map(|(key, label, icon)| (key.to_string(), label.to_string(), icon.to_string()))
        .collect();

    // Default to first available language
    let default_lang = tabs.first().map(|(k, _, _)| k.clone()).unwrap_or_default();
    let mut active_lang = use_signal(move || default_lang.clone());
    let mut copied = use_signal(|| false);

    // Map language key to Prism.js class
    let prism_class = |lang: &str| -> &str {
        match lang {
            "rs" => "language-rust",
            "py" => "language-python",
            "go" => "language-go",
            "js" => "language-javascript",
            _ => "language-text",
        }
    };

    let active = active_lang.read().clone();
    let current_code = codes.get(&active as &str).cloned().unwrap_or_default();
    let current_prism = prism_class(&active).to_string();
    let code_id = format!("{}-{}", id, active);
    let code_for_copy = current_code.clone();

    // HTML-escape the code so it's safe as innerHTML while preserving display
    let html_escaped_code = current_code
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;");

    // Re-trigger Prism syntax highlighting whenever the language/code changes.
    // Reading `active_lang` signal inside the closure creates a reactive
    // subscription so Dioxus re-runs the effect on every language switch.
    // Uses a single JS eval with internal setTimeout to avoid multiple .await
    // points that can throw WASM errors during rapid re-renders.
    let id_for_effect = id.clone();
    use_effect(move || {
        let active = active_lang.read().clone();
        let el_id = format!("{}-{}", id_for_effect, active);
        let js = format!(
            r#"
            (function() {{
                var elId = '{el_id}';
                var attempts = 0;
                var maxAttempts = 30;

                function applyMobileWrap(el) {{
                    var w = window.innerWidth;
                    if (w <= 768 && el) {{
                        var wrap = 'pre-wrap';
                        var brk = 'break-word';
                        var fontSize = w <= 480 ? '0.65rem' : '0.72rem';
                        var lineHeight = w <= 480 ? '1.35' : '1.4';
                        el.style.whiteSpace = wrap;
                        el.style.wordWrap = brk;
                        el.style.overflowWrap = brk;
                        el.style.display = 'block';
                        el.style.maxWidth = '100%';
                        el.style.fontSize = fontSize;
                        el.style.lineHeight = lineHeight;
                        var pre = el.parentElement;
                        if (pre) {{
                            pre.style.whiteSpace = wrap;
                            pre.style.wordWrap = brk;
                            pre.style.overflowWrap = brk;
                            pre.style.overflowX = 'hidden';
                            pre.style.fontSize = fontSize;
                            pre.style.lineHeight = lineHeight;
                        }}
                    }}
                }}

                function tryHighlight() {{
                    try {{
                        var el = document.getElementById(elId);
                        if (!el) return;
                        if (typeof Prism !== 'undefined' && typeof Prism.highlightElement === 'function') {{
                            Prism.highlightElement(el);
                            applyMobileWrap(el);
                        }} else if (attempts < maxAttempts) {{
                            attempts++;
                            setTimeout(tryHighlight, 100);
                        }} else {{
                            applyMobileWrap(el);
                        }}
                    }} catch(e) {{}}
                }}

                // Initial delay to let the DOM update, then start polling
                setTimeout(tryHighlight, 60);
            }})();
            "#,
            el_id = el_id
        );
        let _ = document::eval(&js);
    });

    rsx! {
        div {
            class: "code-container",

            // ── Header row: dots | language tabs | copy button ──
            div {
                class: "code-header",

                // Mac-style dots
                div {
                    class: "code-dots",
                    span { class: "code-dot code-dot--red" }
                    span { class: "code-dot code-dot--yellow" }
                    span { class: "code-dot code-dot--green" }
                }

                // Language tabs
                div {
                    class: "code-tabs",
                    for (key, label, icon) in tabs.iter() {
                        button {
                            key: "{key}",
                            class: if *active_lang.read() == *key { "code-tab code-tab--active" } else { "code-tab" },
                            onclick: {
                                let k = key.clone();
                                move |_| active_lang.set(k.clone())
                            },
                            i { class: "{icon}" }
                            span { class: "code-tab-label", "{label}" }
                        }
                    }
                }

                // Copy button
                button {
                    class: if *copied.read() { "code-copy-btn code-copy-btn--copied" } else { "code-copy-btn" },
                    onclick: {
                        let code_text = code_for_copy.clone();
                        move |_| {
                            let code_text = code_text.clone();
                            copied.set(true);
                            spawn(async move {
                                let escaped = code_text
                                    .replace('\\', "\\\\")
                                    .replace('\'', "\\'")
                                    .replace('\n', "\\n")
                                    .replace('\r', "\\r")
                                    .replace('\t', "\\t");
                                let js = format!(
                                    "navigator.clipboard.writeText('{}')",
                                    escaped
                                );
                                let _ = document::eval(&js).await;
                            });
                            spawn(async move {
                                let _ = document::eval("new Promise(r => setTimeout(r, 2000))").await;
                                copied.set(false);
                            });
                        }
                    },
                    if *copied.read() {
                        i { class: "bi bi-check-lg" }
                        span { class: "code-copy-label", " Copied!" }
                    } else {
                        i { class: "bi bi-clipboard" }
                        span { class: "code-copy-label", " Copy" }
                    }
                }
            }

            // ── Code body — key on wrapper forces full recreation on lang switch ──
            // Using dangerous_inner_html on the <code> element so Dioxus does NOT
            // track its inner DOM nodes. This prevents diffing conflicts when
            // Prism.js mutates the text node into <span> elements for highlighting.
            div {
                key: "{code_id}",
                class: "code-body",
                pre {
                    class: "code-pre",
                    code {
                        id: "{code_id}",
                        class: "{current_prism}",
                        dangerous_inner_html: "{html_escaped_code}",
                    }
                }
            }

            // Prism.js assets
            script { src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js" }
            script { src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-rust.min.js" }
            script { src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-python.min.js" }
            script { src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-go.min.js" }
            script { src: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-javascript.min.js" }
            link {
                rel: "stylesheet",
                href: "https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-okaidia.min.css"
            }
        }

        style { r#"
            /* ================================================================
               CODE CONTAINER
               ================================================================ */

            .code-container {{
                position: relative;
                width: 100%;
                height: 100%;
                background: #282c34;
                border-radius: 10px;
                font-family: 'Fira Code', 'Cascadia Code', 'JetBrains Mono', 'Consolas', monospace;
                box-sizing: border-box;
                overflow: hidden;
                border: 1px solid #3e4451;
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
                display: flex;
                flex-direction: column;
            }}

            /* =========== Header =========== */
            .code-header {{
                display: flex;
                align-items: center;
                padding: 10px 14px;
                background: #21252b;
                border-bottom: 1px solid #3e4451;
                gap: 10px;
                user-select: none;
                flex-shrink: 0;
                flex-wrap: nowrap;
                overflow-x: auto;
            }}

            /* Mac dots */
            .code-dots {{
                display: flex;
                gap: 7px;
                flex-shrink: 0;
            }}
            .code-dot {{
                width: 12px;
                height: 12px;
                border-radius: 50%;
                display: inline-block;
            }}
            .code-dot--red   {{ background-color: #ff5f57; }}
            .code-dot--yellow {{ background-color: #febc2e; }}
            .code-dot--green  {{ background-color: #28c840; }}

            /* =========== Language tabs =========== */
            .code-tabs {{
                display: flex;
                gap: 4px;
                flex-shrink: 1;
                min-width: 0;
                overflow-x: auto;
                -webkit-overflow-scrolling: touch;
                scrollbar-width: none;
            }}
            .code-tabs::-webkit-scrollbar {{
                display: none;
            }}

            .code-tab {{
                display: inline-flex;
                align-items: center;
                gap: 6px;
                padding: 5px 12px;
                font-size: 12px;
                font-weight: 600;
                font-family: 'Poppins', sans-serif;
                color: #7f8590;
                background: transparent;
                border: 1px solid transparent;
                border-radius: 6px;
                cursor: pointer;
                transition: all 0.2s ease;
                white-space: nowrap;
                flex-shrink: 0;
            }}

            .code-tab:hover {{
                color: #abb2bf;
                background: #2c313a;
            }}

            .code-tab--active {{
                color: #fff;
                background: #2c313a;
                border-color: #528bff;
            }}

            .code-tab i {{
                font-size: 14px;
            }}

            /* =========== Copy button =========== */
            .code-copy-btn {{
                margin-left: auto;
                display: inline-flex;
                align-items: center;
                gap: 5px;
                padding: 4px 12px;
                font-size: 12px;
                font-weight: 500;
                font-family: inherit;
                color: #9da5b4;
                background: #2c313a;
                border: 1px solid #3e4451;
                border-radius: 5px;
                cursor: pointer;
                transition: all 0.2s ease;
                white-space: nowrap;
                flex-shrink: 0;
            }}
            .code-copy-btn:hover {{
                color: #fff;
                background: #3e4451;
                border-color: #528bff;
            }}
            .code-copy-btn--copied {{
                color: #28c840;
                border-color: #28c840;
            }}
            .code-copy-btn--copied:hover {{
                color: #28c840;
                border-color: #28c840;
            }}

            /* =========== Code body =========== */
            .code-body {{
                flex: 1;
                overflow-y: auto;
                overflow-x: auto;
                -webkit-overflow-scrolling: touch;
                background: #282c34;
                min-height: 0;
            }}

            .code-pre {{
                margin: 0;
                padding: 16px;
                font-size: clamp(0.75rem, 2.5vw, 0.88rem);
                line-height: 1.6;
                background: transparent !important;
                white-space: pre;
                overflow-x: auto;
                tab-size: 4;
            }}

            /* Constrain inner code element */
            .code-container code[class*="language-"] {{
                display: block;
                max-width: 100%;
                box-sizing: border-box;
            }}

            /* =========== Custom scrollbar =========== */
            .code-body::-webkit-scrollbar {{
                width: 6px;
                height: 6px;
            }}
            .code-body::-webkit-scrollbar-track {{
                background: transparent;
            }}
            .code-body::-webkit-scrollbar-thumb {{
                background: #3e4451;
                border-radius: 3px;
            }}
            .code-body::-webkit-scrollbar-thumb:hover {{
                background: #528bff;
            }}
            .code-body::-webkit-scrollbar-corner {{
                background: transparent;
            }}
            /* Firefox */
            .code-body {{
                scrollbar-width: thin;
                scrollbar-color: #3e4451 transparent;
            }}

            /* =========================================================
               TABLET  (<=768px)
               ========================================================= */
            @media (max-width: 768px) {{
                .code-container {{
                    max-height: 50vh;
                    border-radius: 8px;
                }}

                .code-header {{
                    padding: 8px 10px;
                    gap: 6px;
                }}

                .code-dot {{
                    width: 10px;
                    height: 10px;
                }}

                .code-tab {{
                    padding: 4px 8px;
                    font-size: 11px;
                    gap: 4px;
                }}
                .code-tab i {{
                    font-size: 13px;
                }}

                .code-copy-btn {{
                    padding: 3px 8px;
                    font-size: 11px;
                    gap: 4px;
                }}
                /* Hide copy label on tablet, keep icon */
                .code-copy-label {{
                    display: none;
                }}



                .code-pre {{
                    padding: 10px;
                    font-size: 0.72rem;
                    line-height: 1.4;
                    white-space: pre-wrap !important;
                    word-wrap: break-word !important;
                    overflow-wrap: break-word !important;
                    overflow-x: hidden !important;
                    tab-size: 2;
                }}

                /* Override Prism.js forced white-space:pre */
                .code-container code[class*="language-"],
                .code-container pre[class*="language-"],
                .code-pre code {{
                    white-space: pre-wrap !important;
                    word-wrap: break-word !important;
                    overflow-wrap: break-word !important;
                    overflow-x: hidden !important;
                    font-size: 0.72rem !important;
                    line-height: 1.4 !important;
                    display: block !important;
                    max-width: 100% !important;
                }}
            }}

            /* =========================================================
               SMALL PHONES  (<=480px)
               ========================================================= */
            @media (max-width: 480px) {{
                .code-container {{
                    max-height: 40vh;
                    border-radius: 6px;
                    border-width: 0;
                }}

                .code-header {{
                    padding: 6px 8px;
                    gap: 4px;
                }}

                .code-dot {{
                    width: 8px;
                    height: 8px;
                }}
                .code-dots {{
                    gap: 5px;
                }}

                /* Hide tab labels, show only devicons */
                .code-tab-label {{
                    display: none;
                }}
                .code-tab {{
                    padding: 4px 7px;
                    font-size: 10px;
                    gap: 0;
                    border-radius: 4px;
                }}
                .code-tab i {{
                    font-size: 14px;
                }}

                .code-copy-btn {{
                    padding: 3px 6px;
                    font-size: 10px;
                }}
                .code-copy-label {{
                    display: none;
                }}



                .code-pre {{
                    padding: 8px;
                    font-size: 0.65rem;
                    line-height: 1.35;
                    white-space: pre-wrap !important;
                    word-wrap: break-word !important;
                    overflow-wrap: break-word !important;
                    overflow-x: hidden !important;
                    tab-size: 2;
                }}

                /* Override Prism.js for small phones */
                .code-container code[class*="language-"],
                .code-container pre[class*="language-"],
                .code-pre code {{
                    white-space: pre-wrap !important;
                    word-wrap: break-word !important;
                    overflow-wrap: break-word !important;
                    overflow-x: hidden !important;
                    font-size: 0.65rem !important;
                    line-height: 1.35 !important;
                    display: block !important;
                    max-width: 100% !important;
                }}
            }}
        "# }
    }
}
