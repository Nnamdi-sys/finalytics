use std::collections::HashMap;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Timelike, Utc, Datelike};
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

/// Computes the time to maturity of an option in months
pub fn time_to_maturity(timestamp: i64) -> f64 {
    let current_date = chrono::Local::now().naive_local().date();
    let maturity_date = DateTime::from_timestamp_millis( timestamp * 1000).unwrap().date_naive();
    (maturity_date - current_date).num_days() as f64 / 30.44
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

pub fn convert_to_quarter(dates: Vec<&str>) -> Vec<String> {
    dates.into_iter().map(|date_str| {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let year = if date.month() == 1 { date.year() - 1 } else { date.year() };
        let quarter = match date.month() {
            3 | 4 => 1,
            6 | 7 => 2,
            9 | 10 => 3,
            12 | 1 => 4,
            _ => unreachable!(),
        };
        format!("{year}Q{quarter}")
    }).collect()
}

pub fn convert_to_year(dates: Vec<&str>) -> Vec<String> {
    dates.into_iter().map(|date_str| {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let year = if date.month() <= 4 { date.year() - 1 } else { date.year() };
        year.to_string()
    }).collect()
}

#[derive(Debug, Copy, Clone)]
pub struct IntervalDays {
    pub average: f64,
    pub mode: f64,
}
pub fn interval_days(timestamps: Vec<NaiveDateTime>) -> IntervalDays {
    let mut intervals = Vec::new();
    let mut total_seconds = 0.0;

    // Calculate intervals using NaiveDateTime differences
    for window in timestamps.windows(2) {
        let duration = window[1] - window[0];
        let diff_seconds = duration.num_seconds() as f64;
        let days = diff_seconds / 86400.0;

        intervals.push(days);
        total_seconds += diff_seconds;
    }

    // Calculate average interval in days
    let avg = total_seconds / (intervals.len() as f64 * 86400.0);

    // Calculate modal interval with precision handling
    let mut interval_counts = HashMap::new();
    let mut max_count = 0;
    let mut mode = 0.0;

    for &interval in &intervals {
        // Round to 4 decimal places for mode calculation
        let key = (interval * 10000.0) as i64;
        let count = interval_counts.entry(key).or_insert(0);
        *count += 1;

        // Update mode with tie-breaker for smaller intervals
        if *count > max_count || (*count == max_count && key < (mode * 10000.0) as i64) {
            max_count = *count;
            mode = key as f64 / 10000.0;
        }
    }

    IntervalDays {
        average: avg,
        mode,
    }
}

