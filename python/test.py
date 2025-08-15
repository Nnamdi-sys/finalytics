import polars as pl
from finalytics import Ticker, Tickers, Portfolio, Screener

if __name__ == '__main__':

    # Screener Test
    screener = Screener(
        quote_type="EQUITY",
        filters=[
            '{"operator": "eq", "operands": ["exchange", "NMS"]}'
        ],
        sort_field="intradaymarketcap",
        sort_descending=True,
        offset=0,
        size=10
    )

    print(screener.overview())
    print(screener.metrics())
    screener.display()

    # Tickers Test
    symbols = screener.symbols()
    tickers = Tickers(symbols=symbols,
                      start_date="2023-01-01",
                      end_date="2024-12-31",
                      interval="1d",
                      confidence_level=0.95,
                      risk_free_rate=0.02)

    # Tickers Report
    tickers.report()

    # Ticker Report
    ticker = tickers.get_ticker(symbols[0])
    ticker.report("performance")
    ticker.report("financials")
    ticker.report("options")
    ticker.report("news")

    # Portfolio Report
    portfolio = tickers.optimize()
    portfolio.report()


    # Polars DataFrame Input Test

    # DataFrames
    nvda = pl.read_csv("../examples/datasets/nvda.csv", has_header=True,)
    goog = pl.read_csv("../examples/datasets/goog.csv", has_header=True,)
    aapl = pl.read_csv("../examples/datasets/aapl.csv", has_header=True,)
    msft = pl.read_csv("../examples/datasets/msft.csv", has_header=True,)
    btcusd = pl.read_csv("../examples/datasets/btcusd.csv", has_header=True,)
    gspc = pl.read_csv("../examples/datasets/gspc.csv", has_header=True,)

    # Ticker
    ticker = Ticker(
        symbol="AAPL",
        ticker_data=aapl,
        benchmark_data=gspc,
        confidence_level=0.95,
        risk_free_rate=0.02
    )
    ticker.report("performance")


    # Tickers
    tickers_data = [nvda, goog, aapl, msft, btcusd]
    tickers = Tickers(
        symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
        benchmark_symbol="^GSPC",
        tickers_data=tickers_data,
        benchmark_data=gspc,
        confidence_level=0.95,
        risk_free_rate=0.02
    )

    tickers.report()

    # Portfolio
    portfolio = Portfolio(
        ticker_symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
        benchmark_symbol="^GSPC",
        tickers_data=tickers_data,
        benchmark_data=gspc,
        confidence_level=0.95,
        risk_free_rate=0.02,
        objective_function="max_sharpe",
        asset_constraints=[(0, 1), (0, 1), (0, 1), (0, 1), (0, 1)],
        categorical_constraints=[
            (
                "AssetClass",
                ["EQUITY", "EQUITY", "EQUITY", "EQUITY", "CRYPTO"],
                [("EQUITY", 0.0, 0.8), ("CRYPTO", 0.0, 0.2)]
            )
        ]
    )

    portfolio.report()

