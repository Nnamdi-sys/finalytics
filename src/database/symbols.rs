use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;
use rusqlite::{Connection, Result};
use rusqlite::params;
use crate::data::ticker::Ticker;

#[derive(Debug, Serialize, Deserialize)]
struct SymbolList {
    pub symbols: Vec<Ticker>,
}

#[allow(dead_code)]
async fn save_symbols() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("./src/database/sqlite/finalytics.db")?;
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

#[allow(dead_code)]
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
            &doc.name,
            &doc.category,
            &doc.asset_class,
            &doc.exchange
        ],
    )?;
    dbg!(&doc);
    Ok(())
}

#[allow(dead_code)]
fn symbol_count() -> Result<i64> {
    let conn = Connection::open("./src/database/sqlite/finalytics.db")?;
    let sql = "SELECT COUNT(*) FROM symbols";
    let count: i64 = conn.query_row(sql, [], |row| row.get(0))?;
    Ok(count)
}
