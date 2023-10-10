use std::{env, fs};
use rusqlite::{Connection, Result};
use crate::data::keys::{AssetClass, Category, Exchange};
use crate::data::ticker::Ticker;
use once_cell::sync::Lazy;

pub static DB_PATH: Lazy<String> = Lazy::new(|| {
    let mut db_path = String::new();
    // Get the path to the current executable
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

    // Define the path to the target/debug/build directory
    let target_build_dir = exe_dir.join("build");

    if target_build_dir.is_dir() {
        // Iterate through subdirectories in the target/debug/build directory
        for entry in fs::read_dir(target_build_dir).expect("Failed to read target/debug/build directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // Check if the directory starts with "finalytics"
                    if let Some(dir_name) = path.file_name() {
                        if dir_name.to_string_lossy().starts_with("finalytics") {
                            // Search for the "finalytics" file within this subdirectory
                            for entry in fs::read_dir(&path).ok().unwrap() {
                                if let Ok(entry) = entry {
                                    let file_path = entry.path();
                                    if file_path.is_file() && file_path.file_name() == Some("finalytics".as_ref()) {
                                        db_path = file_path.to_string_lossy().to_string();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let test_dir = exe_dir.to_string_lossy().to_string();
        let test_dir = test_dir.replace("deps", "build");
        for entry in fs::read_dir(test_dir).expect("Failed to read target/debug/build directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // Check if the directory starts with "finalytics"
                    if let Some(dir_name) = path.file_name() {
                        if dir_name.to_string_lossy().starts_with("finalytics") {
                            // Search for the "finalytics" file within this subdirectory
                            for entry in fs::read_dir(&path).ok().unwrap() {
                                if let Ok(entry) = entry {
                                    let file_path = entry.path();
                                    if file_path.is_file() && file_path.file_name() == Some("finalytics".as_ref()) {
                                        db_path = file_path.to_string_lossy().to_string();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    db_path.clone()
});

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
    let conn = Connection::open(&**DB_PATH).expect("Failed to open database");
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
    let conn = Connection::open(&**DB_PATH).expect("Failed to open database");
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
