use std::error::Error;
use std::{fmt, fs};
use std::io::Cursor;
use chrono::DateTime;
use webbrowser;
use polars::prelude::*;
use serde_json::{json, Value};


pub trait DataTableDisplay {
    fn to_datatable(&self, id: &str, ordering: bool, format: DataTableFormat ) -> DataTable;
}

impl DataTableDisplay for DataFrame {
    fn to_datatable(&self, id: &str, ordering: bool, format: DataTableFormat ) -> DataTable {
        DataTable::new(self.clone(), id.to_string(), ordering, format)
    }
}

pub enum DataTableFormat {
    Currency,
    Number,
    Performance,
    Custom(String),
}

impl fmt::Display for DataTableFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataTableFormat::Currency => write!(f, "{CURRENCY_FMT}"),
            DataTableFormat::Number => write!(f, "{NUMBER_FMT}"),
            DataTableFormat::Performance => write!(f, "{PERFORMANCE_TABLE_FMT}"),
            DataTableFormat::Custom(fmt) => write!(f, "{fmt}"),
        }
    }
}

static CURRENCY_FMT: &str = r#"
[
    {
        "targets": 0,
        "render": function(data) { return data; },
    },
    {
        "targets": "_all",
        "render": function(data) {
            if (data == null) return '';

            try {
                let parsed = JSON.parse(data);
                if (typeof parsed === 'number') {
                    return '$' + $.fn.dataTable.render.number(',', '.', 2).display(parsed);
                } else {
                    return parsed;
                }
            } catch (e) {
                return data;
            }
        }
    }
]
"#;

static NUMBER_FMT: &str = r#"
[
    {
        "targets": "_all",
        "render": function(data) {
            if (data == null) return '';

            try {
                let parsed = JSON.parse(data);
                if (typeof parsed === 'number') {
                    return $.fn.dataTable.render.number(',', '.', 2).display(parsed);
                } else {
                    return parsed;
                }
            } catch (e) {
                return data;
            }
        }
    }
]
"#;

static PERFORMANCE_TABLE_FMT: &str = r#"
[
    {
        "targets": 0,
        "render": function(data) { return data; } // Ticker symbol, no formatting
    },
    {
        "targets": [1, 2, 3, 4, 5, 10, 11, 14, 15, 16], // Percentage fields
        "render": function(data) {
            if (data == null || data === '') return '';

            try {
                let parsed = parseFloat(data);
                if (isNaN(parsed)) return data;

                // Handle Infinity and -Infinity
                if (!isFinite(parsed)) {
                    return parsed > 0 ? '∞%' : '-∞%';
                }

                // Handle extremely large values (e.g., > 1e308 or < -1e308)
                if (Math.abs(parsed) > 1e308) {
                    return parsed > 0 ? '>999T%' : '<-999T%';
                }

                // Format as percentage with 2 decimal places
                return $.fn.dataTable.render.number(',', '.', 2).display(parsed) + '%';
            } catch (e) {
                return data;
            }
        }
    },
    {
        "targets": [6, 7, 8, 9, 12, 13], // Decimal fields
        "render": function(data) {
            if (data == null || data === '') return '';

            try {
                let parsed = parseFloat(data);
                if (isNaN(parsed)) return data;

                // Handle Infinity and -Infinity
                if (!isFinite(parsed)) {
                    return parsed > 0 ? '∞' : '-∞';
                }

                // Handle extremely large values
                if (Math.abs(parsed) > 1e308) {
                    return parsed > 0 ? '>999T' : '<-999T';
                }

                // Format as number with 2 decimal places
                return $.fn.dataTable.render.number(',', '.', 2).display(parsed);
            } catch (e) {
                return data;
            }
        }
    }
]
"#;

pub struct DataTable {
    pub data: DataFrame,
    id: String,
    ordering: bool,
    format: DataTableFormat,
}

impl DataTable {
    pub fn new(data: DataFrame, id: String, ordering: bool, format: DataTableFormat) -> Self {
        DataTable { 
            data, 
            id,
            ordering,
            format,
        }
    }

