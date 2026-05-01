# Finalytics — Python Examples
#
# Installation
# ────────────
#   pip install finalytics
#
# Full docs: https://nnamdi.quarto.pub/finalytics/
#
# Run this example (from the repo root)
# ───────────────────────────────────────
#   bash examples/example.sh python

import polars as pl
from finalytics import Portfolio, Screener, Ticker, Tickers


def screener():
    """1. Screener — Large-Cap NASDAQ Technology Stocks with ROE >= 15%"""
    print("=== 1. Screener ===")
    s = Screener(
        quote_type="EQUITY",
        filters=[
            '{"operator": "eq", "operands": ["exchange", "NMS"]}',
            '{"operator": "eq", "operands": ["sector", "Technology"]}',
            '{"operator": "gte", "operands": ["intradaymarketcap", 10000000000]}',
            '{"operator": "gte", "operands": ["returnonequity.lasttwelvemonths", 0.15]}',
        ],
        sort_field="intradaymarketcap",
        sort_descending=True,
        offset=0,
        size=10,
    )

    print(s.overview())
    print(s.metrics())
    s.display()


def ticker():
    """2. Ticker — Single security analysis with all report types"""
    print("=== 2. Ticker ===")
    t = Ticker(
        symbol="AAPL",
        start_date="2023-01-01",
        end_date="2024-12-31",
        interval="1d",
        benchmark_symbol="^GSPC",
        confidence_level=0.95,
        risk_free_rate=0.02,
    )

    t.report("performance")
    t.report("financials")
    t.report("options")
    t.report("news")


def tickers():
    """3. Tickers — Multiple securities analysis"""
    print("=== 3. Tickers ===")
    t = Tickers(
        symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
        start_date="2023-01-01",
        end_date="2024-12-31",
        interval="1d",
        benchmark_symbol="^GSPC",
        confidence_level=0.95,
        risk_free_rate=0.02,
    )

    t.report("performance")


def portfolio_optimization_oos():
    """4. Portfolio — Optimization with Out-of-Sample Evaluation"""
    print("=== 4. Portfolio — Optimization with Out-of-Sample Evaluation ===")

    # Optimize on 2023-2024 data (in-sample)
    p = Portfolio(
        ticker_symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
        benchmark_symbol="^GSPC",
        start_date="2023-01-01",
        end_date="2024-12-31",
        interval="1d",
        confidence_level=0.95,
        risk_free_rate=0.02,
        objective_function="max_sharpe",
    )

    p.report("optimization")

    # Update to 2025 data for out-of-sample evaluation
    p.update_dates("2025-01-01", "2026-01-01")
    p.performance_stats()
    p.report("performance")


def portfolio_optimization_constraints():
    """5. Portfolio — Optimization with Weight & Categorical Constraints"""
    print("=== 5. Portfolio — Optimization with Weight & Categorical Constraints ===")

    p = Portfolio(
        ticker_symbols=["AAPL", "MSFT", "NVDA", "JPM", "XOM", "BTC-USD"],
        benchmark_symbol="^GSPC",
        start_date="2023-01-01",
        end_date="2024-12-31",
        interval="1d",
        confidence_level=0.95,
        risk_free_rate=0.02,
        objective_function="max_sharpe",
        asset_constraints=[
            (0.05, 0.40),  # AAPL
            (0.05, 0.40),  # MSFT
            (0.05, 0.40),  # NVDA
            (0.05, 0.30),  # JPM
            (0.05, 0.20),  # XOM
            (0.05, 0.25),  # BTC-USD
        ],
        categorical_constraints=[
            (
                "Sector",
                ["Tech", "Tech", "Tech", "Finance", "Energy", "Crypto"],
                [
                    ("Tech", 0.30, 0.60),
                    ("Finance", 0.05, 0.30),
                    ("Energy", 0.05, 0.20),
                    ("Crypto", 0.05, 0.25),
                ],
            ),
            (
                "Asset Class",
                ["Equity", "Equity", "Equity", "Equity", "Equity", "Crypto"],
                [
                    ("Equity", 0.70, 0.95),
                    ("Crypto", 0.05, 0.30),
                ],
            ),
        ],
    )

    p.report("optimization")


def portfolio_allocation_rebalancing_dca():
    """6. Portfolio — Explicit Allocation with Rebalancing and DCA"""
    print("=== 6. Portfolio — Explicit Allocation with Rebalancing and DCA ===")

    p = Portfolio(
        ticker_symbols=["AAPL", "MSFT", "NVDA", "BTC-USD"],
        benchmark_symbol="^GSPC",
        start_date="2023-01-01",
        end_date="2024-12-31",
        interval="1d",
        confidence_level=0.95,
        risk_free_rate=0.02,
        weights=[25000.0, 25000.0, 25000.0, 25000.0],
        rebalance_strategy={"type": "calendar", "frequency": "quarterly"},
        scheduled_cash_flows=[
            {
                "amount": 2000.0,
                "frequency": "monthly",
                "start_date": None,
                "end_date": None,
                "allocation": "pro_rata",
            }
        ],
    )

    p.report("performance")


def custom_data():
    """7. Custom Data (KLINE) — Load CSV data and use with Ticker, Tickers, Portfolio"""
    print("=== 7. Custom Data (KLINE) ===")

    # Load data from CSV files
    aapl = pl.read_csv("examples/datasets/aapl.csv", has_header=True)
    msft = pl.read_csv("examples/datasets/msft.csv", has_header=True)
    nvda = pl.read_csv("examples/datasets/nvda.csv", has_header=True)
    goog = pl.read_csv("examples/datasets/goog.csv", has_header=True)
    btcusd = pl.read_csv("examples/datasets/btcusd.csv", has_header=True)
    gspc = pl.read_csv("examples/datasets/gspc.csv", has_header=True)

    # Single Ticker from custom data
    print("--- Custom Ticker ---")
    t = Ticker(
        symbol="AAPL",
        benchmark_symbol="^GSPC",
        confidence_level=0.95,
        risk_free_rate=0.02,
        ticker_data=aapl,
        benchmark_data=gspc,
    )
    t.report("performance")

    # Multiple Tickers from custom data
    print("--- Custom Tickers ---")
    ts = Tickers(
        symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
        benchmark_symbol="^GSPC",
        confidence_level=0.95,
        risk_free_rate=0.02,
        tickers_data=[nvda, goog, aapl, msft, btcusd],
        benchmark_data=gspc,
    )
    ts.report("performance")

    # Portfolio optimization from custom data
    print("--- Custom Portfolio ---")
    p = Portfolio(
        ticker_symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
        benchmark_symbol="^GSPC",
        confidence_level=0.95,
        risk_free_rate=0.02,
        objective_function="max_sharpe",
        tickers_data=[nvda, goog, aapl, msft, btcusd],
        benchmark_data=gspc,
    )
    p.report("optimization")


if __name__ == "__main__":
    screener()
    ticker()
    tickers()
    portfolio_optimization_oos()
    portfolio_optimization_constraints()
    portfolio_allocation_rebalancing_dca()
    custom_data()
