use chrono::DateTime;
use polars::prelude::*;
use serde_json::{json, Value};
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Cursor;
use webbrowser;

/// Public trait to convert a DataFrame into a DataTable wrapper.
pub trait DataTableDisplay {
    fn to_datatable(&self, id: &str, ordering: bool, format: DataTableFormat) -> DataTable;
}

impl DataTableDisplay for DataFrame {
    fn to_datatable(&self, id: &str, ordering: bool, format: DataTableFormat) -> DataTable {
        DataTable::new(self.clone(), id.to_string(), ordering, format)
    }
}

/// Formatting enum used by DataTable to supply DataTables.net columnDefs JSON
/// (or custom rendering JS) when producing HTML.
#[derive(Clone)]
pub enum DataTableFormat {
    Currency,
    Number,
    Performance(String), // "tickers" or "portfolio"
    Custom(String),
}

impl fmt::Display for DataTableFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataTableFormat::Currency => write!(f, "{CURRENCY_FMT}"),
            DataTableFormat::Number => write!(f, "{NUMBER_FMT}"),
            DataTableFormat::Performance(s) => {
                let fmt = match s.as_str() {
                    "tickers" => TICKERS_PERFORMANCE_TABLE_FMT,
                    "portfolio" => PORTFOLIO_PERFORMANCE_TABLE_FMT,
                    _ => return Err(fmt::Error),
                };
                write!(f, "{fmt}")
            }
            DataTableFormat::Custom(s) => write!(f, "{s}"),
        }
    }
}

