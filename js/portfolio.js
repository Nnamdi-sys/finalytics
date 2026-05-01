import ffi from "@2060.io/ffi-napi";
import ref from "ref-napi";
import Polars from "nodejs-polars";
import { Chart, dfToJSON, getNativeLibPath, getLastError } from "./utils.js";

// Define C types
const PortfolioHandle = ref.types.void; // Opaque pointer
const PortfolioHandlePtr = ref.refType(PortfolioHandle);
const CharPtr = ref.types.CString;
const CharPtrPtr = ref.refType(CharPtr);

// Load the finalytics library
const lib = ffi.Library(getNativeLibPath(), {
  finalytics_portfolio_new: [
    PortfolioHandlePtr,
    [
      CharPtr,
      CharPtr,
      CharPtr,
      CharPtr,
      CharPtr,
      "double",
      "double",
      CharPtr,
      CharPtr,
      CharPtr,
      CharPtr,
      CharPtr,
      CharPtr,
      CharPtr,
      CharPtr,
      CharPtr,
    ],
  ],
  finalytics_portfolio_free: ["void", [PortfolioHandlePtr]],
  finalytics_free_string: ["void", [CharPtr]],
  finalytics_portfolio_optimization_results: [
    "int",
    [PortfolioHandlePtr, CharPtrPtr],
  ],
  finalytics_portfolio_optimization_chart: [
    "int",
    [PortfolioHandlePtr, "uint", "uint", CharPtrPtr],
  ],
  finalytics_portfolio_performance_chart: [
    "int",
    [PortfolioHandlePtr, "uint", "uint", CharPtrPtr],
  ],
  finalytics_portfolio_performance_stats: [
    "int",
    [PortfolioHandlePtr, CharPtrPtr],
  ],
  finalytics_portfolio_asset_returns_chart: [
    "int",
    [PortfolioHandlePtr, "uint", "uint", CharPtrPtr],
  ],
  finalytics_portfolio_value_chart: [
    "int",
    [PortfolioHandlePtr, "uint", "uint", CharPtrPtr],
  ],
  finalytics_portfolio_returns_matrix: [
    "int",
    [PortfolioHandlePtr, "uint", "uint", CharPtrPtr],
  ],
  finalytics_portfolio_report: [
    "int",
    [PortfolioHandlePtr, CharPtr, CharPtrPtr],
  ],
  finalytics_portfolio_update_dates: [
    "int",
    [PortfolioHandlePtr, CharPtr, CharPtr],
  ],
  finalytics_portfolio_transaction_history: [
    "int",
    [PortfolioHandlePtr, CharPtrPtr],
  ],
});

/**
 * Portfolio class representing a portfolio of assets with methods for retrieving optimization results and analytics.
 */
class Portfolio {
  /**
   * Creates a new Portfolio instance.
   * @param {Buffer} handle - Opaque pointer to the underlying C PortfolioHandle.
   * @private
   */
  constructor(handle) {
    this.handle = handle;
  }

