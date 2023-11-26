use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};
use polars::prelude::*;
use temp_dir::TempDir;
use regex::Regex;
use serde_json::Value;


/// Get the list of protocols and chains supported by the llamafolio-api for retrieving user balances
///
/// # Returns
///
/// * `HashMap<String, Vec<String>>` - A HashMap containing the protocols and chains supported by the llamafolio-api
///
/// # Example
///
/// ```
/// use finalytics::data::defi::get_protocols;
///
/// let result = get_protocols();
/// println!("{:?}", result);
/// ```
pub fn get_protocols() -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    // Create a temporary directory to clone the repository
    let tmp_dir = TempDir::new()?;
    let llamafolio_api_path = tmp_dir.path().join("llamafolio-api");

    // Clone the repository from GitHub
    let status = Command::new("git")
        .args(&["clone", "https://github.com/llamafolio/llamafolio-api.git"])
        .arg(&llamafolio_api_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err("Failed to clone repository".into());
    }

    // Path to the adapters directory
    let adapters_path = format!("{}/src/adapters", llamafolio_api_path.display());

    // Read the names of all directories in src/adapters
    let adapters = fs::read_dir(&adapters_path)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_dir() {
                    Some(e.file_name().to_string_lossy().into_owned())
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>()
        .join(",");

    let chains_map: HashMap<String, Vec<String>> = adapters
        .split(',')
        .map(|adapter| {
            let chains_vec = fs::read_dir(format!("{}/{}", adapters_path, adapter))
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(|entry| {
                            entry.ok().and_then(|e| {
                                if e.path().is_dir() {
                                    Some(e.file_name().to_string_lossy().into_owned())
                                } else {
                                    None
                                }
                            })
                        })
                        .collect::<Vec<String>>() // Use HashSet to ensure uniqueness
                })
                .unwrap_or_else(Vec::new);

            (adapter.to_string(), chains_vec)
        })
        .collect();

    Ok(chains_map)
}


