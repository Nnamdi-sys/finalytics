from finalytics import Ticker
from finalytics import Portfolio
from finalytics import get_symbols

if __name__ == '__main__':
    # Symbols Test
    print(get_symbols(query="Apple", asset_class="Equity"))
    print(get_symbols(query="Bitcoin", asset_class="Crypto"))
    print(get_symbols(query="S&P 500", asset_class="Index"))
    print(get_symbols(query="EURUSD", asset_class="Currency"))
    print(get_symbols(query="SPY", asset_class="ETF"))


    # Ticker Test
    ticker = Ticker(symbol="AAPL")
    print(ticker.get_current_price())
    print(ticker.get_summary_stats())
    print(ticker.get_price_history(start="2023-01-01", end="2023-10-31", interval="1d"))
    print(ticker.get_options_chain())
    print(ticker.get_news(compute_sentiment=False))
    print(ticker.get_income_statement())
    print(ticker.get_balance_sheet())
    print(ticker.get_cashflow_statement())
    print(ticker.get_financial_ratios())
    print(ticker.compute_performance_stats(start="2023-01-01", end="2023-10-31", interval="1d", benchmark="^GSPC",
                                           confidence_level=0.95, risk_free_rate=0.02))
    ticker.display_performance_chart(start="2023-01-01", end="2023-10-31", interval="1d", benchmark="^GSPC",
                                     confidence_level=0.95, risk_free_rate=0.02, display_format="notebook")
    ticker.display_candlestick_chart(start="2023-01-01", end="2023-10-31", interval="1d", display_format="html")
    ticker.display_options_chart(risk_free_rate=0.02, chart_type="surface", display_format="png")


    # Portfolio Test
    portfolio = Portfolio(ticker_symbols=["AAPL", "GOOG", "MSFT", "BTC-USD"],
                          benchmark_symbol="^GSPC", start_date="2020-01-01", end_date="2022-01-01", interval="1d",
                          confidence_level=0.95, risk_free_rate=0.02, max_iterations=1000,
                          objective_function="max_sharpe")
    print(portfolio.get_optimization_results())
    portfolio.display_portfolio_charts(chart_type="performance", display_format="html")

