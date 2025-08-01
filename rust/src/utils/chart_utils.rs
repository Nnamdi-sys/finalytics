use plotly::Plot;
use plotly::plotly_static::ImageFormat;

pub trait PlotImage {
    fn to_png(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_svg(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_jpeg(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_pdf(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_webp(&self, filename: &str, width: usize, height: usize, scale: f64);
}

impl PlotImage for Plot {
    fn to_png(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::PNG, width, height, scale).unwrap();
    }

    fn to_svg(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::SVG, width, height, scale).unwrap();
    }

    fn to_jpeg(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::JPEG, width, height, scale).unwrap();
    }

    fn to_pdf(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::PDF, width, height, scale).unwrap();
    }

    fn to_webp(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::WEBP, width, height, scale).unwrap();
    }
}