mod server;
mod components;
mod pages;
mod app;
mod forms;
mod dashboards;

use dioxus::prelude::*;
use crate::app::App;


fn main() {

    #[cfg(feature = "web")]
    LaunchBuilder::web().launch(App);

    #[cfg(feature = "server")]
    {
        use axum::Router;
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
                let app = Router::new()
                    .serve_dioxus_application(ServeConfig::new().unwrap(), App);
                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }

}