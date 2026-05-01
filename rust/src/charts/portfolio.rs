use plotly::color::NamedColor;
use plotly::common::{ColorScalePalette, DashType, Fill, Line, Marker, MarkerSymbol, Mode, Title};
use plotly::layout::{Axis, GridPattern, LayoutGrid, RowOrder};

use plotly::{Bar, HeatMap, Histogram, Layout, Plot, Scatter};
use polars::prelude::{Column, DataFrame};
use std::cmp::Ordering;
use std::error::Error;

use chrono::Months;

use crate::analytics::statistics::{
    correlation_matrix, cumulative_returns_list, find_start_index, maximum_drawdown,
    parse_naive_date, PerformancePeriod, PerformanceStats,
};
use crate::charts::set_layout;
use crate::models::portfolio::Portfolio;
use crate::prelude::{DataTable, DataTableDisplay, DataTableFormat};
use crate::reports::table::{build_combined_returns_table, build_period_toggle};

// ---------------------------------------------------------------------------
// PortfolioCharts trait
// ---------------------------------------------------------------------------

pub trait PortfolioCharts {
    fn optimal_symbols(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn optimization_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>>;
    fn performance_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>>;
    fn performance_stats_table(&self) -> Result<DataTable, Box<dyn Error>>;

    /// Unified returns table: returns per-frequency pairs (pct_table, val_table) for all available frequencies.
    fn returns_table(&self) -> Result<DataTable, Box<dyn Error>>;
    fn returns_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>>;
    fn returns_matrix(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>>;
    /// Stacked area chart showing portfolio growth and asset allocation over time.
    fn portfolio_value_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>>;

    /// Transaction history table showing rebalance events, cash flows, and
    /// implied per-asset trades with cumulative TWR and MWR columns.
    fn transaction_history_table(&self) -> Result<Option<DataTable>, Box<dyn Error>>;
}

impl PortfolioCharts for Portfolio {
    /// Returns the Optimal Symbols for the Portfolio
    fn optimal_symbols(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let symbols = self.data.ticker_symbols.clone();

        // Determine weights: prefer performance_stats, fall back to optimization_result
        let weights = if let Some(ref perf) = self.performance_stats {
            perf.weights.clone()
        } else if let Some(ref opt) = self.optimization_result {
            opt.optimal_weights.clone()
        } else {
            return Err("No optimization or performance analysis has been computed yet".into());
        };

        let filtered_results: Vec<_> = symbols
            .iter()
            .zip(weights.iter())
            .filter(|&(_, &weight)| weight.abs() > 0.0)
            .collect();
        let symbols: Vec<String> = filtered_results
            .iter()
            .map(|&(ticker, _)| ticker.clone())
            .collect();
        Ok(symbols)
    }

    fn optimization_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>> {
        let opt = self
            .optimization_result
            .as_ref()
            .ok_or("Optimization has not been run. Call optimize() first.")?;

        let ticker_symbols = self.data.ticker_symbols.clone();

        // Allocation weights as percentages
        let alloc_weights: Vec<f64> = opt.optimal_weights.iter().map(|x| x * 100.0).collect();

        let mut filtered: Vec<_> = ticker_symbols
            .iter()
            .zip(alloc_weights.iter())
            .filter(|&(_, &w)| w.abs() > 0.01)
            .collect();
        filtered.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal));

        let filtered_syms: Vec<String> = filtered.iter().map(|&(t, _)| t.clone()).collect();
        let filtered_wts: Vec<f64> = filtered.iter().map(|&(_, &w)| w).collect();

        let allocation_trace = Bar::new(filtered_syms.clone(), filtered_wts.clone())
            .name("Asset Allocation")
            .x_axis("x2")
            .y_axis("y2")
            .text_array(
                filtered_wts
                    .iter()
                    .map(|w| format!("{w:.2}%"))
                    .collect::<Vec<_>>(),
            );

        let mut plot = Plot::new();

        if opt.objective_function.uses_frontier() && !opt.efficient_frontier.is_empty() {
            // Efficient frontier view
            let days = self.data.interval.mode;
            let annual_days = 365.0 / self.data.interval.average;

            let ef_returns: Vec<f64> = opt
                .efficient_frontier
                .iter()
                .map(|x| (1.0 + x[0] / days).powf(annual_days) - 1.0)
                .collect();
            let ef_risk: Vec<f64> = opt
                .efficient_frontier
                .iter()
                .map(|x| x[1] * annual_days.sqrt())
                .collect();

            let ef_trace = Scatter::new(ef_risk, ef_returns)
                .name("Efficient Frontier")
                .mode(Mode::Markers)
                .marker(Marker::new().size(10));

            let opt_return = (1.0 + opt.optimal_return / days).powf(annual_days) - 1.0;
            let opt_risk = opt.optimal_risk * annual_days.sqrt();

            let point_label = match opt.objective_function {
                crate::analytics::optimization::ObjectiveFunction::MaxSharpe => "Max Sharpe",
                crate::analytics::optimization::ObjectiveFunction::MaxSortino => "Max Sortino",
                crate::analytics::optimization::ObjectiveFunction::MaxReturn => "Max Return",
                crate::analytics::optimization::ObjectiveFunction::MinVol => "Min Volatility",
                _ => "Optimal Portfolio",
            };

            let optimal_point = Scatter::new(vec![opt_risk], vec![opt_return])
                .name(point_label)
                .mode(Mode::Markers)
                .marker(
                    Marker::new()
                        .size(12)
                        .color(NamedColor::Red)
                        .symbol(MarkerSymbol::Star),
                );

            plot.add_trace(ef_trace);
            plot.add_trace(optimal_point);
            plot.add_trace(allocation_trace);

            let title_label = match opt.objective_function {
                crate::analytics::optimization::ObjectiveFunction::MaxSharpe => "Maximize Sharpe",
                crate::analytics::optimization::ObjectiveFunction::MaxSortino => "Maximize Sortino",
                crate::analytics::optimization::ObjectiveFunction::MaxReturn => "Maximize Return",
                crate::analytics::optimization::ObjectiveFunction::MinVol => "Minimize Volatility",
                _ => "Portfolio Optimization",
            };

            let layout = Layout::new()
                .title(Title::from(
                    format!(
                        "<span style=\"font-weight:bold; color:darkgreen;\">{title_label} – Optimization Chart</span>"
                    )
                    .as_str(),
                ))
                .grid(
                    LayoutGrid::new()
                        .rows(2)
                        .columns(1)
                        .pattern(GridPattern::Independent)
                        .row_order(RowOrder::TopToBottom),
                )
                .x_axis(Axis::new().title(Title::from("Annualized Risk")).tick_format(".0%"))
                .y_axis(Axis::new().title(Title::from("Annualized Returns")).tick_format(".0%"))
                .x_axis2(Axis::new().title(Title::from("Portfolio Assets")))
                .y_axis2(Axis::new().title(Title::from("Asset Allocation")));

            let plot = set_layout(plot, layout, height, width);
            Ok(plot)
        } else {
            // Risk contribution view
            let rc = &opt.risk_contributions;
            let rc_total: f64 = rc.iter().sum();

            let prc: Vec<f64> = if rc_total.abs() > 1e-14 {
                rc.iter().map(|c| c / rc_total * 100.0).collect()
            } else {
                vec![100.0 / ticker_symbols.len() as f64; ticker_symbols.len()]
            };

            let mut rc_filtered: Vec<_> = ticker_symbols
                .iter()
                .zip(prc.iter())
                .filter(|&(_, &p)| p.abs() > 0.01)
                .collect();
            rc_filtered.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal));

