use std::error::Error;
use crate::data::defi::{get_balances, get_pool_history, get_pools};
use polars::prelude::*;
use charming::{component::Legend, element::ItemStyle, series::{Pie, PieRoseType}, Chart, HtmlRenderer, ImageRenderer};
use charming::component::{Axis, DataZoom, DataZoomType, Title};
use charming::element::{AreaStyle, AxisType, Color, ColorStop, Label, LabelPosition, LineStyle, Symbol, Tooltip, Trigger};
use charming::series::{Bar, Line};
use num_format::{Locale, ToFormattedString};


pub struct DefiBalances {
    pub protocols: Vec<String>,
    pub chains: Vec<String>,
    pub address: String,
    pub balances: DataFrame,
}

impl DefiBalances {
    /// Fetches the user's balances for the specified protocols and chains
    ///
    /// # Dependencies
    /// This function requires node.js and pnpm to be installed on the system
    /// for macos: brew install node && npm install -g pnpm
    /// for ubuntu: sudo apt install nodejs && npm install -g pnpm
    /// for windows: https://nodejs.org/en/download/ && npm install -g pnpm
    ///
    /// # Arguments
    ///
    /// * `protocols` - Vector of protocols
    /// * `chains` - Vector of chains
    /// * `address` - User's wallet address
    ///
    /// # Returns
    ///
    /// * `DefiBalances` - DefiBalances struct
    ///
    /// # Example
    ///
    /// ```
    /// /*
    /// use finalytics::charts::defi::DefiBalances;
    ///
    /// fn main() {
    ///     let protocols = vec!["wallet".to_string(), "eigenlayer".to_string(), "gearbox".to_string(),
    ///                          "uniswap-v3".to_string(), "ether.fi".to_string(),];
    ///     let chains = vec!["ethereum".to_string(), "arbitrum".to_string()];
    ///     let address = "0x7ac34681f6aaeb691e150c43ee494177c0e2c183".to_string();
    ///     let balances_struct = DefiBalances::new(protocols, chains, address).unwrap();
    ///     println!("{:?}", balances_struct.balances);
    ///     let _ = balances_struct.display_protocols_balance("html", "protocols_balances.html");
    ///     let _ = balances_struct.display_wallet_balance("html", "wallet_balances.html");
    /// }
    /// */
    /// ```
    pub fn new(protocols: Vec<String>, chains: Vec<String>, address: String) -> Result<Self, Box<dyn Error>> {
        let balances = get_balances(protocols.clone(), chains.clone(), &address)?;
        Ok(Self {
            protocols,
            chains,
            address,
            balances,
        })
    }

