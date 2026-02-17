use crate::components::sidebar::SideBar;
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
    let mut is_sidebar_open = use_signal(|| true);

    // On first mount, detect mobile viewport and start with sidebar closed
    use_effect(move || {
        spawn(async move {
            if let Ok(val) = dioxus::document::eval("window.innerWidth").await {
                if val.as_f64().unwrap_or(1024.0) <= 768.0 {
                    is_sidebar_open.set(false);
                }
            }
        });
    });

    rsx! {
        head {
            // Viewport meta tag for proper mobile rendering
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

            // Mobile hamburger/close toggle — always rendered, visible only on small screens
            button {
                class: "mobile-hamburger",
                onclick: move |_| {
                    let current = *is_sidebar_open.read();
                    is_sidebar_open.set(!current);
                },
                i { class: if *is_sidebar_open.read() { "bi bi-x-lg" } else { "bi bi-list" } }
            }

            // Backdrop overlay — visible on mobile when sidebar is open
            if *is_sidebar_open.read() {
                div {
                    class: "sidebar-backdrop",
                    onclick: move |_| is_sidebar_open.set(false),
                }
            }

            SideBar { active_page: active_page, is_sidebar_open: is_sidebar_open }

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
        }

        style { r#"
            /* ---- App Shell ---- */
            .app-shell {{
                display: flex;
                height: 100vh;
                font-family: 'Poppins', sans-serif;
                position: relative;
                overflow: hidden;
            }}

            /* ---- Main Content ---- */
            .main-content {{
                flex: 1;
                padding: 20px;
                overflow-y: auto;
                background-color: #fff;
                min-width: 0; /* prevent flex overflow */
            }}

            /* ---- Mobile Hamburger Button ---- */
            .mobile-hamburger {{
                display: none; /* hidden on desktop */
                position: fixed;
                top: 12px;
                left: 12px;
                z-index: 1350; /* above sidebar (1200) and backdrop (1099) */
                background-color: #000;
                color: #fff;
                border: none;
                border-radius: 50%;
                width: 44px;
                height: 44px;
                font-size: 22px;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                box-shadow: 0 2px 8px rgba(0,0,0,0.25);
            }}

            /* ---- Sidebar Backdrop (mobile overlay) ---- */
            .sidebar-backdrop {{
                display: none; /* hidden on desktop */
            }}

            /* ---- Responsive: tablets and phones ---- */
            @media (max-width: 768px) {{
                .mobile-hamburger {{
                    display: flex;
                }}

                .sidebar-backdrop {{
                    display: block;
                    position: fixed;
                    inset: 0;
                    background: rgba(0, 0, 0, 0.45);
                    z-index: 1099;
                }}

                .main-content {{
                    padding: 16px 10px;
                    /* On mobile the sidebar overlays, so content always takes full width */
                    width: 100%;
                }}
            }}

            @media (max-width: 480px) {{
                .main-content {{
                    padding: 12px 6px;
                }}
            }}
        "# }
    }
}
