use dioxus::prelude::*;
use regex::Regex;

#[component]
pub fn ChartContainer(html: String, id: String) -> Element {
    let html = html.replace("plotly-html-element", id.as_str());
    let regex = Regex::new(r#"<script\s+type="module">\s*(?s)(.*?)</script>"#);
    let script = if let Ok(regex) = regex {
        regex
            .captures(&html)
            .and_then(|caps| {
                caps.get(1).map(|m| {
                    // Remove fillFrame (sizes to window) but keep responsive (sizes to container)
                    m.as_str().trim().replace(r#""fillFrame":true,"#, "")
                })
            })
            .unwrap_or_default()
    } else {
        String::new()
    };

    let id_clone = id.clone();

    use_effect(move || {
        let resize_script = format!(
            r#"
            {script}

            // After Plotly renders, optimize legend and sizing for the viewport
            (function() {{
                var chartId = "{id_clone}";
                var el = document.getElementById(chartId);
                if (!el) return;

                // Helper: safely resize the Plotly chart to fit its container
                function resizePlotly() {{
                    if (typeof Plotly !== 'undefined' && el && el.data) {{
                        Plotly.Plots.resize(el);
                    }}
                }}

                // Helper: apply mobile-optimized legend layout
                function optimizeLegend() {{
                    if (typeof Plotly === 'undefined' || !el || !el.data) return;

                    var isMobile = window.innerWidth <= 768;
                    var isSmallPhone = window.innerWidth <= 480;

                    if (isSmallPhone) {{
                        Plotly.relayout(el, {{
                            'legend.orientation': 'h',
                            'legend.y': -0.25,
                            'legend.x': 0.5,
                            'legend.xanchor': 'center',
                            'legend.yanchor': 'top',
                            'legend.font.size': 8,
                            'legend.itemwidth': 30,
                            'margin.l': 35,
                            'margin.r': 10,
                            'margin.t': 30,
                            'margin.b': 70,
                            'xaxis.title.text': '',
                            'yaxis.title.text': '',
                            'xaxis2.title.text': '',
                            'yaxis2.title.text': '',
                            'xaxis3.title.text': '',
                            'yaxis3.title.text': '',
                            'xaxis4.title.text': '',
                            'yaxis4.title.text': ''
                        }});
                    }} else if (isMobile) {{
                        Plotly.relayout(el, {{
                            'legend.orientation': 'h',
                            'legend.y': -0.2,
                            'legend.x': 0.5,
                            'legend.xanchor': 'center',
                            'legend.yanchor': 'top',
                            'legend.font.size': 9,
                            'legend.itemwidth': 35,
                            'margin.l': 40,
                            'margin.r': 15,
                            'margin.t': 35,
                            'margin.b': 65,
                            'xaxis.title.text': '',
                            'yaxis.title.text': '',
                            'xaxis2.title.text': '',
                            'yaxis2.title.text': '',
                            'xaxis3.title.text': '',
                            'yaxis3.title.text': '',
                            'xaxis4.title.text': '',
                            'yaxis4.title.text': ''
                        }});
                    }} else {{
                        // Desktop: just tighten margins slightly, keep default vertical legend
                        Plotly.relayout(el, {{
                            'legend.font.size': 11,
                            'margin.l': 50,
                            'margin.r': 20,
                            'margin.t': 40,
                            'margin.b': 50
                        }});
                    }}
                }}

                // Use ResizeObserver for container-aware resizing
                if (typeof ResizeObserver !== 'undefined') {{
                    var debounceTimer;
                    var ro = new ResizeObserver(function() {{
                        clearTimeout(debounceTimer);
                        debounceTimer = setTimeout(function() {{
                            resizePlotly();
                            optimizeLegend();
                        }}, 200);
                    }});
                    ro.observe(el);
                    if (el.parentElement) {{
                        ro.observe(el.parentElement);
                    }}
                }}

                // Also listen for window resize as a fallback
                var resizeTimeout;
                window.addEventListener('resize', function() {{
                    clearTimeout(resizeTimeout);
                    resizeTimeout = setTimeout(function() {{
                        resizePlotly();
                        optimizeLegend();
                    }}, 250);
                }});

                // Initial optimization after Plotly finishes rendering
                setTimeout(function() {{
                    resizePlotly();
                    optimizeLegend();
                }}, 400);
                setTimeout(function() {{
                    resizePlotly();
                    optimizeLegend();
                }}, 1000);
            }})();
            "#,
        );
        document::eval(&resize_script);
    });

    // Determine container class based on id
    let container_class = if id == "chart-ticker" || id == "chart-portfolio" {
        "chart-container chart-container--home"
    } else {
        "chart-container chart-container--dashboard"
    };

    rsx! {
        div {
            class: "tab-pane fade show active {container_class}",
            script {
                src: "https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"
            }
            script {
                src: "https://cdn.plot.ly/plotly-2.12.1.min.js"
            }
            div {
                id: "{id}",
                class: "plotly-graph-div",
                style: r#"
                    position: relative;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                "#
            }
        }

        style { r#"
            .chart-container {{
                padding: 5px;
                width: 100%;
                box-sizing: border-box;
                position: relative;
            }}

            .plotly-graph-div {{
                min-height: 0;
            }}

            /* Home page charts (chart-ticker / chart-portfolio) */
            .chart-container--home {{
                height: 70vh;
                overflow: hidden;
            }}

            /* Dashboard charts — fill their parent (.display-chart-container) */
            .chart-container--dashboard {{
                height: 100%;
                overflow: hidden;
            }}

            /* =========== Tablet (<=768px) =========== */
            @media (max-width: 768px) {{
                .chart-container {{
                    padding: 2px;
                }}

                .chart-container--home {{
                    height: 55vh;
                }}

                .plotly-graph-div .main-svg {{
                    max-width: 100% !important;
                }}

                /* Hide Plotly toolbar on mobile */
                .modebar-container {{
                    display: none !important;
                }}
            }}

            /* =========== Small Phones (<=480px) =========== */
            @media (max-width: 480px) {{
                .chart-container {{
                    padding: 1px;
                }}

                .chart-container--home {{
                    height: 45vh;
                }}
            }}
        "# }
    }
}