    pub fn to_html(&self) -> Result<String, Box<dyn Error>> {
        let df = &mut self.data.clone();


        let mut buffer = Cursor::new(Vec::new());
        JsonWriter::new(&mut buffer)
            .with_json_format(JsonFormat::Json)
            .finish(df)?;

        let json_data = String::from_utf8(buffer.into_inner())?;

        let parsed_rows: Vec<Value> = serde_json::from_str(&json_data)?;

        let column_names = df.get_column_names_str();
        let mut values_per_column = vec![Vec::new(); column_names.len()];

        for row in &parsed_rows {
            if let Value::Object(map) = row {
                for (i, name) in column_names.iter().enumerate() {
                    values_per_column[i].push(map.get(*name).cloned().unwrap_or(Value::Null));
                }
            }
        }

        let parsed_json = {
            let cols = column_names.iter()
                .zip(values_per_column.iter())
                .map(|(name, vals)| {
                    json!({
                "name": name,
                "values": vals,
            })
                })
                .collect::<Vec<_>>();
            Value::Object(serde_json::Map::from_iter([
                ("columns".to_string(), Value::Array(cols))
            ]))
        };

        let columns = match parsed_json.get("columns") {
            Some(Value::Array(cols)) => cols,
            _ => return Err("Failed to find columns in JSON.".into()),
        };

        let column_names = df.get_column_names_str();

        let values: Vec<Vec<Value>> = columns
            .iter()
            .filter_map(|col| col.get("values"))
            .filter_map(|v| v.as_array())
            .cloned()
            .collect();

        let num_rows = values.first().map_or(0, |v| v.len());
        for column in &values {
            if column.len() != num_rows {
                return Err("Column lengths do not match.".into());
            }
        }

        // Create the 2D array (dataSet) by combining values from each column
        let data_set: Vec<Vec<String>> = (0..num_rows)
            .map(|row_idx| {
                column_names
                    .iter()
                    .map(|col_name| {
                        let col_idx = column_names.iter().position(|name| name == col_name).unwrap();
                        let value = &values[col_idx][row_idx];

                        // Check if the column has a "Datatype" field with "Datetime"
                        let column_datatype = columns[col_idx].get("datatype");
                        let is_datetime = column_datatype
                            .and_then(|dt| dt.get("Datetime"))
                            .is_some();

                        // Handle timestamps as datetime if applicable
                        match value {
                            // Check if the datatype is Datetime and the value is a number (timestamp in milliseconds)
                            Value::Number(n) if is_datetime => {
                                // Convert the timestamp (milliseconds) to a DateTime string
                                let timestamp_ms = n.as_i64().unwrap();
                                #[allow(deprecated)]
                                let datetime = DateTime::from_timestamp(timestamp_ms / 1000, (timestamp_ms % 1000) as u32 * 1_000_000).unwrap();
                                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                            },
                            // Other types of values (String, Number, Bool)
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            _ => "".to_string(),
                        }
                    })
                    .collect()
            })
            .collect();

        let ordered_json_data = serde_json::to_string(&data_set)?;

        let columns: Vec<String> = column_names
            .iter()
            .map(|name| format!(r#"{{ title: "{name}" }}"#))
            .collect();


        // Build the HTML
        let html = format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <! -- DataTables Options CSS -->
    <link rel="stylesheet" href="https://cdn.datatables.net/1.11.5/css/jquery.dataTables.min.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/2.2.0/css/dataTables.dataTables.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/buttons/2.2.3/css/buttons.dataTables.min.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/fixedcolumns/4.3.0/css/fixedColumns.dataTables.min.css">

    <! -- DataTables Options JS -->
    <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
    <script src="https://cdn.datatables.net/1.11.5/js/jquery.dataTables.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/dataTables.buttons.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/buttons.html5.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/buttons.colVis.min.js"></script>
    <script src="https://cdn.datatables.net/fixedcolumns/4.3.0/js/dataTables.fixedColumns.min.js"></script>

</head>
<body>
    <table id="dataTable" class="{id} display nowrap cell-border" style="width:100%"></table>
    <script>
        $(document).ready(function() {{
            $('table.{id}').DataTable({{
                data: {ordered_json_data},
                columns: [{columns}],
                columnDefs: {column_defs},
                scrollX: "100%",
                scrollY: "600px",
                scrollCollapse: true,
                paging: false,
                ordering: {ordering},
                dom: 'Bfrtip',
                autoWidth: true,
                fixedColumns: {{
                    left: 1
                }},
                buttons: [
                    "copyHtml5",
                    "csvHtml5",
                    "colvis"
                ]
            }});
        }});
    </script>
</body>
</html>
"#,         id = self.id,
            ordered_json_data = ordered_json_data,
            ordering = self.ordering,
            columns = columns.join(", "),
            column_defs = self.format
        );

        Ok(html)
    }

    pub fn show(&self) -> Result<(), Box<dyn Error>> {
        let html_content = self.to_html()?;
        let filename = format!("{}_table.html", self.id);
        let temp_file_path = std::env::temp_dir().join(filename);
        fs::write(&temp_file_path, html_content)?;
        let _ = webbrowser::open(temp_file_path.to_str().unwrap()).map_err(|e| {
            println!("Error opening html file with webbrowser: {e:?}");
            println!("Open the file manually at: {temp_file_path:?}");
        });
        Ok(())
    }
}

