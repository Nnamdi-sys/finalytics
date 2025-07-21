use std::error::Error;
use plotly::layout::Axis;
use plotly::{HeatMap, Plot, Scatter};
use plotly::common::{ColorScalePalette, Mode, Title};
use crate::prelude::{DataTableDisplay, DataTableFormat, Tickers, TickersData};
use crate::analytics::statistics::{correlation_matrix, cumulative_returns_list};
use crate::charts::base_layout;
use crate::reports::table::DataTable;

pub trait TickersCharts {
    fn ohlcv_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn summary_stats_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn performance_stats_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn returns_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn returns_chart(&self, height: Option<usize>, width: Option<usize>) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
    fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
}


impl TickersCharts for Tickers {
    /// Displays the OHLCV Table for the tickers
    async fn ohlcv_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let data = self.get_chart().await?;
        let table = data.to_datatable("ohlcv", true, DataTableFormat::Number);
        Ok(table)
    }

    /// Display a Summary Stats Table for all tickers in the Tickers Struct
    async fn summary_stats_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let df = self.get_ticker_stats().await?;
        let table = df.to_datatable("summary_stats", true, DataTableFormat::Number);
        Ok(table)
    }

    /// Display a Performance Stats Table for all tickers in the Tickers Struct
    async fn performance_stats_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let stats = self.performance_stats().await?;
        let table = stats.to_datatable("performance_stats", true, DataTableFormat::Performance);
        Ok(table)
    }

    /// Display a Returns Table for all tickers in the Tickers Struct
    async fn returns_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let returns = self.returns().await?;
        let table = returns.to_datatable("returns", true, DataTableFormat::Number);
        Ok(table)
    }

    /// Display a Cumulative Returns Chart for all tickers in the Tickers Struct
    async fn returns_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let symbols = self.tickers.iter().map(|x| x.ticker.clone()).collect::<Vec<String>>();
        let asset_returns = self.returns().await?;
        let dates = asset_returns.column("timestamp")?.str()?.into_no_null_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut plot = Plot::new();

        for symbol in symbols {
            match asset_returns.column(&symbol) {
                Ok(returns_series) => {
                    let returns = returns_series.f64().unwrap().to_vec()
                        .iter().map(|x| x.unwrap_or_default()).collect::<Vec<f64>>();
                    let cum_returns = cumulative_returns_list(returns.clone());
                    let cum_returns_trace = Scatter::new(dates.clone(), cum_returns.clone())
                        .name(symbol)
                        .mode(Mode::Lines);
                    plot.add_trace(cum_returns_trace);
                }
                Err(e) => {
                    eprintln!("Unable to fetch returns for {symbol}: {e}");
                }
            }
        }

        let layout = base_layout(height, width)
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Tickers Cumulative Returns</span>"))
            .y_axis(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
                    .tick_format(".0%")
            );

        plot.set_layout(layout);
        Ok(plot)
    }

    /// Display a Returns Table for all tickers in the Tickers Struct
    async fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let mut returns = self.returns().await?;
        let _ = returns.drop_in_place("timestamp");
        let labels = returns.get_column_names().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let corr_matrix = correlation_matrix(&returns)?;
        let corr_matrix = corr_matrix.outer_iter()
            .map(|row| row.to_vec())
            .collect();
        let heatmap = HeatMap::new(labels.to_vec(), labels.to_vec(), corr_matrix)
            .zmin(-1.0)
            .zmax(1.0)
            .color_scale(ColorScalePalette::Jet.into());

        let mut plot = Plot::new();
        plot.add_trace(heatmap);
        plot.set_layout(
            base_layout(height, width)
                .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Returns Correlation Matrix</span>"))
        );

        Ok(plot)
    }
}