  /**
   * Retrieves portfolio optimization results.
   * @returns {Promise<Object>} A promise resolving to a JSON object containing optimization results
   *   (e.g., weights, starting/ending weights, starting/ending values, performance metrics,
   *   efficient frontier, risk contributions, money weighted return).
   * @throws {Error} If optimization results retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const results = await portfolio.optimizationResults();
   * console.log(results);
   * portfolio.free();
   */
  async optimizationResults() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_optimization_results(
        this.handle,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get optimization results"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(JSON.parse(output));
    });
  }

  /**
   * Retrieves the portfolio optimization chart.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the optimization chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const chart = await portfolio.optimizationChart(600, 800);
   * chart.show();
   * portfolio.free();
   */
  async optimizationChart(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_optimization_chart(
        this.handle,
        height,
        width,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get optimization chart"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves the portfolio performance chart.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the performance chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const chart = await portfolio.performanceChart(600, 800);
   * chart.show();
   * portfolio.free();
   */
  async performanceChart(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_performance_chart(
        this.handle,
        height,
        width,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get performance chart"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves performance statistics for the portfolio.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated performance statistics.
   * @throws {Error} If performance stats retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const stats = await portfolio.performanceStats();
   * console.log(stats);
   * portfolio.free();
   */
  async performanceStats() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_performance_stats(
        this.handle,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get performance stats"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the asset returns chart for the portfolio (percentage returns).
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the asset returns chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const chart = await portfolio.assetReturnsChart(600, 800);
   * chart.show();
   * portfolio.free();
   */
  async assetReturnsChart(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_asset_returns_chart(
        this.handle,
        height,
        width,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get asset returns chart"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves the portfolio value over time chart.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the portfolio value chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const chart = await portfolio.portfolioValueChart(600, 800);
   * chart.show();
   * portfolio.free();
   */
  async portfolioValueChart(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_value_chart(
        this.handle,
        height,
        width,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get portfolio value chart"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves the returns correlation matrix for the portfolio.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the returns correlation matrix.
   * @throws {Error} If matrix retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const matrix = await portfolio.returnsMatrix(600, 800);
   * matrix.show();
   * portfolio.free();
   */
  async returnsMatrix(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_returns_matrix(
        this.handle,
        height,
        width,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get returns matrix"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves a comprehensive report for the portfolio.
   * @param {string} reportType - The type of report to display (e.g., 'performance', 'optimization').
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the report.
   * @throws {Error} If report retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const report = await portfolio.report('performance');
   * report.show();
   * portfolio.free();
   */
  async report(reportType) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_report(
        this.handle,
        reportType,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get report"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Updates the portfolio's date range and re-fetches data for out-of-sample evaluation.
   *
   * This method is for portfolios built from Yahoo Finance data (not custom data).
   * It rebuilds all underlying ticker and benchmark data for the new date range.
   * The optimization result (weights) is preserved so they can be evaluated
   * out-of-sample on the new period.
   *
   * After calling this method, call `performanceStats()` to evaluate the
   * optimized weights on the new data (it recomputes automatically).
   *
   * @param {string} startDate - New start date (e.g., '2024-01-01').
   * @param {string} endDate - New end date (e.g., '2024-12-31').
   * @returns {Promise<void>} Resolves when the update is complete.
   * @throws {Error} If the update fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2022-01-01')
   *   .endDate('2023-01-01')
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * // Optimize on 2022 data, then evaluate on 2023 data
   * await portfolio.updateDates('2023-01-01', '2024-01-01');
   * const stats = await portfolio.performanceStats();
   * console.log(stats);
   * portfolio.free();
   */
  async updateDates(startDate, endDate) {
    return new Promise((resolve, reject) => {
      const result = lib.finalytics_portfolio_update_dates(
        this.handle,
        startDate,
        endDate,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to update portfolio dates"));
      }
      resolve();
    });
  }

  /**
   * Retrieves the transaction history table for the portfolio.
   *
   * Returns a table of all transaction events during the simulation, including
   * rebalances, cash flows, and combined events. Each row includes portfolio
   * value before/after, per-asset values, trade amounts, turnover, cumulative TWR and MWR.
   *
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing the transaction history.
   * @throws {Error} If the retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .startDate('2023-01-01')
   *   .endDate('2024-12-31')
   *   .weights(JSON.stringify([25000, 25000]))
   *   .rebalanceStrategy(JSON.stringify({type: 'calendar', frequency: 'quarterly'}))
   *   .build();
   * const history = await portfolio.transactionHistory();
   * console.log(history);
   * portfolio.free();
   */
  async transactionHistory() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_transaction_history(
        this.handle,
        outputPtr,
      );
      if (result !== 0) {
        return reject(getLastError("Failed to get transaction history"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Releases resources associated with the Portfolio.
   * Should be called when the Portfolio is no longer needed to prevent memory leaks.
   */
  free() {
    if (this.handle) {
      lib.finalytics_portfolio_free(this.handle);
      this.handle = null;
    }
  }
}

/**
 * PortfolioBuilder class for constructing Portfolio instances using the builder pattern.
 */
class PortfolioBuilder {
  /**
   * Initializes a new PortfolioBuilder with default values.
   * Defaults:
   * - tickerSymbols: []
   * - benchmarkSymbol: ''
   * - startDate: ''
   * - endDate: ''
   * - interval: '1d'
   * - confidenceLevel: 0.95
   * - riskFreeRate: 0.02
   * - objectiveFunction: 'max_sharpe'
   * - assetConstraints: '{}'
   * - categoricalConstraints: '{}'
   * - weights: '{}'
   * - tickersData: null
   * - benchmarkData: null
   * - transactions: '[]'
   * - rebalanceStrategy: '{}'
   * - scheduledCashFlows: '[]'
   */
  constructor() {
    this.tickerSymbolsValue = [];
    this.benchmarkSymbolValue = "";
    this.startDateValue = "";
    this.endDateValue = "";
    this.intervalValue = "1d";
    this.confidenceLevelValue = 0.95;
    this.riskFreeRateValue = 0.02;
    this.objectiveFunctionValue = "max_sharpe";
    this.assetConstraintsValue = "{}";
    this.categoricalConstraintsValue = "{}";
    this.weightsValue = "{}";
    this.tickersDataValue = null;
    this.benchmarkDataValue = null;
    this.transactionsValue = "[]";
    this.rebalanceStrategyValue = "{}";
    this.scheduledCashFlowsValue = "[]";
  }

  /**
   * Sets the ticker symbols.
   * @param {string[]} value - Array of ticker symbols (e.g., ['AAPL', 'MSFT']).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  tickerSymbols(value) {
    this.tickerSymbolsValue = value;
    return this;
  }

  /**
   * Sets the benchmark symbol.
   * @param {string} value - The benchmark symbol (e.g., '^GSPC').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  benchmarkSymbol(value) {
    this.benchmarkSymbolValue = value;
    return this;
  }

  /**
   * Sets the start date for the data period.
   * @param {string} value - The start date in YYYY-MM-DD format.
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  startDate(value) {
    this.startDateValue = value;
    return this;
  }

  /**
   * Sets the end date for the data period.
   * @param {string} value - The end date in YYYY-MM-DD format.
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  endDate(value) {
    this.endDateValue = value;
    return this;
  }

  /**
   * Sets the data interval.
   * @param {string} value - The data interval (e.g., '2m', '5m', '15m', '30m', '1h', '1d', '1wk', '1mo', '3mo').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  interval(value) {
    this.intervalValue = value;
    return this;
  }

  /**
   * Sets the confidence level for VaR and ES calculations.
   * @param {number} value - The confidence level (e.g., 0.95 for 95% confidence).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  confidenceLevel(value) {
    this.confidenceLevelValue = value;
    return this;
  }

  /**
   * Sets the risk-free rate for calculations.
   * @param {number} value - The risk-free rate (e.g., 0.02 for 2%).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  riskFreeRate(value) {
    this.riskFreeRateValue = value;
    return this;
  }

  /**
   * Sets the objective function for optimization.
   * @param {string} value - The objective function. Supported values:
   *   'max_sharpe', 'max_sortino', 'max_return', 'min_vol', 'min_var',
   *   'min_cvar', 'min_drawdown', 'risk_parity', 'max_diversification',
   *   'hierarchical_risk_parity'
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  objectiveFunction(value) {
    this.objectiveFunctionValue = value;
    return this;
  }

  /**
   * Sets the asset-level constraints for optimization.
   * @param {string} value - JSON string defining asset-level constraints (e.g., '[[0,1],[0,1]]').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  assetConstraints(value) {
    this.assetConstraintsValue = value;
    return this;
  }

  /**
   * Sets the categorical constraints for optimization.
   * @param {string} value - JSON string defining categorical constraints (e.g., '[{"Name":"AssetClass","Categories":["EQUITY","EQUITY"],"Constraints":[["EQUITY",0.0,0.8]]}]').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  categoricalConstraints(value) {
    this.categoricalConstraintsValue = value;
    return this;
  }

  /**
   * Sets explicit dollar amounts for each asset in the portfolio.
   * When provided, the portfolio will evaluate these allocations directly
   * without optimization. The fractional weights are derived as allocation[i] / sum(allocation).
   * @param {string} value - JSON string defining dollar allocations (e.g., '[25000, 25000, 25000, 25000]').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  weights(value) {
    this.weightsValue = value;
    return this;
  }

  /**
   * Sets custom ticker data.
   * @param {Polars.DataFrame[]|null} value - Array of Polars DataFrames containing custom ticker data (null if not used).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  tickersData(value) {
    this.tickersDataValue = value;
    return this;
  }

  /**
   * Sets custom benchmark data.
   * @param {Polars.DataFrame|null} value - A Polars DataFrame containing custom benchmark data (null if not used).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  benchmarkData(value) {
    this.benchmarkDataValue = value;
    return this;
  }

  /**
   * Sets ad-hoc per-asset transactions (additions / withdrawals).
   * @param {string} value - JSON string defining transactions. Format:
   *   '[{"date":"2024-01-15","ticker":"AAPL","amount":5000},{"date":"2024-06-01","ticker":"MSFT","amount":-2000}]'
   *   Positive amounts are additions, negative are withdrawals.
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  transactions(value) {
    this.transactionsValue = value;
    return this;
  }

  /**
   * Sets the rebalancing strategy for the portfolio simulation.
   * @param {string} value - JSON string defining the strategy. Formats:
   *   '{"type":"calendar","frequency":"monthly"}' — rebalance on a fixed calendar schedule
   *   '{"type":"threshold","threshold":0.05}' — rebalance when any weight drifts > threshold
   *   '{"type":"calendar_or_threshold","frequency":"quarterly","threshold":0.05}' — either trigger
   *   Frequency values: "monthly", "quarterly", "semi_annually", "annually"
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  rebalanceStrategy(value) {
    this.rebalanceStrategyValue = value;
    return this;
  }

  /**
   * Sets recurring cash flow schedules for the portfolio simulation.
   * @param {string} value - JSON string defining the schedules. Format:
   *   '[{"amount":2000,"frequency":"monthly","start_date":null,"end_date":null,"allocation":"pro_rata"}]'
   *   Amount: positive = addition, negative = withdrawal.
   *   Allocation: "pro_rata", "rebalance", or {"custom":[0.4,0.3,0.2,0.1]}
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  scheduledCashFlows(value) {
    this.scheduledCashFlowsValue = value;
    return this;
  }

  /**
   * Constructs the Portfolio instance with the configured parameters.
   * The tickerSymbols parameter is required; other parameters are optional and use defaults if not set.
   * @returns {Promise<Portfolio>} A promise resolving to the initialized Portfolio object.
   * @throws {Error} If Portfolio creation fails or tickerSymbols is empty.
   * @example
   * // Optimization example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .objectiveFunction('max_sharpe')
   *   .riskFreeRate(0.02)
   *   .assetConstraints('[[0,1],[0,1]]')
   *   .build();
   *
   * // Explicit allocation with rebalancing and DCA
   * const portfolio2 = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT', 'NVDA', 'BTC-USD'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2024-12-31')
   *   .interval('1d')
   *   .weights('[25000, 25000, 25000, 25000]')
   *   .rebalanceStrategy('{"type":"calendar","frequency":"quarterly"}')
   *   .scheduledCashFlows('[{"amount":2000,"frequency":"monthly","start_date":null,"end_date":null,"allocation":"pro_rata"}]')
   *   .build();
   * portfolio.free();
   */
  async build() {
    if (!this.tickerSymbolsValue.length)
      throw new Error("tickerSymbols is required and cannot be empty");
    const symbolsJson = JSON.stringify(this.tickerSymbolsValue);
    const tickersDataJson = this.tickersDataValue
      ? JSON.stringify(this.tickersDataValue.map((df) => dfToJSON(df)))
      : "";
    const benchmarkDataJson = this.benchmarkDataValue
      ? dfToJSON(this.benchmarkDataValue)
      : "";
    return new Promise((resolve, reject) => {
      const handle = lib.finalytics_portfolio_new(
        symbolsJson,
        this.benchmarkSymbolValue,
        this.startDateValue,
        this.endDateValue,
        this.intervalValue,
        this.confidenceLevelValue,
        this.riskFreeRateValue,
        this.objectiveFunctionValue,
        this.assetConstraintsValue,
        this.categoricalConstraintsValue,
        this.weightsValue,
        tickersDataJson || null,
        benchmarkDataJson || null,
        this.transactionsValue || null,
        this.rebalanceStrategyValue || null,
        this.scheduledCashFlowsValue || null,
      );
      if (!handle || handle.isNull()) {
        return reject(getLastError("Failed to create Portfolio"));
      }
      resolve(new Portfolio(handle));
    });
  }
}

export { Portfolio, PortfolioBuilder };
