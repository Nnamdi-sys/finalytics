use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use polars::prelude::{concat, Column, DataFrame, IntoLazy, UnionArgs};
use serde_json::{json, Value};
use crate::data::yahoo::screeners::{CryptoScreener, EquityScreener, EtfScreener, FieldMetadata, FutureScreener, IndexScreener, MutualFundScreener};
use crate::data::yahoo::web::post_json_response;
use crate::prelude::{QuoteType, Screener};

#[derive(Debug)]
pub enum ScreenerMetric {
    Equity(EquityScreener),
    MutualFund(MutualFundScreener),
    Index(IndexScreener),
    Etf(EtfScreener),
    Future(FutureScreener),
    Crypto(CryptoScreener),
}

impl ScreenerMetric {
    pub fn metrics(&self) -> &'static HashMap<String, FieldMetadata> {
        match self {
            ScreenerMetric::Equity(_) => EquityScreener::metrics(),
            ScreenerMetric::MutualFund(_) => MutualFundScreener::metrics(),
            ScreenerMetric::Index(_) => IndexScreener::metrics(),
            ScreenerMetric::Etf(_) => EtfScreener::metrics(),
            ScreenerMetric::Future(_) => FutureScreener::metrics(),
            ScreenerMetric::Crypto(_) => CryptoScreener::metrics(),
        }
    }

    pub fn metadata(&self) -> &'static FieldMetadata {
        match self {
            ScreenerMetric::Equity(e) => e.metadata(),
            ScreenerMetric::MutualFund(m) => m.metadata(),
            ScreenerMetric::Index(i) => i.metadata(),
            ScreenerMetric::Etf(e) => e.metadata(),
            ScreenerMetric::Future(f) => f.metadata(),
            ScreenerMetric::Crypto(c) => c.metadata(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ScreenerMetric::Equity(e) => e.name(),
            ScreenerMetric::MutualFund(m) => m.name(),
            ScreenerMetric::Index(i) => i.name(),
            ScreenerMetric::Etf(e) => e.name(),
            ScreenerMetric::Future(f) => f.name(),
            ScreenerMetric::Crypto(c) => c.name(),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ScreenerMetric::Equity(e) => e.description(),
            ScreenerMetric::MutualFund(m) => m.description(),
            ScreenerMetric::Index(i) => i.description(),
            ScreenerMetric::Etf(e) => e.description(),
            ScreenerMetric::Future(f) => f.description(),
            ScreenerMetric::Crypto(c) => c.description(),
        }
    }

    pub fn data_type(&self) -> &'static str {
        match self {
            ScreenerMetric::Equity(e) => e.data_type(),
            ScreenerMetric::MutualFund(m) => m.data_type(),
            ScreenerMetric::Index(i) => i.data_type(),
            ScreenerMetric::Etf(e) => e.data_type(),
            ScreenerMetric::Future(f) => f.data_type(),
            ScreenerMetric::Crypto(c) => c.data_type(),
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            ScreenerMetric::Equity(e) => e.unit(),
            ScreenerMetric::MutualFund(m) => m.unit(),
            ScreenerMetric::Index(i) => i.unit(),
            ScreenerMetric::Etf(e) => e.unit(),
            ScreenerMetric::Future(f) => f.unit(),
            ScreenerMetric::Crypto(c) => c.unit(),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            ScreenerMetric::Equity(e) => (*e).into(),
            ScreenerMetric::MutualFund(m) => (*m).into(),
            ScreenerMetric::Index(i) => (*i).into(),
            ScreenerMetric::Etf(e) => (*e).into(),
            ScreenerMetric::Future(f) => (*f).into(),
            ScreenerMetric::Crypto(c) => (*c).into(),
        }
    }
    pub fn validate_value(&self, value: &str) -> Result<String, String> {
        match self {
            ScreenerMetric::Equity(e) => e.validate_value(value),
            ScreenerMetric::MutualFund(m) => m.validate_value(value),
            ScreenerMetric::Index(i) => i.validate_value(value),
            ScreenerMetric::Etf(e) => e.validate_value(value),
            ScreenerMetric::Future(f) => f.validate_value(value),
            ScreenerMetric::Crypto(c) => c.validate_value(value),
        }
    }
}


