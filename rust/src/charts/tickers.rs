use crate::analytics::performance::TickerPerformance;
use crate::analytics::statistics::{
    correlation_matrix, cumulative_returns_list, resample_returns_pct, PerformancePeriod,
    ReturnsFrequency,
};
use crate::charts::set_layout;
use crate::prelude::{DataTableDisplay, DataTableFormat, Tickers, TickersData};
use crate::reports::table::{build_frequency_toggle, build_period_toggle, DataTable};
use plotly::common::{ColorScalePalette, Mode, Title};
use plotly::layout::Axis;
use plotly::{HeatMap, Layout, Plot, Scatter};
use polars::prelude::{Column, DataFrame};
use std::error::Error;

pub trait TickersCharts {
    fn ohlcv_table(&self) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    fn summary_stats_table(
        &self,
    ) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;
    /// Build a composite performance stats `DataTable` covering all applicable periods,
    /// with an embedded period-toggle HTML fragment.
    fn performance_stats_table(
        &self,
    ) -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;

    /// Build a composite returns `DataTable` covering all available frequencies,
    /// with an embedded frequency-toggle HTML fragment.
    fn returns_table(&self)
        -> impl std::future::Future<Output = Result<DataTable, Box<dyn Error>>>;

    fn returns_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
    fn returns_matrix(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> impl std::future::Future<Output = Result<Plot, Box<dyn Error>>>;
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

    async fn performance_stats_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let periods: Vec<PerformancePeriod> = if self.tickers.is_empty() {
            vec![PerformancePeriod::Full]
        } else {
            match self.tickers[0].performance_stats().await {
                Ok(stats) => stats.periodic_stats.iter().map(|(p, _)| *p).collect(),
                Err(_) => vec![PerformancePeriod::Full],
            }
        };

        // Fetch all ticker stats once upfront — avoid redundant async calls per period
        let mut all_stats: Vec<(
            String,
            crate::analytics::performance::TickerPerformanceStats,
        )> = Vec::new();
        for ticker in &self.tickers {
            match ticker.performance_stats().await {
                Ok(s) => all_stats.push((ticker.ticker.clone(), s)),
                Err(e) => eprintln!("Skipping {}: {e}", ticker.ticker),
            }
        }

        const STAT_NAMES: [&str; 16] = [
            "Daily Return",
            "Daily Volatility",
            "Cumulative Return",
            "Annualized Return",
            "Annualized Volatility",
            "Alpha",
            "Beta",
            "Sharpe Ratio",
            "Sortino Ratio",
            "Active Return",
            "Active Risk",
            "Information Ratio",
            "Calmar Ratio",
            "Maximum Drawdown",
            "Value at Risk",
            "Expected Shortfall",
        ];

        let mut period_entries: Vec<(String, String)> = Vec::with_capacity(periods.len());
        let mut primary_df: Option<DataFrame> = None;

        for period in &periods {
            let mut ticker_symbols: Vec<String> = Vec::new();
            let mut numeric_fields: Vec<Vec<f64>> = vec![vec![]; 16];

            for (symbol, stats) in &all_stats {
                let s = stats
                    .periodic_stats
                    .iter()
                    .find(|(p, _)| *p == *period)
                    .map(|(_, s)| s)
                    .unwrap_or(&stats.performance_stats);

                ticker_symbols.push(symbol.clone());
                numeric_fields[0].push(s.daily_return);
                numeric_fields[1].push(s.daily_volatility);
                numeric_fields[2].push(s.cumulative_return);
                numeric_fields[3].push(s.annualized_return);
                numeric_fields[4].push(s.annualized_volatility);
                numeric_fields[5].push(s.alpha.unwrap_or(f64::NAN));
                numeric_fields[6].push(s.beta.unwrap_or(f64::NAN));
                numeric_fields[7].push(s.sharpe_ratio);
                numeric_fields[8].push(s.sortino_ratio);
                numeric_fields[9].push(s.active_return.unwrap_or(f64::NAN));
                numeric_fields[10].push(s.active_risk.unwrap_or(f64::NAN));
                numeric_fields[11].push(s.information_ratio.unwrap_or(f64::NAN));
                numeric_fields[12].push(s.calmar_ratio);
                numeric_fields[13].push(s.maximum_drawdown);
                numeric_fields[14].push(s.value_at_risk);
                numeric_fields[15].push(s.expected_shortfall);
            }

            let mut columns: Vec<Column> = vec![Column::new("Symbol".into(), ticker_symbols)];
            for (ci, name) in STAT_NAMES.iter().enumerate() {
                columns.push(Column::new(
                    (*name).into(),
                    numeric_fields[ci]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                ));
            }
            let df = DataFrame::new(columns)?;
            let id = format!(
                "perf_stats_{}",
                period.to_string().to_lowercase().replace(' ', "_")
            );
            let html = df
                .to_datatable(
                    &id,
                    true,
                    DataTableFormat::Performance("tickers".to_string()),
                )
                .to_html()?;
            period_entries.push((period.to_string(), html));

            if *period == PerformancePeriod::Full && primary_df.is_none() {
                primary_df = Some(df);
            }
        }

        if period_entries.is_empty() {
            return Err("No period stats available".into());
        }
        let primary = primary_df.unwrap_or_else(|| DataFrame::default());
        let toggle_html = build_period_toggle(&period_entries, "tickers_perf_stats");
        Ok(DataTable::new_composite(
            primary,
            "performance_stats".to_string(),
            toggle_html,
        ))
    }

