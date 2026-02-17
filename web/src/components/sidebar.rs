use crate::app::Page;
use dioxus::prelude::*;

static LOGO: Asset = asset!("/public/images/logo.svg");

#[component]
pub fn SideBar(active_page: Signal<Page>, is_sidebar_open: Signal<bool>) -> Element {
    let mut is_ticker_open = use_signal(|| true);

    // Helper: set the active page. Sidebar open/close is manual by the user.
    let mut navigate = move |page: Page| {
        active_page.set(page);
    };

    rsx! {
        nav {
            class: if *is_sidebar_open.read() { "sidebar open" } else { "sidebar collapsed" },

            // Logo
            if *is_sidebar_open.read() {
                img {
                    src: LOGO,
                    width: "180",
                    height: "50",
                    alt: "Logo",
                    style: "margin-bottom: 20px; align-self: center;"
                }
            }

            // Fancy toggle button under logo
            button {
                class: if *is_sidebar_open.read() { "sidebar-toggle fancy" } else { "sidebar-toggle fancy collapsed" },
                onclick: move |_| {
                    let current = { *is_sidebar_open.read() };
                    is_sidebar_open.set(!current);
                },
                i { class: "bi bi-list" }
            }

            // Navigation section
            div {
                style: "display: flex; flex-direction: column; gap: 20px; margin-top: 10px;",

                // Home
                button {
                    class: if *active_page.read() == Page::Home { "sidebar-link active" } else { "sidebar-link" },
                    onclick: move |_| navigate(Page::Home),
                    i { class: "bi bi-house-door-fill" }
                    if *is_sidebar_open.read() { span { "Home" } }
                }

                // Ticker
                div {
                    style: "display: flex; flex-direction: column;",
                    button {
                        class: if matches!(*active_page.read(), Page::Performance | Page::Financials | Page::Options | Page::News) { "sidebar-link active" } else { "sidebar-link" },
                        onclick: move |_| is_ticker_open.set(!is_ticker_open()),
                        i { class: if *is_ticker_open.read() { "bi bi-chevron-down" } else { "bi bi-chevron-right" } }
                        if *is_sidebar_open.read() { span { "Ticker" } }
                    }
                    div {
                        style: if *is_ticker_open.read() {
                            r#"
                                display: flex;
                                flex-direction: column;
                                gap: 10px;
                                padding-left: {{if *is_sidebar_open.read() {{ "20px" }} else {{ "10px" }}}};
                                max-height: 200px;
                                transition: max-height 0.3s ease-in-out;
                            "#
                        } else {
                            r#"
                                max-height: 0;
                                overflow: hidden;
                                transition: max-height 0.3s ease-in-out;
                            "#
                        },
                        button {
                            class: if *active_page.read() == Page::Performance { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                            style: if *is_sidebar_open.read() { "padding: 10px 20px 10px 40px;" } else { "padding: 10px 20px 10px 20px;" },
                            onclick: move |_| navigate(Page::Performance),
                            i { class: "bi bi-speedometer2" }
                            if *is_sidebar_open.read() { span { "Performance" } }
                        }
                        button {
                            class: if *active_page.read() == Page::Financials { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                            style: if *is_sidebar_open.read() { "padding: 10px 20px 10px 40px;" } else { "padding: 10px 20px 10px 20px;" },
                            onclick: move |_| navigate(Page::Financials),
                            i { class: "bi bi-cash-stack" }
                            if *is_sidebar_open.read() { span { "Financials" } }
                        }
                        button {
                            class: if *active_page.read() == Page::Options { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                            style: if *is_sidebar_open.read() { "padding: 10px 20px 10px 40px;" } else { "padding: 10px 20px 10px 20px;" },
                            onclick: move |_| navigate(Page::Options),
                            i { class: "bi bi-bar-chart-fill" }
                            if *is_sidebar_open.read() { span { "Options" } }
                        }
                        button {
                            class: if *active_page.read() == Page::News { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                            style: if *is_sidebar_open.read() { "padding: 10px 20px 10px 40px;" } else { "padding: 10px 20px 10px 20px;" },
                            onclick: move |_| navigate(Page::News),
                            i { class: "bi bi-newspaper" }
                            if *is_sidebar_open.read() { span { "News" } }
                        }
                    }
                }

                // Portfolio
                button {
                    class: if *active_page.read() == Page::Portfolio { "sidebar-link active" } else { "sidebar-link" },
                    onclick: move |_| navigate(Page::Portfolio),
                    i { class: "bi bi-pie-chart-fill" }
                    if *is_sidebar_open.read() { span { "Portfolio" } }
                }

                // Screener
                button {
                    class: if *active_page.read() == Page::Screener { "sidebar-link active" } else { "sidebar-link" },
                    onclick: move |_| navigate(Page::Screener),
                    i { class: "bi bi-search" }
                    if *is_sidebar_open.read() { span { "Screener" } }
                }
            }

            // Footer
            div {
                style: "margin-top: auto; padding-top: 20px; border-top: 1px solid #ddd; display: flex; flex-direction: column; align-items: center; gap: 10px;",
                if *is_sidebar_open.read() {
                    small {
                        style: "font-size: 12px; color: #888; margin-top: 5px;",
                        "© 2026 Finalytics"
                    }
                }
            }
        }

        style { r#"
            /* =========== SIDEBAR BASE =========== */
            .sidebar {{
                width: 240px;
                background-color: #f8f9fa;
                padding: 20px;
                display: flex;
                flex-direction: column;
                border-right: 1px solid #ddd;
                transition: width 0.3s ease-in-out;
                overflow-y: auto;
                overflow-x: hidden;
                flex-shrink: 0;
                height: 100vh;
                box-sizing: border-box;
            }}

            .sidebar.collapsed {{
                width: 70px;
                align-items: center;
                justify-content: flex-start;
            }}

            /* =========== TOGGLE BUTTON =========== */
            .sidebar-toggle.fancy {{
                background-color: #000000;
                color: white;
                border: none;
                border-radius: 50%;
                width: 40px;
                height: 40px;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 20px;
                cursor: pointer;
                margin: 0 auto 20px auto;
                transition: all 0.3s ease;
                box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
                flex-shrink: 0;
            }}
            .sidebar-toggle.fancy:hover {{
                background-color: #333333;
                transform: scale(1.1);
            }}
            .sidebar-toggle.fancy.collapsed {{
                transform: rotate(90deg);
            }}

            /* =========== NAV LINKS =========== */
            .sidebar-link {{
                display: flex;
                align-items: center;
                gap: 16px;
                font-size: 18px;
                padding: 12px 20px;
                text-decoration: none;
                color: #343a40;
                border-radius: 6px;
                transition: background-color 0.2s ease-in-out;
                cursor: pointer;
                background: none;
                border: none;
                width: 100%;
                text-align: left;
            }}
            .sidebar-link:hover, .sidebar-link.active {{
                background-color: #e9ecef;
            }}
            .sub-link {{
                font-size: 16px;
                padding: 10px 20px;
            }}
            .sub-link:hover, .sub-link.active {{
                background-color: #dee2e6;
            }}

            /* =========== MOBILE (≤ 768px) =========== */
            @media (max-width: 768px) {{
                .sidebar {{
                    position: fixed;
                    left: 0;
                    top: 0;
                    bottom: 0;
                    width: 260px;
                    z-index: 1200;
                    transform: translateX(0);
                    transition: transform 0.3s ease-in-out;
                }}

                .sidebar.collapsed {{
                    transform: translateX(-100%);
                    width: 260px; /* keep full width so it slides out properly */
                }}

                /* Hide the in-sidebar toggle on mobile — the app-level hamburger is used instead */
                .sidebar-toggle.fancy {{
                    display: none;
                }}
            }}

            /* =========== SMALL PHONES (≤ 480px) =========== */
            @media (max-width: 480px) {{
                .sidebar {{
                    width: 220px;
                    padding: 16px 12px;
                }}
                .sidebar.collapsed {{
                    width: 220px;
                }}
                .sidebar-link {{
                    font-size: 16px;
                    padding: 10px 14px;
                    gap: 12px;
                }}
                .sub-link {{
                    font-size: 14px;
                    padding: 8px 14px;
                }}
            }}
        "# }
    }
}
