use std::{env, fs};
use std::error::Error;
use ejdb::{bson, Database};
use ejdb::query::{Q, QH};
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
    }else{
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
pub fn get_symbol(symbol: &str) -> Result<Ticker, Box<dyn Error>> {
    let db = Database::open(&**DB_PATH).unwrap();
    let col = db.collection("symbols").unwrap();
    if let Some(document) = col.query(Q.field("symbol").eq(symbol), QH.empty()).find_one().unwrap() {
        let ticker = Ticker {
            symbol: document.get_str("symbol").unwrap().to_string(),
            name: document.get_str("name").unwrap().to_string(),
            category: document.get_str("category").unwrap().to_string(),
            asset_class: document.get_str("asset_class").unwrap().to_string(),
            exchange: document.get_str("exchange").unwrap().to_string(),
        };
        Ok(ticker)
    } else {
        panic!("Invalid Symbol")
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
pub fn get_symbols(asset_class: AssetClass, category: Category, exchange: Exchange) -> Result<Vec<Ticker>, Box<dyn Error>> {
    let db = Database::open(&**DB_PATH).unwrap();
    let col = db.collection("symbols").unwrap();
    let q1 = Q.field("asset_class").contained_in(asset_class.to_string_vec());
    let q2 = Q.field("category").contained_in(category.to_string_vec());
    let q3 = Q.field("exchange").contained_in(exchange.to_string_vec());
    let result = col.query(Q.and(vec![q1,q2,q3]), QH.empty()).find().unwrap();
    let symbols: Vec<Ticker> = result.map(|x| bson::from_bson( x.unwrap().into()).unwrap()).collect();
    Ok(symbols)
}