#[derive(Debug)]
pub enum ScreenerFilter {
    Eq(ScreenerMetric, f64),
    EqStr(ScreenerMetric, &'static str),
    Gte(ScreenerMetric, f64),
    Lte(ScreenerMetric, f64),
    Gt(ScreenerMetric, f64),
    Lt(ScreenerMetric, f64),
    Btwn(ScreenerMetric, f64, f64),
    Custom(String)
}

impl ScreenerFilter {
    fn to_json(&self) -> Value {
        match self {
            ScreenerFilter::Eq(metric, val) => json!({
                "operator": "eq",
                "operands": [
                    metric.as_str(),
                    val
                ]
            }),
            ScreenerFilter::EqStr(metric, val) => json!({
                "operator": "eq",
                "operands": [
                    metric.as_str(),
                    metric.validate_value(val).unwrap()
                ]
            }),
            ScreenerFilter::Gte(metric, val) => json!({
                "operator": "gte",
                "operands": [
                    metric.as_str(),
                    val
                ]
            }),
            ScreenerFilter::Lte(metric, val) => json!({
                "operator": "lte",
                "operands": [
                    metric.as_str(),
                    val
                ]
            }),
            ScreenerFilter::Gt(metric, val) => json!({
                "operator": "gt",
                "operands": [
                    metric.as_str(),
                    val
                ]
            }),
            ScreenerFilter::Lt(metric, val) => json!({
                "operator": "lt",
                "operands": [
                    metric.as_str(),
                    val
                ]
            }),
            ScreenerFilter::Btwn(metric, val1, val2) => json!({
                "operator": "BTWN",
                "operands": [
                    metric.as_str(),
                    val1,
                    val2
                ]
            }),
            ScreenerFilter::Custom(filter_str) => {
                serde_json::from_str(filter_str).unwrap()
            }
        }
    }
}


#[derive(Default)]
pub struct ScreenerBuilder {
    pub quote_type: Option<QuoteType>,
    pub filters: Vec<ScreenerFilter>,
    pub sort_field: Option<ScreenerMetric>,
    pub sort_descending: bool,
    pub offset: usize,
    pub size: usize,
}

impl ScreenerBuilder {
    pub fn quote_type(mut self, quote_type: QuoteType) -> Self {
        self.quote_type = Some(quote_type);
        self
    }

