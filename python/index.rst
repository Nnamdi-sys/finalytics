Welcome to Finalytics Documentation
====================================

.. toctree::
   :maxdepth: 2
   :caption: Contents:

Symbols Module
--------------

This module provides functions related to symbols.

.. function:: get_symbols(query, asset_class) -> List[str]

    Fetches ticker symbols that closely match the specified query and asset class.

    - **Arguments:**
        - `query` (`str`): The query to search for.
        - `asset_class` (`str`): The asset class to search for.

    - **Returns:**
        - `List[str]`: A list of ticker symbols that closely match the query and asset class.

    - **Example**

        .. code-block:: python

            import finalytics

            symbols = finalytics.get_symbols("Apple", "Equity")
            print(symbols)


Ticker Module
-------------

This module contains the `Ticker` class.

.. class:: Ticker

    The `Ticker` class enables you to fetch data for a given ticker symbol and perform security analysis.

    .. method:: __init__(symbol: str) -> Ticker

      Create a new Ticker object.

      - **Arguments:**
            - `symbol` (`str`): The ticker symbol of the asset.

      - **Returns:**
            - `Ticker`: A Ticker object.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            print(ticker.name, ticker.exchange, ticker.category, ticker.asset_class)


    .. method:: get_current_price() -> float

      Get the current price of the ticker.

      - **Returns:**
            - `float`: The current price of the ticker.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            current_price = ticker.get_current_price()


    .. method:: get_summary_stats() -> dict

      Get summary technical and fundamental statistics for the ticker.

      - **Returns:**
            - `dict`: A dictionary containing the summary statistics.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            summary_stats = ticker.get_summary_stats()


    .. method:: get_price_history(start: str, end: str, interval: str) -> DataFrame

      Get the ohlcv data for the ticker for a given time period.

      - **Arguments:**
            - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
            - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
            - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).

      - **Returns:**
            - `DataFrame`: A Polars DataFrame containing the ohlcv data.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            ohlcv = ticker.get_price_history("2020-01-01", "2020-12-31", "1d")


    .. method:: get_options_chain() -> DataFrame

      Get the options chain for the ticker.

      - **Returns:**
            - `DataFrame`: A Polars DataFrame containing the options chain.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            options_chain = ticker.get_options_chain()


    .. method:: get_news(start: str, end: str, compute_sentiment: bool) -> dict

      Get the latest news for the ticker.

      - **Arguments:**
            - `compute_sentiment` (`bool`): Whether to compute the sentiment of the news articles.

      - **Returns:**
            - `dict`: A dictionary containing the news articles (and sentiment results if requested).

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            news = ticker.get_news(False)


    .. method:: get_income_statement() -> DataFrame

      Get the Income Statement for the ticker.

      - **Returns:**
            - `DataFrame`: A Polars DataFrame containing the Income Statement.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            income_statement = ticker.get_income_statement()


    .. method:: get_balance_sheet() -> DataFrame

      Get the Balance Sheet for the ticker.

      - **Returns:**
            - `DataFrame`: A Polars DataFrame containing the Balance Sheet.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            balance_sheet = ticker.get_balance_sheet()


    .. method:: get_cashflow_statement() -> DataFrame

      Get the Cashflow Statement for the ticker.

      - **Returns:**
            - `DataFrame`: A Polars DataFrame containing the Cashflow Statement.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            cashflow_statement = ticker.get_cashflow_statement()


    .. method:: get_financial_ratios() -> DataFrame

      Get the Financial Ratios for the ticker.

      - **Returns:**
            - `DataFrame`: A Polars DataFrame containing the Financial Ratios.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            financial_ratios = ticker.get_financial_ratios()


    .. method:: compute_performance_stats(start: str, end: str, interval: str, benchmark: str, confidence_level: float, risk_free_rate: float) -> dict

      Compute the performance statistics for the ticker.

      - **Arguments:**
            - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
            - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
            - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
            - `benchmark` (`str`): The ticker symbol of the benchmark to compare against.
            - `confidence_level` (`float`): The confidence level for the VaR and ES calculations.
            - `risk_free_rate` (`float`): The risk free rate to use in the calculations.

      - **Returns:**
            - `dict`: A dictionary containing the performance statistics.

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            performance_stats = ticker.compute_performance_stats("2020-01-01", "2020-12-31", "1d", "^GSPC", 0.95, 0.02)


    .. method:: display_performance_chart(start: str, end: str, interval: str, benchmark: str, confidence_level: float, risk_free_rate: float, display_format: str) -> None

      Display the performance chart for the ticker.

      - **Arguments:**
            - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
            - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
            - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
            - `benchmark` (`str`): The ticker symbol of the benchmark to compare against.
            - `confidence_level` (`float`): The confidence level for the VaR and ES calculations.
            - `risk_free_rate` (`float`): The risk free rate to use in the calculations.
            - `display_format` (`str`): The format to display the chart in (png, html, notebook).

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            ticker.display_performance_chart("2020-01-01", "2020-12-31", "1d", "^GSPC", 0.95, 0.02, "html")


    .. method:: display_candlestick_chart(start: str, end: str, interval: str, display_format: str) -> None

      Display the candlestick chart for the ticker.

      - **Arguments:**
            - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
            - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
            - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
            - `display_format` (`str`): The format to display the chart in (png, html, notebook).

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            ticker.display_candlestick_chart("2020-01-01", "2020-12-31", "1d", "html")


    .. method:: display_options_chart(risk_free_rate: float, display_format: str) -> None

      Display the options volatility surface, smile, and term structure charts for the ticker.

      - **Arguments:**
            - `risk_free_rate` (`float`): The risk free rate to use in the calculations.
            - `display_format` (`str`): The format to display the chart in (png, html, notebook, colab).

      - **Example:**

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            ticker.display_options_chart(0.02, "html")




