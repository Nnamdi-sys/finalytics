use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::home::Home;
use crate::components::ticker::Ticker;
use crate::components::portfolio::Portfolio;

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[layout(NavBar)]

    #[route("/")]
    Home {},

    #[route("/ticker")]
    Ticker {},

    #[route("/portfolio")]
    Portfolio {},

    #[end_layout]

    #[route("/:..route")]
    NotFound {
        route: Vec<String>
    }
}


#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "404 Not Found" }
        }
    }
}


#[component]
pub fn NavBar() -> Element {
    rsx! {
        head {
                // Bootstrap CSS
                link {
                    href: "https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css",
                    rel: "stylesheet"
                }
                // Bootstrap Icons
                link {
                    href: "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.7.2/font/bootstrap-icons.css",
                    rel: "stylesheet"
                }

                // Devicons
                link {
                    href: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css",
                    rel: "stylesheet"
                }

                // Poppins Font
                link {
                    href: "https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;600;700&display=swap",
                    rel: "stylesheet"
                }
            }

        nav {
            class: "navbar navbar-expand-lg navbar-light bg-light", // Add background color
            div {
                class: "container-fluid",
                Link {
                    to: Route::Home {},
                    class: "navbar-brand",
                    img {
                        src: "images/logo.svg",
                        width: "200",
                        height: "50",
                        class: "d-inline-block align-top",
                        alt: "Logo",
                    },

                }

                div {
                    class: "collapse navbar-collapse",
                    ul {
                        class: "navbar-nav me-auto", // Align items to the left
                        // Docs
                        li {
                            class: "nav-item",
                            a {
                                class: "nav-link d-flex align-items-center",
                                style: "color: blue; font-weight: bold;", // Bold and blue text
                                href: "https://github.com/Nnamdi-sys/finalytics",
                                target: "_blank",
                                i { class: "bi bi-github me-2" }, // Icon with spacing
                                "Docs"
                            }
                        }

                        // Rust
                        li {
                            class: "nav-item",
                            a {
                                class: "nav-link d-flex align-items-center",
                                style: "color: blue; font-weight: bold; text-decoration: none;", // Bold and blue text
                                href: "https://docs.rs/finalytics/",
                                target: "_blank",
                                i { class: "devicon-rust-plain me-2" }, // Rust icon from Devicons
                                "Rust"
                            }
                        }

                        // Python
                        li {
                            class: "nav-item",
                            a {
                                class: "nav-link d-flex align-items-center",
                                style: "color: blue; font-weight: bold; text-decoration: none;", // Bold and blue text
                                href: "https://nnamdi.quarto.pub/finalytics/",
                                target: "_blank",
                                i { class: "devicon-python-plain me-2" }, // Python icon from Devicons
                                "Python"
                            }
                        }

                        // Ticker
                        li {
                            class: "nav-item",
                            a {
                                class: "nav-link d-flex align-items-center",
                                style: "color: blue; font-weight: bold; text-decoration: none;", // Bold and blue text
                                href: "https://finalytics.rs/ticker",
                                target: "_blank",
                                i { class: "bi bi-graph-up me-2" }, // Icon with spacing
                                "Ticker"
                            }
                        }

                        // Portfolio
                        li {
                            class: "nav-item",
                            a {
                                class: "nav-link d-flex align-items-center",
                                style: "color: blue; font-weight: bold; text-decoration: none;", // Bold and blue text
                                href: "https://finalytics.rs/portfolio",
                                target: "_blank",
                                i { class: "bi bi-pie-chart me-2" }, // Icon with spacing
                                "Portfolio"
                            }
                        }
                    }
                }

            }

        }
        Outlet::<Route> {}
    }
}