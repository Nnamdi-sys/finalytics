use std::fs::File;
use std::io::Write;
use std::error::Error;
use crate::prelude::ReportType;

pub struct TabbedHtml {
    report_type: ReportType,
    tabs: Vec<(String, String)>,
}

impl TabbedHtml {
    /// Creates a new TabbedHtml instance.
    pub fn new(report_type: ReportType, tabs: Vec<(String, String)>) -> Self {
        Self { report_type, tabs }
    }

    /// Generates the HTML as a string.
    pub fn to_html(&self) -> String {
        let mut tabs = String::new();
        let mut contents = String::new();

        for (index, (name, table_html)) in self.tabs.iter().enumerate() {
            let tab_id = format!("tab-{index}");

            // Tabs
            tabs.push_str(&format!(
                r#"<button class="tab-button" onclick="openTab(event, '{tab_id}')">{name}</button>"#
            ));

            // Content
            contents.push_str(&format!(
                r#"<div id="{tab_id}" class="tab-content">{table_html}</div>"#
            ));
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Finalytics Report</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
        }}
        .tab-container {{
            display: flex;
            flex-direction: column;
        }}
        .tab-buttons {{
            display: flex;
            justify-content: flex-start;
            margin-bottom: 10px;
        }}
        .tab-button {{
            padding: 10px 20px;
            background: #f1f1f1;
            border: 1px solid #ccc;
            cursor: pointer;
            margin-right: 5px;
            font-weight: bold;
        }}
        .tab-button:hover {{
            background: #ddd;
        }}
        .tab-button.active {{
            background: #fff;
            border-bottom: 2px solid #000;
        }}
        .tab-content {{
            display: none;
            padding: 10px;
            margin-top: -1px;
        }}
        .tab-content.active {{
            display: block;
        }}
    </style>
</head>
<body>
    <div class="tab-container">
        <div class="tab-buttons">
            {tabs}
        </div>
        {contents}
    </div>
    <script>
        function openTab(event, tabId) {{
            const tabContents = document.querySelectorAll('.tab-content');
            tabContents.forEach(content => content.classList.remove('active'));

            const tabButtons = document.querySelectorAll('.tab-button');
            tabButtons.forEach(button => button.classList.remove('active'));

            document.getElementById(tabId).classList.add('active');
            event.currentTarget.classList.add('active');

            window.dispatchEvent(new Event('resize'));
        }}

        document.addEventListener('DOMContentLoaded', () => {{
            const firstButton = document.querySelector('.tab-button');
            if (firstButton) {{
                firstButton.click();
            }}
        }});
    </script>
</body>
</html>"#
        )
    }

    /// Opens the HTML in the default web browser.
    pub fn show(&self) -> Result<(), Box<dyn Error>> {
        let html_content = self.to_html();
        let filename = format!("{}_report.html", self.report_type);
        let temp_file_path = std::env::temp_dir().join(filename);
        let mut file = File::create(&temp_file_path)?;
        file.write_all(html_content.as_bytes())?;
        let _ = webbrowser::open(temp_file_path.to_str().unwrap()).map_err(|e| {
            println!("Error opening report with webbrowser: {e}");
            println!("Report Saved at: {temp_file_path:?}");
        });
        Ok(())
    }
}