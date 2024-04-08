#!/usr/bin/env cargo eval --
//! `cargo` "language".
//!
//! ```cargo
//! [package]
//! edition = "2021"
//!
//! [dependencies]
//! rusqlite = "0.29.0"
//! reqwest = { version = "0.11.18", features = ["json", "blocking"] }
//! tokio = { version = "1.32.0", features = ["full"] }
//! serde = { version = "1.0.183", features = ["derive"] }
//! serde_json = "1.0.105"
//! scraper = "0.17.1"
//! html-escape = "0.2.13"
//! ```

extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate serde_json;
extern crate rusqlite;
extern crate tokio;
extern crate html_escape;


use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;
use rusqlite::{Connection, Result};
use rusqlite::params;

#[derive(Debug, Serialize, Deserialize)]
struct SymbolList {
    pub symbols: Vec<Ticker>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Ticker {
    pub symbol: String,
    pub name: String,
    pub category: String,
    pub asset_class: String,
    pub exchange: String,
}
async fn save_symbols() -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let db_path = current_dir.to_str().unwrap().to_string() +  "/sqlite/finalytics.db";
    let conn = Connection::open(&db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS symbols (
             symbol TEXT PRIMARY KEY,
             name TEXT,
             category TEXT,
             asset_class TEXT,
             exchange TEXT
         )",
        [],
    )?;

    let base_url = "https://finance.yahoo.com/lookup/";
    let sectors = ["all", "equity", "mutualfund", "etf", "index", "future", "currency"];
    let search_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars();

    for sector in sectors.iter() {
        for c1 in search_set.clone() {
            let symbol = format!("{}", c1);
            dbg!(&symbol);
            let result = scrape_symbols(base_url, sector, &symbol).await?;
            for doc in result {
                if !document_exists_in_db(&conn, &doc) {
                    insert_document(&conn, &doc)?;
                }
            }
        }
    }

    for c1 in search_set.clone() {
        let symbol = format!("^{}", c1);
        dbg!(&symbol);
        let result = scrape_symbols(base_url, "index", &symbol).await?;
        for doc in result {
            if !document_exists_in_db(&conn, &doc) {
                insert_document(&conn, &doc)?;
            }
        }
    }

    Ok(())
}

async fn scrape_symbols(base_url: &str, sector: &str, symbol: &str) -> Result<Vec<Ticker>, Box<dyn Error>> {
    let url = format!("{}{}?s={}&t=A&b=0&c=5000", base_url, sector, symbol);
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let mut result: Vec<Ticker> = Vec::new();

    // Selector for the table rows containing symbol data
    let row_selector = Selector::parse("table tbody tr").unwrap();

    // Extract symbol data
    for row in document.select(&row_selector) {
        let mut columns: Vec<String> = Vec::new();

        // Extract data from each cell in the row
        for cell in row.select(&Selector::parse("td").unwrap()) {
            columns.push(cell.inner_html().trim().to_string());
        }

        if columns.len() >= 6 {
            let symbol_struct = Ticker {
                symbol: {
                    let symbol_html = &columns[0];
                    let symbol_document = Html::parse_fragment(&symbol_html);
                    symbol_document
                        .select(&Selector::parse("a").unwrap())
                        .next()
                        .map(|a| a.value().attr("data-symbol").unwrap_or_default())
                        .unwrap_or_default()
                        .to_string()
                },
                name: columns[1].clone(),
                category: {
                    let category_html = &columns[3];
                    let category_document = Html::parse_fragment(&category_html);
                    category_document
                        .select(&Selector::parse("a").unwrap())
                        .next()
                        .map(|a| a.inner_html().trim().to_string())
                        .unwrap_or("N/A".to_string())
                },
                asset_class: columns[4].clone(),
                exchange: columns[5].clone(),
            };

            result.push(symbol_struct);
        }
    }
    Ok(result)
}

fn document_exists_in_db(conn: &Connection, doc: &Ticker) -> bool {
    let sql = "SELECT COUNT(*) FROM symbols WHERE symbol = ?";
    let count: i64 = conn.query_row(sql, &[&doc.symbol], |row| row.get(0)).unwrap_or(0);

    count > 0
}

fn insert_document(conn: &Connection, doc: &Ticker) -> Result<()> {
    let sql = "INSERT INTO symbols (symbol, name, category, asset_class, exchange) VALUES (?, ?, ?, ?, ?)";
    conn.execute(
        sql,
        params![
            &doc.symbol,
            html_escape::decode_html_entities(&doc.name).to_string(),
            &doc.category,
            &doc.asset_class,
            &doc.exchange
        ],
    )?;
    dbg!(&doc);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    save_symbols().await?;
    Ok(())
}
