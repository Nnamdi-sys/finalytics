use std::error::Error;
use polars::prelude::*;
use plotly::{common::{Title}, layout::{Axis, Layout, LayoutScene}, Plot, Scatter, Surface};
use plotly::common::{Line, LineShape, Mode};
use crate::analytics::statistics::linear_interpolation;
use crate::analytics::stochastics::{implied_volatility_bisection, OptionType};
use crate::data::ticker::Ticker;

#[allow(dead_code)]
#[derive(Debug)]
pub struct OptionCharts {
    ticker_symbol: String,
    risk_free_rate: f64,
    ticker_price: f64,
    expiration_dates: Vec<String>,
    ttms: Vec<f64>,
    strikes: Vec<f64>,
    ivols: Vec<Vec<f64>>,
}

impl OptionCharts {
    /// Creates a new OptionCharts struct
    ///
    /// # Arguments
    ///
    /// * `symbol` - Ticker symbol
    /// * `risk_free_rate` - Risk-free rate of return in decimal (e.g 0.02 for 2%)
    ///
    /// # Returns
    ///
    /// * `OptionCharts` struct
    pub async fn new(symbol: &str, risk_free_rate: f64) -> Result<OptionCharts, Box<dyn Error>> {
        let options_chain = Ticker::new(symbol).await?.get_options().await?;
        let ticker_price = options_chain.ticker_price;
        let expiration_dates = options_chain.expiration_dates;
        let ttms = options_chain.ttms.iter().filter(|x| *x >= &3.0).map(|x| *x).collect::<Vec<f64>>();
        let strikes = options_chain.strikes;
        let df = options_chain.chain;
        let atm_ser = Series::new("inTheMoney", vec![false; df.height()]);
        let mask = df.column("inTheMoney")?.equal(&atm_ser)?;
        let df = df.filter(&mask)?;
        let mut ivols: Vec<Vec<f64>> = Vec::new();

        for x in &ttms {
            let mut vols: Vec<f64> = Vec::new();
            for y in &strikes {
                let mask1 = df.column("ttm")?.equal(*x)?;
                let mask2 = df.column("strike")?.equal(*y)?;
                let mask = mask1 & mask2;
                let vol_df = df.filter(&mask)?;
                if vol_df.height() > 0 {
                    let option_price = vol_df.column("lastPrice")?.f64()?.get(0).unwrap();
                    let option_type = vol_df.column("type")?.utf8()?.get(0).unwrap();
                    let option_type = match option_type {
                        "call" => OptionType::Call,
                        "put" => OptionType::Put,
                        _ => panic!("Invalid option type")
                    };
                    let vol = implied_volatility_bisection(option_price, ticker_price, *y, *x / 12.0,
                                                           risk_free_rate, option_type);
                    vols.push(vol);
                } else {
                    vols.push(0.0);
                }
            }
            let vols_adj = linear_interpolation(vols);
            ivols.push(vols_adj);
        }

        Ok(OptionCharts {
            ticker_symbol: symbol.to_string(),
            risk_free_rate,
            ticker_price,
            expiration_dates,
            ttms,
            strikes,
            ivols,
        })
    }

    /// Generates a volatility surface for the option
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub fn volatility_surface(&self) -> Plot {
        let symbol = self.ticker_symbol.clone();
        let ivols = self.ivols.clone();
        let strikes = self.strikes.clone();
        let ttms = self.ttms.clone();
        let trace = Surface::new(ivols).x(strikes).y(ttms);
        let mut plot = Plot::new();
        plot.add_trace(trace);

        let layout = Layout::new()
            .height(800)
            .width(1200)
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{symbol} Volatility Surface</span>")))
            .scene(
                LayoutScene::new()
                    .x_axis(
                        Axis::new()
                            .title(Title::from("Strike"))
                    )
                    .y_axis(
                        Axis::new()
                            .title(Title::from("Time to Maturity"))
                    )
                    .z_axis(Axis::new()
                        .title(Title::from("Implied Volatility")))

            );
        plot.set_layout(layout);

        plot
    }

    /// Generates the volatility smile curve for each expiration date
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub fn volatility_smile(&self) -> Plot {
        let symbol = self.ticker_symbol.clone();
        let strikes = self.strikes.clone();
        let mut traces = Vec::new();

        for (index, ttm) in self.ttms.iter().enumerate() {
            let ivols = self.ivols[index].clone();
            let trace = Scatter::new(strikes.clone(), ivols)
                .mode(Mode::LinesMarkers)
                .line(Line::new().shape(LineShape::Spline))
                .name(&*format!("Volatility Smile - {:.1} Months Expiration", ttm));

            traces.push(trace);
        }

        let layout = Layout::new()
            .height(800)
            .width(1200)
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{symbol} Volatility Smile</span>")))
            .x_axis(Axis::new().title(Title::from("Strike")))
            .y_axis(Axis::new().title(Title::from("Implied Volatility")));

        let mut plot = Plot::new();
        for trace in traces {
            plot.add_trace(trace);
        }
        plot.set_layout(layout);
        plot
    }

    /// Generates the volatility term structure curve for each strike
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// * `Plot` Plotly Chart struct
    pub fn volatility_cone(&self) -> Plot {
        let symbol = self.ticker_symbol.clone();
        let ttms = self.ttms.clone();
        let ivols = self.ivols.clone();
        let rows = ivols[0].len();
        let cols = ivols.len();
        let mut strike_vols: Vec<Vec<f64>>= vec![vec![Default::default(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                strike_vols[i][j] = ivols[j][i].clone();
            }
        }
        let mut traces = Vec::new();


        for (index, strike) in self.strikes.iter().enumerate() {
            let ivols = strike_vols[index].clone();
            let trace = Scatter::new(ttms.clone(), ivols)
                .mode(Mode::LinesMarkers)
                .line(Line::new().shape(LineShape::Spline))
                .name(&*format!("Volatility Cone - {} Strike", strike));

            traces.push(trace);
        }

        let layout = Layout::new()
            .height(800)
            .width(1200)
            .title(Title::from(&*format!("<span style=\"font-weight:bold; color:darkgreen;\">{symbol} Volatility Term Structure</span>")))
            .x_axis(Axis::new().title(Title::from("Time to Maturity (Months)")))
            .y_axis(Axis::new().title(Title::from("Implied Volatility")));

        let mut plot = Plot::new();
        for trace in traces {
            plot.add_trace(trace);
        }
        plot.set_layout(layout);
        plot
    }
}



