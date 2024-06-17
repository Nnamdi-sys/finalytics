use std::sync::Arc;
use finalytics::prelude::*;
use yahoo_finance_symbols::keys::{AssetClass, Category, Exchange};
use yahoo_finance_symbols::get_symbols;
use teloxide::{prelude::*, utils::command::BotCommands, update_listeners::webhooks};
use crate::telegram::utils::html_to_png;

pub async fn telegram_bot() {
    let bot = Bot::from_env();
    let addr = ([0, 0, 0, 0], 8443).into();
    let url = "https://finalytics.rs:8443".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    Command::repl_with_listener(bot, answer, listener).await;
}


fn get_command_description(command: String) -> String {
    match command.as_str() {
        "symbols" => r###"
                    This command helps you search for a ticker symbol.

                    Usage: `/symbols <asset_class> <query>`

                    Example: `/symbols equity microsoft`

                    Parameters:
                    - `<asset_class>`: the asset class of the security (options: equity, etf, index, mutual_fund, currency, crypto)
                    - `<query>`: the query to search for symbols.
                 "###.to_string(),

        "ticker" => r###"
                    This is a command for security analysis.

                    Usage: `/ticker <symbol> <start_date> <end_date> <interval> <benchmark_symbol> <confidence_level> <risk_free_rate> <chart_type>`

                    Example: `/ticker AAPL 2021-01-01 2023-01-01 1d ^GSPC 0.95 0.02 price_charts`

                    Parameters:
                    - `<symbol>`: the ticker symbol of the security to be analyzed (e.g., AAPL)
                    - `<start_date>`: the start date of the analysis period (e.g., 2021-01-01)
                    - `<end_date>`: the end date of the analysis period (e.g., 2023-01-01)
                    - `<interval>`: the interval of the data (options: 1m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo)
                    - `<benchmark_symbol>`: the ticker symbol of the benchmark (e.g., ^GSPC)
                    - `<confidence_level>`: the confidence level of the VaR (e.g., 0.95)
                    - `<risk_free_rate>`: the risk-free rate (e.g., 0.02)
                    - `<chart_type>`: the type of the chart (options: price_charts, performance_charts, financials, options_charts)
                "###.to_string(),

        "portfolio" => r###"
                    This is a command for portfolio optimization.

                    Usage: `/portfolio <symbols> <benchmark_symbol> <start_date> <end_date> <interval> <confidence_level> <risk_free_rate> <max_iterations> <objective_function>`

                    Example: `/portfolio AAPL,GOOG,TSLA,META ^GSPC 2021-01-01 2023-01-01 1d 0.95 0.02 1000 max_sharpe`

                    Parameters:
                    - `<symbols>`: the ticker symbols of the securities to be analyzed (e.g., AAPL,GOOG,TSLA,META)
                    - `<benchmark_symbol>`: the ticker symbol of the benchmark (e.g., ^GSPC)
                    - `<start_date>`: the start date of the analysis period (e.g., 2021-01-01)
                    - `<end_date>`: the end date of the analysis period (e.g., 2023-01-01)
                    - `<interval>`: the interval of the data (options: 1m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo)
                    - `<confidence_level>`: the confidence level of the VaR (e.g., 0.95)
                    - `<risk_free_rate>`: the risk-free rate (e.g., 0.02)
                    - `<max_iterations>`: the maximum number of iterations (e.g., 1000)
                    - `<objective_function>`: the objective function (options: max_sharpe, min_vol, max_return, max_drawdown, min_var, min_cvar)
                "###.to_string(),

        "news" => r###"
                    This is a command for news sentiment analysis.

                    Usage: `/news <symbol> <max>`

                    Example: `/news AAPL 5

                    Parameters:
                    - `<symbol>`: the ticker symbol of the security to be analyzed (e.g., AAPL)
                    - `<max>`: the maximum number of news articles to fetch (e.g., 5)
                "###.to_string(),

        _ => r###"use any of the following commands to get help:
                /help symbols
                /help ticker
                /help portfolio
                /help news
        "###.to_string()
    }
}


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Welcome to Finalytics Bot! You can use the following commands:")]
enum Command {
    #[command(description = "Display All Commands")]
    Start,

    #[command(description = "Get command description")]
    Help {command: String},

    #[command(description = "Search for ticker symbols", parse_with = "split")]
    Symbols {asset_class: String, query: String},

    #[command(description = "Analyze a security", parse_with = "split")]
    Ticker {symbol: String, start_date: String, end_date: String, interval: String, benchmark_symbol: String,
        confidence_level: f64, risk_free_rate: f64, chart_type: String},

    #[command(description = "Optimize a portfolio", parse_with = "split")]
    Portfolio {symbols: String, benchmark_symbol: String, start_date: String, end_date: String, interval: String,
        confidence_level: f64, risk_free_rate: f64, max_iterations: u64, objective_function: String},

    #[command(description = "Analyze news sentiment", parse_with = "split")]
    News {symbol: String, max: usize},

}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let bot = Arc::clone(&Arc::new(bot));
    let msg = Arc::clone(&Arc::new(msg));
    let handle = tokio::runtime::Handle::current();
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
        },

        Command::Help {command} => {
            let message = get_command_description(command);
            bot.send_message(msg.chat.id, message).await?
        }

        Command::Symbols {asset_class, query} => {
            let message = handle_symbols_command(asset_class, query);
            bot.send_message(msg.chat.id, message).await?

        }

        Command::Ticker {symbol, start_date, end_date, interval, benchmark_symbol,
            confidence_level, risk_free_rate, chart_type} => {
            let bot_clone = Arc::clone(&bot);
            let msg_clone = Arc::clone(&msg);
            let message =  std::thread::spawn(move || {
                handle.block_on(handle_ticker_command(symbol, start_date, end_date, interval, benchmark_symbol,
                    confidence_level, risk_free_rate, chart_type, bot_clone, msg_clone))
            }).join().unwrap();
            bot.send_message(msg.chat.id, message.to_string()).await?
        }

        Command::Portfolio {symbols, benchmark_symbol, start_date, end_date, interval,
            confidence_level, risk_free_rate, max_iterations, objective_function} => {
            let bot_clone = Arc::clone(&bot);
            let msg_clone = Arc::clone(&msg);
            let message = std::thread::spawn(move || {
                handle.block_on(handle_portfolio_command(symbols, benchmark_symbol, start_date, end_date, interval,
                    confidence_level, risk_free_rate, max_iterations, objective_function, bot_clone, msg_clone))
            }).join().unwrap();
            bot.send_message(msg.chat.id, message.to_string()).await?
        }

        Command::News {symbol, max} => {
            let bot_clone = Arc::clone(&bot);
            let msg_clone = Arc::clone(&msg);
            let message = std::thread::spawn(move || {
                handle.block_on(handle_news_command(symbol, max, bot_clone, msg_clone))
            }).join().unwrap();
            bot.send_message(msg.chat.id, message.to_string()).await?
        }
        
    };

    Ok(())
}


fn handle_symbols_command(asset_class: String, query: String) -> String {
    let asset_class_enum = match asset_class.as_str() {
        "equity" => AssetClass::Stocks,
        "etf" => AssetClass::ETFs,
        "index" => AssetClass::Indices,
        "mutual_fund" => AssetClass::MutualFunds,
        "currency" => AssetClass::Currencies,
        "crypto" => AssetClass::Cryptocurrencies,
        _ => return "Invalid Asset Class. Get example with `/help symbols` ".to_string(),
    };

    // Fetch symbols based on the asset class and query
    let tickers = get_symbols(asset_class_enum, Category::All, Exchange::All);

    if let Ok(tickers) = tickers {
        // Filter symbols based on the query
        let filtered_symbols: Vec<String> = tickers
            .iter()
            .filter(|tc| tc.symbol.to_lowercase().contains(&query.to_lowercase())
                || tc.name.to_lowercase().contains(&query.to_lowercase()))
            .map(|tc| format!("{}: {}", tc.symbol.clone(), tc.name.clone()))
            .collect();

        let filtered_symbols = filtered_symbols[0..std::cmp::min(100, filtered_symbols.len())].to_vec();

        if filtered_symbols.is_empty() {
            "No matching symbols found.".to_string()
        } else {
            let response_text = format!("Matching symbols:\n\n {}", filtered_symbols.join("\n\n "));
            response_text
        }
    } else {
        "Invalid Parameters for symbols command. Get example with `/help symbols` ".to_string()
    }

}

async fn handle_ticker_command(symbol: String, start_date: String, end_date: String, interval: String, benchmark_symbol: String,
                               confidence_level: f64, risk_free_rate: f64, chart_type: String, bot: Arc<Bot>, msg: Arc<Message>) -> String {

    // Create TickerCharts and generate the appropriate chart based on the selected chart type

    let tc = TickerBuilder::new().ticker(&symbol).start_date(&start_date)
        .end_date(&end_date)
        .interval(Interval::from_str(&interval))
        .benchmark_symbol(&benchmark_symbol)
        .confidence_level(confidence_level)
        .risk_free_rate(risk_free_rate)
        .build();
    let charts = match chart_type.as_str() {
        "price_charts" => vec![
            if let Ok(cc) = tc.candlestick_chart(800, 1200).await {
                cc
            } else {
                return "Invalid Parameters for ticker command. Get example with `/help ticker` ".to_string();
            },
            if let Ok(ss) = tc.summary_stats_table(800, 1200).await {
                ss
            } else {
                return "Invalid Parameters for ticker command. Get example with `/help ticker` ".to_string();
            }],
        "performance_charts" => vec![
            if let Ok(pc) = tc.performance_chart(800, 1200).await {
                pc
            } else {
                return "Invalid Parameters for ticker command. Get example with `/help ticker` ".to_string();
            },
            if let Ok(ps) = tc.performance_stats_table(800, 1200).await {
                ps
            } else {
                return "Invalid Parameters for ticker command. Get example with `/help ticker` ".to_string();
            }],
        "financials" => if let Ok(fs) = tc.financials_tables(800, 1200).await{
            vec![fs["Income Statement"].clone(), fs["Balance Sheet"].clone(), fs["Cashflow Statement"].clone(), fs["Financial Ratios"].clone()]
        } else {
            return "Invalid Parameters for ticker command. Get example with `/help ticker` ".to_string();
        } ,
        "options_charts" => if let Ok(oc) = tc.options_charts(800, 1200).await{
            vec![oc["Volatility Surface"].clone(), oc["Volatility Smile"].clone(), oc["Volatility Term Structure"].clone()]
        } else {
            return "Invalid Parameters for ticker command. Get example with `/ticker` ".to_string();
        },
        _ => vec![
            if let Ok(cc) = tc.candlestick_chart(800, 1200).await {
                cc
            } else {
                return "Invalid Parameters for ticker command. Get example with `/ticker` ".to_string();
            },
            if let Ok(ss) = tc.summary_stats_table(800, 1200).await {
                ss
            } else {
                return "Invalid Parameters for ticker command. Get example with `/ticker` ".to_string();
            }],
        };

        for chart in charts {
            if let Ok(()) = html_to_png(&chart.to_html(), bot.clone(), msg.clone()).await{
                // Chart sent successfully
            } else {
                return "Failed to generate chart. Please try again later.".to_string();
            }
        }
        format!("{} Ticker {} generated successfully.", symbol, chart_type)

}

async fn handle_portfolio_command(symbols: String, benchmark_symbol: String, start_date: String, end_date: String, interval: String,
                                  confidence_level: f64, risk_free_rate: f64, max_iterations: u64, objective_function: String,
                                    bot: Arc<Bot>, msg: Arc<Message>) -> String {
    
    // Create PortfolioCharts and generate portfolio optimization charts
    let pc = PortfolioBuilder::new()
        .ticker_symbols(symbols.split(",").map(|x| x.trim()).collect())
        .benchmark_symbol(&benchmark_symbol)
        .start_date(&start_date)
        .end_date(&end_date)
        .interval(Interval::from_str(&interval))
        .confidence_level(confidence_level)
        .risk_free_rate(risk_free_rate)
        .max_iterations(max_iterations)
        .objective_function(ObjectiveFunction::from_str(&objective_function))
        .build().await;

    if let Ok(pc) = pc {
        let charts = vec![
            if let Ok(pc) = pc.optimization_chart(800, 1200) {
                pc
            } else {
                return "Invalid Parameters for portfolio command. Get example with `/portfolio` ".to_string();
            },
            if let Ok(pc) = pc.performance_chart(800, 1200) {
                pc
            } else {
                return "Invalid Parameters for portfolio command. Get example with `/portfolio` ".to_string();
            },
            if let Ok(pc) = pc.asset_returns_chart(800, 1200) {
                pc
            } else {
                return "Invalid Parameters for portfolio command. Get example with `/portfolio` ".to_string();
            },
            if let Ok(pc) = pc.performance_stats_table(800, 1200) {
                pc
            } else {
                return "Invalid Parameters for portfolio command. Get example with `/portfolio` ".to_string();
            },
        ];

        for chart in charts {
            if let Ok(()) = html_to_png(&chart.to_html(),  bot.clone(), msg.clone()).await{
                // Chart sent successfully
            } else {
                return "Failed to generate chart. Please try again later.".to_string();
            }
        }

        let symbols = pc.performance_stats.ticker_symbols;
        let weights = pc.performance_stats.optimal_weights;
        let response_text = format!("Optimal Portfolio Weights:\n\n {}",
                                    symbols.iter().zip(weights.iter()).map(|(s, w)|
                                        format!("{}: {:.2}%", s, w * 100.0)).collect::<Vec<String>>()
                                        .join("\n\n "));
        match bot.send_message(msg.chat.id, response_text).await{
            Ok(_) => {},
            Err(_) => {
                return "Failed to send Portfolio Weights. Please try again later.".to_string();
            }
        };

        "Portfolio optimization charts generated successfully.".to_string()
    } else {
        "Invalid Parameters for portfolio command. Get example with `/portfolio` ".to_string()
    }
}

async fn handle_news_command(symbol: String, max: usize, bot: Arc<Bot>, msg: Arc<Message>) -> String {
    let tc = TickerBuilder::new().ticker(&symbol).build();
    let news = tc.get_news(false).await;

    if let Ok(news) = news {
        let max = std::cmp::min(max, news.len());
        if news.is_empty() {
            "No news found for the given parameters.".to_string()
        } else {
            for n in &news[0..max] {
                let response_text = format!("{}", n.link);
                bot.send_message(msg.chat.id, response_text)
                    .await
                    .expect("Failed to send message");
            }
            format!("{} {} News Articles Sent successfully.", max, symbol)
        }
    } else {
        "Invalid Parameters for news command. Get example with `/news help` ".to_string()
    }
}




