use bincode;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use finalytics::data::db::get_symbols;
use finalytics::data::keys::{AssetClass, Category, Exchange};

fn main() {
    let tickers = get_symbols(
        AssetClass::All,
        Category::All,
        Exchange::All
    ).unwrap();

    let mut map = HashMap::new();
    for ticker in tickers {
        map.insert(ticker.symbol, ticker.name);
    }

    let serialized_data = bincode::serialize(&map).unwrap();

    let mut file = File::create("datalist.bin").unwrap();
    file.write_all(&serialized_data).unwrap();
}