Portfolio Module
----------------

This module contains the `Portfolio` class.

.. class:: Portfolio

   The Portfolio class enables you perform portfolio optimization and compute portfolio performance statistics.

   .. method:: __init__(ticker_symbols: List[str], benchmark_symbol: str, start_date: str, end_date: str, interval: str, confidence_level: float, risk_free_rate: float, max_iterations: int, objective_function: str) -> Portfolio

      Create a new Portfolio object.

      - **Arguments:**
            - `ticker_symbols` (`List[str]`): List of ticker symbols for the assets in the portfolio.
            - `benchmark_symbol` (`str`): The ticker symbol of the benchmark to compare against.
            - `start_date` (`str`): The start date of the time period in the format YYYY-MM-DD.
            - `end_date` (`str`): The end date of the time period in the format YYYY-MM-DD.
            - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
            - `confidence_level` (`float`): The confidence level for the VaR and ES calculations.
            - `risk_free_rate` (`float`): The risk-free rate to use in the calculations.
            - `max_iterations` (`int`): The maximum number of iterations to use in the optimization.
            - `objective_function` (`str`): The objective function to use in the optimization (max_sharpe, min_vol, max_return, nin_var, min_cvar, min_drawdown).

      - **Returns:**
            - `Portfolio`: A Portfolio object.

      - **Example:**

         .. code-block:: python

            import finalytics

            portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")


   .. method:: get_optimization_results() -> dict

      Get the portfolio optimization results.

      - **Returns:**
            - `dict`: A dictionary containing optimization results.

      - **Example:**

         .. code-block:: python

            import finalytics

            portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
            optimization_results = portfolio.get_optimization_results()


   .. method:: display_portfolio_charts(display_format: str) -> None

      Display the portfolio optimization charts.

      - **Arguments:**
            - `chart_type` (`str`): The type of chart to display (optimization, performance, asset_returns).
            - `display_format` (`str`): The format to display the charts in (html, png, notebook, colab).

      - **Example:**

         .. code-block:: python

            import finalytics

            portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
            portfolio.display_portfolio_charts("optimization","html")