// -- JS format payloads used as columnDefs in DataTables initialization --
static CURRENCY_FMT: &str = r#"
[
    {
        "targets": 0,
        "render": function(data) { return data; }
    },
    {
        "targets": "_all",
        "render": function(data) {
            if (data == null || data === '') return '';
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
        "targets": 0,
        "render": function(data) { return data; }
    },
    {
        "targets": "_all",
        "render": function(data) {
            if (data == null || data === '') return '';
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

static PORTFOLIO_PERFORMANCE_TABLE_FMT: &str = r#"
[
    {
        "targets": 0,
        "render": function(data) { return data; }
    },
    {
        "targets": [1,2,5,6,7,8,9,14,15,18,19,20],
        "render": function(data) {
            if (data == null || data === '') return '';
            try {
                let parsed = parseFloat(data);
                if (isNaN(parsed)) return data;
                if (!isFinite(parsed)) { return parsed > 0 ? '∞%' : '-∞%'; }
                return $.fn.dataTable.render.number(',', '.', 2).display(parsed) + '%';
            } catch (e) { return data; }
        }
    },
    {
        "targets": [3,4],
        "render": function(data) {
            if (data == null || data === '') return '';
            try {
                var parsed = parseFloat(data);
                if (isNaN(parsed)) return data;
                if (!isFinite(parsed)) return parsed > 0 ? '∞' : '-∞';
                return '$' + $.fn.dataTable.render.number(',', '.', 2).display(parsed);
            } catch (e) { return data; }
        }
    },
    {
        "targets": [10,11,12,13,16,17],
        "render": function(data) {
            if (data == null || data === '') return '';
            try {
                var parsed = parseFloat(data);
                if (isNaN(parsed)) return data;
                if (!isFinite(parsed)) return parsed > 0 ? '∞' : '-∞';
                return $.fn.dataTable.render.number(',', '.', 2).display(parsed);
            } catch (e) { return data; }
        }
    }
]
"#;

static TICKERS_PERFORMANCE_TABLE_FMT: &str = r#"
[
    {
        "targets": 0,
        "render": function(data) { return data; }
    },
    {
        "targets": [1,2,3,4,5,10,11,14,15,16],
        "render": function(data) {
            if (data == null || data === '') return '';
            try {
                var parsed = parseFloat(data);
                if (isNaN(parsed)) return data;
                if (!isFinite(parsed)) return parsed > 0 ? '∞%' : '-∞%';
                return $.fn.dataTable.render.number(',', '.', 2).display(parsed) + '%';
            } catch (e) { return data; }
        }
    },
    {
        "targets": [6,7,8,9,12,13],
        "render": function(data) {
            if (data == null || data === '') return '';
            try {
                var parsed = parseFloat(data);
                if (isNaN(parsed)) return data;
                if (!isFinite(parsed)) return parsed > 0 ? '∞' : '-∞';
                return $.fn.dataTable.render.number(',', '.', 2).display(parsed);
            } catch (e) { return data; }
        }
    }
]
"#;

/// Lightweight DataTable wrapper which stores the DataFrame and rendering metadata.
pub struct DataTable {
    pub data: DataFrame,
    id: String,
    ordering: bool,
    format: DataTableFormat,
    composite_html: Option<String>,
}

impl DataTable {
    pub fn new(data: DataFrame, id: String, ordering: bool, format: DataTableFormat) -> Self {
        DataTable {
            data,
            id,
            ordering,
            format,
            composite_html: None,
        }
    }

    /// Construct a composite DataTable that holds a primary DataFrame for programmatic
    /// access (`.data`) and a pre-built toggle HTML string that `to_html()` / `show()`
    /// will render instead of generating a single-table page.
    pub fn new_composite(data: DataFrame, id: String, html: String) -> Self {
        DataTable {
            data,
            id,
            ordering: true,
            format: DataTableFormat::Number,
            composite_html: Some(html),
        }
    }

    /// Produce an HTML page which renders the DataTable using DataTables.net.
    /// The generated HTML is self-contained (loads CDN JS/CSS).
    pub fn to_html(&self) -> Result<String, Box<dyn Error>> {
        // Composite DataTable: wrap the pre-built toggle HTML in a full standalone page.
        if let Some(composite) = &self.composite_html {
            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="https://cdn.datatables.net/1.11.5/css/jquery.dataTables.min.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/buttons/2.2.3/css/buttons.dataTables.min.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/fixedcolumns/4.3.0/css/fixedColumns.dataTables.min.css">
    <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
    <script src="https://cdn.datatables.net/1.11.5/js/jquery.dataTables.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/dataTables.buttons.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/buttons.html5.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.2.3/js/buttons.colvis.min.js"></script>
    <script src="https://cdn.datatables.net/fixedcolumns/4.3.0/js/dataTables.fixedColumns.min.js"></script>
</head>
<body>
{composite}
</body>
</html>"#
            );
            return Ok(html);
        }
        // Serialize DataFrame to JSON rows
        let mut df = self.data.clone();

        let mut buffer = Cursor::new(Vec::new());
        JsonWriter::new(&mut buffer)
            .with_json_format(JsonFormat::Json)
            .finish(&mut df)?;

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
            let cols = column_names
                .iter()
                .zip(values_per_column.iter())
                .map(|(name, vals)| {
                    json!({
                        "name": name,
                        "values": vals,
                    })
                })
                .collect::<Vec<_>>();
            Value::Object(serde_json::Map::from_iter([(
                "columns".to_string(),
                Value::Array(cols),
            )]))
        };

        let columns_meta = match parsed_json.get("columns") {
            Some(Value::Array(cols)) => cols,
            _ => return Err("Failed to produce columns".into()),
        };

        let values: Vec<Vec<Value>> = columns_meta
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

        // Build dataset for DataTables: array of rows where each cell is stringified
        let data_set: Vec<Vec<String>> = (0..num_rows)
            .map(|row_idx| {
                column_names
                    .iter()
                    .enumerate()
                    .map(|(col_idx, _col_name)| {
                        let value = &values[col_idx][row_idx];
                        // Check if column has a Datatype indicating datetime
                        let is_datetime = columns_meta[col_idx]
                            .get("datatype")
                            .and_then(|dt| dt.get("Datetime"))
                            .is_some();

                        match value {
                            Value::Number(n) if is_datetime => {
                                // Interpret as milliseconds timestamp
                                let ts = n.as_i64().unwrap_or_default();
                                #[allow(deprecated)]
                                let dt = DateTime::from_timestamp(
                                    ts / 1000,
                                    (ts % 1000) as u32 * 1_000_000,
                                )
                                .unwrap();
                                dt.format("%Y-%m-%d %H:%M:%S").to_string()
                            }
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
        let columns_def: Vec<String> = column_names
            .iter()
            .map(|name| format!(r#"{{ title: "{}" }}"#, name))
            .collect();

        let column_defs = format!("{}", self.format);

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <!-- DataTables CSS/JS (CDN) -->
    <link rel="stylesheet" href="https://cdn.datatables.net/1.11.5/css/jquery.dataTables.min.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/buttons/2.2.3/css/buttons.dataTables.min.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/fixedcolumns/4.3.0/css/fixedColumns.dataTables.min.css">

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
                data: {data},
                columns: [{cols}],
                columnDefs: {coldefs},
                scrollX: true,
                scrollY: '600px',
                scrollCollapse: true,
                paging: false,
                ordering: {ordering},
                dom: 'Bfrtip',
                autoWidth: true,
                fixedColumns: {{ left: 1 }},
                buttons: ['copyHtml5','csvHtml5','colvis']
            }});
        }});
    </script>
</body>
</html>
"#,
            id = self.id,
            data = ordered_json_data,
            cols = columns_def.join(", "),
            coldefs = column_defs,
            ordering = if self.ordering { "true" } else { "false" },
        );

        Ok(html)
    }

    /// Convenience helper: open this DataTable HTML in the system browser.
    pub fn show(&self) -> Result<(), Box<dyn Error>> {
        let html_content = self.to_html()?;
        let filename = format!("{}_table.html", self.id);
        let path = std::env::temp_dir().join(filename);
        fs::write(&path, html_content)?;
        let _ = webbrowser::open(path.to_str().ok_or("Invalid temp path")?);
        Ok(())
    }
}

// -- HTML toggle builders exported for reports ---

/// Builds a combined Returns Data HTML block with a 2D toggle:
///   - mode buttons (pct | val)
///   - frequency buttons (multiple)
/// `entries` is (freq_label, pct_html, val_html). First entry is default.
pub fn build_combined_returns_table(entries: &[(String, String, String)]) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let uid = "returns_toggle";

    let mut freq_buttons = String::new();
    for (i, (label, _, _)) in entries.iter().enumerate() {
        let freq_key = label.to_lowercase();
        let active = if i == 0 { " active" } else { "" };
        freq_buttons.push_str(&format!(
            r#"<button class="toggle-btn freq-btn{active}" data-freq="{freq_key}" onclick="window.__returnsToggle2(this, '{uid}', null, '{freq_key}')">{label}</button>"#,
            active = active,
            freq_key = freq_key,
            uid = uid,
            label = label
        ));
    }

    let mut panes = String::new();
    for (i, (label, pct_html, val_html)) in entries.iter().enumerate() {
        let freq_key = label.to_lowercase();
        let pct_active = if i == 0 { " active" } else { "" };
        panes.push_str(&format!(
            r#"<div class="toggle-pane{pct_active}" data-mode="pct" data-freq="{freq_key}">{pct_html}</div>"#,
            pct_active = pct_active,
            freq_key = freq_key,
            pct_html = pct_html
        ));
        panes.push_str(&format!(
            r#"<div class="toggle-pane" data-mode="val" data-freq="{freq_key}">{val_html}</div>"#,
            freq_key = freq_key,
            val_html = val_html
        ));
    }

    format!(
        r##"<div class="returns-toggle-container" id="{uid}">
  <style>
    .returns-toggle-container .toggle-row {{ display:flex; gap:4px; margin-bottom:6px; }}
    .returns-toggle-container .toggle-btn {{ padding:6px 18px; border:1px solid #ccc; background:#f1f1f1; cursor:pointer; font-weight:bold; border-radius:4px; }}
    .returns-toggle-container .toggle-btn.active {{ background:#fff; border-bottom:2px solid #006400; color:#006400; }}
    .returns-toggle-container .toggle-pane {{ display:none; }}
    .returns-toggle-container .toggle-pane.active {{ display:block; }}
  </style>
  <div class="toggle-row">
    <button class="toggle-btn mode-btn active" data-mode="pct" onclick="window.__returnsToggle2(this, '{uid}', 'pct', null)">%</button>
    <button class="toggle-btn mode-btn" data-mode="val" onclick="window.__returnsToggle2(this, '{uid}', 'val', null)">$</button>
  </div>
  <div class="toggle-row">
    {freq_buttons}
  </div>
  {panes}
  <script>
    (function() {{
      if (!window.__rtState) window.__rtState = {{}};
      window.__rtState['{uid}'] = {{ mode: 'pct', freq: '{default_freq}' }};
      window.__returnsToggle2 = function(btn, containerId, newMode, newFreq) {{
        var state = window.__rtState[containerId];
        if (!state) return;
        var container = document.getElementById(containerId);
        if (!container) return;
        if (newMode) {{
          state.mode = newMode;
          var modeBtns = container.querySelectorAll('.mode-btn'); modeBtns.forEach(function(b) {{ b.classList.remove('active'); }});
          modeBtns.forEach(function(b) {{ if (b.getAttribute('data-mode') === newMode) b.classList.add('active'); }});
        }}
        if (newFreq) {{
          state.freq = newFreq;
          var freqBtns = container.querySelectorAll('.freq-btn'); freqBtns.forEach(function(b) {{ b.classList.remove('active'); }});
          freqBtns.forEach(function(b) {{ if (b.getAttribute('data-freq') === newFreq) b.classList.add('active'); }});
        }}
        var panes = container.querySelectorAll('.toggle-pane');
        panes.forEach(function(p) {{
          if (p.getAttribute('data-mode') === state.mode && p.getAttribute('data-freq') === state.freq) {{
            p.classList.add('active');
          }} else {{
            p.classList.remove('active');
          }}
        }});
        window.dispatchEvent(new Event('resize'));
      }};
    }})();
  </script>
</div>"##,
        uid = uid,
        freq_buttons = freq_buttons,
        panes = panes,
        default_freq = entries[0].0.to_lowercase()
    )
}

/// Build a simple frequency-only toggle (label, html) => interactive block.
pub fn build_frequency_toggle(entries: &[(String, String)], uid: &str) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut freq_buttons = String::new();
    for (i, (label, _)) in entries.iter().enumerate() {
        let key = label.to_lowercase();
        let active = if i == 0 { " active" } else { "" };
        freq_buttons.push_str(&format!(
            r#"<button class="toggle-btn freq-btn{active}" data-freq="{key}" onclick="window.__freqToggle(this, '{uid}', '{key}')">{label}</button>"#,
            active = active,
            key = key,
            uid = uid,
            label = label
        ));
    }

    let mut panes = String::new();
    for (i, (_label, html)) in entries.iter().enumerate() {
        let key = entries[i].0.to_lowercase();
        let active = if i == 0 { " active" } else { "" };
        panes.push_str(&format!(
            r#"<div class="toggle-pane{active}" data-freq="{key}">{html}</div>"#,
            active = active,
            key = key,
            html = html
        ));
    }

    format!(
        r##"<div class="freq-toggle-container" id="{uid}">
  <style>
    .freq-toggle-container .toggle-row {{ display:flex; gap:4px; margin-bottom:6px; }}
    .freq-toggle-container .toggle-btn {{ padding:6px 18px; border:1px solid #ccc; background:#f1f1f1; cursor:pointer; font-weight:bold; border-radius:4px; }}
    .freq-toggle-container .toggle-btn.active {{ background:#fff; border-bottom:2px solid #006400; color:#006400; }}
    .freq-toggle-container .toggle-pane {{ display:none; }}
    .freq-toggle-container .toggle-pane.active {{ display:block; }}
  </style>
  <div class="toggle-row">
    {freq_buttons}
  </div>
  {panes}
  <script>
    (function() {{
      window.__freqToggle = function(btn, containerId, newFreq) {{
        var container = document.getElementById(containerId);
        if (!container) return;
        var btns = container.querySelectorAll('.freq-btn'); btns.forEach(function(b) {{ b.classList.remove('active'); }});
        btn.classList.add('active');
        var panes = container.querySelectorAll('.toggle-pane');
        panes.forEach(function(p) {{
          if (p.getAttribute('data-freq') === newFreq) p.classList.add('active'); else p.classList.remove('active');
        }});
        window.dispatchEvent(new Event('resize'));
      }};
    }})();
  </script>
</div>"##,
        uid = uid,
        freq_buttons = freq_buttons,
        panes = panes
    )
}

/// Build a period toggle (label, html) => interactive block.
pub fn build_period_toggle(entries: &[(String, String)], uid: &str) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut period_buttons = String::new();
    for (i, (label, _)) in entries.iter().enumerate() {
        let key = label.to_lowercase();
        let active = if i == 0 { " active" } else { "" };
        period_buttons.push_str(&format!(
            r#"<button class="toggle-btn period-btn{active}" data-period="{key}" onclick="window.__periodToggle(this, '{uid}', '{key}')">{label}</button>"#,
            active = active,
            key = key,
            uid = uid,
            label = label
        ));
    }

    let mut panes = String::new();
    for (i, (_label, html)) in entries.iter().enumerate() {
        let key = entries[i].0.to_lowercase();
        let active = if i == 0 { " active" } else { "" };
        panes.push_str(&format!(
            r#"<div class="toggle-pane{active}" data-period="{key}">{html}</div>"#,
            active = active,
            key = key,
            html = html
        ));
    }

    format!(
        r##"<div class="period-toggle-container" id="{uid}">
  <style>
    .period-toggle-container .toggle-row {{ display:flex; gap:4px; margin-bottom:6px; }}
    .period-toggle-container .toggle-btn {{ padding:6px 18px; border:1px solid #ccc; background:#f1f1f1; cursor:pointer; font-weight:bold; border-radius:4px; }}
    .period-toggle-container .toggle-btn.active {{ background:#fff; border-bottom:2px solid #006400; color:#006400; }}
    .period-toggle-container .toggle-pane {{ display:none; }}
    .period-toggle-container .toggle-pane.active {{ display:block; }}
  </style>
  <div class="toggle-row">
    {period_buttons}
  </div>
  {panes}
  <script>
    (function() {{
      window.__periodToggle = function(btn, containerId, newPeriod) {{
        var container = document.getElementById(containerId);
        if (!container) return;
        var btns = container.querySelectorAll('.period-btn'); btns.forEach(function(b) {{ b.classList.remove('active'); }});
        btn.classList.add('active');
        var panes = container.querySelectorAll('.toggle-pane');
        panes.forEach(function(p) {{
          if (p.getAttribute('data-period') === newPeriod) p.classList.add('active'); else p.classList.remove('active');
        }});
        window.dispatchEvent(new Event('resize'));
      }};
    }})();
  </script>
</div>"##,
        uid = uid,
        period_buttons = period_buttons,
        panes = panes
    )
}
