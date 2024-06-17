![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![pypi](https://img.shields.io/pypi/v/finalytics)](https://pypi.org/project/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![Documentation Status](https://readthedocs.org/projects/finalytics/badge/?version=latest)](https://finalytics.readthedocs.io/en/latest/?badge=latest)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)
![Python Version](https://img.shields.io/badge/Python-3.7%20%7C%203.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12-blue)
![PePy](https://static.pepy.tech/personalized-badge/finalytics?period=total&units=international_system&left_color=black&right_color=blue&left_text=Downloads)



This is a python binding for [Finalytics Rust Library](https://github.com/Nnamdi-sys/finalytics) designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

```bash
pip install finalytics
```

## Documentation

View Library documentation on readthedocs [here](https://finalytics.readthedocs.io/en/latest/)


### Security Analysis

```python
from finalytics import Ticker

ticker = Ticker(symbol="AAPL",
                start="2023-01-01",
                end="2023-10-31",
                interval="1d",
                confidence_level=0.95,
                risk_free_rate=0.02)

print(ticker.performance_stats())
ticker.performance_chart().show()
```

### Portfolio Optimization

```python
from finalytics import Portfolio

portfolio = Portfolio(ticker_symbols=["AAPL", "GOOG", "MSFT", "BTC-USD"],
                      benchmark_symbol="^GSPC", start_date="2020-01-01", end_date="2022-01-01", interval="1d",
                      confidence_level=0.95, risk_free_rate=0.02, max_iterations=1000,
                      objective_function="max_sharpe")

print(portfolio.get_optimization_results())
portfolio.portfolio_chart("performance").show()
```




