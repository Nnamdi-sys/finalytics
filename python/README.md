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

Finalytics Python exposes four core modules for financial analytics:

### 1. Screener

Efficiently filter and rank securities (equities, crypto, etc.) using advanced metrics and custom filters.

**Usage Example:**
```python
from finalytics import Screener

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

symbols = ["AAPL", "MSFT", "GOOG"]
tickers = Tickers(
    symbols=symbols,
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

**Usage Example:**
```python
symbols = ["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"]
portfolio = Portfolio(
    symbols=symbols,
    benchmark_symbol="^GSPC",
    start_date="2023-01-01",
    end_date="2024-12-31",
    interval="1d",
    confidence_level=0.95,
    risk_free_rate=0.02,
    objective_function="max_sharpe"
)

portfolio.report("performance")
```

---

## 📚 Documentation

- See the [Quarto documentation](https://nnamdi.quarto.pub/finalytics/) for full details.

---

## 🗂️ Multi-language Bindings

Finalytics is also available in:
- [Rust](../../rust/README.md)
- [Node.js](../../js/README.md)
- [Go](../../go/README.md)
- [Web Application](../../web/README.md)

---

**Finalytics** — Modular, high-performance financial analytics for Python.