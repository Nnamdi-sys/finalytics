use dioxus::prelude::*;

static LOGO: Asset = asset!("/public/images/logo.svg");

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer bg-dark text-light", // Change background color to dark and text color to light
            div {
                class: "container",
                div {
                    class: "row align-items-center justify-content-between", // Align items vertically and justify content between them
                    // Logo and Catchphrase
                    div {
                        class: "col-md-auto", // Adjust column size to auto-expand
                        a {
                            class: "navbar-brand",
                            href: "#",
                            img {
                                src: LOGO,
                                width: "200", // Adjust logo width
                                height: "50", // Adjust logo height
                                class: "d-inline-block align-top ml-0", // Move logo a bit more to the left
                                alt: "Logo",
                                style: "filter: invert(100%);", // Invert the logo to make it visible on dark background
                            }
                        },
                        p {
                            class: "mt-3 mb-4 text-sm text-gray-500",
                            "A Rust and Python library for financial data analysis."
                        }
                    },
                    // Rust Documentation
                    div {
                        class: "col-md-auto", // Adjust column size to auto-expand
                        div { class: "pt-4" }, // Add top padding
                        h5 { "Rust Documentation" },
                        ul {
                            class: "list-unstyled",
                            li {
                                a {
                                    href: "https://crates.io/crates/finalytics",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-box text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "Crates.io"
                                }
                            },
                            li {
                                a {
                                    href: "https://docs.rs/finalytics/latest/finalytics/",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-file-text text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "Docs.rs"
                                }
                            },
                            li {
                                a {
                                    href: "https://github.com/Nnamdi-sys/finalytics",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-github text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "GitHub"
                                }
                            }
                        }
                    },
                    // Python Documentation
                    div {
                        class: "col-md-auto", // Adjust column size to auto-expand
                        div { class: "pt-4" }, // Add top padding
                        h5 { "Python Documentation" },
                        ul {
                            class: "list-unstyled",
                            li {
                                a {
                                    href: "https://pypi.org/project/finalytics/",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-box text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "PyPi"
                                }
                            },
                            li {
                                a {
                                    href: "https://nnamdi.quarto.pub/finalytics/",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-file-text text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "Quarto"
                                }
                            },
                            li {
                                a {
                                    href: "https://github.com/Nnamdi-sys/finalytics",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-github text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "GitHub"
                                }
                            }
                        }
                    },
                    // Sample Applications
                    div {
                        class: "col-md-auto", // Adjust column size to auto-expand
                        div { class: "pt-4" }, // Add top padding
                        h5 { "Dashboards" },
                        ul {
                            class: "list-unstyled",
                            li {
                                a {
                                    href: "https://finalytics.rs/ticker",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-graph-up text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "Ticker"
                                }
                            },
                            li {
                                a {
                                    href: "https://finalytics.rs/portfolio",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-pie-chart text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "Portfolio"
                                }
                            }
                            li {
                                a {
                                    href: "https://finalytics.rs/screener",
                                    target: "_blank", // Open link in a new tab
                                    i { class: "bi bi-search text-light me-2" }, // Add text-light class for proper icon color and spacing
                                    "Screener"
                                }
                            }
                        }
                    }
                }
            },
            div {
                class: "container mx-auto py-4 px-5 flex flex-wrap flex-col sm:flex-row",
                p {
                    class: "text-gray-600 text-sm text-center sm:text-left font-poppins",
                    "© 2024 Finalytics — ",
                    a {
                        class: "text-gray-700 ml-1",
                        href: "https://twitter.com/finalytics_rs",
                        target: "_blank",
                        "@finalytics_rs"
                    }
                }
            }
        }
    }

}