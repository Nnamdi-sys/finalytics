use actix_web::{get, HttpResponse, Responder};
use yahoo_finance_symbols::keys::{AssetClass, Category, Exchange};
use yahoo_finance_symbols::get_symbols;


#[get("/get_symbols")]
pub async fn get_all_symbols() -> impl Responder {
    let tickers = get_symbols(
        AssetClass::All,
        Category::All,
        Exchange::All)
        .await.unwrap();
    HttpResponse::Ok().json(tickers)
}