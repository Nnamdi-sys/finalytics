Welcome to Finalytics Documentation
====================================

Ticker Module
-------------

.. py:class:: Ticker

   A class representing a Ticker object.

   .. py:method:: __init__(symbol: str, start_date: Optional[str], end_date: Optional[str], interval: Optional[str], benchmark_symbol: Optional[str], confidence_level: Optional[float], risk_free_rate: Optional[float]) -> Ticker

      Create a new Ticker object.

      :param symbol: The ticker symbol of the asset.
      :type symbol: str
      :param start_date: Optional start date for historical data, defaults to None.
      :type start_date: Optional[str]
      :param end_date: Optional end date for historical data, defaults to None.
      :type end_date: Optional[str]
      :param interval: Optional data interval (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo), defaults to None.
      :type interval: Optional[str]
      :param benchmark_symbol: Optional benchmark symbol, defaults to None.
      :type benchmark_symbol: Optional[str]
      :param confidence_level: Optional confidence level for statistics, defaults to None.
      :type confidence_level: Optional[float]
      :param risk_free_rate: Optional risk-free rate for calculations, defaults to None.
      :type risk_free_rate: Optional[float]
      :return: A Ticker object.
      :rtype: Ticker

   .. py:method:: get_quote() -> dict

      Get the current ticker quote stats.

      :return: Dictionary containing current ticker quote stats.
      :rtype: dict

   .. py:method:: get_summary_stats() -> dict

      Get summary technical and fundamental statistics for the ticker.

      :return: Dictionary containing summary statistics.
      :rtype: dict

   .. py:method:: get_price_history() -> DataFrame

      Get the OHLCV data for the ticker for a given time period.

      :return: Polars DataFrame containing OHLCV data.
      :rtype: DataFrame

   .. py:method:: get_options_chain() -> DataFrame

      Get the options chain for the ticker.

      :return: Polars DataFrame containing the options chain.
      :rtype: DataFrame

   .. py:method:: get_news(compute_sentiment: bool) -> dict

      Get the latest news for the given ticker.

      :param compute_sentiment: Whether to compute sentiment of news articles.
      :type compute_sentiment: bool
      :return: Dictionary containing news articles and sentiment results if requested.
      :rtype: dict

   .. py:method:: get_income_statement() -> DataFrame

      Get the Income Statement for the ticker.

      :return: Polars DataFrame containing the Income Statement.
      :rtype: DataFrame

   .. py:method:: get_balance_sheet() -> DataFrame

      Get the Balance Sheet for the ticker.

      :return: Polars DataFrame containing the Balance Sheet.
      :rtype: DataFrame

   .. py:method:: get_cashflow_statement() -> DataFrame

      Get the Cashflow Statement for the ticker.

      :return: Polars DataFrame containing the Cashflow Statement.
      :rtype: DataFrame

   .. py:method:: get_financial_ratios() -> DataFrame

      Get the Financial Ratios for the ticker.

      :return: Polars DataFrame containing the Financial Ratios.
      :rtype: DataFrame

   .. py:method:: volatility_surface() -> DataFrame

      Get the implied volatility surface for the ticker options chain.

      :return: Polars DataFrame containing the implied volatility surface.
      :rtype: DataFrame

   .. py:method:: performance_stats() -> dict

      Compute the performance statistics for the ticker.

      :return: Dictionary containing performance statistics.
      :rtype: dict

   .. py:method:: performance_chart(height: Optional[int], width: Optional[int]) -> Plot

      Display the performance chart for the ticker.

      :param height: Optional height of the plot in pixels, defaults to None.
      :type height: Optional[int]
      :param width: Optional width of the plot in pixels, defaults to None.
      :type width: Optional[int]
      :return: Plot object containing the performance chart.
      :rtype: Plot

   .. py:method:: candlestick_chart(height: Optional[int], width: Optional[int]) -> Plot

      Display the candlestick chart for the ticker.

      :param height: Optional height of the plot in pixels, defaults to None.
      :type height: Optional[int]
      :param width: Optional width of the plot in pixels, defaults to None.
      :type width: Optional[int]
      :return: Plot object containing the candlestick chart.
      :rtype: Plot

   .. py:method:: options_chart(chart_type: str, height: Optional[int], width: Optional[int]) -> Plot

      Display the options volatility surface, smile and term structure charts for the ticker.

      :param chart_type: Type of options chart (volatility_surface, smile, term_structure).
      :type chart_type: str
      :param height: Optional height of the plot in pixels, defaults to None.
      :type height: Optional[int]
      :param width: Optional width of the plot in pixels, defaults to None.
      :type width: Optional[int]
      :return: Plot object containing the options chart.
      :rtype: Plot


Portfolio Module
----------------

.. py:class:: Portfolio

   A class representing a Portfolio object.

   .. py:method:: __init__(symbols: List[str], benchmark_symbol: str, start_date: str, end_date: str, interval: str, confidence_level: float, risk_free_rate: float, max_iterations: int, objective_function: str) -> Portfolio

      Create a new Portfolio object.

      :param symbols: List of ticker symbols for the assets in the portfolio.
      :type symbols: List[str]
      :param benchmark_symbol: The ticker symbol of the benchmark to compare against.
      :type benchmark_symbol: str
      :param start_date: The start date of the time period in the format YYYY-MM-DD.
      :type start_date: str
      :param end_date: The end date of the time period in the format YYYY-MM-DD.
      :type end_date: str
      :param interval: The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
      :type interval: str
      :param confidence_level: The confidence level for the VaR and ES calculations.
      :type confidence_level: float
      :param risk_free_rate: The risk free rate to use in the calculations.
      :type risk_free_rate: float
      :param max_iterations: The maximum number of iterations to use in the optimization.
      :type max_iterations: int
      :param objective_function: The objective function to use in the optimization (max_sharpe, min_vol, max_return, min_var, min_cvar, min_drawdown).
      :type objective_function: str
      :return: A Portfolio object.
      :rtype: Portfolio

   .. py:method:: portfolio_chart(chart_type: str, height: Optional[int], width: Optional[int]) -> Plot

      Display the Portfolio Optimization Chart.

      :param chart_type: Type of portfolio chart (optimization, performance, asset_returns).
      :type chart_type: str
      :param height: Optional height of the plot in pixels, defaults to 800.
      :type height: Optional[int]
      :param width: Optional width of the plot in pixels, defaults to 1200.
      :type width: Optional[int]
      :return: Plot object containing the portfolio chart.
      :rtype: Plot


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
