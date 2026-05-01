use dioxus::prelude::*;

/// Renders a self-contained DataTable HTML page (single-table or composite toggle)
/// inside an iframe so that all embedded scripts, styles, and toggle logic execute
/// correctly without any fragile HTML parsing.
#[component]
pub fn TableContainer(html: String, title: String) -> Element {
    // Register a one-time global auto-sizer for all .table-responsive-wrapper iframes.
    // Because DataTables initialises asynchronously inside $(document).ready(), we poll
    // the iframe's contentDocument.scrollHeight every 100 ms for ~3 s after load and
    // update the iframe height each tick so it expands to its true rendered size.
    use_effect(|| {
        document::eval(
            r#"
            (function () {
                if (window.__tableAutoSizeInited) return;
                window.__tableAutoSizeInited = true;

                function autoSize(iframe) {
                    iframe.addEventListener('load', function () {
                        var polls = 0;
                        var interval = setInterval(function () {
                            try {
                                var doc = iframe.contentDocument || iframe.contentWindow.document;
                                var h = doc.documentElement.scrollHeight || doc.body.scrollHeight;
                                if (h > 50) {
                                    iframe.style.height = h + 'px';
                                }
                            } catch (e) {}
                            polls++;
                            if (polls >= 30) clearInterval(interval);
                        }, 100);
                    });
                }

                // Apply to iframes already present in the DOM on first render.
                document.querySelectorAll('.table-responsive-wrapper iframe').forEach(autoSize);

                // Watch for iframes added later by Dioxus re-renders.
                var observer = new MutationObserver(function (mutations) {
                    mutations.forEach(function (m) {
                        m.addedNodes.forEach(function (node) {
                            if (node.nodeType !== 1) return;
                            if (node.matches && node.matches('.table-responsive-wrapper iframe')) {
                                autoSize(node);
                            }
                            if (node.querySelectorAll) {
                                node.querySelectorAll('.table-responsive-wrapper iframe').forEach(autoSize);
                            }
                        });
                    });
                });
                observer.observe(document.body, { childList: true, subtree: true });
            })();
        "#,
        );
    });

    rsx! {
        div {
            class: "table-responsive-wrapper",

            // Title above the table
            div {
                class: "table-title",
                "{title}"
            }

            // Embed the full, self-contained HTML page produced by DataTable::to_html().
            // srcdoc renders the HTML in an isolated browsing context; sandbox flags
            // permit the scripts (jQuery / DataTables / toggle logic) and CSV downloads.
            // Height starts at 150px and is expanded by the auto-size effect above.
            iframe {
                srcdoc: "{html}",
                "sandbox": "allow-scripts allow-same-origin allow-downloads",
                style: "width: 100%; height: 150px; border: none; display: block;",
            }
        }

        style { r#"
            /* ========== Table Wrapper ========== */
            .table-responsive-wrapper {{
                padding: 5px;
                width: 100%;
                box-sizing: border-box;
            }}

            .table-title {{
                font-weight: bold;
                color: #006400;
                font-size: 18px;
                margin-bottom: 10px;
                text-align: center;
            }}

            /* ========== Tablet (<=768px) ========== */
            @media (max-width: 768px) {{
                .table-responsive-wrapper {{
                    padding: 3px;
                }}

                .table-title {{
                    font-size: 15px;
                    margin-bottom: 6px;
                }}
            }}

            /* ========== Small Phones (<=480px) ========== */
            @media (max-width: 480px) {{
                .table-responsive-wrapper {{
                    padding: 2px;
                }}

                .table-title {{
                    font-size: 14px;
                    margin-bottom: 4px;
                }}
            }}
        "# }
    }
}
