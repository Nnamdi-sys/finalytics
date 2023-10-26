#[cfg(feature = "kaleido")]
use plotly::{Plot, ImageFormat};

#[cfg(feature = "kaleido")]
pub trait PlotImage {
    fn save_image(&self, filename: &str, format: ImageFormat, width: usize, height: usize, scale: f64);
}

#[cfg(feature = "kaleido")]
impl PlotImage for Plot {
    fn save_image(&self, filename: &str, format: ImageFormat, width: usize, height: usize, scale: f64) {
        self.write_image(filename, format, width, height, scale);
    }
}