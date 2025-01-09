use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::Bytes;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use finalytics::prelude::*;

#[derive(Deserialize, Serialize)]
struct PortfolioFormData {
    symbols: String,
    benchmark_symbol: String,
    start_date: String,
    end_date: String,
    interval: String,
    confidence_level: f64,
    risk_free_rate: f64,
    objective_function: String
}

#[get("/portfolio")]
pub async fn portfolio(form_data: Bytes) -> impl Responder {
    let rendered_html = portfolio_html(form_data).await;
    HttpResponse::Ok().body(rendered_html)
}
#[post("/portfolio_report")]
pub async fn portfolio_report(form_data: Bytes) -> impl Responder {
    let rendered_html = portfolio_html(form_data).await;
    HttpResponse::Ok().body(rendered_html)
}

async fn portfolio_html(form_data: Bytes) -> String {
    let form_data_str = String::from_utf8(form_data.to_vec())
        .map_err(|e| {
            HttpResponse::BadRequest().body(format!("Invalid form data: {}", e))
        }).unwrap();

    let data: PortfolioFormData = serde_urlencoded::from_str(&form_data_str)
        .map_err(|e| {
            HttpResponse::BadRequest().body(format!("Invalid form data: {}", e))
        }).unwrap_or(PortfolioFormData {
        symbols: "AAPL,MSFT,NVDA,BTC-USD".to_string(),
        benchmark_symbol: "^GSPC".to_string(),
        start_date: "2023-01-01".to_string(),
        end_date: "2024-12-31".to_string(),
        interval: "1d".to_string(),
        confidence_level: 0.95,
        risk_free_rate: 0.02,
        objective_function: "max_sharpe".to_string()
    });

    let pf = PortfolioBuilder::new()
        .ticker_symbols(data.symbols.split(",").into_iter().map(|x| x).collect())
        .benchmark_symbol(&data.benchmark_symbol)
        .start_date(&data.start_date)
        .end_date(&data.end_date)
        .interval(Interval::from_str(&data.interval))
        .confidence_level(data.confidence_level)
        .risk_free_rate(data.risk_free_rate)
        .objective_function(ObjectiveFunction::from_str(&data.objective_function))
        .build()
        .await.unwrap();

    let report_html = pf.report(Some(ReportType::Performance)).await.unwrap().to_html();

    // Create a Tera instance and load your HTML template
    let tera = Tera::new("src/templates/*").expect("Failed to initialize Tera");

    // Create a context to pass data to the template
    let mut context = Context::new();
    context.insert("chart_content", &report_html);
    context.insert("form_data", &data);

    // Render the HTML template with the data
    let rendered_html = tera
        .render("portfolio.html", &context)
        .expect("Failed to render HTML");

    rendered_html
}