    pub fn add_filter(mut self, filter: ScreenerFilter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn sort_by(mut self, metric: ScreenerMetric, descending: bool) -> Self {
        self.sort_field = Some(metric);
        self.sort_descending = descending;
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub async fn build(self) -> Result<Screener, Box<dyn Error>> {
        let url = "https://query2.finance.yahoo.com/v1/finance/screener".to_string();
        let filters_json: Vec<Value> = self.filters.iter().map(|f| f.to_json()).collect();
        let quote_type_val = self.quote_type.unwrap(); // Take ownership for final Screener
        let quote_type_str = quote_type_val.as_ref();
        let default_sort_field = match quote_type_val {
            QuoteType::Equity => "intradaymarketcap",
            QuoteType::MutualFund => "fundnetassets",
            QuoteType::Index => "percentchange",
            QuoteType::Etf => "fundnetassets",
            QuoteType::Future => "percentchange",
            QuoteType::Crypto => "intradaymarketcap",
        };
        let sort_field = self.sort_field.as_ref().map(|m| m.as_str()).unwrap_or(default_sort_field);
        let sort_type = if self.sort_descending { "DESC" } else { "ASC" };

        let mut current_offset = self.offset;
        let mut remaining = self.size;
        let mut all_quotes_dfs = Vec::new();
        let mut all_symbols = Vec::with_capacity(self.size);

        while remaining > 0 {
            let chunk_size = std::cmp::min(remaining, 250);
            let payload = json!({
            "offset": current_offset,
            "size": chunk_size,
            "sortField": sort_field,
            "sortType": sort_type,
            "quoteType": quote_type_str,
            "query": {
                "operator": "AND",
                "operands": &filters_json
            }
        });

            let result = post_json_response(url.clone(), payload).await?;
            let json_array = result["finance"]["result"][0]["quotes"].as_array().ok_or("No quotes array")?;
            let df = json_to_df(json_array)?;
            let rows_retrieved = df.height();

            if rows_retrieved == 0 {
                break; // No more data available
            }

            // Process symbols from this chunk
            let symbols: Vec<String> = df.column("Symbol")?.str()?.into_no_null_iter()
                .map(|s| s.trim_matches('"').to_string())
                .collect();
            all_symbols.extend(symbols);
            all_quotes_dfs.push(df.lazy());

            // Update tracking variables
            current_offset += rows_retrieved;
            remaining = remaining.saturating_sub(rows_retrieved);

            // Exit early if we got fewer items than requested
            if rows_retrieved < chunk_size {
                break;
            }
        }

        // Combine all dataframes
        let result_df = concat(&all_quotes_dfs, UnionArgs {
            diagonal: true,
            ..Default::default()
        })?.collect()?;

        Ok(Screener {
            quote_type: quote_type_val,
            symbols: all_symbols,
            result: result_df
        })
    }
}

fn json_to_df(json_array: &[Value]) -> Result<DataFrame, Box<dyn Error>> {
    // Flatten keys across all objects to build a full schema
    let mut all_keys = BTreeSet::new();
    for obj in json_array {
        if let Value::Object(map) = obj {
            for k in map.keys() {
                all_keys.insert(k.clone());
            }
        }
    }

    // Build columns as hashmaps of vectors
    let mut columns: HashMap<String, Vec<Option<Value>>> = HashMap::new();
    for key in &all_keys {
        columns.insert(key.clone(), Vec::with_capacity(json_array.len()));
    }

    for obj in json_array {
        if let Value::Object(map) = obj {
            for key in &all_keys {
                columns.get_mut(key).unwrap().push(map.get(key).cloned());
            }
        }
    }

    // Define column name mappings
    let column_mappings = vec![
        ("symbol", "Symbol"),
        ("longName", "Name"),
        ("fullExchangeName", "Exchange"),
        ("region", "Region"),
        ("currency", "Currency"),
        ("marketCap", "Market Cap"),
        ("netAssets", "Net Assets"),
        ("netExpenseRatio", "Expense Ratio"),
        ("ytdReturn", "YTD Return"),
        ("trailingThreeMonthsReturn", "3-Month Return"),
        ("regularMarketPrice", "Price"),
        ("regularMarketDayRange", "Day Range"),
        ("regularMarketChangePercent", "Change %"),
        ("regularMarketVolume", "Volume"),
        ("fiftyTwoWeekRange", "52-Week Range"),
        ("fiftyTwoWeekChangePercent", "52-Week Change %"),
        ("fiftyDayAverage", "50-Day Avg"),
        ("twoHundredDayAverage", "200-Day Avg"),
        ("epsCurrentYear", "EPS (Current Year)"),
        ("priceEpsCurrentYear", "Price/EPS"),
        ("priceToBook", "Price/Book"),
        ("averageAnalystRating", "Analyst Rating"),
    ];

    // Convert columns to Series with new names
    let mut series = Vec::with_capacity(column_mappings.len());

    for (original_key, display_name) in column_mappings {
        if let Some(col) = columns.get(original_key) {
            let s = build_series(display_name, col);
            series.push(s);
        }
    }

    let df = DataFrame::new(series)?;
    Ok(df)
}

fn build_series(key: &str, col: &[Option<Value>]) -> Column {
    if col.iter().all(|v| v.as_ref().map(|v| v.is_number()).unwrap_or(true)) {
        let vals: Vec<Option<f64>> = col.iter().map(|v| v.as_ref().and_then(|v| v.as_f64())).collect();
        Column::new(key.into(), vals)
    } else if col.iter().all(|v| v.as_ref().map(|v| v.is_boolean()).unwrap_or(true)) {
        let vals: Vec<Option<bool>> = col.iter().map(|v| v.as_ref().and_then(|v| v.as_bool())).collect();
        Column::new(key.into(), vals)
    } else {
        let vals: Vec<Option<String>> = col.iter().map(|v| v.as_ref().map(|v| v.to_string())).collect();
        Column::new(key.into(), vals)
    }
}


