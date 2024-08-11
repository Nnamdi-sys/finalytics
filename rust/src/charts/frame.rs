use std::error::Error;
use plotly::{Bar, Layout, Plot, Scatter};
use plotly::common::{Mode, Title};
use plotly::layout::Axis;
use polars::frame::DataFrame;

pub trait PolarsPlot {
    fn scatter_plot(&self, x_column: &str, y_column: &str, title: Option<&str>, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>>;
    fn line_plot(&self, x_column: &str, y_column: &str, title: Option<&str>, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>>;
    fn bar_plot(&self, x_column: &str, y_column: &str, title: Option<&str>, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>>;
}

impl PolarsPlot for DataFrame {
    fn scatter_plot(&self, x_column: &str, y_column: &str, title: Option<&str>, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let (x_values, y_values) = extract_columns(self, x_column, y_column)?;
        let scatter = Scatter::new(x_values, y_values)
            .mode(Mode::Markers)
            .name(y_column);

        let mut plot = Plot::new();
        plot.add_trace(scatter);

        let layout = plot_layout(x_column, y_column, title, height, width);
        plot.set_layout(layout);

        Ok(plot)
    }

    fn line_plot(&self, x_column: &str, y_column: &str, title: Option<&str>, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let (x_values, y_values) = extract_columns(self, x_column, y_column)?;
        let line = Scatter::new(x_values, y_values)
            .mode(Mode::Lines)
            .name(y_column);

        let mut plot = Plot::new();
        plot.add_trace(line);

        let layout = plot_layout(x_column, y_column, title, height, width);
        plot.set_layout(layout);

        Ok(plot)
    }

    fn bar_plot(&self, x_column: &str, y_column: &str, title: Option<&str>, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let (x_values, y_values) = extract_columns(self, x_column, y_column)?;
        let bar = Bar::new(x_values, y_values).name(y_column);

        let mut plot = Plot::new();
        plot.add_trace(bar);

        let layout = plot_layout(x_column, y_column, title, height, width);
        plot.set_layout(layout);

        Ok(plot)
    }
}

fn extract_columns(df: &DataFrame, x_column: &str, y_column: &str) -> Result<(Vec<String>, Vec<f64>), Box<dyn Error>> {
    let x_series = df.column(x_column).unwrap();
    let y_series = df.column(y_column).unwrap();

    let x_values = x_series.str()?
        .into_no_null_iter()
        .map(|x| x.to_string())
        .collect();

    let y_values = y_series.f64()?
        .into_no_null_iter()
        .collect();

    Ok((x_values, y_values))
}

fn plot_layout(x_column: &str, y_column: &str, title: Option<&str>, height: Option<usize>, width: Option<usize>) -> Layout {
    let title = title.unwrap_or("");
    Layout::new()
        .height(height.unwrap_or(800))
        .width(width.unwrap_or(1200))
        .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{title}</span>")))
        .x_axis(Axis::new()
            .title(Title::from(x_column))
            .color("purple")
            .show_grid(false))
        .y_axis(Axis::new()
            .title(Title::from(y_column))
            .color("purple")
            .show_grid(false))
}