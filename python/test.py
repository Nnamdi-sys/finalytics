from finalytics import Ticker
from finalytics import Portfolio

if __name__ == '__main__':
    # Ticker Test
    ticker = Ticker(symbol="AAPL",
                    start_date="2023-01-01",
                    end_date="2023-10-31",
                    interval="1d",
                    confidence_level=0.95,
                    risk_free_rate=0.02)
    print(ticker.get_quote())
    print(ticker.get_summary_stats())
    print(ticker.get_price_history())
    print(ticker.get_options_chain())
    print(ticker.get_news(False))
    print(ticker.get_income_statement())
    print(ticker.get_balance_sheet())
    print(ticker.get_cashflow_statement())
    print(ticker.get_financial_ratios())
    print(ticker.volatility_surface())
    print(ticker.performance_stats())
    ticker.performance_chart().show()
    ticker.candlestick_chart().show()
    ticker.options_chart(chart_type="surface").show()


    # Portfolio Test
    portfolio = Portfolio(ticker_symbols=["AAPL", "GOOG", "MSFT", "BTC-USD"],
                          benchmark_symbol="^GSPC", start_date="2020-01-01", end_date="2022-01-01", interval="1d",
                          confidence_level=0.95, risk_free_rate=0.02, max_iterations=1000,
                          objective_function="max_sharpe")
    print(portfolio.get_optimization_results())
    portfolio.portfolio_chart(chart_type="optimization").show()
    portfolio.portfolio_chart(chart_type="performance").show()
    portfolio.portfolio_chart(chart_type="asset_returns").show()
