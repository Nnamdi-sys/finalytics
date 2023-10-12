use rusqlite::{Connection, Result};
use crate::data::keys::{AssetClass, Category, Exchange};
use crate::data::ticker::Ticker;


static EMBEDDED_DATABASE: &[u8] = include_bytes!("sqlite/finalytics.db");

fn open_database_connection() -> Result<Connection> {
    // Open a connection to an in-memory SQLite database
    let conn = Connection::open_in_memory()?;

    // Write the contents of the embedded database to a temporary file
    std::fs::write("temp_embedded.db", EMBEDDED_DATABASE).expect("Failed to write embedded database to file");

    // Attach the temporary database
    conn.execute("ATTACH DATABASE 'temp_embedded.db' AS embedded_db", [])?;

    Ok(conn)
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
    let conn = open_database_connection().expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT * FROM embedded_db.symbols WHERE symbol = ?")
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
    let conn = open_database_connection().expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT * FROM embedded_db.symbols WHERE asset_class IN (?) AND category IN (?) AND exchange IN (?)")
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
