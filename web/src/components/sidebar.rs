use dioxus::prelude::*;
use crate::app::Page;

static LOGO: Asset = asset!("/public/images/logo.svg");

#[component]
pub fn SideBar(active_page: Signal<Page>) -> Element {
    let mut is_ticker_open = use_signal(|| true);
    let mut is_sidebar_open = use_signal(|| true);

    rsx! {
        div {
            style: "display: flex; height: 100vh; font-family: 'Poppins', sans-serif;",

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
                    onclick: move |_| is_sidebar_open.set(!is_sidebar_open()),
                    i { class: "bi bi-list" }
                }

                // Navigation section
                div {
                    style: "display: flex; flex-direction: column; gap: 20px; margin-top: 10px;",

                    // Home
                    button {
                        class: if *active_page.read() == Page::Home { "sidebar-link active" } else { "sidebar-link" },
                        onclick: move |_| active_page.set(Page::Home),
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
                                onclick: move |_| active_page.set(Page::Performance),
                                i { class: "bi bi-speedometer2" }
                                if *is_sidebar_open.read() { span { "Performance" } }
                            }
                            button {
                                class: if *active_page.read() == Page::Financials { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                                style: if *is_sidebar_open.read() { "padding: 10px 20px 10px 40px;" } else { "padding: 10px 20px 10px 20px;" },
                                onclick: move |_| active_page.set(Page::Financials),
                                i { class: "bi bi-cash-stack" }
                                if *is_sidebar_open.read() { span { "Financials" } }
                            }
                            button {
                                class: if *active_page.read() == Page::Options { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                                style: if *is_sidebar_open.read() { "padding: 10px 20px 10px 40px;" } else { "padding: 10px 20px 10px 20px;" },
                                onclick: move |_| active_page.set(Page::Options),
                                i { class: "bi bi-bar-chart-fill" }
                                if *is_sidebar_open.read() { span { "Options" } }
                            }
                            button {
                                class: if *active_page.read() == Page::News { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                                style: if *is_sidebar_open.read() { "padding: 10px 20px 10px 40px;" } else { "padding: 10px 20px 10px 20px;" },
                                onclick: move |_| active_page.set(Page::News),
                                i { class: "bi bi-newspaper" }
                                if *is_sidebar_open.read() { span { "News" } }
                            }
                        }
                    }

                    // Portfolio
                    button {
                        class: if *active_page.read() == Page::Portfolio { "sidebar-link active" } else { "sidebar-link" },
                        onclick: move |_| active_page.set(Page::Portfolio),
                        i { class: "bi bi-pie-chart-fill" }
                        if *is_sidebar_open.read() { span { "Portfolio" } }
                    }

                    // Screener
                    button {
                        class: if *active_page.read() == Page::Screener { "sidebar-link active" } else { "sidebar-link" },
                        onclick: move |_| active_page.set(Page::Screener),
                        i { class: "bi bi-search" }
                        if *is_sidebar_open.read() { span { "Screener" } }
                    }
                }

                // Footer
                div {
                    style: "margin-top: auto; padding-top: 20px; border-top: 1px solid #ddd; display: flex; flex-direction: column; align-items: center; gap: 10px;",
                    if *is_sidebar_open.read() {
                        div {
                            style: "display: flex; flex-direction: row; justify-content: center; align-items: center; gap: 16px;",
                            a {
                                href: "https://github.com/Nnamdi-sys/finalytics",
                                target: "_blank",
                                style: "line-height: 0;",
                                i {
                                    class: "bi bi-github",
                                    style: "font-size: 24px; color: #000; vertical-align: middle;"
                                }
                            }
                            a {
                                href: "https://docs.rs/finalytics/",
                                target: "_blank",
                                style: "line-height: 0;",
                                img {
                                    src: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg",
                                    alt: "Rust",
                                    style: "height: 26px; width: 26px; vertical-align: middle;"
                                }
                            }
                            a {
                                href: "https://nnamdi.quarto.pub/finalytics/",
                                target: "_blank",
                                style: "line-height: 0;",
                                img {
                                    src: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/python/python-original.svg",
                                    alt: "Python",
                                    style: "height: 26px; width: 26px; vertical-align: middle;"
                                }
                            }
                        }
                        small {
                            style: "font-size: 12px; color: #888; margin-top: 5px;",
                            "© 2025 Finalytics"
                        }
                    }
                }
            }
        }

        style { r#"
            .sidebar {{
                width: 240px;
                background-color: #f8f9fa;
                padding: 20px;
                display: flex;
                flex-direction: column;
                border-right: 1px solid #ddd;
                transition: width 0.3s ease-in-out;
            }}
            .sidebar.collapsed {{
                width: 70px;
                align-items: center;
                justify-content: center;
            }}
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
            }}
            .sidebar-toggle.fancy:hover {{
                background-color: #333333;
                transform: scale(1.1);
            }}
            .sidebar-toggle.fancy.collapsed {{
                transform: rotate(90deg);
            }}
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
            @media (max-width: 768px) {{
                .sidebar {{
                    position: fixed;
                    left: 0;
                    top: 0;
                    bottom: 0;
                    transform: translateX(0);
                    z-index: 1000;
                }}
                .sidebar.collapsed {{
                    transform: translateX(-100%);
                }}
            }}
        "# }
    }
}