/// Fetch all user balances for the given protocols and wallet address
/// To retrieve unstaked balances in wallet ensure to include 'wallet' in the protocols list
///
/// # Arguments
///
/// * `protocols` - A vector of protocols to retrieve balances from
/// * `chains` - A vector of chains to retrieve balances from
/// * `address` - The wallet address to retrieve balances for
///
/// # Returns
///
/// * `DataFrame` - A DataFrame containing the balances for the given protocols and wallet address
///
/// # Dependencies
/// This function requires node.js and pnpm to be installed on the system
/// for macos: brew install node && npm install -g pnpm
/// for ubuntu: sudo apt install nodejs && npm install -g pnpm
/// for windows: https://nodejs.org/en/download/ && npm install -g pnpm
///
/// # Example
///
/// ```
/// /*
/// let protocols = vec!["wallet".to_string(), "eigenlayer".to_string(), "gearbox".to_string(),
///                          "uniswap-v3".to_string(), "ether.fi".to_string(),];
/// let chains = vec!["ethereum".to_string(), "arbitrum".to_string()];
/// let address = "0x7ac34681f6aaeb691e150c43ee494177c0e2c183".to_string();
/// let balances_struct = DefiBalances::new(protocols, chains, address).unwrap();
/// println!("{:?}", balances_struct.balances);
/// let _ = balances_struct.display_protocols_balance("html", "protocols_balances.html");
/// let _ = balances_struct.display_wallet_balance("html", "wallet_balances.html");
/// */
/// ```
pub fn get_balances(protocols: Vec<String>, chains: Vec<String>, address: &str) -> Result<DataFrame, Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    // Create a temporary directory to clone the repository
    let tmp_dir = TempDir::new()?;
    let llamafolio_api_path = tmp_dir.path().join("llamafolio-api");

    // Clone the repository from GitHub
    let status = Command::new("git")
        .args(&["clone", "https://github.com/llamafolio/llamafolio-api.git"])
        .arg(&llamafolio_api_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err("Failed to clone repository".into());
    }

    // Set the current working directory to the llamafolio-api path
    std::env::set_current_dir(&llamafolio_api_path)?;

    // Install dependencies
    let status = Command::new("pnpm")
        .arg("install")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        return Err("Failed to install dependencies. Please install node.js and pnpm manually\n
        for macos: brew install node && npm install -g pnpm\n
        for ubuntu: sudo apt install nodejs && npm install -g pnpm\n
        for windows: https://nodejs.org/en/download/ && npm install -g pnpm\n
        ".into());
    }

    // Run the build script
    let status = Command::new("pnpm")
        .arg("run")
        .arg("build")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        return Err("Failed to build the project".into());
    }

    // Path to the adapters directory
    let adapters_path = format!("{}/src/adapters", llamafolio_api_path.display());

    // Read the names of all directories in src/adapters
    let adapters = fs::read_dir(&adapters_path)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_dir() {
                    Some(e.file_name().to_string_lossy().into_owned())
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>()
        .join(",");

    let mut chains_map: HashMap<String, Vec<String>> = adapters
        .split(',')
        .map(|adapter| {
            let chains_vec = fs::read_dir(format!("{}/{}", adapters_path, adapter))
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(|entry| {
                            entry.ok().and_then(|e| {
                                if e.path().is_dir() {
                                    Some(e.file_name().to_string_lossy().into_owned())
                                } else {
                                    None
                                }
                            })
                        })
                        .collect::<Vec<String>>() // Use HashSet to ensure uniqueness
                })
                .unwrap_or_else(Vec::new);

            (adapter.to_string(), chains_vec)
        })
        .collect();

    if protocols.contains(&"wallet".to_string()) {
        chains_map.insert("wallet".to_string(), chains.clone());
    }

    // Create a HashMap to store the results
    let mut results_map: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();


    for (protocol, chains_list) in chains_map.iter() {
        if !protocols.contains(&protocol) {
            continue;
        }
        let protocol = protocol.clone();
        let address = address.to_string();


        for chain in chains_list.iter().cloned() {
            if !chains.contains(&chain) {
                continue;
            }
            if let Ok(output) = run_adapter(&protocol, &chain, &address) {
                if !output.is_empty() && output.contains("Found")
                    && !output.contains("Found 0 non zero balances")
                    && !output.contains("Failed to run adapter") {
                    println!("Found Output for {}-{}", protocol, chain);
                    println!("{}", &output);
                    let key = format!("{}-{}", protocol, chain);
                    results_map.insert(key, extract_table_data(&output)?);
                }
            } else {
                eprintln!("Error for {}-{}", protocol, chain);
            }
        }
    }

    // Delete the temporary directory
    tmp_dir.cleanup()?;

    // Set the working directory back to its previous state
    std::env::set_current_dir(current_dir)?;

    let mut final_df = DataFrame::default();

    for (k, v) in results_map.iter(){
        let df = DataFrame::new(vec![
            Series::new("protocol", v.iter().map(|_| k.clone()).collect::<Vec<String>>()),
            Series::new("chain", v.iter().map(|x|
                x["chain"].clone().replace("'", "")
            ).collect::<Vec<String>>()),
            Series::new("category", v.iter().map(|x|
                x["category"].clone().replace("'", "")
            ).collect::<Vec<String>>()),
            Series::new("symbol", v.iter().map(|x|
                x["symbol"].clone().replace("'", "")
            ).collect::<Vec<String>>()),
            Series::new("address", v.iter().map(|x|
                x["address"].clone().replace("'", "")
            ).collect::<Vec<String>>()),
            Series::new("balance", v.iter().map(|x|
                parse_monetary_value(&x["balance"].clone())
                    .expect(&format!("Failed to parse balance string {} to float", &x["balance"]))
            ).collect::<Vec<f64>>()),
            Series::new("balance_usd", v.iter().map(|x|
                parse_monetary_value(&x["balanceUSD"].clone())
                    .expect(&format!("Failed to parse balance string {} to float", &x["balanceUSD"]))
            ).collect::<Vec<f64>>()),
            Series::new("stable", v.iter().map(|x| x["stable"].clone()).collect::<Vec<String>>()),
            Series::new("reward", v.iter().map(|x|
                x.get("reward").unwrap_or(&"".to_string()).clone().replace("'", "")
            ).collect::<Vec<String>>()),
            Series::new("underlying", v.iter().map(|x|
                x.get("underlying").unwrap_or(&"".to_string()).clone().replace("'", "")
            ).collect::<Vec<String>>()),
        ])?;
        final_df = final_df.vstack(&df)?;
    }

    Ok(final_df)

}


/// Get all Defi Yield Pools from the defillama api
///
/// # Returns
///
/// * `DataFrame` - A DataFrame containing the Defi Yield Pools
///
/// # Example
///
/// ```
/// use finalytics::data::defi::get_pools;
///
/// #[tokio::main]
/// async fn main() {
/// let result = get_pools().await;
/// println!("{:?}", result);
/// }
/// ```
pub async fn get_pools() -> Result<DataFrame, Box<dyn Error>> {
    let url = "https://yields.llama.fi/pools";

    let response = reqwest::get(url).await?;

    let data = response.json::<Value>().await?;

    let data = data["data"].as_array().expect("data should be an array");
    let pools = data.iter().map(|x| x["symbol"].as_str().expect("data should contain symbol")).collect::<Vec<&str>>();
    let pool_ids = data.iter().map(|x| x["pool"].as_str().expect("data should contain pool")).collect::<Vec<&str>>();
    let protocols = data.iter().map(|x| x["project"].as_str().expect("data should contains project")).collect::<Vec<&str>>();
    let chains = data.iter().map(|x| x["chain"].as_str().expect("data should contain chain")).collect::<Vec<&str>>();
    let tvl_usd = data.iter().map(|x| x["tvlUsd"].as_f64().expect("data should contain tvlUSD")).collect::<Vec<f64>>();
    let apy_mean_30d = data.iter().map(|x| x["apyMean30d"].as_f64().expect("data should contain apyMean30d")).collect::<Vec<f64>>();
    let il_risk = data.iter().map(|x| if x["ilRisk"].as_str().expect("data should contain ilRisk") =="yes" {true } else {false}).collect::<Vec<bool>>();
    let stable_coin = data.iter().map(|x| x["stablecoin"].as_bool().expect("data should contain stablecoin")).collect::<Vec<bool>>();

    let df = DataFrame::new(vec![
        Series::new("pool", pools),
        Series::new("pool_id", pool_ids),
        Series::new("protocol", protocols),
        Series::new("chain", chains),
        Series::new("tvl_usd", tvl_usd),
        Series::new("apy_mean_30d", apy_mean_30d),
        Series::new("il_risk", il_risk),
        Series::new("stable_coin", stable_coin),
    ])?;

    Ok(df)
}

