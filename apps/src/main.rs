mod web;
mod telegram;


use dioxus::prelude::*;
use crate::web::navbar::Route;
#[cfg(feature = "server")]
use crate::telegram::server::telegram_bot;


fn main() {

    #[cfg(feature = "server")]
    {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(telegram_bot());
        });
    }

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