use dioxus::prelude::*;

#[component]
pub fn TableContainer(html: String, title: String) -> Element {
    let class = html
        .clone()
        .split("class=\"")
        .nth(1)
        .and_then(|s| s.split('\"').next())
        .unwrap_or("display nowrap cell-border")
        .to_string();

    let mut script = html
        .clone()
        .split("<script>")
        .nth(1)
        .and_then(|s| s.split("</script>").next())
        .unwrap_or("")
        .trim()
        .to_string();

    // Modify script to include title and use class-based selector
    if !script.is_empty() {
        script = script.replace(
            &format!("$('table.{}').DataTable({{", class),
            &format!(
                r#"$('table.{}').DataTable({{
                    title: '{}',"#,
                class, title
            ),
        );
    }

    use_effect(move || {
        document::eval(&script);
    });

    rsx! {
        div {
            class: "tab-pane fade show active table-responsive-wrapper",
            // Fallback title above the table
            div {
                class: "table-title",
                "{title}"
            }
            // Required CSS
            link { rel: "stylesheet", href: "https://cdn.datatables.net/1.11.5/css/jquery.dataTables.min.css" }
            link { rel: "stylesheet", href: "https://cdn.datatables.net/2.2.0/css/dataTables.dataTables.css" }
            link { rel: "stylesheet", href: "https://cdn.datatables.net/buttons/2.2.3/css/buttons.dataTables.min.css" }
            link { rel: "stylesheet", href: "https://cdn.datatables.net/fixedcolumns/4.3.0/css/fixedColumns.dataTables.min.css" }

            // Required JS
            script { src: "https://code.jquery.com/jquery-3.6.0.min.js" }
            script { src: "https://cdn.datatables.net/1.11.5/js/jquery.dataTables.min.js" }
            script { src: "https://cdn.datatables.net/buttons/2.2.3/js/dataTables.buttons.min.js" }
            script { src: "https://cdn.datatables.net/buttons/2.2.3/js/buttons.html5.min.js" }
            script { src: "https://cdn.datatables.net/buttons/2.2.3/js/buttons.colVis.min.js" }
            script { src: "https://cdn.datatables.net/fixedcolumns/4.3.0/js/dataTables.fixedColumns.min.js" }

            // Table element
            table {
                id: "dataTable",
                class: "{class}",
                style: "width:100%;"
            }
        }

        style { r#"
            /* ========== Table Wrapper ========== */
            .table-responsive-wrapper {{
                padding: 5px;
                width: 100%;
                overflow-x: auto;
                -webkit-overflow-scrolling: touch;
                box-sizing: border-box;
            }}

            .table-title {{
                font-weight: bold;
                color: #006400;
                font-size: 18px;
                margin-bottom: 10px;
                text-align: center;
            }}

            /* Make DataTables controls stack better on mobile */
            .table-responsive-wrapper .dataTables_wrapper {{
                width: 100%;
                overflow-x: auto;
            }}

            /* ========== Tablet (<=768px) ========== */
            @media (max-width: 768px) {{
                .table-responsive-wrapper {{
                    padding: 3px;
                }}

                .table-title {{
                    font-size: 15px;
                    margin-bottom: 6px;
                }}

                /* Ensure table doesn't overflow — allow horizontal scroll */
                .table-responsive-wrapper table {{
                    font-size: 13px;
                }}

                .table-responsive-wrapper .dataTables_wrapper .dataTables_filter,
                .table-responsive-wrapper .dataTables_wrapper .dataTables_length {{
                    float: none;
                    text-align: center;
                    margin-bottom: 8px;
                }}

                .table-responsive-wrapper .dataTables_wrapper .dataTables_info,
                .table-responsive-wrapper .dataTables_wrapper .dataTables_paginate {{
                    float: none;
                    text-align: center;
                    margin-top: 8px;
                }}
            }}

            /* ========== Small Phones (<=480px) ========== */
            @media (max-width: 480px) {{
                .table-responsive-wrapper {{
                    padding: 2px;
                }}

                .table-title {{
                    font-size: 14px;
                    margin-bottom: 4px;
                }}

                .table-responsive-wrapper table {{
                    font-size: 11px;
                }}

                .table-responsive-wrapper table th,
                .table-responsive-wrapper table td {{
                    padding: 4px 6px;
                    white-space: nowrap;
                }}
            }}
        "# }
    }
}
