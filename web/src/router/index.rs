use std::fs;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index_html() -> impl Responder {
    let html_output = fs::read_to_string("src/templates/index.html").expect("Failed to read index html file");
   HttpResponse::Ok().body(html_output)
}
