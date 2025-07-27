use polars::prelude::*;
use std::error::Error;
use chrono::{DateTime, NaiveDateTime};
use crate::data::yahoo::web::get_json_response;
use crate::data::yahoo::financials::{income_statement, balance_sheet, cashflow_statement, financial_ratios};
use crate::data::yahoo::config::{Interval, OptionContract, Options, Quote, StatementFrequency, StatementType, TickerSummaryStats};
use crate::utils::date_utils::{round_datetime_to_day, round_datetime_to_hour, round_datetime_to_minute, time_to_maturity, to_date, to_datetime, to_timestamp};



/// Fetches Current Ticker Price from Yahoo Finance
pub async fn get_quote(symbol: &str) -> Result<Quote, Box<dyn Error>> {
    let url = format!("https://query2.finance.yahoo.com/v7/finance/options/{symbol}");
    let result= get_json_response(url).await?;
    let value = &result["optionChain"]["result"][0]["quote"].to_string();
    let quote: Quote = serde_json::from_value(value.parse()?)
        .map_err(|e| format!("Failed to deserialize into Quote: {e}"))?;
    Ok(quote)
}

/// Fetches Ticker Current Summary Stats from Yahoo Finance
pub async fn get_ticker_stats(symbol: &str) -> Result<TickerSummaryStats, Box<dyn Error>> {
    let url = format!("https://query2.finance.yahoo.com/v10/finance/quoteSummary/{symbol}?modules=defaultKeyStatistics,financialData,summaryDetail");
    let result = get_json_response(url).await?;
    let value = &result["quoteSummary"]["result"][0].to_string();
    let stats: TickerSummaryStats = serde_json::from_value(value.parse()?)
        .map_err(|e| format!("Failed to deserialize into TickerSummaryStats: {e}"))?;
    Ok(stats)
}

/// Returns the Ticker OHLCV Data from Yahoo Finance for a given time range
pub async fn get_chart(symbol: &str, start_date: &str, end_date: &str, interval: Interval) -> Result<DataFrame, Box<dyn Error>> {
    let period1 = to_timestamp(start_date)?;
    let period2 = to_timestamp(end_date)?;
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{symbol}?period1={period1}&period2={period2}&interval={interval}"
    );
    let result= get_json_response(url).await?;

    let value = &result["chart"]["result"][0];
    let timestamp = &value["timestamp"]
        .as_array()
        .ok_or(format!("timestamp array not found for {symbol}: {result}"))?
        .iter()
        .map(|ts| {
            let timestamp = ts.as_i64().unwrap();
            match interval {
                Interval::OneDay | Interval::FiveDays | Interval::OneWeek | Interval::OneMonth | Interval::ThreeMonths => {
                    round_datetime_to_day(DateTime::from_timestamp(timestamp, 0).unwrap())
                }
                Interval::SixtyMinutes | Interval::OneHour => {
                    round_datetime_to_hour(DateTime::from_timestamp(timestamp, 0).unwrap())
                },
                Interval::NinetyMinutes | Interval::ThirtyMinutes | Interval::FifteenMinutes | Interval::FiveMinutes | Interval::TwoMinutes => {
                    round_datetime_to_minute(DateTime::from_timestamp(timestamp, 0).unwrap())
                },
            }
        })
        .collect::<Vec<NaiveDateTime>>();

    let indicators = &value["indicators"]["quote"][0];

    let open = indicators["open"]
        .as_array()
        .ok_or(format!("open array not found for {symbol}: {result}"))?
        .iter()
        .map(|o| o.as_f64().unwrap_or(0.0))
        .collect::<Vec<f64>>();

    let high = indicators["high"]
        .as_array()
        .ok_or(format!("high array not found for {symbol}: {result}"))?
        .iter()
        .map(|h| h.as_f64().unwrap_or(0.0))
        .collect::<Vec<f64>>();

    let low = indicators["low"]
        .as_array()
        .ok_or(format!("low array not found for {symbol}: {result}"))?
        .iter()
        .map(|l| l.as_f64().unwrap_or(0.0))
        .collect::<Vec<f64>>();

    let close = indicators["close"]
        .as_array()
        .ok_or(format!("close array not found for {symbol}: {result}"))?
        .iter()
        .map(|c| c.as_f64().unwrap_or(0.0))
        .collect::<Vec<f64>>();

    let volume = indicators["volume"]
        .as_array()
        .ok_or(format!("volume array not found for {symbol}: {result}"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0))
        .collect::<Vec<f64>>();

    let adjclose = &value["indicators"]["adjclose"][0]["adjclose"]
        .as_array()
        .unwrap_or_else(|| {
            indicators["close"]
                .as_array()
                .ok_or(format!("close array not found for {symbol}: {result}"))
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
    let mask = df.column("adjclose")?.as_series().unwrap().gt(0.0)?;
    let df = df.filter(&mask)?;

    // check id any returned dates greater than end date
    let dt = to_datetime(end_date)?;
    let mask = df["timestamp"]
        .datetime()?
        .as_datetime_iter()
        .map(|x| x.unwrap() < dt)
        .collect();
    let df = df.filter(&mask)?;
    Ok(df)
}

/// Returns Ticker Option Chain Data from Yahoo Finance for all available expirations
pub async fn get_options(symbol: &str) -> Result<Options, Box<dyn Error>> {
    let url = format!("https://query2.finance.yahoo.com/v7/finance/options/{symbol}");
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
            "https://query2.finance.yahoo.com/v7/finance/options/{symbol}?date={t}"
        );
        let result = get_json_response(url).await?;
        let expiration = to_date(*t);
        let ttm = time_to_maturity(*t);

        let calls = &result["optionChain"]["result"][0]["options"][0]["calls"];
        let calls_vec: Vec<OptionContract> = serde_json::from_value(calls.clone())
            .map_err(|e| format!("Failed to deserialize calls into OptionContract: {e}"))?;

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
            .map_err(|e| format!("Failed to deserialize puts into OptionContract: {e}"))?;

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


/// Returns Ticker Financials from Yahoo Finance for a given statement type and frequency
///
/// # Arguments
///
/// * `statement_type` - StatementType
/// * `frequency` - StatementFrequency
///
/// # Returns
///
/// * `DataFrame` - Ticker Financials
pub async fn get_financials(
    symbol: &str,
    statement_type: StatementType,
    frequency: StatementFrequency
) -> Result<DataFrame, Box<dyn Error>> {
    let df = match statement_type {
        StatementType::IncomeStatement => { income_statement(symbol, frequency).await? }
        StatementType::BalanceSheet => { balance_sheet(symbol, frequency).await? }
        StatementType::CashFlowStatement => { cashflow_statement(symbol, frequency).await? }
        StatementType::FinancialRatios => { financial_ratios(symbol, frequency).await? }
    };
    Ok(df)
}