    async fn returns_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let native = ReturnsFrequency::from_interval_enum(self.interval);
        let freqs = native.available_frequencies();

        // Fetch returns once and reuse across frequencies
        let mut raw_returns = self.returns().await?;
        let dates: Vec<String> = raw_returns
            .column("timestamp")?
            .str()?
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect();
        let _ = raw_returns.drop_in_place("timestamp")?;

        let mut freq_entries: Vec<(String, String)> = Vec::with_capacity(freqs.len());
        let mut primary_df: Option<DataFrame> = None;

        for freq in &freqs {
            let pct_df = if *freq == native {
                // Native frequency: use raw returns, scale to percentage
                let col_names: Vec<String> = raw_returns
                    .get_column_names()
                    .iter()
                    .map(|n| n.to_string())
                    .collect();
                let mut cols: Vec<Column> = vec![Column::new("Timestamp".into(), dates.clone())];
                for name in &col_names {
                    let series = raw_returns.column(name.as_str())?.f64()?;
                    let vals: Vec<f64> = series
                        .into_no_null_iter()
                        .map(|v| (v * 100.0 * 100.0).round() / 100.0)
                        .collect();
                    cols.push(Column::new(name.as_str().into(), vals));
                }
                DataFrame::new(cols)?
            } else {
                // Coarser frequency: resample then scale to percentage
                let (labels, resampled_df) = resample_returns_pct(&dates, &raw_returns, *freq)?;
                let col_names: Vec<String> = resampled_df
                    .get_column_names()
                    .iter()
                    .map(|n| n.to_string())
                    .collect();
                let mut cols: Vec<Column> = vec![Column::new("Timestamp".into(), labels.clone())];
                for name in &col_names {
                    let series = resampled_df.column(name.as_str())?.f64()?;
                    let vals: Vec<f64> = series
                        .into_no_null_iter()
                        .map(|v| (v * 100.0 * 100.0).round() / 100.0)
                        .collect();
                    cols.push(Column::new(name.as_str().into(), vals));
                }
                DataFrame::new(cols)?
            };

            let id = format!("returns_{}", freq.to_string().to_lowercase());
            let html = pct_df
                .to_datatable(&id, true, DataTableFormat::Number)
                .to_html()?;
            freq_entries.push((freq.to_string(), html));

            if *freq == native && primary_df.is_none() {
                primary_df = Some(pct_df);
            }
        }

        if freq_entries.is_empty() {
            return Err("No returns data available".into());
        }
        let primary = primary_df.ok_or("Native frequency data not found")?;
        let toggle_html = build_frequency_toggle(&freq_entries, "tickers_returns");
        Ok(DataTable::new_composite(
            primary,
            "returns_table".to_string(),
            toggle_html,
        ))
    }

    /// Display a Cumulative Returns Chart for all tickers in the Tickers Struct
    async fn returns_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>> {
        let symbols = self
            .tickers
            .iter()
            .map(|x| x.ticker.clone())
            .collect::<Vec<String>>();
        let asset_returns = self.returns().await?;
        let dates = asset_returns
            .column("timestamp")?
            .str()?
            .into_no_null_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut plot = Plot::new();

        for symbol in symbols {
            match asset_returns.column(&symbol) {
                Ok(returns_series) => {
                    let returns = returns_series
                        .f64()
                        .unwrap_or_else(|_| panic!("returns column '{}' is not Float64", symbol))
                        .to_vec()
                        .iter()
                        .map(|x| x.unwrap_or_default())
                        .collect::<Vec<f64>>();
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

        let layout = Layout::new()
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Tickers Cumulative Returns</span>"))
            .y_axis(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
                    .tick_format(".0%")
            );

        let plot = set_layout(plot, layout, height, width);
        Ok(plot)
    }

    /// Display a Returns Correlation Matrix for all tickers in the Tickers Struct
    async fn returns_matrix(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>> {
        let mut returns = self.returns().await?;
        let _ = returns.drop_in_place("timestamp");
        let labels = returns
            .get_column_names()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let corr_matrix = correlation_matrix(&returns)?;
        let corr_matrix = corr_matrix.outer_iter().map(|row| row.to_vec()).collect();
        let heatmap = HeatMap::new(labels.to_vec(), labels.to_vec(), corr_matrix)
            .zmin(-1.0)
            .zmax(1.0)
            .color_scale(ColorScalePalette::Jet.into());

        let mut plot = Plot::new();
        plot.add_trace(heatmap);
        let layout = Layout::new().title(Title::from(
            "<span style=\"font-weight:bold; color:darkgreen;\">Returns Correlation Matrix</span>",
        ));
        let plot = set_layout(plot, layout, height, width);
        Ok(plot)
    }
}
