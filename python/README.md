![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![pypi](https://img.shields.io/pypi/v/finalytics)](https://pypi.org/project/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
[![Documentation Status](https://readthedocs.org/projects/finalytics-py/badge/?version=latest)](https://finalytics-py.readthedocs.io/en/latest/?badge=latest)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)
![Python Version](https://img.shields.io/badge/Python-3.7%20%7C%203.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12-blue)
![PePy](https://static.pepy.tech/personalized-badge/finalytics?period=total&units=international_system&left_color=black&right_color=blue&left_text=Downloads)
[![CodeFactor](https://www.codefactor.io/repository/github/nnamdi-sys/finalytics-py/badge)](https://www.codefactor.io/repository/github/nnamdi-sys/finalytics-py)



This is a python binding for [Finalytics Rust Library](https://github.com/Nnamdi-sys/finalytics) designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

```bash
pip install finalytics
```

## Documentation

View Library documentation on readthedocs [here](https://finalytics-py.readthedocs.io/en/latest/)


### Symbol Search

```python
from finalytics import get_symbols

print(get_symbols(query="Apple", asset_class="Equity"))
print(get_symbols(query="Bitcoin", asset_class="Crypto"))
print(get_symbols(query="S&P 500", asset_class="Index"))
print(get_symbols(query="EURUSD", asset_class="Currency"))
print(get_symbols(query="SPY", asset_class="ETF"))
```

### Security Analysis
[![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/Nnamdi-sys/finalytics-py/HEAD?labpath=examples%2Fsecurity_analysis.ipynb)

```python
from finalytics import Ticker

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
```

### Portfolio Optimization
[![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/Nnamdi-sys/finalytics-py/HEAD?labpath=examples%2Fportfolio_optimization.ipynb)

```python
from finalytics import Portfolio

portfolio = Portfolio(ticker_symbols=["AAPL", "GOOG", "MSFT", "BTC-USD"], 
                      benchmark_symbol="^GSPC", start_date="2020-01-01", end_date="2022-01-01", interval="1d", 
                      confidence_level=0.95, risk_free_rate=0.02, max_iterations=1000, 
                      objective_function="max_sharpe")
print(portfolio.get_optimization_results())
portfolio.display_portfolio_charts("performance", "html")
```




