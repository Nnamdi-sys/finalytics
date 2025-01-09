use std::error::Error;
use std::fs;
use chrono::DateTime;
use webbrowser;
use polars::prelude::*;
use serde_json::Value;

pub enum TableType {
    OHLCV,
    OptionsChain,
    VolatilitySurface,
    Returns,
    NewsSentiment,
    AnnualIncomeStatement,
    QuarterlyIncomeStatement,
    AnnualBalanceSheet,
    QuarterlyBalanceSheet,
    AnnualCashflowStatement,
    QuarterlyCashflowStatement,
    AnnualFinancialRatios,
    QuarterlyFinancialRatios,
    PerformanceStats,
    SummaryStats
}

impl TableType {
    pub fn id(&self) -> &str {
        match self {
            TableType::OHLCV => "ohlcvTable",
            TableType::OptionsChain => "optionsChain",
            TableType::VolatilitySurface => "volatilitySurface",
            TableType::Returns => "returnsTable",
            TableType::NewsSentiment => "newsSentiment",
            TableType::AnnualIncomeStatement => "annualIncomeStatement",
            TableType::QuarterlyIncomeStatement => "quarterlyIncomeStatement",
            TableType::AnnualBalanceSheet => "annualBalanceSheet",
            TableType::QuarterlyBalanceSheet => "quarterlyBalanceSheet",
            TableType::AnnualCashflowStatement => "annualCashflowStatement",
            TableType::QuarterlyCashflowStatement => "quarterlyCashflowStatement",
            TableType::AnnualFinancialRatios => "annualFinancialRatios",
            TableType::QuarterlyFinancialRatios => "quarterlyFinancialRatios",
            TableType::PerformanceStats => "performanceStats",
            TableType::SummaryStats => "summaryStats"
        }
    }

    pub fn ordering(&self) -> bool {
        match self {
            TableType::OHLCV => true,
            TableType::OptionsChain => true,
            TableType::VolatilitySurface => true,
            TableType::Returns => true,
            TableType::NewsSentiment => true,
            TableType::AnnualIncomeStatement => false,
            TableType::QuarterlyIncomeStatement => false,
            TableType::AnnualBalanceSheet => false,
            TableType::QuarterlyBalanceSheet => false,
            TableType::AnnualCashflowStatement => false,
            TableType::QuarterlyCashflowStatement => false,
            TableType::AnnualFinancialRatios => false,
            TableType::QuarterlyFinancialRatios => false,
            TableType::PerformanceStats => false,
            TableType::SummaryStats => false,
        }
    }

    pub fn column_defs(&self) -> String {
        match self {
            TableType::OHLCV => NUMBER_FMT.to_string(),
            TableType::OptionsChain => OPTIONS_FMT.to_string(),
            TableType::VolatilitySurface => NUMBER_FMT.to_string(),
            TableType::Returns => NUMBER_FMT.to_string(),
            TableType::NewsSentiment => NEWS_FMT.to_string(),
            TableType::AnnualIncomeStatement => CURRENCY_FMT.to_string(),
            TableType::QuarterlyIncomeStatement => CURRENCY_FMT.to_string(),
            TableType::AnnualBalanceSheet => CURRENCY_FMT.to_string(),
            TableType::QuarterlyBalanceSheet => CURRENCY_FMT.to_string(),
            TableType::AnnualCashflowStatement => CURRENCY_FMT.to_string(),
            TableType::QuarterlyCashflowStatement => CURRENCY_FMT.to_string(),
            TableType::AnnualFinancialRatios => NUMBER_FMT.to_string(),
            TableType::QuarterlyFinancialRatios => NUMBER_FMT.to_string(),
            TableType::PerformanceStats => NO_FMT.to_string(),
            TableType::SummaryStats => NO_FMT.to_string(),
        }
    }
}

pub struct DataTable {
    data: DataFrame,
    table_type: TableType,
}

impl DataTable {
    pub fn new(data: DataFrame, table_type: TableType) -> Self {
        DataTable { data, table_type }
    }

    pub fn to_html(&self) -> Result<String, Box<dyn Error>> {
        let df = &mut self.data.clone();

        let json_data = serde_json::to_string(df)?;

        let parsed_json: Value = serde_json::from_str(&json_data)?;

        let columns = match parsed_json.get("columns") {
            Some(Value::Array(cols)) => cols,
            _ => return Err("Failed to find columns in JSON.".into()),
        };

        let column_names = df.get_column_names();

        let values: Vec<Vec<Value>> = columns
            .iter()
            .filter_map(|col| col.get("values"))
            .filter_map(|v| v.as_array())
            .map(|arr| arr.clone())
            .collect();

        let num_rows = values.get(0).map_or(0, |v| v.len());
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
            .map(|name| format!(r#"{{ title: "{}" }}"#, name))
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

    <! -- DataTables Options JS -->
    <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
    <script src="https://cdn.datatables.net/1.11.5/js/jquery.dataTables.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/dataTables.buttons.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/buttons.html5.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/buttons.colVis.min.js"></script>

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
"#,         id = self.table_type.id(),
            ordered_json_data = ordered_json_data,
            ordering = self.table_type.ordering(),
            columns = columns.join(", "),
            column_defs = self.table_type.column_defs()
        );

        Ok(html)
    }

    pub fn show(&self) -> Result<(), Box<dyn Error>> {
        let html_content = self.to_html()?;
        let filename = format!("{}_table.html", self.table_type.id());
        let temp_file_path = std::env::temp_dir().join(filename);
        fs::write(&temp_file_path, html_content)?;
        webbrowser::open(temp_file_path.to_str().unwrap())?;
        Ok(())
    }
}

static NO_FMT: &str = r#"
[
    {
        "targets": "_all",
        "render": function(data) { return data; },
    }
]
"#;

static OPTIONS_FMT: &str = r#"
[
    {
        "targets": [0, 13],
        "render": function(data) { return data; },
    },
    {
        "targets": "_all",
        "render": function(data) { return data != null ? $.fn.dataTable.render.number(',', '.', 2).display(data) : ''; },
    }
]
"#;

static NEWS_FMT: &str = r#"
[
    {
        "targets": [0, 1, 2],
        "render": function(data) { return data; },
    },
    {
        "targets": 3,
        "render": function(data) { return data != null ? $.fn.dataTable.render.number(',', '.', 2).display(data) : ''; },
    }
]
"#;


static NUMBER_FMT: &str = r#"
[
    {
        "targets": 0,
        "render": function(data) { return data; },
    },
    {
        "targets": "_all",
        "render": function(data) { return data != null ? $.fn.dataTable.render.number(',', '.', 2).display(data) : ''; },
    }
]
"#;

static CURRENCY_FMT: &str = r#"
[
    {
        "targets": 0,
        "render": function(data) { return data; },
    },
    {
        "targets": "_all",
        "render": function(data) { return data != null ? '$' + $.fn.dataTable.render.number(',', '.', 2).display(data) : ''; },
    },
]
"#;