    /// Displays Pie Chart of the user's protocol balances
    ///
    /// # Arguments
    ///
    /// * `display_format` - Display format (html or svg)
    /// * `file_path` - File path to save the chart
    pub fn display_protocols_balance(&self, display_format: &str, file_path: &str) -> Result<(), Box<dyn Error>> {

        let mask = self.balances.column("category")?.utf8()?
            .into_iter().map(|x| x.expect("category") != "wallet").collect::<BooleanChunked>();
        let protocols_df = self.balances.clone().filter(&mask)?;
        println!("{:?}", &protocols_df);

        let total_balance_mask = protocols_df.clone().column("category").unwrap().utf8().unwrap()
            .into_iter().map(|x| x.expect("category") != "borrow").collect::<BooleanChunked>();
        let total_balance = protocols_df.clone().filter(&total_balance_mask)?.column("balance_usd")?.sum::<f64>()
            .ok_or("Failed to parse balance to float")?;
        let total_balance_str = format!("${}", (total_balance as i64).to_formatted_string(&Locale::en));

        let total_debt_mask = protocols_df.clone().column("category")?.utf8()?
            .into_iter().map(|x| x.expect("category") == "borrow").collect::<BooleanChunked>();
        let total_debt = protocols_df.clone().filter(&total_debt_mask)?.column("balance_usd")?.sum::<f64>()
            .ok_or("Failed to parse balance to float")?;
        let total_debt_str = format!("${}", (total_debt as i64).to_formatted_string(&Locale::en));

        let net_worth = total_balance - total_debt;
        let net_worth_str = format!("${}", (net_worth as i64).to_formatted_string(&Locale::en));

        let protocols_df = protocols_df.lazy()
            .group_by_stable([col("protocol")])
            .agg([
                col("balance_usd").sum().alias("protocol_balance"),
            ]).collect()?;
        println!("{:?}", &protocols_df);

        let protocols_vec =  protocols_df.clone().column("protocol")?
            .utf8().unwrap().into_iter().map(|x| x.expect("protocol").to_string()).collect::<Vec<String>>();
        let balances_vec = protocols_df.clone().column("protocol_balance")?
            .f64()?.into_iter().map(|x| x.expect("protocol_balance")).collect::<Vec<f64>>();

        let mut balances_tuples: Vec<(f64, String)> = Vec::new();
        let mut others = 0.0;

        for (balance, protocol) in balances_vec.iter().zip(protocols_vec.iter()) {
            if *balance > 1000.0 {
                balances_tuples.push((*balance, protocol.to_string()));
            } else {
                others += balance;
            }
        }

        if others > 0.0 {
            balances_tuples.push((others.round(), "Others".to_string()));
        }

        let chart = Chart::new()
            .title(Title::new().text(format!("PROTOCOLS BALANCE\n\nNet Worth: {}  Total Balance: {}  Total Debt: {}",
            net_worth_str, total_balance_str, total_debt_str)).left("center"))
            .legend(Legend::new().top("bottom"))
            .series(
                Pie::new()
                    .name("Protocol Balances")
                    .rose_type(PieRoseType::Radius)
                    .radius(vec!["50", "250"])
                    .center(vec!["50%", "50%"])
                    .item_style(ItemStyle::new().border_radius(8))
                    .label(Label::new().show(true).formatter("{b}: ${c} ({d}%)"))
                    .data(balances_tuples),
            );

        match display_format {
            "html" => {
                let mut renderer = HtmlRenderer::new("DEFI BALANCES".to_string(), 1000, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            "svg" => {
                let mut renderer = ImageRenderer::new(1000, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            _ => {
                println!("Invalid display format. Please choose from html or svg.")
            },
        }


        Ok(())
    }

    /// Displays Pie Chart of the user's wallet balances
    ///
    /// # Arguments
    ///
    /// * `display_format` - Display format (html or svg)
    /// * `file_path` - File path to save the chart
    pub fn display_wallet_balance(&self, display_format: &str, file_path: &str) -> Result<(), Box<dyn Error>> {

        let mask = self.balances.clone().column("category")?.utf8()?
            .into_iter().map(|x| x.expect("category") == "wallet").collect::<BooleanChunked>();
        let wallet_df = self.balances.clone().filter(&mask)?;

        let total_balance = wallet_df.clone().column("balance_usd")?.sum::<f64>().ok_or("Failed to parse balance to float")?;
        let total_balance = format!("${}", (total_balance as i64).to_formatted_string(&Locale::en));

        let symbols_vec =  wallet_df.clone().column("symbol")?
            .utf8()?.into_iter().map(|x| x.expect("symbol").to_string()).collect::<Vec<String>>();
        let chains_vec =  wallet_df.clone().column("chain")?
            .utf8()?.into_iter().map(|x| x.expect("chain").to_string()).collect::<Vec<String>>();
        let balances_vec = wallet_df.clone().column("balance_usd")?
            .f64()?.into_iter().map(|x| x.expect("balance_usd")).collect::<Vec<f64>>();


        let mut balances_tuples: Vec<(f64, String)> = Vec::new();
        let mut others = 0.0;

        for (&balance, (symbol, chain)) in balances_vec.iter().zip(symbols_vec.iter().zip(chains_vec.iter())) {

            if balance > 1000.0 {
                balances_tuples.push((balance, format!("{} ({})", symbol, chain)));
            } else {
                others += balance;
            }
        }

        if others > 0.0 {
            balances_tuples.push((others.round(), "Others".to_string()));
        }

        let chart = Chart::new()
            .title(Title::new().text(format!("WALLET BALANCE\n\nTotal Balance: {}", total_balance)).left("center"))
            .legend(Legend::new().top("bottom"))
            .series(
                Pie::new()
                    .name("Wallet Balances")
                    .rose_type(PieRoseType::Radius)
                    .radius(vec!["50", "250"])
                    .center(vec!["50%", "50%"])
                    .item_style(ItemStyle::new().border_radius(8))
                    .label(
                        Label::new().show(true).formatter("{b}: ${c} ({d}%)")
                    )
                    .data(balances_tuples),
            );

        match display_format {
            "html" => {
                let mut renderer = HtmlRenderer::new("WALLET BALANCE".to_string(), 1200, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            "svg" => {
                let mut renderer = ImageRenderer::new(1200, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            _ => {
                println!("Invalid display format. Please choose from html or svg.")
            },
        }


        Ok(())
    }

}

#[derive(Debug, Clone)]
pub struct DefiPools {
    pub pools_data: DataFrame,
    pub unique_pools: Vec<String>,
    pub unique_protocols: Vec<String>,
    pub unique_chains: Vec<String>,
    pub no_il_pools: Vec<String>,
    pub stable_coin_pools: Vec<String>,
    pub total_value_locked: f64,
}


impl DefiPools {
    /// Get all Defi Yield Pools from the defillama api
    ///
    /// # Returns
    ///
    /// * `DefiPools` - DefiPools struct
    ///
    /// # Example
    ///
    /// ```
    /// use finalytics::charts::defi::DefiPools;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let pools = DefiPools::new().await.unwrap();
    /// println!("Total Value Locked: ${:.2}", pools.total_value_locked);
    /// println!("{:?}", pools.pools_data);
    /// }
    /// ```
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let pools_data = get_pools().await?;
        let unique_pools = pools_data.clone().column("pool")?.utf8()?.unique()?
            .into_iter().map(|x| x.expect("pool").to_string()).collect::<Vec<String>>();
        let unique_protocols = pools_data.clone().column("protocol")?.utf8()?.unique()?
            .into_iter().map(|x| x.expect("protocol").to_string()).collect::<Vec<String>>();
        let unique_chains = pools_data.clone().column("chain")?.utf8()?.unique()?
            .into_iter().map(|x| x.expect("chain").to_string()).collect::<Vec<String>>();
        let no_il_mask = !pools_data.clone().column("il_risk")?.bool()?;
        let no_il_pools = pools_data.clone().filter(&no_il_mask)?.column("pool")?.utf8()?
            .into_iter().map(|x| x.expect("pool").to_string()).collect::<Vec<String>>();
        let stable_coin_mask = pools_data.clone().column("stable_coin")?.bool()?
            .into_iter().map(|x| x.expect("stable_coin")).collect::<BooleanChunked>();
        let stable_coin_pools = pools_data.clone().filter(&stable_coin_mask)?.column("pool")?.utf8()?
            .into_iter().map(|x| x.expect("pool").to_string()).collect::<Vec<String>>();
        let total_value_locked = pools_data.clone().column("tvl_usd")?.sum::<f64>().ok_or("Failed to parse tvl to float")?;

        Ok(Self {
            pools_data,
            unique_pools,
            unique_protocols,
            unique_chains,
            no_il_pools,
            stable_coin_pools,
            total_value_locked,
        })
    }

    /// Search for list of pools matching the specified query symbol
    ///
    /// # Arguments
    ///
    /// * `query` - Search query
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - Vector of pool symbols
    ///
    /// # Example
    ///
    /// ```
    /// use finalytics::charts::defi::DefiPools;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let pools = DefiPools::new().await.unwrap();
    /// let results = pools.search_pools("USDC");
    /// println!("{:?}", results);
    /// }
    /// ```
    pub fn search_pools(&self, query: &str) -> Vec<String> {
        let pools = self.unique_pools.clone();
        let mut results: Vec<String> = Vec::new();
        for pool in pools {
            if pool.to_lowercase().contains(&query.to_lowercase()) {
                results.push(pool);
            }
        }
        results
    }

    /// display a bar chart of the top protocols of a given liguidity pool by total value locked
    ///
    /// # Arguments
    ///
    /// * 'symbol' - Pool symbol
    /// * `n` - Number of protocols to display
    /// * `display_format` - Display format (html or svg)
    /// * `file_path` - File path to save the chart
    ///
    /// # Example
    ///
    /// ```
    /// use finalytics::charts::defi::DefiPools;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let pools = DefiPools::new().await.unwrap();
    /// let _ = pools.display_top_protocols_by_tvl("USDC-USDT", 20, "html", "top_tvl_pools.html");
    /// }
    /// ```
    pub fn display_top_protocols_by_tvl(&self, symbol: &str, n: usize, display_format: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mask = self.pools_data.clone().column("pool")?.utf8()?
            .into_iter().map(|x| x.expect("pool") == symbol).collect::<BooleanChunked>();
        let pools_df = self.pools_data.clone().filter(&mask)?;
        let pools_df = pools_df.clone().sort(vec!["tvl_usd"], vec![true], false).unwrap();
        let top_pools_df = pools_df.head(Some(n));

        // create a Vec<string> of the top pools containing protocol-chain
        let top_protocols_vec = top_pools_df.clone().column("protocol")?.utf8()?
            .into_iter().map(|x| x.expect("protocol").to_string()).collect::<Vec<String>>();
        let top_chains_vec = top_pools_df.clone().column("chain")?.utf8()?
            .into_iter().map(|x| x.expect("chain").to_string()).collect::<Vec<String>>();
        let top_pools_vec = top_protocols_vec.iter().zip(top_chains_vec.iter()).map(
            |(protocol, chain)| format!("{} ({})", protocol, chain)).collect::<Vec<String>>();

        // create a Vec<f64> of the top pools' tvl
        let top_pools_tvl_vec = top_pools_df.clone().column("tvl_usd")?.f64()?
            .into_iter().map(|x| x.expect("tvl_usd")).collect::<Vec<f64>>();

        let chart = Chart::new()
            .tooltip(Tooltip::new().trigger(Trigger::Axis))
            .title(Title::new().text(format!("TOP {} POOLS BY TVL", symbol)).left("center"))
            .x_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(top_pools_tvl_vec)
                .label(Label::new().show(true).position(LabelPosition::Outside).formatter("${c}")))
            .y_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(top_pools_vec),
            );

        match display_format {
            "html" => {
                let mut renderer = HtmlRenderer::new("Top Pools By TVL".to_string(), 1600, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            "svg" => {
                let mut renderer = ImageRenderer::new(1600, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            _ => {
                println!("Invalid display format. Please choose from html or svg.")
            },
        }

        Ok(())

    }

    /// display a bar chart of the top protocols of a given liquidity pool by 30d mean apy
    ///
    /// # Arguments
    ///
    /// * 'symbol' - Pool symbol
    /// * `n` - Number of protocols to display
    /// * `display_format` - Display format (html or svg)
    /// * `file_path` - File path to save the chart
    ///
    /// # Example
    ///
    /// ```
    /// use finalytics::charts::defi::DefiPools;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let pools = DefiPools::new().await.unwrap();
    /// let _ = pools.display_top_protocols_by_apy("USDC-USDT", 20, "html", "top_apy_pools.html");
    /// }
    /// ```
    pub fn display_top_protocols_by_apy(&self, symbol: &str, n: usize, display_format: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mask = self.pools_data.clone().column("pool")?.utf8()?
            .into_iter().map(|x| x.expect("pool") == symbol).collect::<BooleanChunked>();
        let pools_df = self.pools_data.clone().filter(&mask)?;
        let pools_df = pools_df.clone().sort(vec!["apy_mean_30d"], vec![true], false)?;
        let top_pools_df = pools_df.head(Some(n));

        // create a Vec<string> of the top pools containing protocol-chain
        let top_protocols_vec = top_pools_df.clone().column("protocol")?.utf8()?
            .into_iter().map(|x| x.expect("protocol").to_string()).collect::<Vec<String>>();
        let top_chains_vec = top_pools_df.clone().column("chain")?.utf8()?
            .into_iter().map(|x| x.expect("chain").to_string()).collect::<Vec<String>>();
        let top_pools_vec = top_protocols_vec.iter().zip(top_chains_vec.iter()).map(
            |(protocol, chain)| format!("{} ({})", protocol, chain)).collect::<Vec<String>>();

        // create a Vec<f64> of the top pools' tvl
        let top_pools_apy_vec = top_pools_df.clone().column("apy_mean_30d")?.f64()?
            .into_iter().map(|x| x.expect("apy_mean_30d").round()).collect::<Vec<f64>>();

        let chart = Chart::new()
            .tooltip(Tooltip::new().trigger(Trigger::Axis))
            .title(Title::new().text(format!("TOP {} POOLS BY 30D MEAN APY", symbol)).left("center"))
            .x_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(top_pools_apy_vec)
                .label(Label::new().show(true).position(LabelPosition::Outside).formatter("{c}%")))
            .y_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(top_pools_vec),
            );

        match display_format {
            "html" => {
                let mut renderer = HtmlRenderer::new(format!("TOP {} POOLS BY 30D MEAN APY", symbol), 1600, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            "svg" => {
                let mut renderer = ImageRenderer::new(1600, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            _ => {
                println!("Invalid display format. Please choose from html or svg.")
            },
        }

        Ok(())
    }

    /// display an area chart of the total value locked history of a given liquidity pool
    ///
    /// # Arguments
    ///
    /// * 'symbol' - Pool symbol
    /// * 'protocol' - Pool protocol
    /// * 'chain' - Pool chain
    /// * `display_format` - Display format (html or svg)
    /// * `file_path` - File path to save the chart
    ///
    /// # Example
    ///
    /// ```
    /// use finalytics::charts::defi::DefiPools;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let pools = DefiPools::new().await.unwrap();
    /// let _ = pools.display_pool_tvl_history("USDC-USDT", "uniswap-v3", "ethereum", "html", "pool_tvl_history.html").await;
    /// }
    /// ```
    pub async fn display_pool_tvl_history(&self, symbol: &str, protocol: &str, chain: &str, display_format: &str,
                                          file_path: &str) -> Result<(), Box<dyn Error>> {
        let mask1 = self.pools_data.clone().column("pool")?.utf8()?
            .into_iter().map(|x| x.expect("pool").to_lowercase() == symbol.to_lowercase()).collect::<BooleanChunked>();
        let mask2 = self.pools_data.clone().column("protocol")?.utf8()?
            .into_iter().map(|x| x.expect("protocol").to_lowercase() == protocol.to_lowercase()).collect::<BooleanChunked>();
        let mask3 = self.pools_data.clone().column("chain")?.utf8()?
            .into_iter().map(|x| x.expect("chain").to_lowercase() == chain.to_lowercase()).collect::<BooleanChunked>();
        let mask = mask1 & mask2 & mask3;
        let pool_id = self.pools_data.clone().filter(&mask)?.column("pool_id")?.get(0)?.to_string().replace("\"", "");
        let pool_history = get_pool_history(&pool_id).await?;
        let dates = pool_history.clone().column("date")?.utf8()?
            .into_iter().map(|x| x.expect("date").to_string().split("T").collect::<Vec<&str>>()[0].to_string())
            .collect::<Vec<String>>();
        let tvl = pool_history.clone().column("tvl_usd")?.f64()?
            .into_iter().map(|x| x.expect("tvl_usd")).collect::<Vec<f64>>();

        let chart = Chart::new()
            .tooltip(Tooltip::new().trigger(Trigger::Axis))
            .title(Title::new().left("center").text(format!("{} {} {} TOTAL VALUE LOCKED", symbol, protocol.to_uppercase(),
                                                            chain.to_uppercase())))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .boundary_gap(false)
                    .data(dates),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .data_zoom(DataZoom::new().type_(DataZoomType::Inside).start(0).end(5000))
            .data_zoom(DataZoom::new().start(0).end(5000))
            .series(
                Line::new()
                    .name("TVL")
                    .symbol(Symbol::None)
                    .line_style(LineStyle::new().color("rgb(255, 70, 131"))
                    .area_style(AreaStyle::new().color(Color::LinearGradient {
                        x: 0.,
                        y: 0.,
                        x2: 0.,
                        y2: 1.,
                        color_stops: vec![
                            ColorStop::new(0, "rgb(255, 158, 68)"),
                            ColorStop::new(1, "rgb(255, 70, 131)"),
                        ],
                    }))
                    .data(tvl),
            );

        match display_format {
            "html" => {
                let mut renderer = HtmlRenderer::new(format!("{}-{}-{} Total Value Locked", symbol, protocol, chain), 1600, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            "svg" => {
                let mut renderer = ImageRenderer::new(1600, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            _ => {
                println!("Invalid display format. Please choose from html or svg.")
            },
        }

        Ok(())
    }

    /// display an area chart of the APY history of a given liquidity pool
    ///
    /// # Arguments
    ///
    /// * 'symbol' - Pool symbol
    /// * 'protocol' - Pool protocol
    /// * 'chain' - Pool chain
    /// * `display_format` - Display format (html or svg)
    /// * `file_path` - File path to save the chart
    ///
    /// # Example
    ///
    /// ```
    /// use finalytics::charts::defi::DefiPools;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let pools = DefiPools::new().await.unwrap();
    /// let _ = pools.display_pool_apy_history("USDC-USDT", "solidly-v3", "Ethereum", "html", "pool_apy_history.html").await;
    /// }
    pub async fn display_pool_apy_history(&self, symbol: &str, protocol: &str, chain: &str, display_format: &str,
                                          file_path: &str) -> Result<(), Box<dyn Error>> {
        let mask1 = self.pools_data.clone().column("pool")?.utf8()?
            .into_iter().map(|x| x.expect("pool").to_lowercase() == symbol.to_lowercase()).collect::<BooleanChunked>();
        let mask2 = self.pools_data.clone().column("protocol")?.utf8()?
            .into_iter().map(|x| x.expect("protocol").to_lowercase() == protocol.to_lowercase()).collect::<BooleanChunked>();
        let mask3 = self.pools_data.clone().column("chain")?.utf8()?
            .into_iter().map(|x| x.expect("chain").to_lowercase() == chain.to_lowercase()).collect::<BooleanChunked>();
        let mask = mask1 & mask2 & mask3;
        let pool_id = self.pools_data.clone().filter(&mask)?.column("pool_id")?.get(0)?.to_string().replace("\"", "");
        let pool_history = get_pool_history(&pool_id).await?;
        let dates = pool_history.clone().column("date")?.utf8()?
            .into_iter().map(|x| x.expect("date").to_string().split("T").collect::<Vec<&str>>()[0].to_string())
            .collect::<Vec<String>>();
        let apy = pool_history.clone().column("apy")?.f64()?
            .into_iter().map(|x| x.expect("apy").round()).collect::<Vec<f64>>();

        let chart = Chart::new()
            .tooltip(Tooltip::new().trigger(Trigger::Axis))
            .title(Title::new().left("center").text(format!("{} {} {} ANNUAL PERCENTAGE YIELD", symbol, protocol.to_uppercase(),
                                                            chain.to_uppercase())))
            .x_axis(Axis::new().type_(AxisType::Category).data(dates))
            .y_axis(Axis::new().type_(AxisType::Value))
            .data_zoom(DataZoom::new().type_(DataZoomType::Inside).start(0).end(5000))
            .data_zoom(DataZoom::new().start(0).end(5000))
            .series(Bar::new().data(apy)
            //.label(Label::new().show(true).position(LabelPosition::Outside).formatter("{c}%"))
            );

        match display_format {
            "html" => {
                let mut renderer = HtmlRenderer::new(format!("{}-{}-{} APY", symbol, protocol, chain), 1600, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            "svg" => {
                let mut renderer = ImageRenderer::new(1600, 800);
                renderer.save(&chart, file_path).expect("failed to save chart");
                println!("Chart saved to {}", file_path)
            },
            _ => {
                println!("Invalid display format. Please choose from html or svg.")
            },
        }

        Ok(())
    }
}

