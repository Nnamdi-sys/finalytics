pub mod portfolio;
pub mod ticker;
pub mod tickers;


use plotly::{Configuration, Layout, Plot};

pub fn set_layout(mut plot: Plot, mut layout: Layout, height: Option<usize>, width: Option<usize>) -> Plot {
    let plot = if let (Some(h), Some(w)) = (height, width) {
        layout = layout.height(h).width(w);
        plot.set_layout(layout);
        plot
    } else {
        plot.set_layout(layout);
        plot.set_configuration(Configuration::default().responsive(true).fill_frame(true));
        plot
    };
    plot
}
