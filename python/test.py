from finalytics import Tickers
from finalytics import IndicatorType

if __name__ == '__main__':
    # Tickers Test
    tickers = Tickers(symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
                      start_date="2020-01-01",
                      end_date="2024-01-01",
                      interval="1d",
                      confidence_level=0.95,
                      risk_free_rate=0.02)

    print(tickers.get_summary_stats())
    print(tickers.get_price_history())
    print(tickers.get_options_chain())
    print(tickers.get_income_statement())
    print(tickers.get_balance_sheet())
    print(tickers.get_cashflow_statement())
    print(tickers.get_financial_ratios())
    print(tickers.performance_stats())
    print(tickers.returns())
    tickers.returns_chart().show()
    tickers.returns_matrix().show()

    # Ticker Test
    ticker = tickers.get_ticker("AAPL")
    roc = IndicatorType.ROC(14, "adjclose")
    print(ticker.technicals(roc))
    ticker.summary_stats_table().show()
    ticker.performance_stats_table().show()
    ticker.performance_chart().show()
    ticker.candlestick_chart().show()
    print(ticker.volatility_surface())
    ticker.options_chart(chart_type="surface").show()
    ticker.financials_tables(chart_type="Income Statement").show()


    # Portfolio Test
    portfolio = tickers.optimize()
    print(portfolio.optimization_results())
    portfolio.optimization_chart().show()
    portfolio.performance_chart().show()
    portfolio.asset_returns_chart().show()
    portfolio.performance_stats_table().show()
