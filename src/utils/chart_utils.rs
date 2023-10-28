#[cfg(feature = "kaleido")]
use plotly::{Plot, ImageFormat};

#[cfg(feature = "kaleido")]
pub enum ImgFormat {
    PNG,
    SVG,
    JPEG,
    PDF,
    EPS,
    WEBP,
}

#[cfg(feature = "kaleido")]
impl ImgFormat {
    pub fn format(&self) -> ImageFormat {
        match self {
            ImgFormat::PNG => ImageFormat::PNG,
            ImgFormat::SVG => ImageFormat::SVG,
            ImgFormat::JPEG => ImageFormat::JPEG,
            ImgFormat::PDF => ImageFormat::PDF,
            ImgFormat::EPS => ImageFormat::EPS,
            ImgFormat::WEBP => ImageFormat::WEBP,
        }
    }
}

#[cfg(feature = "kaleido")]
pub trait PlotImage {
    fn save_image(&self, filename: &str, format: ImgFormat, width: usize, height: usize, scale: f64);
}

#[cfg(feature = "kaleido")]
impl PlotImage for Plot {
    fn save_image(&self, filename: &str, format: ImgFormat, width: usize, height: usize, scale: f64) {
        self.write_image(filename, format.format(), width, height, scale);
    }
}