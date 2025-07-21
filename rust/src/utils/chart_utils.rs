#[cfg(feature = "kaleido")]
use plotly::{Plot, ImageFormat};
use polars::frame::DataFrame;
use crate::prelude::{DataTable, DataTableFormat};

#[cfg(feature = "kaleido")]
pub trait PlotImage {
    fn to_png(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_svg(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_jpeg(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_pdf(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_webp(&self, filename: &str, width: usize, height: usize, scale: f64);
    fn to_eps(&self, filename: &str, width: usize, height: usize, scale: f64);
}

#[cfg(feature = "kaleido")]
impl PlotImage for Plot {
    fn to_png(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::PNG, width, height, scale);
    }

    fn to_svg(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::SVG, width, height, scale);
    }

    fn to_jpeg(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::JPEG, width, height, scale);
    }

    fn to_pdf(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::PDF, width, height, scale);
    }

    fn to_webp(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::WEBP, width, height, scale);
    }

    fn to_eps(&self, filename: &str, width: usize, height: usize, scale: f64) {
        self.write_image(filename, ImageFormat::EPS, width, height, scale);
    }
}


pub trait DataTableDisplay {
    fn to_datatable(&self, id: &str, ordering: bool, format: DataTableFormat ) -> DataTable;
}

impl DataTableDisplay for DataFrame {
    fn to_datatable(&self, id: &str, ordering: bool, format: DataTableFormat ) -> DataTable {
        DataTable::new(self.clone(), id.to_string(), ordering, format)
    }
}
