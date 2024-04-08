use std::collections::HashMap;
use std::error::Error;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::ticker::Ticker;
use crate::utils::date_utils::{round_datetime_to_day, round_datetime_to_hour, round_datetime_to_minute, time_to_maturity, to_date, to_datetime, to_timestamp};
use crate::data::keys::Fundamentals;


pub trait TickerData {
    fn get_quote(&self) -> impl std::future::Future<Output = Result<f64, Box<dyn Error>>>;
    fn get_ticker_stats(&self) -> impl std::future::Future<Output = Result<TickerSummaryStats, Box<dyn Error>>>;
    fn get_chart(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn get_options(&self) -> impl std::future::Future<Output = Result<Options, Box<dyn Error>>>;
    fn get_fundamentals(&self, statement_type: &str, frequency: &str) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerSummaryStats {
    pub symbol: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
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
    #[serde(default)]
    pub fifty_two_week_change_percent: f64,
    pub fifty_day_average: f64,
    pub two_hundred_day_average: f64,
    #[serde(default)]
    #[serde(rename = "epsTrailingTwelveMonths")]
    pub trailing_eps: f64,
    #[serde(default)]
    #[serde(rename = "epsCurrentYear")]
    pub current_eps: f64,
    #[serde(default)]
    pub eps_forward: f64,
    #[serde(default)]
    #[serde(rename = "trailingPE")]
    pub trailing_pe: f64,
    #[serde(default)]
    #[serde(rename = "priceEpsCurrentYear")]
    pub current_pe: f64,
    #[serde(default)]
    #[serde(rename = "forwardPE")]
    pub forward_pe: f64,
    #[serde(default)]
    pub dividend_rate: f64,
    #[serde(default)]
    pub dividend_yield: f64,
    #[serde(default)]
    pub book_value: f64,
    #[serde(default)]
    pub price_to_book: f64,
    #[serde(default)]
    pub market_cap: f64,
    #[serde(default)]
    pub shares_outstanding: f64,
    #[serde(default)]
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
struct FundamentalsResponse {
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

    pub fn to_days(&self) -> f64 {
        match self {
            Interval::TwoMinutes => 2.0 / 24.0 * 60.0,
            Interval::FiveMinutes => 5.0 / 24.0 * 60.0,
            Interval::FifteenMinutes => 15.0 / 24.0 * 60.0,
            Interval::ThirtyMinutes => 30.0 / 24.0 * 60.0,
            Interval::SixtyMinutes => 60.0 / 24.0 * 60.0,
            Interval::OneHour => 60.0 / 24.0 * 60.0,
            Interval::NinetyMinutes => 90.0 / 24.0 * 60.0,
            Interval::OneDay => 1.0,
            Interval::FiveDays => 5.0,
            Interval::OneWeek => 5.0,
            Interval::OneMonth => 20.0,
            Interval::ThreeMonths => 60.0,
        }
    }
}

impl TickerData for Ticker {
    /// Fetches Current Ticker Price from Yahoo Finance
    async fn get_quote(&self) -> Result<f64, Box<dyn Error>> {
        let url = format!("https://query2.finance.yahoo.com/v6/finance/options/{}", self.ticker.symbol);
        let response = reqwest::get(&url).await?;
        let result= response.json::<Value>().await?;
        let quote = result["optionChain"]["result"][0]["quote"]["regularMarketPrice"].as_f64().unwrap();
        Ok(quote)
    }

    /// Fetches Ticker Current Summary Stats from Yahoo Finance
    async fn get_ticker_stats(&self) -> Result<TickerSummaryStats, Box<dyn Error>> {
        let url = format!("https://query2.finance.yahoo.com/v6/finance/options/{}", self.ticker.symbol);
        let response = reqwest::get(&url).await?;
        let result = response.json::<Value>().await?;
        let value = &result["optionChain"]["result"][0]["quote"].to_string();
        let stats: TickerSummaryStats = serde_json::from_value(value.parse()?).expect("Failed to deserialize into TickerSummaryStats");
        Ok(stats)
    }


    /// Returns the Ticker OHLCV Data from Yahoo Finance for a given time range
    async fn get_chart(&self) -> Result<DataFrame, Box<dyn Error>> {
        let period1 = to_timestamp(&self.start_date)?;
        let period2 = to_timestamp(&self.end_date)?;
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?period1={}&period2={}&interval={}",
            self.ticker.symbol, period1, period2, self.interval.to_string()
        );
        let response = reqwest::get(&url).await?;
        let result= response.json::<Value>().await?;

        let value = &result["chart"]["result"][0];
        let timestamp = &value["timestamp"]
            .as_array()
            .ok_or(format!("timestamp array not found: {result}"))?
            .iter()
            .map(|ts| {
                let timestamp = ts.as_i64().unwrap();
                let datetime = match self.interval {
                    Interval::OneDay | Interval::FiveDays | Interval::OneWeek | Interval::OneMonth | Interval::ThreeMonths => {
                        round_datetime_to_day(DateTime::from_timestamp(timestamp, 0).unwrap())
                    }
                    Interval::SixtyMinutes | Interval::OneHour => {
                        round_datetime_to_hour(DateTime::from_timestamp(timestamp, 0).unwrap())
                    },
                    Interval::NinetyMinutes | Interval::ThirtyMinutes | Interval::FifteenMinutes | Interval::FiveMinutes | Interval::TwoMinutes => {
                        round_datetime_to_minute(DateTime::from_timestamp(timestamp, 0).unwrap())
                    },
                };
                datetime
            })
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

        let df = df!(
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
        let df = df.filter(&mask)?;

        // check id any returned dates greater than end date
        let dt = to_datetime(&self.end_date)?;
        let mask = df["timestamp"]
            .datetime()?
            .as_datetime_iter()
            .map(|x| x.unwrap() < dt)
            .collect();
        let df = df.filter(&mask)?;
        Ok(df)
    }

    /// Returns Ticker Option Chain Data from Yahoo Finance for all available expirations
    async fn get_options(&self) -> Result<Options, Box<dyn Error>> {
        let url = format!("https://query2.finance.yahoo.com/v6/finance/options/{}", self.ticker.symbol);
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
            let url = format!("https://query2.finance.yahoo.com/v6/finance/options/{}?date={}", self.ticker.symbol, t);
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
                "lastTradeDate" => calls_vec.iter().map(|x| DateTime::from_timestamp(x.lastTradeDate, 0).unwrap().naive_local()).collect::<Vec<NaiveDateTime>>(),
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
                "lastTradeDate" => puts_vec.iter().map(|x| DateTime::from_timestamp(x.lastTradeDate, 0).unwrap().naive_local()).collect::<Vec<NaiveDateTime>>(),
                "impliedVolatility" => puts_vec.iter().map(|x| x.impliedVolatility).collect::<Vec<f64>>(),
                "inTheMoney" => puts_vec.iter().map(|x| x.inTheMoney).collect::<Vec<bool>>(),
            )?;
            let df = calls_df.vstack(&puts_df)?;
            options_chain.vstack_mut(&df)?;
        }

        Ok(Options {
            ticker_price,
            expiration_dates,
            ttms,
            strikes,
            chain: options_chain
        })
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
    async fn get_fundamentals(
        &self,
        statement_type: &str,
        frequency: &str
    ) -> Result<DataFrame, Box<dyn Error>> {
        if self.ticker.asset_class != "Stocks"{panic!("Asset class must be stocks")}
        let symbol = self.ticker.symbol.clone();
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
        let data: FundamentalsResponse = serde_json::from_value(result).expect("Failed to parse JSON");
        let mut columns: Vec<Series> = vec![];
        let mut temp_items: HashMap<String, Value> = HashMap::new();
        let mut init = 0;
        for item in &data.timeseries.result{
            // convert to polars dataframe
            for (key, value) in item {
                if _type_clone.contains(&key.as_str()){
                    let items: Vec<Object> = serde_json::from_value(value.to_string().parse()?)
                        .expect("Failed to deserialize into Object");
                    let date_vec = items.iter().map(|x| x.asOfDate.clone()).collect::<Vec<String>>();
                    if date_vec.len() < 4 {
                        temp_items.insert(key.clone(), value.clone());
                        break;
                    }
                    if init == 0 {
                        let date_series = Series::new("asOfDate", &date_vec);
                        columns.push(date_series);
                        init += 1;
                    }

                    if items.len() == columns[0].len(){
                        let vars_vec = items.iter().map(|x| x.reportedValue.raw).collect::<Vec<f64>>();
                        let vars_series = Series::new(&*key.as_str().replace(frequency, ""), &vars_vec);
                        columns.push(vars_series);
                    }
                    else {
                        let mut vars_vec: Vec<f64> = vec![];
                        for d in columns[0].iter(){
                            let mut found = false;
                            for i in 0..items.len(){
                                if items[i].asOfDate == d.to_string(){
                                    vars_vec.push(items[i].reportedValue.raw);
                                    found = true;
                                    break;
                                }
                            }
                            if !found{
                                vars_vec.push(0.0);
                            }
                        }
                        let vars_series = Series::new(&*key.as_str().replace(frequency, ""), &vars_vec);
                        columns.push(vars_series);
                    }

                }
            }
        }

        if temp_items.len() > 0 {
            for (key, value) in temp_items {
                let items: Vec<Object> = serde_json::from_value(value.to_string().parse()?)
                    .expect("Failed to deserialize into Object");
                let mut vars_vec: Vec<f64> = vec![];
                for d in columns[0].iter(){
                    let mut found = false;
                    for i in 0..items.len(){
                        if format!("\"{}\"", items[i].asOfDate) == d.to_string(){
                            vars_vec.push(items[i].reportedValue.raw);
                            found = true;
                            break;
                        }
                    }
                    if !found{
                        vars_vec.push(0.0);
                    }
                }
                let vars_series = Series::new(&*key.as_str().replace(frequency, ""), &vars_vec);
                columns.push(vars_series);
            }
        }
        let df = DataFrame::new(columns).unwrap();
        Ok(df)
    }

}