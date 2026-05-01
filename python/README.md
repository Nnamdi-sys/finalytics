![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![PyPI](https://img.shields.io/pypi/v/finalytics)](https://pypi.org/project/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![Documentation Status](https://img.shields.io/badge/docs-quarto-blue)](https://nnamdi.quarto.pub/finalytics/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)
![Python Version](https://img.shields.io/badge/Python-3.9%20%7C%203.10%20%7C%203.11%20%7C%203.12%20%7C%203.13-blue)
[![PyPI Downloads](https://static.pepy.tech/badge/finalytics)](https://pepy.tech/projects/finalytics)

---

# Finalytics Python Binding

**Finalytics** is a high-performance Python binding for the Finalytics Rust library, designed for retrieving financial data, security analysis, and portfolio optimization.
It provides a fast, modular interface for advanced analytics, and powers dashboards and applications across platforms.

---

## 🚀 Installation

```bash
pip install finalytics
```

---

## 🐍 Main Modules

Finalytics Python exposes five core modules for financial analytics:

### 1. Screener

Efficiently filter and rank securities using advanced metrics and custom filters.

**Usage Example:**
```python
from finalytics import Screener

screener = Screener(
    quote_type="EQUITY",
    filters=[
        '{"operator": "eq", "operands": ["exchange", "NMS"]}',
        '{"operator": "eq", "operands": ["sector", "Technology"]}',
        '{"operator": "gte", "operands": ["intradaymarketcap", 10000000000]}',
        '{"operator": "gte", "operands": ["returnonequity.lasttwelvemonths", 0.15]}'
    ],
    sort_field="intradaymarketcap",
    sort_descending=True,
    offset=0,
    size=10
)

print(screener.overview())
print(screener.metrics())
screener.display()
```

---

### 2. Ticker

Analyze a single security in depth: performance, financials, options, news, and more.

**Usage Example:**
```python
from finalytics import Ticker

ticker = Ticker(
    symbol="AAPL",
    start_date="2023-01-01",
    end_date="2024-12-31",
    interval="1d",
    benchmark_symbol="^GSPC",
    confidence_level=0.95,
    risk_free_rate=0.02
)

ticker.report("performance")
ticker.report("financials")
ticker.report("options")
ticker.report("news")
```

---

### 3. Tickers

Work with multiple securities at once—aggregate reports, batch analytics, and portfolio construction.

**Usage Example:**
```python
from finalytics import Tickers

tickers = Tickers(
    symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
    start_date="2023-01-01",
    end_date="2024-12-31",
    interval="1d",
    benchmark_symbol="^GSPC",
    confidence_level=0.95,
    risk_free_rate=0.02
)

tickers.report("performance")
```

---

### 4. Portfolio

Optimize and analyze portfolios using advanced objective functions and constraints.
Supports rebalancing strategies, scheduled cash flows (DCA), ad-hoc transactions,
and out-of-sample evaluation.

**Objective Functions:**
`max_sharpe`, `max_sortino`, `max_return`, `min_vol`, `min_var`, `min_cvar`, `min_drawdown`,
`risk_parity`, `max_diversification`, `hierarchical_risk_parity`

**Usage Example: Optimization with Out-of-Sample Evaluation**
```python
from finalytics import Portfolio

# Optimize on 2023 - 2024 data (in-sample)
portfolio = Portfolio(
    ticker_symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
    benchmark_symbol="^GSPC",
    start_date="2023-01-01",
    end_date="2024-12-31",
    interval="1d",
    confidence_level=0.95,
    risk_free_rate=0.02,
    objective_function="max_sharpe"
)

portfolio.report("optimization")

# Update to 2025 data for out-of-sample evaluation
portfolio.update_dates("2025-01-01", "2026-01-01")
portfolio.performance_stats()
portfolio.report("performance")
```

**Usage Example: Explicit Allocation with Rebalancing and DCA**
```python
from finalytics import Portfolio

portfolio = Portfolio(
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
            "allocation": "pro_rata"
        }
    ]
)

portfolio.report("performance")
```

**Usage Example: Optimization with Weight & Categorical Constraints**
```python
from finalytics import Portfolio

portfolio = Portfolio(
    ticker_symbols=["AAPL", "MSFT", "NVDA", "JPM", "XOM", "BTC-USD"],
    benchmark_symbol="^GSPC",
    start_date="2023-01-01",
    end_date="2024-12-31",
    interval="1d",
    confidence_level=0.95,
    risk_free_rate=0.02,
    objective_function="max_sharpe",
    # Per-asset bounds: (lower, upper) in the same order as ticker_symbols
    asset_constraints=[
        (0.05, 0.40),  # AAPL
        (0.05, 0.40),  # MSFT
        (0.05, 0.40),  # NVDA
        (0.05, 0.30),  # JPM
        (0.05, 0.20),  # XOM
        (0.05, 0.25),  # BTC-USD
    ],
    # Categorical constraints: (name, category_per_symbol, weight_per_category)
    categorical_constraints=[
        (
            "Sector",
            ["Tech", "Tech", "Tech", "Finance", "Energy", "Crypto"],
            [
                ("Tech",    0.30, 0.60),
                ("Finance", 0.05, 0.30),
                ("Energy",  0.05, 0.20),
                ("Crypto",  0.05, 0.25),
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

portfolio.report("optimization")
```

---

### 5. Custom Data

Load your own price data from CSV files as Polars DataFrames and use it with any Finalytics module.
DataFrames must have columns: `timestamp` (unix epoch i64), `open`, `high`, `low`, `close`, `volume`, `adjclose`.

**Usage Example:**
```python
import polars as pl
from finalytics import Ticker, Tickers, Portfolio

# Load data from CSV files
aapl = pl.read_csv("examples/datasets/aapl.csv")
msft = pl.read_csv("examples/datasets/msft.csv")
nvda = pl.read_csv("examples/datasets/nvda.csv")
goog = pl.read_csv("examples/datasets/goog.csv")
btcusd = pl.read_csv("examples/datasets/btcusd.csv")
gspc = pl.read_csv("examples/datasets/gspc.csv")

# Single Ticker from custom data
ticker = Ticker(
    symbol="AAPL",
    benchmark_symbol="^GSPC",
    confidence_level=0.95,
    risk_free_rate=0.02,
    ticker_data=aapl,
    benchmark_data=gspc
)
ticker.report("performance")

# Multiple Tickers from custom data
tickers = Tickers(
    symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
    benchmark_symbol="^GSPC",
    confidence_level=0.95,
    risk_free_rate=0.02,
    tickers_data=[nvda, goog, aapl, msft, btcusd],
    benchmark_data=gspc
)
tickers.report("performance")

# Portfolio optimization from custom data
portfolio = Portfolio(
    ticker_symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
    benchmark_symbol="^GSPC",
    confidence_level=0.95,
    risk_free_rate=0.02,
    objective_function="max_sharpe",
    tickers_data=[nvda, goog, aapl, msft, btcusd],
    benchmark_data=gspc
)
portfolio.report("optimization")
```

---

## 📚 Documentation

- See the [Quarto documentation](https://nnamdi.quarto.pub/finalytics/) for full details.

---

## 🗂️ Multi-language Bindings

Finalytics is also available in:
- [Rust](../rust/README.md)
- [Node.js](../js/README.md)
- [Go](../go/README.md)
- [Web Application](../web/README.md)

---

**Finalytics** — Modular, high-performance financial analytics for Python.
