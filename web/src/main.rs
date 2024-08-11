mod server;
mod components;

use dioxus::prelude::*;
use crate::components::navbar::Route;


fn main() {

    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::default());

    let mut config = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
        config = config.addr(addr);
    }

    LaunchBuilder::new().with_cfg(config).launch(App)

}


#[component]
fn App() -> Element {
    rsx! {
        div {
            Router::<Route> {}
        }
    }
}