use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "app-footer",

            // ── Left: Copyright ──
            div {
                class: "footer-copy",
                span { "\u{00a9} 2026 Finalytics" }
            }

            // ── Right: Ecosystem links ──
            div {
                class: "footer-links",
                a {
                    href: "https://github.com/Nnamdi-sys/finalytics",
                    target: "_blank",
                    title: "GitHub",
                    class: "footer-link",
                    i { class: "bi bi-github" }
                }
                a {
                    href: "https://crates.io/crates/finalytics",
                    target: "_blank",
                    title: "Crates.io (Rust)",
                    class: "footer-link",
                    i { class: "devicon-rust-plain" }
                }
                a {
                    href: "https://pypi.org/project/finalytics",
                    target: "_blank",
                    title: "PyPI (Python)",
                    class: "footer-link",
                    i { class: "devicon-python-plain" }
                }
                a {
                    href: "https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics",
                    target: "_blank",
                    title: "Go Reference",
                    class: "footer-link",
                    i { class: "devicon-go-plain" }
                }
                a {
                    href: "https://www.npmjs.com/package/finalytics",
                    target: "_blank",
                    title: "npm (Node.js)",
                    class: "footer-link",
                    i { class: "devicon-nodejs-plain" }
                }
            }
        }

        style { r#"
            /* ============================
               FOOTER BASE
               ============================ */
            .app-footer {{
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 20px 32px;
                background: #f8f9fa;
                border-top: 1px solid #e8eaed;
                flex-shrink: 0;
                box-sizing: border-box;
                gap: 16px;
            }}

            /* ── Copyright ── */
            .footer-copy {{
                font-size: 13px;
                font-family: 'Poppins', sans-serif;
                color: #80868b;
                white-space: nowrap;
            }}

            /* ── Ecosystem links ── */
            .footer-links {{
                display: flex;
                align-items: center;
                gap: 4px;
            }}

            .footer-link {{
                display: inline-flex;
                align-items: center;
                justify-content: center;
                width: 36px;
                height: 36px;
                color: #80868b;
                font-size: 18px;
                border-radius: 8px;
                text-decoration: none;
                transition: all 0.2s ease;
            }}
            .footer-link:hover {{
                color: #2e7d32;
                background: rgba(46,125,50,0.06);
            }}

            /* ============================
               TABLET  (<=768px)
               ============================ */
            @media (max-width: 768px) {{
                .app-footer {{
                    flex-direction: column;
                    gap: 12px;
                    padding: 18px 16px;
                    text-align: center;
                }}

                .footer-copy {{
                    font-size: 12px;
                    order: 1;
                }}

                .footer-links {{
                    gap: 2px;
                    order: 0;
                }}

                .footer-link {{
                    width: 38px;
                    height: 38px;
                    font-size: 19px;
                }}
            }}

            /* ============================
               SMALL PHONES  (<=480px)
               ============================ */
            @media (max-width: 480px) {{
                .app-footer {{
                    padding: 14px 12px;
                    gap: 10px;
                }}

                .footer-copy {{
                    font-size: 11px;
                }}

                .footer-link {{
                    width: 36px;
                    height: 36px;
                    font-size: 17px;
                }}
            }}
        "# }
    }
}
