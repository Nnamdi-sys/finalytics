use std::collections::HashMap;
use finalytics::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
#[pyo3(name = "get_symbols")]
/// Fetches ticker symbols that closely match the specified query and asset class
///
/// # Arguments
///
/// * `query` - `str` - ticker symbol query
/// * `asset_class` - `str` - asset class (Equity, ETF, Mutual Fund, Index, Currency, Futures, Crypto)
///
/// # Returns
///
/// `dict` - dictionary of ticker symbols and names
///
/// # Example
///
/// ```
/// import finalytics
///
/// symbols = finalytics.get_symbols("Apple", "Equity")
/// print(symbols)
/// ```
pub fn get_symbols_py(query: String, asset_class: String) -> PyObject {
    let asset_class = match asset_class.as_str() {
        "Equity" => AssetClass::Stocks,
        "ETF" => AssetClass::ETFs,
        "Mutual Fund" => AssetClass::MutualFunds,
        "Index" => AssetClass::Indices,
        "Currency" => AssetClass::Currencies,
        "Futures" => AssetClass::Futures,
        "Crypto" => AssetClass::Cryptocurrencies,
        _ => panic!("Asset class must be one of: Equity, ETF, Mutual Fund, Index, Currency, Futures, Crypto"),
    };
    let tickers = get_symbols(asset_class, Category::All, Exchange::All).unwrap();
    let symbols = tickers
        .iter()
        .filter(|tc| tc.symbol.to_lowercase().contains(&query.to_lowercase())
            || tc.name.to_lowercase().contains(&query.to_lowercase()))
        .map(|tc| (tc.symbol.clone(), tc.name.clone()))
        .collect::<HashMap<String, String>>();
    Python::with_gil(|py| {
        let py_dict = PyDict::new(py);
        for (symbol, name) in symbols {
            py_dict.set_item(symbol, name).unwrap();
        }
        py_dict.into()
    })
}
