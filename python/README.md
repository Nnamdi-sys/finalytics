![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![pypi](https://img.shields.io/pypi/v/finalytics)](https://pypi.org/project/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![Documentation Status](https://img.shields.io/badge/docs-quarto-blue)](https://nnamdi.quarto.pub/finalytics/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)
![Python Version](https://img.shields.io/badge/Python-3.7%20%7C%203.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12%20%7C%203.13-blue)
![PePy](https://static.pepy.tech/personalized-badge/finalytics?period=total&units=international_system&left_color=black&right_color=blue&left_text=Downloads)



This is a python binding for [Finalytics Rust Library](https://github.com/Nnamdi-sys/finalytics) designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

```bash
pip install finalytics
```

## Example

View the [documentation](https://nnamdi.quarto.pub/finalytics/) for more information.

```python
from finalytics import Tickers

# Instantiate a Multiple Ticker Object
tickers = Tickers(symbols=["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"],
                  start_date="2023-01-01",
                  end_date="2024-12-31",
                  interval="1d",
                  confidence_level=0.95,
                  risk_free_rate=0.02)

# Generate a Single Ticker Report
ticker = tickers.get_ticker("AAPL")
ticker.report("performance")
ticker.report("financials")
ticker.report("options")
ticker.report("news")

# Generate a Multiple Ticker Report
tickers.report("performance")

# Perform a Portfolio Optimization
portfolio = tickers.optimize(objective_function="max_sharpe")

# Generate a Portfolio Report
portfolio.report("performance")

```




