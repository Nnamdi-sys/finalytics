use dioxus::prelude::*;

#[component]
pub fn TableContainer(html: String) -> Element {
    let class = html.clone()
        .split("class=\"")
        .nth(1)
        .and_then(|s| s.split('\"').next())
        .unwrap_or("display nowrap cell-border")
        .to_string();

    let script = html.clone()
        .split("<script>")
        .nth(1)
        .and_then(|s| s.split("</script>").next())
        .unwrap_or("")
        .trim()
        .to_string();
    
    use_effect(move || {
        document::eval(&script);
    });

    rsx! {
        div {
            class: "tab-pane fade show active",
            style: "padding: 5px;",
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
    }
}