            let rc_syms: Vec<String> = rc_filtered.iter().map(|&(t, _)| t.clone()).collect();
            let rc_pcts: Vec<f64> = rc_filtered.iter().map(|&(_, &p)| p).collect();

            let rc_trace = Bar::new(rc_syms.clone(), rc_pcts.clone())
                .name("Risk Contribution")
                .text_array(
                    rc_pcts
                        .iter()
                        .map(|p| format!("{p:.1}%"))
                        .collect::<Vec<_>>(),
                );

            plot.add_trace(rc_trace);
            plot.add_trace(allocation_trace);

            let obj_label = match opt.objective_function {
                crate::analytics::optimization::ObjectiveFunction::MinVol => "Minimize Volatility",
                crate::analytics::optimization::ObjectiveFunction::MinVar => {
                    "Minimize Value at Risk"
                }
                crate::analytics::optimization::ObjectiveFunction::MinCVaR => "Minimize CVaR",
                crate::analytics::optimization::ObjectiveFunction::MinDrawdown => {
                    "Minimize Drawdown"
                }
                crate::analytics::optimization::ObjectiveFunction::RiskParity => "Risk Parity",
                crate::analytics::optimization::ObjectiveFunction::MaxDiversification => {
                    "Max Diversification"
                }
                crate::analytics::optimization::ObjectiveFunction::HierarchicalRiskParity => {
                    "Hierarchical Risk Parity"
                }
                _ => "Portfolio Optimization",
            };

