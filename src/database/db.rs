use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Result, ToSql};
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
///
/// # Example
///
/// ```
/// use std::error::Error;
/// use finalytics::database::db::get_symbol;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let result = get_symbol("AAPL")?;
///     println!("{:?}", result);
///     Ok(())
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use std::error::Error;
/// use finalytics::data::keys::{AssetClass, Category, Exchange};
/// use finalytics::database::db::get_symbols;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let result = get_symbols(AssetClass::Stocks, Category::Technology, Exchange::NASDAQ)?;
///     println!("{:?}", result);
///     let result = get_symbols(AssetClass::ETFs, Category::All, Exchange::All)?;
///     println!("{:?}", result);
///     let result = get_symbols(AssetClass::Futures, Category::All, Exchange::All)?;
///     println!("{:?}", result);
///     let result = get_symbols(AssetClass::Indices, Category::All, Exchange::All)?;
///     println!("{:?}", result);
///     let result = get_symbols(AssetClass::MutualFunds, Category::All, Exchange::All)?;
///     println!("{:?}", result);
///     let result = get_symbols(AssetClass::Cryptocurrencies, Category::All, Exchange::All)?;
///     println!("{:?}", result);
///     let result = get_symbols(AssetClass::Currencies, Category::All, Exchange::All)?;
///     println!("{:?}", result);
///     Ok(())
/// }
/// ```
pub fn get_symbols(asset_class: AssetClass, category: Category, exchange: Exchange) -> Result<Vec<Ticker>> {
    let conn = DATABASE_POOL.clone().get().expect("Failed to get connection from pool");

    // Prepare a dynamic number of placeholders and values based on the provided filters
    let (mut placeholders, mut values): (Vec<String>, Vec<&dyn ToSql>) = (Vec::new(), Vec::new());

    let asset_classes = asset_class.to_string_vec();
    let categories = category.to_string_vec();
    let exchanges = exchange.to_string_vec();

    placeholders.push(format!("asset_class IN ({})", (0..asset_classes.len()).map(|_| "?").collect::<Vec<_>>().join(",")));
    values.extend(asset_classes.iter().map(|s| s as &dyn ToSql));

    placeholders.push(format!("category IN ({})", (0..categories.len()).map(|_| "?").collect::<Vec<_>>().join(",")));
    values.extend(categories.iter().map(|s| s as &dyn ToSql));

    placeholders.push(format!("exchange IN ({})", (0..exchanges.len()).map(|_| "?").collect::<Vec<_>>().join(",")));
    values.extend(exchanges.iter().map(|s| s as &dyn ToSql));

    let query = format!("SELECT * FROM symbols WHERE {}", placeholders.join(" AND "));

    let mut stmt = conn.prepare(&query).expect("Failed to prepare statement");

    let rows = stmt.query_map(&*values, |row| {
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

pub fn get_symbols_count() -> Result<i64> {
    let conn = DATABASE_POOL.clone().get().expect("Failed to get connection from pool");
    let sql = "SELECT COUNT(*) FROM symbols";
    let count: i64 = conn.query_row(sql, [], |row| row.get(0))?;
    Ok(count)
}

pub fn get_distinct_exchanges() -> Result<Vec<String>> {
    let conn = DATABASE_POOL.clone().get().expect("Failed to get connection from pool");
    let mut stmt = conn
        .prepare("SELECT DISTINCT exchange FROM symbols")
        .expect("Failed to prepare statement");

    let rows = stmt.query_map([], |row| {
        Ok( row.get(0)? )
    })?;

    let exchanges: Result<Vec<String>> = rows.collect();
    exchanges
}

pub fn get_distinct_categories() -> Result<Vec<String>> {
    let conn = DATABASE_POOL.clone().get().expect("Failed to get connection from pool");
    let mut stmt = conn
        .prepare("SELECT DISTINCT category FROM symbols")
        .expect("Failed to prepare statement");

    let rows = stmt.query_map([], |row| {
        Ok( row.get(0)? )
    })?;

    let categories: Result<Vec<String>> = rows.collect();
    categories
}

pub fn get_distinct_asset_classes() -> Result<Vec<String>> {
    let conn = DATABASE_POOL.clone().get().expect("Failed to get connection from the pool");
    let mut stmt = conn
        .prepare("SELECT DISTINCT asset_class FROM symbols")
        .expect("Failed to prepare statement");

    let rows = stmt.query_map([], |row| {
        Ok( row.get(0)? )
    })?;

    let asset_classes: Result<Vec<String>> = rows.collect();
    asset_classes
}


