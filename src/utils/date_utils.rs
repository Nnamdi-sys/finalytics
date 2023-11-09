use chrono::{NaiveDate, NaiveDateTime, Timelike};
use std::error::Error;
use std::str::FromStr;

/// Converts a date string in YYYY-MM-DD format to a Unix Timestamp
pub fn to_timestamp(date_str: &str) -> Result<i64, Box<dyn Error>> {
    let parsed_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let datetime = NaiveDateTime::new(parsed_date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let unix_timestamp = datetime.timestamp() as i64;
    Ok(unix_timestamp)
}

/// Converts a Unix Timestamp to a date string in YYYY-MM-DD format
pub fn to_date(unix_timestamp: i64) -> String {
    let datetime = NaiveDateTime::from_timestamp_opt(unix_timestamp, 0).unwrap();
    let date = datetime.date();
    date.to_string()
}

pub fn to_datetime(date_str: &str) -> Result<NaiveDateTime, Box<dyn Error>> {
    let parsed_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let datetime = NaiveDateTime::new(parsed_date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    Ok(datetime)
}

/// Generates a vector of dates between a start date and an end date
pub fn generate_dates(start_date: &str, end_date: &str, interval_days: i64) -> Vec<String> {
    let start_date = NaiveDate::from_str(start_date).expect("Invalid start date");
    let end_date = NaiveDate::from_str(end_date).expect("Invalid end date");

    let mut current_date = start_date;
    let mut date_vector = Vec::new();

    while current_date <= end_date {
        date_vector.push(current_date.to_string());
        current_date += chrono::Duration::days(interval_days);
    }

    date_vector
}

/// Computes the the time to maturity of an option in months
pub fn time_to_maturity(timestamp: i64) -> f64{
    let current_date = chrono::Local::now().naive_local().date();
    let maturity_date = NaiveDateTime::from_timestamp_millis( timestamp * 1000).unwrap().date();
    let ttm = (maturity_date - current_date).num_days() as f64 / 30.44;
    ttm
}

pub fn round_datetime_to_day(datetime: NaiveDateTime) -> NaiveDateTime {
    datetime.with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap()
}

pub fn round_datetime_to_hour(datetime: NaiveDateTime) -> NaiveDateTime {
    datetime.with_minute(0).unwrap().with_second(0).unwrap()
}

pub fn round_datetime_to_minute(datetime: NaiveDateTime) -> NaiveDateTime {
    datetime.with_second(0).unwrap()
}