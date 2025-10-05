import ffi from '@2060.io/ffi-napi';
import ref from 'ref-napi';
import Polars from 'nodejs-polars';
import { Chart, dfToJSON, getNativeLibPath } from './utils.js';

// Define C types
const PortfolioHandle = ref.types.void; // Opaque pointer
const PortfolioHandlePtr = ref.refType(PortfolioHandle);
const CharPtr = ref.types.CString;
const CharPtrPtr = ref.refType(CharPtr);

// Load the finalytics library
const lib = ffi.Library(getNativeLibPath(), {
  finalytics_portfolio_new: [PortfolioHandlePtr, [
    CharPtr, CharPtr, CharPtr, CharPtr, CharPtr,
    'double', 'double', CharPtr, CharPtr, CharPtr, CharPtr, CharPtr, CharPtr
  ]],
  finalytics_portfolio_free: ['void', [PortfolioHandlePtr]],
  finalytics_free_string: ['void', [CharPtr]],
  finalytics_portfolio_optimization_results: ['int', [PortfolioHandlePtr, CharPtrPtr]],
  finalytics_portfolio_optimization_chart: ['int', [PortfolioHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_portfolio_performance_chart: ['int', [PortfolioHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_portfolio_performance_stats: ['int', [PortfolioHandlePtr, CharPtrPtr]],
  finalytics_portfolio_asset_returns_chart: ['int', [PortfolioHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_portfolio_returns_matrix: ['int', [PortfolioHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_portfolio_report: ['int', [PortfolioHandlePtr, CharPtr, CharPtrPtr]],
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
   * @returns {Promise<Object>} A promise resolving to a JSON object containing optimization results (e.g., weights, expected return, volatility).
   * @throws {Error} If optimization results retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC)
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
      const result = lib.finalytics_portfolio_optimization_results(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get optimization results: error code ${result}`));
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
   *   .benchmarkSymbol('^GSPC)
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
      const result = lib.finalytics_portfolio_optimization_chart(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get optimization chart: error code ${result}`));
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
   *   .benchmarkSymbol('^GSPC)
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
      const result = lib.finalytics_portfolio_performance_chart(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get performance chart: error code ${result}`));
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
   *   .benchmarkSymbol('^GSPC)
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .objectiveFunction('max_sharpe')
   *   .build();
   * const stats = await portfolio.performanceStats();
   * console.log(stats);
   * tickers.free();
   */
  async performanceStats() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_portfolio_performance_stats(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get performance stats: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the asset returns chart for the portfolio.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the asset returns chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC)
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
      const result = lib.finalytics_portfolio_asset_returns_chart(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get asset returns chart: error code ${result}`));
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
   *   .benchmarkSymbol('^GSPC)
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
      const result = lib.finalytics_portfolio_returns_matrix(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get returns matrix: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves a comprehensive report for the portfolio.
   * @param {string} reportType - The type of report to display (e.g., 'performance').
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the report.
   * @throws {Error} If report retrieval fails.
   * @example
   * const portfolio = await new PortfolioBuilder()
   *   .tickerSymbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC)
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
      const result = lib.finalytics_portfolio_report(this.handle, reportType, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get report: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
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
   */
  constructor() {
    this.tickerSymbolsValue = [];
    this.benchmarkSymbolValue = '';
    this.startDateValue = '';
    this.endDateValue = '';
    this.intervalValue = '1d';
    this.confidenceLevelValue = 0.95;
    this.riskFreeRateValue = 0.02;
    this.objectiveFunctionValue = 'max_sharpe';
    this.assetConstraintsValue = '{}';
    this.categoricalConstraintsValue = '{}';
    this.weightsValue = '{}';
    this.tickersDataValue = null;
    this.benchmarkDataValue = null;
  }

  /**
   * Sets the ticker symbols.
   * @param {string[]} value - Array of ticker symbols (e.g., ['AAPL', 'MSFT']).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  tickerSymbols(value) { this.tickerSymbolsValue = value; return this; }

  /**
   * Sets the benchmark symbol.
   * @param {string} value - The benchmark symbol (e.g., '^GSPC').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  benchmarkSymbol(value) { this.benchmarkSymbolValue = value; return this; }

  /**
   * Sets the start date for the data period.
   * @param {string} value - The start date in YYYY-MM-DD format.
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  startDate(value) { this.startDateValue = value; return this; }

  /**
   * Sets the end date for the data period.
   * @param {string} value - The end date in YYYY-MM-DD format.
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  endDate(value) { this.endDateValue = value; return this; }

  /**
   * Sets the data interval.
   * @param {string} value - The data interval (e.g., '2m', '5m', '15m', '30m', '1h', '1d', '1wk', '1mo', '3mo').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  interval(value) { this.intervalValue = value; return this; }

  /**
   * Sets the confidence level for VaR and ES calculations.
   * @param {number} value - The confidence level (e.g., 0.95 for 95% confidence).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  confidenceLevel(value) { this.confidenceLevelValue = value; return this; }

  /**
   * Sets the risk-free rate for calculations.
   * @param {number} value - The risk-free rate (e.g., 0.02 for 2%).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  riskFreeRate(value) { this.riskFreeRateValue = value; return this; }

  /**
   * Sets the objective function for optimization.
   * @param {string} value - The objective function (e.g., 'max_sharpe', 'max_sortino', 'max_return', 'min_vol', 'min_var', 'min_cvar', 'min_drawdown').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  objectiveFunction(value) { this.objectiveFunctionValue = value; return this; }

  /**
   * Sets the asset-level constraints for optimization.
   * @param {string} value - JSON string defining asset-level constraints (e.g., '[[0,1],[0,1]]').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  assetConstraints(value) { this.assetConstraintsValue = value; return this; }

  /**
   * Sets the categorical constraints for optimization.
   * @param {string} value - JSON string defining categorical constraints (e.g., '[{"Name":"AssetClass","Categories":["EQUITY","EQUITY"],"Constraints":[["EQUITY",0.0,0.8]]}]').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  categoricalConstraints(value) { this.categoricalConstraintsValue = value; return this; }

  /**
   * Sets the portfolio-level constraints for optimization.
   * @param {string} value - JSON string defining portfolio-level constraints (e.g., '{}').
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  weights(value) { this.weightsValue = value; return this; }

  /**
   * Sets custom ticker data.
   * @param {Polars.DataFrame[]|null} value - Array of Polars DataFrames containing custom ticker data (null if not used).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  tickersData(value) { this.tickersDataValue = value; return this; }

  /**
   * Sets custom benchmark data.
   * @param {Polars.DataFrame|null} value - A Polars DataFrame containing custom benchmark data (null if not used).
   * @returns {PortfolioBuilder} The builder instance for method chaining.
   */
  benchmarkData(value) { this.benchmarkDataValue = value; return this; }

  /**
   * Constructs the Portfolio instance with the configured parameters.
   * The tickerSymbols parameter is required; other parameters are optional and use defaults if not set.
   * @returns {Promise<Portfolio>} A promise resolving to the initialized Portfolio object.
   * @throws {Error} If Portfolio creation fails or tickerSymbols is empty.
   * @example
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
   * portfolio.free();
   */
  async build() {
    if (!this.tickerSymbolsValue.length) throw new Error('tickerSymbols is required and cannot be empty');
    const symbolsJson = JSON.stringify(this.tickerSymbolsValue);
    const tickersDataJson = this.tickersDataValue
      ? JSON.stringify(this.tickersDataValue.map(df => dfToJSON(df)))
      : '';
    const benchmarkDataJson = this.benchmarkDataValue ? dfToJSON(this.benchmarkDataValue) : '';
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
        benchmarkDataJson || null
      );
      if (!handle || handle.isNull()) {
        return reject(new Error('Failed to create Portfolio'));
      }
      resolve(new Portfolio(handle));
    });
  }
}

export { Portfolio, PortfolioBuilder };