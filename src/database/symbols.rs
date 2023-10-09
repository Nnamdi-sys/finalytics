use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;
use ejdb::{bson, Database};
use ejdb::bson::ordered::OrderedDocument;
use ejdb::query::{Q, QH, Query};
use crate::data::ticker::Ticker;


#[derive(Debug, Serialize, Deserialize)]
struct SymbolList {
    pub symbols: Vec<Ticker>,
}

#[allow(dead_code)]
async fn save_symbols() -> Result<(), Box<dyn Error>> {
    let db = Database::open("./src/database/ejdb/finalytics").unwrap();
    let col = db.collection("symbols").unwrap();
    let base_url = "https://finance.yahoo.com/lookup/";
    let sectors = [ "all", "equity", "mutualfund", "etf", "index", "future", "currency"];
    let search_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars();

    for c1 in search_set.clone() {
        for c2 in search_set.clone() {
            let symbol = format!("^{}{}", c1, c2);
            let result = scrape_symbols(base_url, "index", &symbol).await?;
            dbg!(&result);
            for doc in result{
                if doc_query(&doc).iter().all(|x| col.query(x, QH.empty()).count().unwrap() == 0) {
                    col.save(doc).expect("Doc Must be a Valid Bson Document");
                }
            }
        }
    }

    for sector in sectors.iter() {
        for c1 in search_set.clone() {
            let symbol = format!("{}", c1);
            let result = scrape_symbols(base_url, sector, &symbol).await?;
            for doc in result{
                if doc_query(&doc).iter().all(|x| col.query(x, QH.empty()).count().unwrap() == 0) {
                    col.save(doc).expect("Doc Must be a Valid Bson Document");
                }
            }
        }
    }

    for sector in sectors.iter() {
        for c1 in search_set.clone() {
            for c2 in search_set.clone() {
                let symbol = format!("{}{}", c1, c2);
                let result = scrape_symbols(base_url, sector, &symbol).await?;
                for doc in result{
                    if doc_query(&doc).iter().all(|x| col.query(x, QH.empty()).count().unwrap() == 0) {
                        col.save(doc).expect("Doc Must be a Valid Bson Document");
                    }
                }
            }
        }
    }


    Ok(())
}


#[allow(dead_code)]
async fn scrape_symbols(base_url: &str, sector: &str, symbol: &str) -> Result<Vec<OrderedDocument>, Box<dyn Error>> {
    let url = format!("{}{}?s={}&t=A&b=0&c=1000", base_url, sector, symbol);
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let mut result: Vec<OrderedDocument> = Vec::new();

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

            let symbol = symbol_struct.symbol.clone();
            let name = symbol_struct.name.clone();
            let category = symbol_struct.category.clone();
            let asset_class = symbol_struct.asset_class.clone();
            let exchange = symbol_struct.exchange.clone();
            let doc = bson! {
                            "symbol" => symbol,
                            "name" => name,
                            "category" => category,
                            "asset_class" => asset_class,
                            "exchange" => exchange
            };
            result.push(doc);
        }
    }
    Ok(result)
}

#[allow(dead_code)]
fn doc_query(doc: &OrderedDocument) -> Vec<Query> {
    let queries = vec![
        Q.field("symbol").eq(doc.get_str("symbol").unwrap()),
        Q.field("name").eq(doc.get_str("name").unwrap())];
    queries

}

