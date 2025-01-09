use actix_web::{get, web, HttpResponse, Responder};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub async fn highlight_code(code: String, lang: String) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ss.find_syntax_by_extension(&lang).unwrap();
    let code = highlighted_html_for_string(&code, &ss, syntax, &ts.themes["base16-ocean.dark"]).unwrap();
    code
}

#[get("/code_examples/{category}")]
pub async fn get_code_examples(category: web::Path<String>) -> impl Responder {

    let ticker_rs = r###"
    use std::error::Error;
    use finalytics::prelude::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {

        // Construct Ticker Object
        let ticker = TickerBuilder::new()
               .ticker("AAPL")
               .start_date("2023-01-01")
               .end_date("2024-12-31")
               .interval(Interval::OneDay)
               .benchmark_symbol("^GSPC")
               .confidence_level(0.95)
               .risk_free_rate(0.02)
               .build();

        // Display Ticker Performance Chart
        ticker.performance_chart(Some(800), Some(1200)).await?.show();

        Ok(())
    }
        "###.to_string();

    let ticker_py = r###"
    from finalytics import Ticker

    # Construct Ticker Object
    ticker = Ticker(symbol="AAPL",
                    start_date="2023-01-01",
                    end_date="2024-12-31",
                    interval="1d",
                    benchmark="^GSPC",
                    confidence_level=0.95,
                    risk_free_rate=0.02)

    # Display Ticker Performance Chart
    ticker.performance_chart(height=800, width=1200).show()
        "###.to_string();


    let portfolio_rs = r###"
    use std::error::Error;
    use finalytics::prelude::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {

        // Construct Portfolio Object
        let portfolio = PortfolioBuilder::new()
                .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
                .benchmark_symbol("^GSPC")
                .start_date("2023-01-01")
                .end_date("2024-12-31")
                .interval(Interval::OneDay)
                .confidence_level(0.95)
                .risk_free_rate(0.02)
                .objective_function(ObjectiveFunction::MaxSharpe)
                .build().await?;

        // Display Portfolio Optimization Chart
        portfolio.optimization_chart(Some(800), Some(1200))?.show();

        Ok(())
    }
        "###.to_string();


    let portfolio_py = r###"
    from finalytics import Portfolio

    # Construct Portfolio Object
    portfolio = Portfolio(symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
                            benchmark_symbol="^GSPC",
                            start_date="2020-01-01",
                            end_date="2024-01-01",
                            interval="1d",
                            confidence_level=0.95,
                            risk_free_rate=0.02,
                            objective_function="max_sharpe")

    # Display Portfolio Optimization Chart
    portfolio.optimization_chart(height=800, width=1200).show()
        "###.to_string();

    let code = match category.as_str() {
        "ticker_rs" => highlight_code(ticker_rs, "rs".to_string()).await,
        "ticker_py" => highlight_code(ticker_py, "py".to_string()).await,
        "portfolio_rs" => highlight_code(portfolio_rs, "rs".to_string()).await,
        "portfolio_py" => highlight_code(portfolio_py, "py".to_string()).await,
        _ => "".to_string()
    };

    HttpResponse::Ok().body(code)
}