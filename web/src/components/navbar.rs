use crate::app::Page;
use dioxus::prelude::*;

static LOGO: Asset = asset!("/public/images/logo.svg");

#[component]
pub fn NavBar(active_page: Signal<Page>, is_mobile_menu_open: Signal<bool>) -> Element {
    let mut is_ticker_dropdown_open = use_signal(|| false);

    // Navigate to a page and close all menus
    let mut navigate = move |page: Page| {
        active_page.set(page);
        is_mobile_menu_open.set(false);
        is_ticker_dropdown_open.set(false);
    };

    let is_ticker_active = matches!(
        *active_page.read(),
        Page::Performance | Page::Financials | Page::Options | Page::News
    );

    rsx! {
        nav {
            class: "navbar",

            // ── Left: Logo ──
            div {
                class: "navbar-brand",
                onclick: move |_| navigate(Page::Home),
                img {
                    src: LOGO,
                    alt: "Finalytics",
                    class: "navbar-logo",
                }
            }

            // ── Center: Desktop nav links ──
            div {
                class: "navbar-links",

                button {
                    class: if *active_page.read() == Page::Home { "nav-link nav-link--active" } else { "nav-link" },
                    onclick: move |_| navigate(Page::Home),
                    i { class: "bi bi-house-door-fill nav-link-icon" }
                    span { "Home" }
                }

                // Ticker dropdown
                div {
                    class: "nav-dropdown",

                    button {
                        class: if is_ticker_active { "nav-link nav-link--active" } else { "nav-link" },
                        onclick: move |_| is_ticker_dropdown_open.set(!is_ticker_dropdown_open()),
                        i { class: "bi bi-graph-up nav-link-icon" }
                        span { "Ticker" }
                        i {
                            class: if *is_ticker_dropdown_open.read() {
                                "bi bi-chevron-up nav-link-chevron"
                            } else {
                                "bi bi-chevron-down nav-link-chevron"
                            }
                        }
                    }

                    if *is_ticker_dropdown_open.read() {
                        // Full-screen backdrop — clicking outside closes the dropdown
                        div {
                            style: r#"
                                position: fixed;
                                top: 0;
                                left: 0;
                                width: 100vw;
                                height: 100vh;
                                z-index: 1099;
                            "#,
                            onclick: move |_| is_ticker_dropdown_open.set(false),
                        }
                        div {
                            class: "nav-dropdown-menu",
                            button {
                                class: if *active_page.read() == Page::Performance { "nav-dropdown-item nav-dropdown-item--active" } else { "nav-dropdown-item" },
                                onclick: move |_| navigate(Page::Performance),
                                i { class: "bi bi-speedometer2" }
                                span { "Performance" }
                            }
                            button {
                                class: if *active_page.read() == Page::Financials { "nav-dropdown-item nav-dropdown-item--active" } else { "nav-dropdown-item" },
                                onclick: move |_| navigate(Page::Financials),
                                i { class: "bi bi-cash-stack" }
                                span { "Financials" }
                            }
                            button {
                                class: if *active_page.read() == Page::Options { "nav-dropdown-item nav-dropdown-item--active" } else { "nav-dropdown-item" },
                                onclick: move |_| navigate(Page::Options),
                                i { class: "bi bi-bar-chart-fill" }
                                span { "Options" }
                            }
                            button {
                                class: if *active_page.read() == Page::News { "nav-dropdown-item nav-dropdown-item--active" } else { "nav-dropdown-item" },
                                onclick: move |_| navigate(Page::News),
                                i { class: "bi bi-newspaper" }
                                span { "News" }
                            }
                        }
                    }
                }

                button {
                    class: if *active_page.read() == Page::Portfolio { "nav-link nav-link--active" } else { "nav-link" },
                    onclick: move |_| navigate(Page::Portfolio),
                    i { class: "bi bi-pie-chart-fill nav-link-icon" }
                    span { "Portfolio" }
                }

                button {
                    class: if *active_page.read() == Page::Screener { "nav-link nav-link--active" } else { "nav-link" },
                    onclick: move |_| navigate(Page::Screener),
                    i { class: "bi bi-search nav-link-icon" }
                    span { "Screener" }
                }
            }

            // ── Right: GitHub link (desktop) + hamburger (mobile) ──
            div {
                class: "navbar-right",

                a {
                    href: "https://github.com/Nnamdi-sys/finalytics",
                    target: "_blank",
                    class: "navbar-github",
                    title: "GitHub",
                    i { class: "bi bi-github" }
                }

                button {
                    class: "navbar-hamburger",
                    onclick: move |_| {
                        let current = *is_mobile_menu_open.read();
                        is_mobile_menu_open.set(!current);
                        if current {
                            is_ticker_dropdown_open.set(false);
                        }
                    },
                    if *is_mobile_menu_open.read() {
                        i { class: "bi bi-x-lg" }
                    } else {
                        i { class: "bi bi-list" }
                    }
                }
            }
        }

        // ── Mobile slide-down menu ──
        if *is_mobile_menu_open.read() {
            div {
                class: "mobile-menu",

                button {
                    class: if *active_page.read() == Page::Home { "mobile-menu-link mobile-menu-link--active" } else { "mobile-menu-link" },
                    onclick: move |_| navigate(Page::Home),
                    i { class: "bi bi-house-door-fill" }
                    span { "Home" }
                }

                // Ticker accordion
                div {
                    class: "mobile-menu-group",
                    button {
                        class: if is_ticker_active { "mobile-menu-link mobile-menu-link--active" } else { "mobile-menu-link" },
                        onclick: move |_| is_ticker_dropdown_open.set(!is_ticker_dropdown_open()),
                        i { class: "bi bi-graph-up" }
                        span { "Ticker" }
                        i {
                            class: if *is_ticker_dropdown_open.read() {
                                "bi bi-chevron-up mobile-menu-chevron"
                            } else {
                                "bi bi-chevron-down mobile-menu-chevron"
                            }
                        }
                    }
                    if *is_ticker_dropdown_open.read() {
                        div {
                            class: "mobile-menu-sub",
                            button {
                                class: if *active_page.read() == Page::Performance { "mobile-menu-link mobile-menu-sublink mobile-menu-link--active" } else { "mobile-menu-link mobile-menu-sublink" },
                                onclick: move |_| navigate(Page::Performance),
                                i { class: "bi bi-speedometer2" }
                                span { "Performance" }
                            }
                            button {
                                class: if *active_page.read() == Page::Financials { "mobile-menu-link mobile-menu-sublink mobile-menu-link--active" } else { "mobile-menu-link mobile-menu-sublink" },
                                onclick: move |_| navigate(Page::Financials),
                                i { class: "bi bi-cash-stack" }
                                span { "Financials" }
                            }
                            button {
                                class: if *active_page.read() == Page::Options { "mobile-menu-link mobile-menu-sublink mobile-menu-link--active" } else { "mobile-menu-link mobile-menu-sublink" },
                                onclick: move |_| navigate(Page::Options),
                                i { class: "bi bi-bar-chart-fill" }
                                span { "Options" }
                            }
                            button {
                                class: if *active_page.read() == Page::News { "mobile-menu-link mobile-menu-sublink mobile-menu-link--active" } else { "mobile-menu-link mobile-menu-sublink" },
                                onclick: move |_| navigate(Page::News),
                                i { class: "bi bi-newspaper" }
                                span { "News" }
                            }
                        }
                    }
                }

                button {
                    class: if *active_page.read() == Page::Portfolio { "mobile-menu-link mobile-menu-link--active" } else { "mobile-menu-link" },
                    onclick: move |_| navigate(Page::Portfolio),
                    i { class: "bi bi-pie-chart-fill" }
                    span { "Portfolio" }
                }

                button {
                    class: if *active_page.read() == Page::Screener { "mobile-menu-link mobile-menu-link--active" } else { "mobile-menu-link" },
                    onclick: move |_| navigate(Page::Screener),
                    i { class: "bi bi-search" }
                    span { "Screener" }
                }
            }

            // Backdrop to close mobile menu
            div {
                class: "mobile-menu-backdrop",
                onclick: move |_| {
                    is_mobile_menu_open.set(false);
                    is_ticker_dropdown_open.set(false);
                },
            }
        }

        // ── Navbar styles ──
        style { r#"
            /* ============================
               NAVBAR BASE
               ============================ */
            .navbar {{
                display: flex;
                align-items: center;
                justify-content: space-between;
                height: 64px;
                padding: 0 32px;
                background: #ffffff;
                border-bottom: 1px solid #e8eaed;
                position: sticky;
                top: 0;
                z-index: 1000;
                flex-shrink: 0;
                box-sizing: border-box;
            }}

            /* ── Brand / Logo ── */
            .navbar-brand {{
                display: flex;
                align-items: center;
                cursor: pointer;
                flex-shrink: 0;
                transition: opacity 0.2s ease;
            }}
            .navbar-brand:hover {{
                opacity: 0.8;
            }}
            .navbar-logo {{
                height: 36px;
                width: auto;
            }}

            /* ── Desktop nav links ── */
            .navbar-links {{
                display: flex;
                align-items: center;
                gap: 4px;
                background: #f4f5f7;
                border: 1.5px solid #e1e3e6;
                border-radius: 50px;
                padding: 4px 6px;
            }}

            .nav-link {{
                display: inline-flex;
                align-items: center;
                gap: 8px;
                padding: 8px 18px;
                font-size: 14.5px;
                font-weight: 500;
                font-family: 'Poppins', sans-serif;
                color: #5f6368;
                background: transparent;
                border: none;
                border-radius: 50px;
                cursor: pointer;
                transition: all 0.2s ease;
                white-space: nowrap;
                position: relative;
            }}
            .nav-link:hover {{
                color: #202124;
                background: rgba(0,0,0,0.05);
            }}
            .nav-link--active {{
                color: #2e7d32;
                background: #ffffff;
                box-shadow: 0 1px 3px rgba(0,0,0,0.08);
            }}
            .nav-link--active:hover {{
                color: #2e7d32;
                background: #ffffff;
            }}
            .nav-link-icon {{
                font-size: 16px;
            }}
            .nav-link-chevron {{
                font-size: 10px;
                margin-left: 1px;
                opacity: 0.5;
            }}

            /* ── Ticker dropdown (desktop) ── */
            .nav-dropdown {{
                position: relative;
            }}
            .nav-dropdown-menu {{
                position: absolute;
                top: calc(100% + 6px);
                left: 50%;
                transform: translateX(-50%);
                min-width: 200px;
                background: #ffffff;
                border: 1px solid #e8eaed;
                border-radius: 12px;
                box-shadow: 0 8px 28px rgba(0,0,0,0.1), 0 2px 6px rgba(0,0,0,0.04);
                padding: 6px;
                z-index: 1100;
                animation: dropdownFadeIn 0.15s ease;
            }}
            @keyframes dropdownFadeIn {{
                from {{ opacity: 0; transform: translateX(-50%) translateY(-6px); }}
                to   {{ opacity: 1; transform: translateX(-50%) translateY(0); }}
            }}
            .nav-dropdown-item {{
                display: flex;
                align-items: center;
                gap: 10px;
                width: 100%;
                padding: 10px 14px;
                font-size: 13px;
                font-weight: 500;
                font-family: 'Poppins', sans-serif;
                color: #5f6368;
                background: transparent;
                border: none;
                border-radius: 8px;
                cursor: pointer;
                transition: all 0.15s ease;
                text-align: left;
            }}
            .nav-dropdown-item:hover {{
                color: #202124;
                background: #f1f3f4;
            }}
            .nav-dropdown-item--active {{
                color: #2e7d32;
                background: rgba(46,125,50,0.06);
            }}
            .nav-dropdown-item--active:hover {{
                color: #2e7d32;
                background: rgba(46,125,50,0.1);
            }}
            .nav-dropdown-item i {{
                font-size: 14px;
                width: 18px;
                text-align: center;
            }}

            /* ── Right section ── */
            .navbar-right {{
                display: flex;
                align-items: center;
                gap: 6px;
                flex-shrink: 0;
            }}
            .navbar-github {{
                display: inline-flex;
                align-items: center;
                justify-content: center;
                width: 38px;
                height: 38px;
                color: #5f6368;
                font-size: 20px;
                border-radius: 8px;
                transition: all 0.2s ease;
                text-decoration: none;
            }}
            .navbar-github:hover {{
                color: #202124;
                background: #f1f3f4;
            }}

            /* ── Hamburger (hidden on desktop) ── */
            .navbar-hamburger {{
                display: none;
                align-items: center;
                justify-content: center;
                width: 40px;
                height: 40px;
                font-size: 22px;
                color: #5f6368;
                background: transparent;
                border: none;
                border-radius: 8px;
                cursor: pointer;
                transition: all 0.2s ease;
            }}
            .navbar-hamburger:hover {{
                color: #202124;
                background: #f1f3f4;
            }}

            /* ── Mobile menu (hidden on desktop) ── */
            .mobile-menu {{
                display: none;
            }}
            .mobile-menu-backdrop {{
                display: none;
            }}

            /* ============================
               MOBILE  (<=768px)
               ============================ */
            @media (max-width: 768px) {{
                .navbar {{
                    padding: 0 16px;
                    height: 56px;
                }}
                .navbar-logo {{
                    height: 30px;
                }}

                /* Hide desktop links, show hamburger */
                .navbar-links {{
                    display: none;
                }}
                .navbar-hamburger {{
                    display: flex;
                }}

                /* Mobile slide-down menu */
                .mobile-menu {{
                    display: flex;
                    flex-direction: column;
                    position: fixed;
                    top: 56px;
                    left: 0;
                    right: 0;
                    background: #ffffff;
                    border-bottom: 1px solid #e8eaed;
                    padding: 8px 12px 14px 12px;
                    z-index: 999;
                    box-shadow: 0 8px 24px rgba(0,0,0,0.08);
                    animation: mobileMenuSlide 0.2s ease;
                    max-height: calc(100vh - 56px);
                    overflow-y: auto;
                }}
                @keyframes mobileMenuSlide {{
                    from {{ opacity: 0; transform: translateY(-8px); }}
                    to   {{ opacity: 1; transform: translateY(0); }}
                }}

                .mobile-menu-backdrop {{
                    display: block;
                    position: fixed;
                    top: 56px;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background: rgba(0,0,0,0.15);
                    z-index: 998;
                }}

                .mobile-menu-link {{
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    width: 100%;
                    padding: 12px 16px;
                    font-size: 15px;
                    font-weight: 500;
                    font-family: 'Poppins', sans-serif;
                    color: #5f6368;
                    background: transparent;
                    border: none;
                    border-radius: 8px;
                    cursor: pointer;
                    transition: all 0.15s ease;
                    text-align: left;
                }}
                .mobile-menu-link:hover {{
                    color: #202124;
                    background: #f1f3f4;
                }}
                .mobile-menu-link--active {{
                    color: #2e7d32;
                    background: rgba(46,125,50,0.06);
                }}

                .mobile-menu-chevron {{
                    margin-left: auto;
                    font-size: 12px;
                    opacity: 0.4;
                }}

                .mobile-menu-group {{
                    display: flex;
                    flex-direction: column;
                }}
                .mobile-menu-sub {{
                    display: flex;
                    flex-direction: column;
                    padding-left: 12px;
                    border-left: 2px solid #e8eaed;
                    margin-left: 22px;
                    margin-top: 2px;
                }}
                .mobile-menu-sublink {{
                    font-size: 14px;
                    padding: 10px 16px;
                }}
            }}

            /* ============================
               SMALL PHONES  (<=480px)
               ============================ */
            @media (max-width: 480px) {{
                .navbar {{
                    padding: 0 12px;
                    height: 52px;
                }}
                .navbar-logo {{
                    height: 26px;
                }}
                .mobile-menu {{
                    top: 52px;
                    max-height: calc(100vh - 52px);
                    padding: 6px 8px 12px 8px;
                }}
                .mobile-menu-backdrop {{
                    top: 52px;
                }}
                .mobile-menu-link {{
                    font-size: 14px;
                    padding: 10px 14px;
                    gap: 10px;
                }}
                .mobile-menu-sublink {{
                    font-size: 13px;
                    padding: 9px 14px;
                }}
            }}
        "# }
    }
}
