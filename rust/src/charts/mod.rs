pub mod portfolio;
pub mod ticker;
pub mod tickers;


use plotly::Layout;

pub fn base_layout(height: Option<usize>, width: Option<usize>) -> Layout {
    let mut layout = Layout::new();

    match (height, width) {
        (Some(h), Some(w)) => {
            layout = layout.height(h).width(w);
        }
        _ => {
            layout = layout.height(800).width(1200);
        }
    }

    layout
}
