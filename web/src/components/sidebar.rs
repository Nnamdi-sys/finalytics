use dioxus::prelude::*;
use crate::app::Page;

static LOGO: Asset = asset!("/public/images/logo.svg");

#[component]
pub fn SideBar(active_page: Signal<Page>) -> Element {
    let mut is_ticker_open = use_signal(|| true);

    rsx! {
        div {
            style: "display: flex; height: 100vh; font-family: 'Poppins', sans-serif;",

            // Sidebar
            nav {
                style: r#"
                    width: 240px;
                    background-color: #f8f9fa;
                    padding: 20px;
                    display: flex;
                    flex-direction: column;
                    justify-content: space-between;
                    border-right: 1px solid #ddd;
                "#,

                // Upper section (Logo + Nav Links)
                div {
                    style: "display: flex; flex-direction: column; gap: 20px;",

                    // Logo
                    img {
                        src: LOGO,
                        width: "180",
                        height: "50",
                        alt: "Logo",
                        style: "margin-bottom: 20px; align-self: center;"
                    }

                    // Nav Links (Buttons)
                    button {
                        class: if *active_page.read() == Page::Home { "sidebar-link active" } else { "sidebar-link" },
                        onclick: move |_| active_page.set(Page::Home),
                        i { class: "bi bi-house-door-fill" }
                        span { "Home" }
                    }
                    // Ticker with sub-menu
                    div {
                        style: "display: flex; flex-direction: column;",
                        button {
                            class: if matches!(*active_page.read(), Page::Performance | Page::Financials | Page::Options | Page::News) { "sidebar-link active" } else { "sidebar-link" },
                            onclick: move |_| {
                                if *is_ticker_open.read() {
                                    is_ticker_open.set(false);
                                } else {
                                    is_ticker_open.set(true);
                                };
                            },
                            i { class: if *is_ticker_open.read() { "bi bi-chevron-down" } else { "bi bi-chevron-right" } }
                            span { "Ticker" }
                        }
                        // Sub-menu
                        div {
                            style: if *is_ticker_open.read() {
                                r#"
                                    display: flex;
                                    flex-direction: column;
                                    gap: 10px;
                                    padding-left: 20px;
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
                                onclick: move |_| active_page.set(Page::Performance),
                                i { class: "bi bi-speedometer2" }
                                span { "Performance" }
                            }
                            button {
                                class: if *active_page.read() == Page::Financials { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                                onclick: move |_| active_page.set(Page::Financials),
                                i { class: "bi bi-cash-stack" }
                                span { "Financials" }
                            }
                            button {
                                class: if *active_page.read() == Page::Options { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                                onclick: move |_| active_page.set(Page::Options),
                                i { class: "bi bi-bar-chart-fill" }
                                span { "Options" }
                            }
                            button {
                                class: if *active_page.read() == Page::News { "sidebar-link sub-link active" } else { "sidebar-link sub-link" },
                                onclick: move |_| active_page.set(Page::News),
                                i { class: "bi bi-newspaper" }
                                span { "News" }
                            }
                        }
                    }
                    button {
                        class: if *active_page.read() == Page::Portfolio { "sidebar-link active" } else { "sidebar-link" },
                        onclick: move |_| active_page.set(Page::Portfolio),
                        i { class: "bi bi-pie-chart-fill" }
                        span { "Portfolio" }
                    }
                    button {
                        class: if *active_page.read() == Page::Screener { "sidebar-link active" } else { "sidebar-link" },
                        onclick: move |_| active_page.set(Page::Screener),
                        i { class: "bi bi-search" }
                        span { "Screener" }
                    }
                }

                // Bottom section (Icons + Footer)
                div {
                    style: "margin-top: auto; padding-top: 20px; border-top: 1px solid #ddd; display: flex; flex-direction: column; align-items: center; gap: 10px;",

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
                        "@finalytics 2025"
                    }
                }
            }
        }

        // Sidebar styles
        style { r#"
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
                padding: 10px 20px 10px 40px;
            }}

            .sub-link:hover, .sub-link.active {{
                background-color: #dee2e6;
            }}

            .icon-colored {{
                font-size: 22px;
                color: #0d6efd;
            }}
        "# }
    }
}