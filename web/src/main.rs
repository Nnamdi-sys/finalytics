mod server;
mod components;
mod app;
mod dashboards;

use dioxus::prelude::*;
use crate::app::App;

fn main() {
    match std::env::var("ENV").as_deref().unwrap_or("dev") {
        "prod" => {
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
        _ => {
            launch(App);
        }
    }
}