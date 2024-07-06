use chrono::{DateTime, NaiveDate, NaiveDateTime, Timelike, Utc};
use std::error::Error;

/// Converts a date string in YYYY-MM-DD format to a Unix Timestamp
pub fn to_timestamp(date_str: &str) -> Result<i64, Box<dyn Error>> {
    let parsed_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let datetime = NaiveDateTime::new(parsed_date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let unix_timestamp = datetime.and_utc().timestamp();
    Ok(unix_timestamp)
}

/// Converts a Unix Timestamp to a date string in YYYY-MM-DD format
pub fn to_date(unix_timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp(unix_timestamp, 0).unwrap();
    let date = datetime.date_naive();
    date.to_string()
}

pub fn to_datetime(date_str: &str) -> Result<NaiveDateTime, Box<dyn Error>> {
    let parsed_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let datetime = NaiveDateTime::new(parsed_date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    Ok(datetime)
}

/// Computes the the time to maturity of an option in months
pub fn time_to_maturity(timestamp: i64) -> f64{
    let current_date = chrono::Local::now().naive_local().date();
    let maturity_date = DateTime::from_timestamp_millis( timestamp * 1000).unwrap().date_naive();
    let ttm = (maturity_date - current_date).num_days() as f64 / 30.44;
    ttm
}

pub fn round_datetime_to_day(datetime: DateTime<Utc>) -> NaiveDateTime {
    datetime.with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().naive_local()
}

pub fn round_datetime_to_hour(datetime: DateTime<Utc>) -> NaiveDateTime {
    datetime.with_minute(0).unwrap().with_second(0).unwrap().naive_local()
}

pub fn round_datetime_to_minute(datetime: DateTime<Utc>) -> NaiveDateTime {
    datetime.with_second(0).unwrap().naive_local()
}