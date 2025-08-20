use polars::prelude::*;
use serde_json::Value;
use std::io::Cursor;

pub fn dataframe_to_json(df: &mut DataFrame) -> Result<String, PolarsError> {
    let mut buffer = Cursor::new(Vec::new());
    JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(df)?;

    let json_data = String::from_utf8(buffer.into_inner())
        .map_err(|_| PolarsError::ComputeError("Failed to convert buffer to UTF-8".into()))?;
    Ok(json_data)
}

pub fn series_to_json(series: &Series) -> Result<String, serde_json::Error> {
    let values: Vec<serde_json::Value> = series
        .iter()
        .map(|v| match v {
            AnyValue::Null => serde_json::Value::Null,
            AnyValue::Int64(i) => serde_json::Value::from(i),
            AnyValue::Int32(i) => serde_json::Value::from(i),
            AnyValue::Float64(f) => serde_json::Value::from(f),
            AnyValue::Float32(f) => serde_json::Value::from(f),
            AnyValue::String(s) => serde_json::Value::from(s),
            AnyValue::Boolean(b) => serde_json::Value::from(b),
            other => serde_json::Value::from(other.to_string()),
        })
        .collect();

    serde_json::to_string(&values)
}

pub fn dataframe_from_json(json_str: &str) -> PolarsResult<DataFrame> {
    let parsed: Value = serde_json::from_str(json_str)
        .map_err(|e| PolarsError::ComputeError(format!("JSON parse error: {e}").into()))?;

    let obj = parsed
        .as_object()
        .ok_or_else(|| PolarsError::ComputeError("Expected top-level JSON object".into()))?;

    let mut columns_vec = Vec::new();

    for (key, value) in obj {
        let arr = value.as_array().ok_or_else(|| {
            PolarsError::ComputeError(format!("Column {key} is not an array").into())
        })?;

        // Convert JSON array into Vec<AnyValue>
        let vals: Vec<AnyValue> = arr
            .iter()
            .map(|v| match v {
                Value::Null => AnyValue::Null,
                Value::Bool(b) => AnyValue::Boolean(*b),
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        AnyValue::Int64(i)
                    } else if let Some(f) = n.as_f64() {
                        AnyValue::Float64(f)
                    } else {
                        AnyValue::StringOwned(n.to_string().into())
                    }
                }
                Value::String(s) =>  match s.parse::<f64>() {
                        Ok(f) => AnyValue::Float64(f),
                        Err(_) => AnyValue::String(s)
                    },
                _ => AnyValue::StringOwned(v.to_string().into()),
            })
            .collect();

        // Series::new infers the right dtype
        let c = Column::new(key.as_str().into(), vals);
        columns_vec.push(c);
    }

    DataFrame::new(columns_vec)
}
