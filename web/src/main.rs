use actix_web::{App, HttpServer};
use env_logger::Env;
use crate::router::index::index_html;
use crate::router::portfolio::{portfolio, portfolio_report};
use crate::router::symbols::get_all_symbols;
use crate::router::ticker::{ticker, ticker_report};
use crate::router::code::{get_code_examples};

mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new( || {
        App::new()
            .service(actix_files::Files::new("/components", "src/components").show_files_listing())
            .service(actix_files::Files::new("/images", "src/images").show_files_listing())
            .service(index_html)
            .service(ticker)
            .service(ticker_report)
            .service(portfolio)
            .service(portfolio_report)
            .service(get_all_symbols)
            .service(get_code_examples)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
