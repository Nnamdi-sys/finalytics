use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use std::path::PathBuf;
use crate::data::keys::{AssetClass, Category, Exchange};
use crate::data::ticker::Ticker;


static EMBEDDED_DATABASE: &[u8] = include_bytes!("sqlite/finalytics.db");

lazy_static::lazy_static! {
    static ref DATABASE_POOL: Pool<SqliteConnectionManager> = {
        let db_file = "temp_embedded.db";
        let db_path = PathBuf::from(db_file);

        if !db_path.exists() {
            std::fs::write(db_file, EMBEDDED_DATABASE)
                .expect("Failed to write embedded database to file");
        }
        let manager = SqliteConnectionManager::file(db_file);
        let pool = Pool::new(manager).expect("Failed to create database connection pool");

        pool
    };
}


/// Fetches a symbol from the database
///
/// # Arguments
///
/// * `symbol` - Symbol string
///
/// # Returns
///
/// * `Symbol` - Symbol struct
pub fn get_symbol(symbol: &str) -> Result<Ticker> {
    let conn = DATABASE_POOL.clone().get().expect("Failed to get connection from pool");
    let mut stmt = conn.prepare("SELECT * FROM symbols WHERE symbol = ?")
        .expect("Failed to prepare statement");

    let symbol_row = stmt.query_row(&[symbol], |row| {
        Ok(Ticker {
            symbol: row.get(0)?,
            name: row.get(1)?,
            category: row.get(2)?,
            asset_class: row.get(3)?,
            exchange: row.get(4)?,
        })
    });

    match symbol_row {
        Ok(ticker) => Ok(ticker),
        Err(_) => panic!("Invalid Symbol"),
    }
}

/// Fetches symbols that match the specified asset class, category, and exchange from the database
///
/// # Arguments
///
/// * `asset_class` - Asset class enum
/// * `category` - Category enum
/// * `exchange` - Exchange enum
///
/// # Returns
///
/// * `Vec<Symbol>` - Vector of symbols
pub fn get_symbols(asset_class: AssetClass, category: Category, exchange: Exchange) -> Result<Vec<Ticker>> {
    let conn = DATABASE_POOL.clone().get().expect("Failed to get connection from pool");
    let mut stmt = conn.prepare("SELECT * FROM symbols WHERE asset_class IN (?) AND category IN (?) AND exchange IN (?)")
        .expect("Failed to prepare statement");

    let asset_class_str = &*asset_class.to_string_vec()[0];
    let category_str = &*category.to_string_vec()[0];
    let exchange_str = &*exchange.to_string_vec()[0];

    let rows = stmt.query_map(&[&asset_class_str, &category_str, &exchange_str], |row| {
        Ok(Ticker {
            symbol: row.get(0)?,
            name: row.get(1)?,
            category: row.get(2)?,
            asset_class: row.get(3)?,
            exchange: row.get(4)?,
        })
    })?;

    let symbols: Result<Vec<Ticker>> = rows.collect();
    symbols
}
