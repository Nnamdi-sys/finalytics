from finalytics import Tickers

if __name__ == '__main__':
    # Tickers Test
    tickers = Tickers(symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
                      start_date="2023-01-01",
                      end_date="2024-12-31",
                      interval="1d",
                      confidence_level=0.95,
                      risk_free_rate=0.02)

    # Tickers Report
    tickers.report()

    # Ticker Report
    ticker = tickers.get_ticker("AAPL")
    ticker.report("performance")
    ticker.report("financials")
    ticker.report("options")
    ticker.report("news")

    # Portfolio Report
    portfolio = tickers.optimize()
    portfolio.report()
