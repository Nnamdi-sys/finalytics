use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::pages::home::Home;
use crate::pages::ticker::Ticker;
use crate::pages::portfolio::Portfolio;
use crate::pages::screener::Screener;
use crate::components::navbar::NavBar;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            Router::<Route> {}
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[layout(NavBar)]

    #[route("/")]
    Home {},

    #[route("/ticker")]
    Ticker {},

    #[route("/portfolio")]
    Portfolio {},

    #[route("/screener")]
    Screener {},

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