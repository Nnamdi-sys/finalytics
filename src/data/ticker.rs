use reqwest;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::error::Error;
use polars::prelude::*;
use std::collections::HashMap;
use chrono::{Duration, NaiveDateTime, Utc};
use crate::analytics::sentiment::{News, scrape_news};
use crate::utils::date_utils::{time_to_maturity, to_date, to_timestamp};
use crate::data::keys::Fundamentals;
use crate::database::db::get_symbol;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ticker {
    pub symbol: String,
    pub name: String,
    pub category: String,
    pub asset_class: String,
    pub exchange: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerSummaryStats {
    pub symbol: String,
    pub display_name: String,
    pub full_exchange_name: String,
    pub currency: String,
    pub regular_market_time: i64,
    pub regular_market_price: f64,
    pub regular_market_change_percent: f64,
    pub regular_market_volume: f64,
    pub regular_market_open: f64,
    pub regular_market_day_high: f64,
    pub regular_market_day_low: f64,
    pub regular_market_previous_close: f64,
    pub fifty_two_week_high: f64,
    pub fifty_two_week_low: f64,
    pub fifty_two_week_change_percent: f64,
    pub fifty_day_average: f64,
    pub two_hundred_day_average: f64,
    #[serde(rename = "epsTrailingTwelveMonths")]
    pub trailing_eps: f64,
    #[serde(rename = "epsCurrentYear")]
    pub current_eps: f64,
    pub eps_forward: f64,
    #[serde(rename = "trailingPE")]
    pub trailing_pe: f64,
    #[serde(rename = "priceEpsCurrentYear")]
    pub current_pe: f64,
    #[serde(rename = "forwardPE")]
    pub forward_pe: f64,
    #[serde(default)]
    pub dividend_rate: f64,
    #[serde(default)]
    pub dividend_yield: f64,
    pub book_value: f64,
    pub price_to_book: f64,
    pub market_cap: f64,
    pub shares_outstanding: f64,
    pub average_analyst_rating: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct OptionContract {
    contractSymbol: String,
    strike: f64,
    currency: String,
    lastPrice: f64,
    #[serde(default)]
    change: f64,
    #[serde(default)]
    percentChange: f64,
    #[serde(default)]
    openInterest: f64,
    #[serde(default)]
    bid: f64,
    #[serde(default)]
    ask: f64,
    contractSize: String,
    expiration: i64,
    lastTradeDate: i64,
    impliedVolatility: f64,
    inTheMoney: bool,
}

#[derive(Debug)]
pub struct Options {
    pub ticker_price: f64,
    pub expiration_dates: Vec<String>,
    pub ttms: Vec<f64>,
    pub strikes: Vec<f64>,
    pub chain: DataFrame
}

#[derive(Debug, Deserialize)]
struct Financials {
    timeseries: TimeSeries,
}

#[derive(Debug, Deserialize)]
struct TimeSeries {
    result: Vec<HashMap<String, Value>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Object {
    asOfDate: String,
    reportedValue: Figure,
}

#[derive(Debug, Deserialize)]
struct Figure {
    raw: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Interval {
    TwoMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    SixtyMinutes,
    NinetyMinutes,
    OneHour,
    OneDay,
    FiveDays,
    OneWeek,
    OneMonth,
    ThreeMonths,
}

impl Interval {
    pub fn to_string(&self) -> String {
        match self {
            Interval::TwoMinutes => "2m".to_string(),
            Interval::FiveMinutes => "5m".to_string(),
            Interval::FifteenMinutes => "15m".to_string(),
            Interval::ThirtyMinutes => "30m".to_string(),
            Interval::SixtyMinutes => "60m".to_string(),
            Interval::NinetyMinutes => "90m".to_string(),
            Interval::OneHour => "1h".to_string(),
            Interval::OneDay => "1d".to_string(),
            Interval::FiveDays => "5d".to_string(),
            Interval::OneWeek => "1wk".to_string(),
            Interval::OneMonth => "1mo".to_string(),
            Interval::ThreeMonths => "3mo".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Interval {
        match s {
            "2m" => Interval::TwoMinutes,
            "5m" => Interval::FiveMinutes,
            "15m" => Interval::FifteenMinutes,
            "30m" => Interval::ThirtyMinutes,
            "60m" => Interval::SixtyMinutes,
            "90m" => Interval::NinetyMinutes,
            "1h" => Interval::OneHour,
            "1d" => Interval::OneDay,
            "5d" => Interval::FiveDays,
            "1wk" => Interval::OneWeek,
            "1mo" => Interval::OneMonth,
            "3mo" => Interval::ThreeMonths,
            _ => Interval::OneDay,
        }
    }
}

impl Ticker {
    /// Creates a new Ticker struct if the symbol is valid
    ///
    /// # Arguments
    ///
    /// * `symbol` - Ticker Symbol (e.g. AAPL)
    ///
    /// # Returns
    ///
    /// * `Ticker` - Ticker Metadata
    pub async fn new(symbol: &str) -> Result<Ticker, Box<dyn Error>> {
        let ticker = get_symbol(symbol)?;
        Ok(ticker)
    }

    /// Fetches Current Ticker Price from Yahoo Finance
    ///
    /// # Returns
    ///
    /// * `f64` - Ticker Price
    pub async fn get_quote(&self) -> Result<f64, Box<dyn Error>> {
        let url = format!("https://query2.finance.yahoo.com/v7/finance/options/{}", self.symbol);
        let response = reqwest::get(&url).await?;
        let result= response.json::<Value>().await?;
        let quote = result["optionChain"]["result"][0]["quote"]["regularMarketPrice"].as_f64().unwrap();
        Ok(quote)
    }

    /// Fetches Ticker Current Summary Stats from Yahoo Finance
    ///
    /// # Returns
    ///
    /// * `Value` - Ticker Summary Stats
    pub async fn get_ticker_stats(&self) -> Result<TickerSummaryStats, Box<dyn Error>> {
        let url = format!("https://query2.finance.yahoo.com/v7/finance/options/{}", self.symbol);
        let response = reqwest::get(&url).await?;
        let result= response.json::<Value>().await?;
        let value = &result["optionChain"]["result"][0]["quote"].to_string();
        let stats: TickerSummaryStats = serde_json::from_value(value.parse()?).expect("Failed to deserialize into MyStruct");
        Ok(stats)
    }

    /// Returns the Ticker OHLCV Data from Yahoo Finance for a given time range
    ///
    /// # Arguments
    ///
    /// * `start` - Start Date in YYYY-MM-DD format
    /// * `end` - End Date in YYYY-MM-DD format
    /// * `interval` - Time interval enum
    ///
    /// # Returns
    /// * `DataFrame` - Ticker OHLCV Data
    pub async fn get_chart(
        &self,
        start: &str,
        end: &str,
        interval: Interval
    ) -> Result<DataFrame, Box<dyn Error>> {
        let period1 = to_timestamp(start)?;
        let period2 = to_timestamp(end)?;
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?period1={}&period2={}&interval={}",
            self.symbol, period1, period2, interval.to_string()
        );
        let response = reqwest::get(&url).await?;
        let result= response.json::<Value>().await?;

        let value = &result["chart"]["result"][0];
        let timestamp = &value["timestamp"]
            .as_array()
            .ok_or(format!("timestamp array not found: {result}"))?
            .iter()
            .filter_map(|ts| Some(NaiveDateTime::from_timestamp_opt(ts.as_i64()?, 0)?))
            .collect::<Vec<NaiveDateTime>>();

        let indicators = &value["indicators"]["quote"][0];

        let open = indicators["open"]
            .as_array()
            .ok_or("open array not found")?
            .iter()
            .map(|o| o.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let high = indicators["high"]
            .as_array()
            .ok_or("high array not found")?
            .iter()
            .map(|h| h.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let low = indicators["low"]
            .as_array()
            .ok_or("low array not found")?
            .iter()
            .map(|l| l.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let close = indicators["close"]
            .as_array()
            .ok_or("close array not found")?
            .iter()
            .map(|c| c.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let volume = indicators["volume"]
            .as_array()
            .ok_or("volume array not found")?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let adjclose = &value["indicators"]["adjclose"][0]["adjclose"]
            .as_array()
            .unwrap_or_else(|| {
                indicators["close"]
                    .as_array()
                    .ok_or("close array not found")
                    .unwrap_or_else(|_| {
                        indicators["close"]
                            .as_array()
                            .expect("close array not found")
                    })
            })
            .iter()
            .map(|c| c.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let mut df = df!(
        "timestamp" => &timestamp,
        "open" => &open,
        "high" => &high,
        "low" => &low,
        "close" => &close,
        "volume" => &volume,
        "adjclose" => &adjclose
    )?;

        // check if any adjclose values are 0.0
        let mask = df.column("adjclose")?.gt(0.0)?;
        df = df.filter(&mask)?;
        Ok(df)
    }

    /// Returns Ticker Option Chain Data from Yahoo Finance for all available expirations
    ///
    /// # Returns
    ///
    /// * `Options` - Ticker Option Chain Data
    pub async fn get_options(&self) -> Result<Options, Box<dyn Error>> {
        let url = format!("https://query2.finance.yahoo.com/v7/finance/options/{}", self.symbol);
        let response = reqwest::get(&url).await?;
        let result = response.json::<Value>().await?;
        let ticker_price = result["optionChain"]["result"][0]["quote"]["regularMarketPrice"].as_f64().unwrap();
        let expiration_dates = &result["optionChain"]["result"][0]["expirationDates"];
        let strike_values = &result["optionChain"]["result"][0]["strikes"];
        let strikes = strike_values.as_array().unwrap()
            .iter().map(|x| x.as_f64().unwrap()).collect::<Vec<f64>>();
        let timestamps = expiration_dates.as_array().unwrap().iter()
            .map(|x| x.as_i64().unwrap()).collect::<Vec<i64>>();
        let ttms = timestamps.clone().iter().map(|x| time_to_maturity(*x))
            .collect::<Vec<f64>>();
        let expiration_dates = timestamps.iter().map(|x| to_date(*x))
            .collect::<Vec<String>>();
        let mut options_chain = DataFrame::default();
        for t in timestamps.iter() {
            let url = format!("https://query2.finance.yahoo.com/v7/finance/options/{}?date={}", self.symbol, t);
            let response = reqwest::get(&url).await?;
            let result = response.json::<Value>().await?;
            let expiration = to_date(*t);
            let ttm = time_to_maturity(*t);
            let calls = &result["optionChain"]["result"][0]["options"][0]["calls"];
            let calls_vec: Vec<OptionContract> = serde_json::from_value(calls.to_string().parse()?)
                .expect("Failed to deserialize into Option Chain");
            let calls_df = df!(
                "expiration" => calls_vec.iter().map(|_| &*expiration).collect::<Vec<&str>>(),
                "ttm" => calls_vec.iter().map(|_| ttm).collect::<Vec<f64>>(),
                "type" => calls_vec.iter().map(|_| "call").collect::<Vec<&str>>(),
                "contractSymbol" => calls_vec.iter().map(|x| x.contractSymbol.as_str()).collect::<Vec<&str>>(),
                "strike" => calls_vec.iter().map(|x| x.strike).collect::<Vec<f64>>(),
                "currency" => calls_vec.iter().map(|x| x.currency.as_str()).collect::<Vec<&str>>(),
                "lastPrice" => calls_vec.iter().map(|x| x.lastPrice).collect::<Vec<f64>>(),
                "change" => calls_vec.iter().map(|x| x.change).collect::<Vec<f64>>(),
                "percentChange" => calls_vec.iter().map(|x| x.percentChange).collect::<Vec<f64>>(),
                "openInterest" => calls_vec.iter().map(|x| x.openInterest).collect::<Vec<f64>>(),
                "bid" => calls_vec.iter().map(|x| x.bid).collect::<Vec<f64>>(),
                "ask" => calls_vec.iter().map(|x| x.ask).collect::<Vec<f64>>(),
                "contractSize" => calls_vec.iter().map(|x| x.contractSize.as_str()).collect::<Vec<&str>>(),
                "lastTradeDate" => calls_vec.iter().map(|x| NaiveDateTime::from_timestamp_opt(x.lastTradeDate, 0).unwrap()).collect::<Vec<NaiveDateTime>>(),
                "impliedVolatility" => calls_vec.iter().map(|x| x.impliedVolatility).collect::<Vec<f64>>(),
                "inTheMoney" => calls_vec.iter().map(|x| x.inTheMoney).collect::<Vec<bool>>(),
            )?;
            let puts = &result["optionChain"]["result"][0]["options"][0]["puts"];
            let puts_vec: Vec<OptionContract> = serde_json::from_value(puts.to_string().parse()?)
                .expect("Failed to deserialize into Option Chain");
            let puts_df = df!(
                "expiration" => puts_vec.iter().map(|_| &*expiration).collect::<Vec<&str>>(),
                "ttm" => puts_vec.iter().map(|_| ttm).collect::<Vec<f64>>(),
                "type" => puts_vec.iter().map(|_| "put").collect::<Vec<&str>>(),
                "contractSymbol" => puts_vec.iter().map(|x| x.contractSymbol.as_str()).collect::<Vec<&str>>(),
                "strike" => puts_vec.iter().map(|x| x.strike).collect::<Vec<f64>>(),
                "currency" => puts_vec.iter().map(|x| x.currency.as_str()).collect::<Vec<&str>>(),
                "lastPrice" => puts_vec.iter().map(|x| x.lastPrice).collect::<Vec<f64>>(),
                "change" => puts_vec.iter().map(|x| x.change).collect::<Vec<f64>>(),
                "percentChange" => puts_vec.iter().map(|x| x.percentChange).collect::<Vec<f64>>(),
                "openInterest" => puts_vec.iter().map(|x| x.openInterest).collect::<Vec<f64>>(),
                "bid" => puts_vec.iter().map(|x| x.bid).collect::<Vec<f64>>(),
                "ask" => puts_vec.iter().map(|x| x.ask).collect::<Vec<f64>>(),
                "contractSize" => puts_vec.iter().map(|x| x.contractSize.as_str()).collect::<Vec<&str>>(),
                "lastTradeDate" => puts_vec.iter().map(|x| NaiveDateTime::from_timestamp_opt(x.lastTradeDate, 0).unwrap()).collect::<Vec<NaiveDateTime>>(),
                "impliedVolatility" => puts_vec.iter().map(|x| x.impliedVolatility).collect::<Vec<f64>>(),
                "inTheMoney" => puts_vec.iter().map(|x| x.inTheMoney).collect::<Vec<bool>>(),
            )?;
            let df = calls_df.vstack(&puts_df)?;
            options_chain.vstack_mut(&df)?;
        }

        Ok(Options{
            ticker_price,
            expiration_dates,
            ttms,
            strikes,
            chain: options_chain
        })
    }

    /// Returns Ticker News from Google News Search for a given time range
    ///
    /// # Arguments
    ///
    /// * `start` - Start Date in YYYY-MM-DD format
    /// * `end` - End Date in YYYY-MM-DD format
    ///
    /// # Returns
    ///
    /// * `Vec<News>` - Ticker News
    pub async fn get_news(
        &self,
        start: &str,
        end: &str
    ) -> Result<Vec<News>, Box<dyn Error>> {
        let symbol = if self.asset_class == "CRYPTOCURRENCY" {self.symbol.replace("-USD", "")} else {self.symbol.clone()};
        let token = format!("({} OR {})", &symbol, &self.name);
        let result = scrape_news(&token, start, end).await?;
        Ok(result)
    }

    /// Returns Ticker Fundamental Data from Yahoo Finance for a given statement type and frequency
    ///
    /// # Arguments
    ///
    /// * `statement_type` - Statement Type (e.g. income-statement, balance-sheet, cash-flow)
    /// * `frequency` - Frequency (e.g. annual, quarterly)
    ///
    /// # Returns
    ///
    /// * `DataFrame` - Ticker Fundamental Data
    pub async fn get_fundamentals(
        &self,
        statement_type: &str,
        frequency: &str
    ) -> Result<DataFrame, Box<dyn Error>> {
        if self.asset_class != "Stocks"{panic!("Asset class must be stocks")}
        let symbol = self.symbol.clone();
        let period1 = (Utc::now() - Duration::days(365 * 5)).timestamp();
        let period2 = Utc::now().timestamp();
        let _type = match statement_type {
            "income-statement" => Fundamentals.get_income_statement_items(frequency),
            "balance-sheet" => Fundamentals.get_balance_sheet_items(frequency),
            "cash-flow" => Fundamentals.get_cash_flow_items(frequency),
            _ => unimplemented!("Statement Type Not Supported"),
        };
        let _type_clone = _type.clone();
        let url = format!("https://query2.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/\
        timeseries/{symbol}?symbol={symbol}&type={_type}&period1={period1}&period2={period2}");
        let response = reqwest::get(&url).await?;
        let result = response.json::<Value>().await?;
        let data: Financials = serde_json::from_value(result).expect("Failed to parse JSON");
        let mut columns: Vec<Series> = vec![];
        for item in &data.timeseries.result{
            // convert to polars dataframe
            for (key, value) in item {
                if _type_clone.contains(&key.as_str()){
                    let items: Vec<Object> = serde_json::from_value(value.to_string().parse()?)
                        .expect("Failed to deserialize into Object");
                    if columns.len() == 0{
                        let date_vec = items.iter().map(|x| x.asOfDate.clone()).collect::<Vec<String>>();
                        let date_series = Series::new("asOfDate", &date_vec);
                        columns.push(date_series);
                    }

                    if items.len() == columns[0].len(){
                        let vars_vec = items.iter().map(|x| x.reportedValue.raw).collect::<Vec<f64>>();
                        let vars_series = Series::new(&*key.as_str().replace(frequency, ""), &vars_vec);
                        columns.push(vars_series);
                    }

                }
            }
        }
        let df = DataFrame::new(columns).unwrap();
        Ok(df)
    }

}













