use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::Bytes;
use serde::{Deserialize, Serialize};
use finalytics::prelude::*;
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerFormData {
    symbol: String,
    start_date: String,
    end_date: String,
    interval: String,
    benchmark_symbol: String,
    confidence_level: f64,
    risk_free_rate: f64,
    report_type: String
}


#[get("/ticker")]
pub async fn ticker(form_data: Bytes) -> impl Responder {
    let rendered_html = ticker_html(form_data).await;
    HttpResponse::Ok().body(rendered_html)
}

#[post("/ticker_report")]
pub async fn ticker_report(form_data: Bytes) -> impl Responder {
    let rendered_html = ticker_html(form_data).await;
    HttpResponse::Ok().body(rendered_html)
}

async fn ticker_html(form_data: Bytes) -> String {
    let form_data_str = String::from_utf8(form_data.to_vec())
        .map_err(|e| {
            HttpResponse::BadRequest().body(format!("Invalid form data: {}", e))
        }).unwrap();

    let data: TickerFormData = serde_urlencoded::from_str(&form_data_str)
        .map_err(|e| {
            HttpResponse::BadRequest().body(format!("Invalid form data: {}", e))
        }).unwrap_or(TickerFormData {
        symbol: "AAPL".to_string(),
        start_date: "2023-01-01".to_string(),
        end_date: "2024-12-31".to_string(),
        interval: "1d".to_string(),
        benchmark_symbol: "^GSPC".to_string(),
        confidence_level: 0.95,
        risk_free_rate: 0.02,
        report_type: "performance".to_string()
    });

    let tc = TickerBuilder::new()
        .ticker(&data.symbol)
        .start_date(&data.start_date)
        .end_date(&data.end_date)
        .interval(Interval::from_str(&data.interval))
        .benchmark_symbol(&data.benchmark_symbol)
        .confidence_level(data.confidence_level)
        .risk_free_rate(data.risk_free_rate)
        .build();

    let report_html = tc.report(Some(ReportType::from_str(&data.report_type))).await.unwrap().to_html();

    // Create a Tera instance and load your HTML template
    let tera = Tera::new("src/templates/*").expect("Failed to initialize Tera");

    // Create a context to pass data to the template
    let mut context = Context::new();
    context.insert("chart_content", &report_html);
    context.insert("form_data", &data);

    // Render the HTML template with the data
    let rendered_html = tera
        .render("ticker.html", &context)
        .expect("Failed to render HTML");

    rendered_html
}
