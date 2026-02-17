use crate::components::footer::Footer;
use crate::components::navbar::NavBar;
use crate::components::utils::Loading;
use crate::dashboards::financials::Financials;
use crate::dashboards::home::Home;
use crate::dashboards::news::News;
use crate::dashboards::options::Options;
use crate::dashboards::performance::Performance;
use crate::dashboards::portfolio::Portfolio;
use crate::dashboards::screener::Screener;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Page {
    Home,
    Performance,
    Financials,
    Options,
    Portfolio,
    Screener,
    News,
    NotFound,
}

#[component]
pub fn App() -> Element {
    let active_page = use_signal(|| Page::Home);
    let is_mobile_menu_open = use_signal(|| false);

    rsx! {
        head {
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no"
            }
            link {
                href: "https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css",
                rel: "stylesheet"
            }
            link {
                href: "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.7.2/font/bootstrap-icons.css",
                rel: "stylesheet"
            }
            link {
                href: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css",
                rel: "stylesheet"
            }
            link {
                href: "https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;600;700&display=swap",
                rel: "stylesheet"
            }
            link {
                href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/css/all.min.css",
                rel: "stylesheet"
            }
        }

        div {
            class: "app-shell",

            // ── Navbar (sticky, always visible) ──
            NavBar { active_page: active_page, is_mobile_menu_open: is_mobile_menu_open }

            // ── Scrollable area: content + footer ──
            div {
                class: "app-scroll-area",

                // Main content
                div {
                    class: "main-content",

                    SuspenseBoundary {
                        fallback: |_| rsx! { Loading {} },
                        match *active_page.read() {
                            Page::Home => rsx! { Home {} },
                            Page::Performance => rsx! { Performance {} },
                            Page::Financials => rsx! { Financials {} },
                            Page::Options => rsx! { Options {} },
                            Page::Portfolio => rsx! { Portfolio {} },
                            Page::Screener => rsx! { Screener {} },
                            Page::News => rsx! { News {} },
                            Page::NotFound => rsx! { h1 { "404 Not Found" } },
                        }
                    }
                }

                // Footer (scrolls with content)
                Footer {}
            }
        }

        style { r#"
            /* ---- Reset ---- */
            *, *::before, *::after {{
                box-sizing: border-box;
            }}

            /* ---- App Shell ---- */
            .app-shell {{
                display: flex;
                flex-direction: column;
                height: 100vh;
                font-family: 'Poppins', sans-serif;
                overflow: hidden;
            }}

            /* ---- Scrollable area (content + footer) ---- */
            .app-scroll-area {{
                flex: 1;
                overflow-y: auto;
                display: flex;
                flex-direction: column;
                min-height: 0;
            }}

            /* ---- Main Content ---- */
            .main-content {{
                flex: 1;
                padding: 20px;
                background-color: #fff;
            }}

            /* ---- Responsive: tablets ---- */
            @media (max-width: 768px) {{
                .main-content {{
                    padding: 16px 10px;
                }}
            }}

            /* ---- Responsive: small phones ---- */
            @media (max-width: 480px) {{
                .main-content {{
                    padding: 12px 6px;
                }}
            }}
        "# }
    }
}
