use polars::prelude::*;
use serde_json::Value;
use std::error::Error;
use std::collections::HashMap;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Semaphore;
use crate::models::ticker::Ticker;
use crate::utils::web_utils::{fetch_news, get_json_response};
use crate::utils::date_utils::{round_datetime_to_day, round_datetime_to_hour, round_datetime_to_minute, time_to_maturity, to_date, to_datetime, to_timestamp};
use crate::data::config::{Fundamentals, FundamentalsResponse, Interval, Object, OptionContract, Options, Quote, StatementFrequency, StatementType, TickerSummaryStats};


pub trait TickerData {
    fn get_quote(&self) -> impl std::future::Future<Output = Result<Quote, Box<dyn Error>>>;
    fn get_ticker_stats(&self) -> impl std::future::Future<Output = Result<TickerSummaryStats, Box<dyn Error>>>;
    fn get_chart(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn get_options(&self) -> impl std::future::Future<Output = Result<Options, Box<dyn Error>>>;
    fn get_fundamentals(&self, statement_type: StatementType, frequency: StatementFrequency) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn get_news(&self) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
}

impl TickerData for Ticker {
    /// Fetches Current Ticker Price from Yahoo Finance
    async fn get_quote(&self) -> Result<Quote, Box<dyn Error>> {
        let url = format!("https://query2.finance.yahoo.com/v6/finance/options/{}", self.ticker);
        let result= get_json_response(url).await?;
        let value = &result["optionChain"]["result"][0]["quote"].to_string();
        let quote: Quote = serde_json::from_value(value.parse()?)
            .map_err(|e| format!("Failed to deserialize into Quote: {}", e))?;
        Ok(quote)
    }

    /// Fetches Ticker Current Summary Stats from Yahoo Finance
    async fn get_ticker_stats(&self) -> Result<TickerSummaryStats, Box<dyn Error>> {
        let url = format!("https://query2.finance.yahoo.com/v6/finance/options/{}", self.ticker);
        let result = get_json_response(url).await?;
        let value = &result["optionChain"]["result"][0]["quote"].to_string();
        let stats: TickerSummaryStats = serde_json::from_value(value.parse()?)
            .map_err(|e| format!("Failed to deserialize into TickerSummaryStats: {}", e))?;
        Ok(stats)
    }


    /// Returns the Ticker OHLCV Data from Yahoo Finance for a given time range
    async fn get_chart(&self) -> Result<DataFrame, Box<dyn Error>> {
        let period1 = to_timestamp(&self.start_date)?;
        let period2 = to_timestamp(&self.end_date)?;
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?period1={}&period2={}&interval={}",
            self.ticker, period1, period2, self.interval.to_string()
        );
        let result= get_json_response(url).await?;