/// Get the historical APY, TVL and IL for a Defi Yield Pool from the defillama api
///
/// # Arguments
///
/// * `pool_id` - The pool id of the Defi Yield Pool
///
/// # Returns
///
/// * `DataFrame` - A DataFrame containing the historical APY, TVL and IL for the Defi Yield Pool
///
/// # Example
///
/// ```
/// #[tokio::main]
/// async fn main() {
/// use finalytics::data::defi::get_pool_history;
///
/// let result = get_pool_history("747c1d2a-c668-4682-b9f9-296708a3dd90").await;
/// println!("{:?}", result);
/// }
/// ```
pub async fn get_pool_history(pool_id: &str) -> Result<DataFrame, Box<dyn Error>> {
    let url = format!("https://yields.llama.fi/chart/{}", pool_id);
    let response = reqwest::get(&url).await?;
    let data = response.json::<Value>().await?;
    let data = data["data"].as_array().expect("data should be an array");

    let date = data.iter().map(|x| x["timestamp"].as_str().expect("data should contain timestamp")).collect::<Vec<&str>>();
    let tvl_usd = data.iter().map(|x| x["tvlUsd"].as_f64().expect("data should contain tvlUsd")).collect::<Vec<f64>>();
    let apy = data.iter().map(|x| x["apy"].as_f64().expect("data should contain apy")).collect::<Vec<f64>>();
    let il_7d = data.iter().map(|x| x["il7d"].as_f64()).collect::<Vec<Option<f64>>>();

    let df = DataFrame::new(vec![
        Series::new("date", date),
        Series::new("tvl_usd", tvl_usd),
        Series::new("apy", apy),
        Series::new("il_7d", il_7d),
    ])?;

    Ok(df)
}

fn run_adapter(protocol: &str, chain: &str, address: &str) -> Result<String, Box<dyn Error>> {
    // Your existing logic to run the adapter command
    let output = Command::new("pnpm")
        .args(&["run", "adapter", protocol, chain, address])
        .stderr(Stdio::null())
        //.spawn()?
        //.wait_with_output()?;
        .output()?;


    match output.status.success() {
        true => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        false => Err("Failed to get user balances".into()),
    }
}


fn extract_table_data(input: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let mut result = Vec::new();

    // Define regular expression for extracting rows
    let row_regex = Regex::new(r"│(?P<value>[^│]+)")?;

    // Extract rows
    let data_lines: Vec<&str> = input
        .lines()
        .filter(|line| line.contains("│"))
        .collect();

    // Determine the position of the line containing "chain"
    let chain_position = data_lines
        .iter()
        .position(|line| line.contains("chain"))
        .unwrap_or(0);

    // Extract headers dynamically based on the position of "chain"
    let headers: Vec<String> = data_lines[chain_position]
        .split('│')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect();

    // Extract values for each row in the second table starting from "chain"
    for data_line in data_lines.iter().skip(chain_position) {
        let mut values = Vec::new();

        for capture in row_regex.captures_iter(data_line) {
            if let Some(value) = capture.name("value") {
                values.push(value.as_str().trim().to_string());
            }
        }

        // Create a HashMap from headers and values
        let row_data: HashMap<String, String> = headers
            .iter()
            .cloned()
            .zip(values.into_iter())
            .collect();

        if let Some(balance_str) = row_data.get("balanceUSD") {
            if let Some(second_char) = balance_str.chars().nth(1) {
                if second_char != '$' {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            continue;
        }

        result.push(row_data);
    }

    Ok(result)
}


fn parse_monetary_value(value: &str) -> Result<f64, std::num::ParseFloatError> {
    let cleaned_value = value
        .replace(" ", "")
        .replace("'", "")
        .replace('$', "")
        .replace("k", "e3")
        .replace("M", "e6")
        .replace("B", "e9")
        .replace(",", "");

    let result = cleaned_value.parse::<f64>();
    result
}