            let layout = Layout::new()
                .title(Title::from(
                    format!("<span style=\"font-weight:bold; color:darkgreen;\">{obj_label} – Optimization Chart</span>")
                        .as_str(),
                ))
                .grid(
                    LayoutGrid::new()
                        .rows(2)
                        .columns(1)
                        .pattern(GridPattern::Independent)
                        .row_order(RowOrder::TopToBottom),
                )
                .x_axis(Axis::new().title(Title::from("Portfolio Assets")))
                .y_axis(Axis::new().title(Title::from("Risk Contribution %")))
                .x_axis2(Axis::new().title(Title::from("Portfolio Assets")))
                .y_axis2(Axis::new().title(Title::from("Asset Allocation %")));

            let plot = set_layout(plot, layout, height, width);
            Ok(plot)
        }
    }

    fn performance_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>> {
        let perf = self
            .performance_stats
            .as_ref()
            .ok_or("Performance stats have not been computed. Call performance_stats() first.")?;

        let dates = self.data.dates_array.clone();

        let returns = perf
            .portfolio_returns
            .clone()
            .f64()
            .unwrap_or_else(|_| panic!("portfolio_returns is not Float64"))
            .to_vec()
            .iter()
            .map(|x| x.unwrap_or_default())
            .collect::<Vec<f64>>();

        let benchmark_returns = self.data.benchmark_returns.as_ref().map(|br| {
            br.f64()
                .unwrap_or_else(|_| panic!("benchmark_returns is not Float64"))
                .to_vec()
                .iter()
                .map(|x| x.unwrap_or_default())
                .collect::<Vec<f64>>()
        });

        let cum_returns = cumulative_returns_list(returns.clone());
        let benchmark_cum_returns = benchmark_returns
            .as_ref()
            .map(|br| cumulative_returns_list(br.clone()));

        let (drawdowns, _) = maximum_drawdown(&perf.portfolio_returns)?;

        let returns_trace = Scatter::new(dates.clone(), returns.clone())
            .name("Portfolio Returns")
            .mode(Mode::Markers)
            .fill(Fill::ToZeroY);

        let returns_dist_trace = Histogram::new(returns.clone())
            .name("Portfolio Returns Distribution")
            .x_axis("x2")
            .y_axis("y2");

        let cum_returns_trace = Scatter::new(dates.clone(), cum_returns.clone())
            .name("Portfolio Cumulative Returns")
            .mode(Mode::Lines)
            .fill(Fill::ToZeroY)
            .x_axis("x3")
            .y_axis("y3");

        let benchmark_cum_returns_trace = benchmark_cum_returns.as_ref().map(|bcr| {
            Scatter::new(dates.clone(), bcr.clone())
                .name("Benchmark Cumulative Returns")
                .mode(Mode::Lines)
                .fill(Fill::ToZeroY)
                .x_axis("x3")
                .y_axis("y3")
        });

        let drawdown_trace = Scatter::new(dates.clone(), drawdowns.clone())
            .name("Portfolio Drawdown")
            .mode(Mode::Lines)
            .fill(Fill::ToZeroY)
            .x_axis("x4")
            .y_axis("y4");

        let mut plot = Plot::new();
        plot.add_trace(returns_trace);
        plot.add_trace(returns_dist_trace);
        plot.add_trace(cum_returns_trace);
        if let Some(trace) = benchmark_cum_returns_trace {
            plot.add_trace(trace);
        }
        plot.add_trace(drawdown_trace);

        let layout = Layout::new()
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Performance Chart</span>"))
            .grid(
                LayoutGrid::new()
                    .rows(4)
                    .columns(1)
                    .pattern(GridPattern::Independent)
                    .row_order(RowOrder::TopToBottom)
            )
            .y_axis(
                Axis::new()
                    .title(Title::from("Returns"))
                    .tick_format(".0%")
            )
            .y_axis2(
                Axis::new()
                    .title(Title::from("Returns Distribution"))
            )
            .x_axis2(
                Axis::new()
                    .tick_format(".0%")
            )
            .y_axis3(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
                    .tick_format(".0%")
            )
            .y_axis4(
                Axis::new()
                    .title(Title::from("Drawdown"))
                    .tick_format(".0%")
            );

        let plot = set_layout(plot, layout, height, width);

        Ok(plot)
    }

    fn returns_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>> {
        let _perf = self
            .performance_stats
            .as_ref()
            .ok_or("Performance stats have not been computed. Call performance_stats() first.")?;

        let symbols = self.optimal_symbols()?;
        let asset_returns = self.data.portfolio_returns.clone();
        let dates = self.data.dates_array.clone();
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
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Assets Cumulative Returns</span>"))
            .y_axis(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
                    .tick_format(".0%")
            );

        let plot = set_layout(plot, layout, height, width);
        Ok(plot)
    }

    fn returns_matrix(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>> {
        let symbols = self.optimal_symbols()?;
        let returns = self.data.portfolio_returns.clone();
        let returns = returns.select(&symbols)?;
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

    fn portfolio_value_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> Result<Plot, Box<dyn Error>> {
        let perf = self
            .performance_stats
            .as_ref()
            .ok_or("Performance stats have not been computed. Call performance_stats() first.")?;

        let dates = self.data.dates_array.clone();
        let num_rows = dates.len();
        let all_symbols = self.data.ticker_symbols.clone();
        let num_assets = all_symbols.len();
        let threshold = 2.0_f64; // min ending-weight % to show individually

        // --- Determine which assets to show vs group as "Others" ---
        let ending_total: f64 = perf.ending_values.iter().sum();
        let ending_weights_pct: Vec<f64> = if ending_total > 0.0 {
            perf.ending_values
                .iter()
                .map(|v| {
                    if ending_total > 0.0 {
                        v / ending_total * 100.0
                    } else {
                        0.0
                    }
                })
                .collect()
        } else {
            vec![0.0; num_assets]
        };

        // Compute average weight for sort order (largest at bottom = most stable)
        let avg_weights: Vec<f64> = (0..num_assets)
            .map(|i| {
                let sum: f64 = perf
                    .asset_values_over_time
                    .iter()
                    .zip(perf.portfolio_values.iter())
                    .map(|(av, &pv)| if pv > 0.0 { av[i] / pv } else { 0.0 })
                    .sum();
                if num_rows > 0 {
                    sum / num_rows as f64
                } else {
                    0.0
                }
            })
            .collect();

        // Split into above-threshold and below-threshold based on ending weight
        let mut above_indices: Vec<usize> = Vec::new();
        let mut others_indices: Vec<usize> = Vec::new();
        for i in 0..num_assets {
            if ending_weights_pct[i].abs() >= threshold {
                above_indices.push(i);
            } else {
                others_indices.push(i);
            }
        }

        // Sort above-threshold by average weight ascending (largest at bottom of stack)
        above_indices.sort_by(|&a, &b| {
            avg_weights[a]
                .partial_cmp(&avg_weights[b])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Build per-asset value series for each visible asset + "Others"
        let has_others = !others_indices.is_empty();
        let mut trace_names: Vec<String> = Vec::new();
        let mut trace_values: Vec<Vec<f64>> = Vec::new();

        // Add above-threshold assets (sorted largest-avg-weight first = bottom of stack)
        for &idx in &above_indices {
            trace_names.push(all_symbols[idx].clone());
            let values: Vec<f64> = perf
                .asset_values_over_time
                .iter()
                .map(|row| row.get(idx).copied().unwrap_or(0.0))
                .collect();
            trace_values.push(values);
        }

        // Add \"Others\" bucket
        if has_others {
            trace_names.push("Others".to_string());
            let values: Vec<f64> = perf
                .asset_values_over_time
                .iter()
                .map(|row| {
                    others_indices
                        .iter()
                        .map(|&idx| row.get(idx).copied().unwrap_or(0.0))
                        .sum()
                })
                .collect();
            trace_values.push(values);
        }

        // --- Build stacked area traces ---
        let mut plot = Plot::new();

        for (t_idx, (name, values)) in trace_names.iter().zip(trace_values.iter()).enumerate() {
            // Compute hover text: \"$XX,XXX (YY.Y%)\"
            let hover: Vec<String> = values
                .iter()
                .zip(perf.portfolio_values.iter())
                .map(|(&v, &pv)| {
                    let pct = if pv > 0.0 { v / pv * 100.0 } else { 0.0 };
                    format!("{name}: ${v:.2} ({pct:.1}%)")
                })
                .collect();

            let mut trace = Scatter::new(dates.clone(), values.clone())
                .name(name)
                .mode(Mode::Lines)
                .stack_group("portfolio")
                .hover_text_array(hover);

            if t_idx == 0 {
                trace = trace.fill(Fill::ToZeroY);
            }

            plot.add_trace(trace);
        }

        // --- Dashed total portfolio value line overlay ---
        let total_hover: Vec<String> = perf
            .portfolio_values
            .iter()
            .map(|&pv| format!("Total: ${pv:.2}"))
            .collect();

        let total_trace = Scatter::new(dates.clone(), perf.portfolio_values.clone())
            .name("Total Portfolio Value")
            .mode(Mode::Lines)
            .line(
                Line::new()
                    .dash(DashType::Dash)
                    .color(NamedColor::Black)
                    .width(1.5),
            )
            .hover_text_array(total_hover);

        plot.add_trace(total_trace);

        let layout = Layout::new()
            .title(Title::from(
                "<span style=\"font-weight:bold; color:darkgreen;\">Portfolio Growth</span>",
            ))
            .y_axis(Axis::new().title(Title::from("Portfolio Value ($)")));

        let plot = set_layout(plot, layout, height, width);
        Ok(plot)
    }

    fn performance_stats_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let perf = self
            .performance_stats
            .as_ref()
            .ok_or("Performance stats have not been computed. Call performance_stats() first.")?;

        let symbols = self.optimal_symbols()?;
        let all_symbols = &self.data.ticker_symbols;
        let dates = &self.data.dates_array;
        let num_assets = all_symbols.len();

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

        let fmt_stats = |s: &PerformanceStats| -> Vec<String> {
            vec![
                format!("{:.6}%", s.daily_return * 100.0),
                format!("{:.6}%", s.daily_volatility * 100.0),
                format!("{:.4}%", s.cumulative_return * 100.0),
                format!("{:.4}%", s.annualized_return * 100.0),
                format!("{:.4}%", s.annualized_volatility * 100.0),
                s.alpha.map_or("N/A".to_string(), |v| format!("{:.4}", v)),
                s.beta.map_or("N/A".to_string(), |v| format!("{:.4}", v)),
                format!("{:.4}", s.sharpe_ratio),
                format!("{:.4}", s.sortino_ratio),
                s.active_return
                    .map_or("N/A".to_string(), |v| format!("{:.4}%", v * 100.0)),
                s.active_risk
                    .map_or("N/A".to_string(), |v| format!("{:.4}%", v * 100.0)),
                s.information_ratio
                    .map_or("N/A".to_string(), |v| format!("{:.4}", v)),
                format!("{:.4}", s.calmar_ratio),
                format!("{:.4}%", s.maximum_drawdown * 100.0),
                format!("{:.4}%", s.value_at_risk * 100.0),
                format!("{:.4}%", s.expected_shortfall * 100.0),
            ]
        };

        let periods = self.data.applicable_periods();
        let mut period_entries: Vec<(String, String)> = Vec::with_capacity(periods.len());
        let mut primary_df: Option<DataFrame> = None;

        for period in &periods {
            // Determine period-specific starting/ending values
            let (period_starting_values, period_ending_values) =
                if *period == PerformancePeriod::Full {
                    (perf.starting_values.clone(), perf.ending_values.clone())
                } else if let Some(months) = period.months() {
                    let end_date = dates.last().and_then(|s| parse_naive_date(s));
                    let target_start =
                        end_date.and_then(|ed| ed.checked_sub_months(Months::new(months)));
                    let start_row = target_start.and_then(|ts| find_start_index(dates, ts));
                    let last_row = perf.asset_values_over_time.len().saturating_sub(1);

                    let sv = if let Some(sr) = start_row {
                        let lookup = if sr > 0 { sr - 1 } else { sr };
                        perf.asset_values_over_time
                            .get(lookup)
                            .cloned()
                            .unwrap_or_else(|| perf.starting_values.clone())
                    } else {
                        perf.starting_values.clone()
                    };
                    let ev = perf
                        .asset_values_over_time
                        .get(last_row)
                        .cloned()
                        .unwrap_or_else(|| perf.ending_values.clone());
                    (sv, ev)
                } else {
                    (perf.starting_values.clone(), perf.ending_values.clone())
                };

            let starting_total: f64 = period_starting_values.iter().sum();
            let ending_total: f64 = period_ending_values.iter().sum();

            let period_starting_weights: Vec<f64> = if starting_total > 0.0 {
                period_starting_values
                    .iter()
                    .map(|v| v / starting_total * 100.0)
                    .collect()
            } else {
                vec![0.0; num_assets]
            };
            let period_ending_weights: Vec<f64> = if ending_total > 0.0 {
                period_ending_values
                    .iter()
                    .map(|v| v / ending_total * 100.0)
                    .collect()
            } else {
                vec![0.0; num_assets]
            };

            let asset_stats_vec = match perf.periodic_stats_per_asset.get(period) {
                Some(v) => v,
                None => continue,
            };

            let mut row_symbols: Vec<String> = Vec::new();
            let mut row_sw: Vec<String> = Vec::new();
            let mut row_ew: Vec<String> = Vec::new();
            let mut row_sv: Vec<String> = Vec::new();
            let mut row_ev: Vec<String> = Vec::new();
            let mut stat_cols: Vec<Vec<String>> = vec![Vec::new(); 16];

            for sym in &symbols {
                if let Some(idx) = all_symbols.iter().position(|s| s == sym) {
                    row_symbols.push(sym.clone());
                    row_sw.push(
                        period_starting_weights
                            .get(idx)
                            .copied()
                            .unwrap_or(0.0)
                            .to_string(),
                    );
                    row_ew.push(
                        period_ending_weights
                            .get(idx)
                            .copied()
                            .unwrap_or(0.0)
                            .to_string(),
                    );
                    row_sv.push(
                        period_starting_values
                            .get(idx)
                            .copied()
                            .unwrap_or(0.0)
                            .to_string(),
                    );
                    row_ev.push(
                        period_ending_values
                            .get(idx)
                            .copied()
                            .unwrap_or(0.0)
                            .to_string(),
                    );
                    let s = &asset_stats_vec[idx];
                    for (ci, v) in fmt_stats(s).into_iter().enumerate() {
                        stat_cols[ci].push(v);
                    }
                }
            }

            // Portfolio aggregate row
            let portfolio_stats = perf
                .periodic_stats
                .iter()
                .find(|(p, _)| *p == *period)
                .map(|(_, s)| s)
                .unwrap_or(&perf.performance_stats);
            row_symbols.push("Portfolio".to_string());
            row_sw.push(period_starting_weights.iter().sum::<f64>().to_string());
            row_ew.push(period_ending_weights.iter().sum::<f64>().to_string());
            row_sv.push(starting_total.to_string());
            row_ev.push(ending_total.to_string());
            for (ci, v) in fmt_stats(portfolio_stats).into_iter().enumerate() {
                stat_cols[ci].push(v);
            }

            let mut columns: Vec<Column> = vec![
                Column::new("Symbol".into(), row_symbols),
                Column::new("Starting Weight (%)".into(), row_sw),
                Column::new("Ending Weight (%)".into(), row_ew),
                Column::new("Starting Value ($)".into(), row_sv),
                Column::new("Ending Value ($)".into(), row_ev),
            ];
            for (ci, name) in STAT_NAMES.iter().enumerate() {
                columns.push(Column::new((*name).into(), stat_cols[ci].clone()));
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
                    DataTableFormat::Performance("portfolio".to_string()),
                )
                .to_html()?;
            period_entries.push((period.to_string(), html));

            if *period == PerformancePeriod::Full && primary_df.is_none() {
                primary_df = Some(df);
            }
        }

        if period_entries.is_empty() {
            return Err("No period stats available to build performance stats table".into());
        }
        let primary = primary_df.unwrap_or_default();
        let toggle_html = build_period_toggle(&period_entries, "portfolio_perf_stats");
        Ok(DataTable::new_composite(
            primary,
            "performance_stats".to_string(),
            toggle_html,
        ))
    }

    fn returns_table(&self) -> Result<DataTable, Box<dyn Error>> {
        let perf = self
            .performance_stats
            .as_ref()
            .ok_or("Performance stats have not been computed. Call performance_stats() first.")?;

        let symbols = self.optimal_symbols()?;
        let native = self.data.native_frequency();
        let freqs = self.data.available_frequencies();

        let mut entries: Vec<(String, String, String)> = Vec::with_capacity(freqs.len());
        let mut primary_df: Option<DataFrame> = None;

        for freq in &freqs {
            let (labels, resampled_df) = match perf.returns_by_frequency.get(freq) {
                Some(data) => data,
                None => continue,
            };

            // Pct DataFrame: Timestamp + per-asset returns scaled to % + Portfolio aggregate
            let mut pct_cols: Vec<Column> = vec![Column::new("Timestamp".into(), labels.clone())];
            for sym in &symbols {
                if let Ok(series) = resampled_df.column(sym.as_str()) {
                    if let Ok(ca) = series.f64() {
                        let vals: Vec<f64> = ca
                            .into_no_null_iter()
                            .map(|v| (v * 100.0 * 100.0).round() / 100.0)
                            .collect();
                        pct_cols.push(Column::new(sym.as_str().into(), vals));
                    }
                }
            }
            if let Some((_, port_vals)) = perf.portfolio_returns_by_frequency.get(freq) {
                let port_pct: Vec<f64> = port_vals
                    .iter()
                    .map(|v| (v * 100.0 * 100.0).round() / 100.0)
                    .collect();
                pct_cols.push(Column::new("Portfolio".into(), port_pct));
            }
            let pct_df = DataFrame::new(pct_cols)?;
            let pct_id = format!("returns_pct_{}", freq.to_string().to_lowercase());
            let pct_html = pct_df
                .to_datatable(&pct_id, true, DataTableFormat::Number)
                .to_html()?;

            // Val DataFrame: Timestamp + per-asset dollar values + Portfolio total
            let val_html = if let Some((val_labels, asset_vals, port_vals)) =
                perf.values_by_frequency.get(freq)
            {
                let mut val_cols: Vec<Column> =
                    vec![Column::new("Timestamp".into(), val_labels.clone())];
                for (i, sym) in symbols.iter().enumerate() {
                    let vals: Vec<f64> = asset_vals
                        .iter()
                        .map(|row| row.get(i).copied().unwrap_or(0.0))
                        .collect();
                    val_cols.push(Column::new(sym.as_str().into(), vals));
                }
                val_cols.push(Column::new("Portfolio Value ($)".into(), port_vals.clone()));
                let val_df = DataFrame::new(val_cols)?;
                let val_id = format!("returns_val_{}", freq.to_string().to_lowercase());
                val_df
                    .to_datatable(&val_id, true, DataTableFormat::Currency)
                    .to_html()?
            } else {
                pct_html.clone()
            };

            entries.push((freq.to_string(), pct_html, val_html));

            if *freq == native && primary_df.is_none() {
                primary_df = Some(pct_df);
            }
        }

        if entries.is_empty() {
            return Err("No returns data available to build returns table".into());
        }
        let primary = primary_df.ok_or("Native frequency data not found in precomputed results")?;
        let toggle_html = build_combined_returns_table(&entries);
        Ok(DataTable::new_composite(
            primary,
            "returns_table".to_string(),
            toggle_html,
        ))
    }

    fn transaction_history_table(&self) -> Result<Option<DataTable>, Box<dyn Error>> {
        let perf = self
            .performance_stats
            .as_ref()
            .ok_or("Performance stats have not been computed. Call performance_stats() first.")?;

        let events = &perf.transaction_events;
        if events.is_empty() {
            return Ok(None);
        }

        let symbols = self.optimal_symbols()?;

        // Build column vectors
        let mut dates: Vec<String> = Vec::with_capacity(events.len());
        let mut event_types: Vec<String> = Vec::with_capacity(events.len());
        let mut portfolio_values: Vec<f64> = Vec::with_capacity(events.len());
        let mut turnovers: Vec<f64> = Vec::with_capacity(events.len());
        let mut cash_flows: Vec<f64> = Vec::with_capacity(events.len());
        let mut cum_twrs: Vec<f64> = Vec::with_capacity(events.len());
        let mut cum_mwrs: Vec<f64> = Vec::with_capacity(events.len());

        let num_assets = symbols.len();
        let mut asset_trades: Vec<Vec<f64>> = vec![Vec::with_capacity(events.len()); num_assets];

        for evt in events {
            dates.push(evt.date.clone());
            event_types.push(evt.event_type.to_string());
            portfolio_values.push(evt.portfolio_value_after);
            turnovers.push(evt.turnover * 100.0); // turnover is already a fraction, display as %
            cash_flows.push(evt.cash_flow_amount);
            cum_twrs.push(evt.cumulative_twr * 100.0);
            cum_mwrs.push(evt.cumulative_mwr.map(|v| v * 100.0).unwrap_or(f64::NAN));

            for (i, trades) in asset_trades.iter_mut().enumerate() {
                let trade = evt.trade_amounts.get(i).copied().unwrap_or(0.0);
                trades.push(trade);
            }
        }

        // Build DataFrame columns
        let mut columns: Vec<Column> = Vec::new();
        columns.push(Column::new("Date".into(), dates));
        columns.push(Column::new("Event".into(), event_types));
        columns.push(Column::new("Portfolio Value ($)".into(), portfolio_values));
        columns.push(Column::new("Turnover (%)".into(), turnovers));
        columns.push(Column::new("Cash Flow ($)".into(), cash_flows));

        for (i, sym) in symbols.iter().enumerate() {
            let col_name = format!("{} Trade ($)", sym);
            columns.push(Column::new(col_name.into(), asset_trades[i].clone()));
        }

        columns.push(Column::new("Cumulative TWR (%)".into(), cum_twrs));
        columns.push(Column::new("Cumulative MWR (%)".into(), cum_mwrs));

        let df = DataFrame::new(columns)?;

        // Build a custom format for rendering in DataTables
        let last_two_start = 5 + num_assets;
        let asset_targets: Vec<String> = (5..5 + num_assets).map(|i| i.to_string()).collect();
        let asset_targets_str = asset_targets.join(", ");
        let pct_targets = format!("3, {}, {}", last_two_start, last_two_start + 1);

        let custom_fmt = format!(
            r#"
[
    {{
        "targets": [0, 1],
        "render": function(data) {{ return data; }}
    }},
    {{
        "targets": [2, 4, {asset_targets}],
        "render": function(data) {{
            if (data == null || data === '') return '';
            try {{
                let parsed = parseFloat(data);
                if (isNaN(parsed)) return data;
                if (!isFinite(parsed)) return parsed > 0 ? '∞' : '-∞';
                let sign = parsed >= 0 ? '' : '-';
                let abs = Math.abs(parsed);
                return sign + '$' + $.fn.dataTable.render.number(',', '.', 2).display(abs);
            }} catch (e) {{ return data; }}
        }}
    }},
    {{
        "targets": [{pct_targets}],
        "render": function(data) {{
            if (data == null || data === '') return '';
            try {{
                let parsed = parseFloat(data);
                if (isNaN(parsed)) return '—';
                if (!isFinite(parsed)) return parsed > 0 ? '∞%' : '-∞%';
                return $.fn.dataTable.render.number(',', '.', 2).display(parsed) + '%';
            }} catch (e) {{ return data; }}
        }}
    }}
]
"#,
            asset_targets = asset_targets_str,
            pct_targets = pct_targets
        );

        let table = df.to_datatable(
            "transaction_history",
            true,
            DataTableFormat::Custom(custom_fmt),
        );
        Ok(Some(table))
    }
}