        let value = &result["chart"]["result"][0];
        let timestamp = &value["timestamp"]
            .as_array()
            .ok_or(format!("timestamp array not found for {}: {}", self.ticker, result))?
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
            .ok_or(format!("open array not found for {}: {}", self.ticker, result))?
            .iter()
            .map(|o| o.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let high = indicators["high"]
            .as_array()
            .ok_or(format!("high array not found for {}: {}", self.ticker, result))?
            .iter()
            .map(|h| h.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let low = indicators["low"]
            .as_array()
            .ok_or(format!("low array not found for {}: {}", self.ticker, result))?
            .iter()
            .map(|l| l.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let close = indicators["close"]
            .as_array()
            .ok_or(format!("close array not found for {}: {}", self.ticker, result))?
            .iter()
            .map(|c| c.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let volume = indicators["volume"]
            .as_array()
            .ok_or(format!("volume array not found for {}: {}", self.ticker, result))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let adjclose = &value["indicators"]["adjclose"][0]["adjclose"]
            .as_array()
            .unwrap_or_else(|| {
                indicators["close"]
                    .as_array()
                    .ok_or(format!("close array not found for {}: {}", self.ticker, result))
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
        let url = format!("https://query2.finance.yahoo.com/v6/finance/options/{}", self.ticker);
        let result = get_json_response(url).await?;

        let ticker_price = result["optionChain"]["result"][0]["quote"]["regularMarketPrice"]
            .as_f64()
            .ok_or("Failed to parse regularMarketPrice as f64")?;

        let expiration_dates = result["optionChain"]["result"][0]["expirationDates"]
            .as_array()
            .ok_or("Failed to parse expirationDates as array")?;

        let strike_values = result["optionChain"]["result"][0]["strikes"]
            .as_array()
            .ok_or("Failed to parse strikes as array")?;

        let strikes = strike_values
            .iter()
            .map(|x| x.as_f64().ok_or("Failed to parse strike as f64"))
            .collect::<Result<Vec<f64>, _>>()?;

        let timestamps = expiration_dates
            .iter()
            .map(|x| x.as_i64().ok_or("Failed to parse expiration date as i64"))
            .collect::<Result<Vec<i64>, _>>()?;

        let ttms = timestamps
            .iter()
            .map(|x| time_to_maturity(*x))
            .collect::<Vec<f64>>();

        let expiration_dates = timestamps
            .iter()
            .map(|x| to_date(*x))
            .collect::<Vec<String>>();

        let mut options_chain = DataFrame::default();

        for t in &timestamps {
            let url = format!(
                "https://query2.finance.yahoo.com/v6/finance/options/{}?date={}",
                self.ticker, t
            );
            let result = get_json_response(url).await?;
            let expiration = to_date(*t);
            let ttm = time_to_maturity(*t);

            let calls = &result["optionChain"]["result"][0]["options"][0]["calls"];
            let calls_vec: Vec<OptionContract> = serde_json::from_value(calls.clone())
                .map_err(|e| format!("Failed to deserialize calls into OptionContract: {}", e))?;

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
            let puts_vec: Vec<OptionContract> = serde_json::from_value(puts.clone())
                .map_err(|e| format!("Failed to deserialize puts into OptionContract: {}", e))?;

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
            chain: options_chain,
        })
    }


    /// Returns Ticker Fundamental Data from Yahoo Finance for a given statement type and frequency
    ///
    /// # Arguments
    ///
    /// * `statement_type` - StatementType
    /// * `frequency` - StatementFrequency
    ///
    /// # Returns
    ///
    /// * `DataFrame` - Ticker Fundamental Data
    async fn get_fundamentals(
        &self,
        statement_type: StatementType,
        frequency: StatementFrequency
    ) -> Result<DataFrame, Box<dyn Error>> {
        let symbol = self.ticker.clone();
        let period1 = (Utc::now() - Duration::days(365 * 5)).timestamp();
        let period2 = Utc::now().timestamp();
        let _type = match statement_type {
            StatementType::IncomeStatement => Fundamentals.get_income_statement_items(frequency),
            StatementType::BalanceSheet => Fundamentals.get_balance_sheet_items(frequency),
            StatementType::CashFlowStatement => Fundamentals.get_cash_flow_items(frequency),
            _ => unimplemented!("Statement Type Not Supported"),
        };
        let _type_clone = _type.clone();
        let url = format!("https://query2.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/\
        timeseries/{symbol}?symbol={symbol}&type={_type}&period1={period1}&period2={period2}");
        let result = get_json_response(url).await?;
        let data: FundamentalsResponse = serde_json::from_value(result)
            .map_err(|e| format!("Failed to deserialize into FundamentalsResponse: {}", e))?;
        let mut columns: Vec<Series> = vec![];
        let mut temp_items: HashMap<String, Value> = HashMap::new();
        let mut init = 0;
        for item in &data.timeseries.result{
            // convert to polars dataframe
            for (key, value) in item {
                if _type_clone.contains(&key.as_str()){
                    let items: Vec<Object> = serde_json::from_value(value.to_string().parse()?)
                        .map_err(|e| format!("Failed to deserialize into Object: {}", e))?;
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
                        let vars_series = Series::new(&*key.as_str().replace(&frequency.to_string(), ""), &vars_vec);
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
                        let vars_series = Series::new(&*key.as_str().replace(&frequency.to_string(), ""), &vars_vec);
                        columns.push(vars_series);
                    }

                }
            }
        }

        if temp_items.len() > 0 {
            for (key, value) in temp_items {
                let items: Vec<Object> = serde_json::from_value(value.to_string().parse()?)
                    .map_err(|e| format!("Failed to deserialize into Object: {}", e))?;
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
                let vars_series = Series::new(&*key.as_str().replace(&frequency.to_string(), ""), &vars_vec);
                columns.push(vars_series);
            }
        }
        let df = DataFrame::new(columns)?;
        Ok(df)
    }

    async fn get_news(&self) -> Result<DataFrame, Box<dyn Error>> {
        let quote = self.get_quote().await?;
        let symbol = if quote.asset_class == "CRYPTOCURRENCY" {
            self.ticker.replace("-USD", "")
        } else {
            self.ticker.clone()
        };
        let token = format!("({} OR {})", &symbol, &quote.name);

        let start_date = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d")?;

        let mut current_date = start_date;
        let mut futures = Vec::new();

        // Define the maximum number of concurrent tasks
        let max_concurrent_tasks = 20;
        let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

        // Create and configure the progress bar
        let total_steps = (end_date - start_date).num_days()/3;
        let pb = ProgressBar::new(total_steps as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
                .progress_chars("#>-"),
        );

        while current_date < end_date {
            let next_date = (current_date + Duration::days(3)).min(end_date);
            let token_clone = token.clone();
            let current_date_clone = current_date.clone();
            let next_date_clone = next_date.clone();
            let semaphore = semaphore.clone();

            let fut = tokio::task::spawn(async move {
                let permit = semaphore.acquire().await.unwrap();
                let result = fetch_news(&token_clone, current_date_clone, next_date_clone, true).await;
                drop(permit);
                match result {
                    Ok(df) => Ok(df),
                    Err(e) => Err(format!("Error fetching news for {} to {}: {}", current_date_clone, next_date_clone, e)),
                }
            });

            futures.push(fut);
            current_date = next_date;
        }

        let results = join_all(futures).await;
        let mut combined_df = DataFrame::default();

        for res in results {
            match res {
                Ok(Ok(df)) => {
                    match combined_df.vstack(&df) {
                        Ok(jdf) => combined_df = jdf,
                        Err(e) => eprintln!("Unable to Vstack {:?}: {}", &df, e),
                    }
                }
                Ok(Err(_)) => continue,
                Err(e) => eprintln!("Error in task: {}", e),
            }
        }

        combined_df.sort(&["Published Date"], SortMultipleOptions::new().with_order_descending(false))?;

        pb.finish_with_message(format!("News Data Fetched for {}", &symbol));

        Ok(combined_df)
    }

}