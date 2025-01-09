use std::error::Error;
use plotly::layout::Axis;
use plotly::{HeatMap, Layout, Plot, Scatter};
use plotly::common::{ColorScalePalette, Mode, Title};
use polars::prelude::{NamedFrom, Series};
use crate::prelude::{Tickers, TickersData};
use crate::analytics::statistics::{correlation_matrix, cumulative_returns_list};
use crate::charts::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::reports::table::{DataTable, TableType};

pub trait TickersCharts {
    fn ohlcv_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn performance_stats_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn returns_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn returns_chart(&self, height: Option<usize>, width: Option<usize>) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
    fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
}


impl TickersCharts for Tickers {
    /// Displays the OHLCV Table for the tickers
    fn ohlcv_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>> {
        async move {
            let data = self.get_chart().await?;
            let table_chart = DataTable::new(data, TableType::OHLCV);
            Ok(table_chart)
        }
    }

    /// Display a Performance Stats Table for all tickers in the Tickers Struct
    async fn performance_stats_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let mut stats = self.performance_stats().await?;
        let columns = stats.column("Symbol")?.str()?.into_no_null_iter()
            .map(|x| x.to_string()).collect::<Vec<String>>();
        stats = stats.drop("Symbol")?;
        let items = Series::new("Items", stats.get_column_names());
        let mut stats_df = stats.transpose(None, None)?;
        let _ =  stats_df.set_column_names(&columns)?;
        let _ = stats_df.insert_column(0, items)?;
        let table = DataTable::new(stats_df, TableType::PerformanceStats);
        Ok(table)
    }

    /// Display a Returns Table for all tickers in the Tickers Struct
    async fn returns_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let returns = self.returns().await?;
        let table = DataTable::new(returns, TableType::Returns);
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
                        .name(format!("{}", symbol))
                        .mode(Mode::Lines);
                    plot.add_trace(cum_returns_trace);
                }
                Err(e) => {
                    eprintln!("Unable to fetch returns for {}: {}", symbol, e);
                }
            }
        }

        let layout = Layout::new()
            .height(height.unwrap_or(DEFAULT_HEIGHT))
            .width(width.unwrap_or(DEFAULT_WIDTH))
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
            Layout::new()
                .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Returns Correlation Matrix</span>"))
                .height(height.unwrap_or(DEFAULT_HEIGHT))
                .width(width.unwrap_or(DEFAULT_WIDTH))
        );

        Ok(plot)
    }
}