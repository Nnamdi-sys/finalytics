import ffi from '@2060.io/ffi-napi';
import ref from 'ref-napi';
import Polars from 'nodejs-polars';
import { Ticker } from './ticker.js'
import { Portfolio } from './portfolio.js';
import { Chart, dfToJSON, getNativeLibPath } from './utils.js';

// Define C types
const TickersHandle = ref.types.void; // Opaque pointer
const TickersHandlePtr = ref.refType(TickersHandle);
const CharPtr = ref.types.CString;
const CharPtrPtr = ref.refType(CharPtr);

// Load the finalytics library
const lib = ffi.Library(getNativeLibPath(), {
  finalytics_tickers_new: [TickersHandlePtr, [
    CharPtr, CharPtr, CharPtr, CharPtr, CharPtr,
    'double', 'double', CharPtr, CharPtr
  ]],
  finalytics_tickers_free: ['void', [TickersHandlePtr]],
  finalytics_free_string: ['void', [CharPtr]],
  finalytics_tickers_get_summary_stats: ['int', [TickersHandlePtr, CharPtrPtr]],
  finalytics_tickers_get_price_history: ['int', [TickersHandlePtr, CharPtrPtr]],
  finalytics_tickers_get_options_chain: ['int', [TickersHandlePtr, CharPtrPtr]],
  finalytics_tickers_get_news: ['int', [TickersHandlePtr, CharPtrPtr]],
  finalytics_tickers_get_income_statement: ['int', [TickersHandlePtr, CharPtr, 'int', CharPtrPtr]],
  finalytics_tickers_get_balance_sheet: ['int', [TickersHandlePtr, CharPtr, 'int', CharPtrPtr]],
  finalytics_tickers_get_cashflow_statement: ['int', [TickersHandlePtr, CharPtr, 'int', CharPtrPtr]],
  finalytics_tickers_get_financial_ratios: ['int', [TickersHandlePtr, CharPtr, CharPtrPtr]],
  finalytics_tickers_returns: ['int', [TickersHandlePtr, CharPtrPtr]],
  finalytics_tickers_performance_stats: ['int', [TickersHandlePtr, CharPtrPtr]],
  finalytics_tickers_returns_chart: ['int', [TickersHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_tickers_returns_matrix: ['int', [TickersHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_tickers_report: ['int', [TickersHandlePtr, CharPtr, CharPtrPtr]],
  finalytics_tickers_get_ticker: [TickersHandlePtr, [TickersHandlePtr, CharPtr]],
  finalytics_tickers_optimize: [TickersHandlePtr, [TickersHandlePtr, CharPtr, CharPtr, CharPtr, CharPtr]],
});

/**
 * Tickers class representing a collection of financial tickers with methods for retrieving aggregated data and analytics.
 */
class Tickers {
  /**
   * Creates a new Tickers instance.
   * @param {Buffer} handle - Opaque pointer to the underlying C TickersHandle.
   * @private
   */
  constructor(handle) {
    this.handle = handle;
  }

  /**
   * Retrieves summary technical and fundamental statistics for the tickers.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated summary statistics.
   * @throws {Error} If stats retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).build();
   * const summary = await tickers.getSummaryStats();
   * console.log(summary);
   * tickers.free();
   */
  async getSummaryStats() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_get_summary_stats(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get summary stats: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves OHLCV price history for the tickers.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated price history data.
   * @throws {Error} If price history retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).startDate('2023-01-01').endDate('2023-12-31').build();
   * const history = await tickers.getPriceHistory();
   * console.log(history);
   * tickers.free();
   */
  async getPriceHistory() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_get_price_history(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get price history: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the options chain for the tickers.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated options chain data.
   * @throws {Error} If options chain retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).build();
   * const options = await tickers.getOptionsChain();
   * console.log(options);
   * tickers.free();
   */
  async getOptionsChain() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_get_options_chain(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get options chain: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the latest news headlines for the tickers.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated news data.
   * @throws {Error} If news retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).build();
   * const news = await tickers.getNews();
   * console.log(news);
   * tickers.free();
   */
  async getNews() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_get_news(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get news: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the income statements for the tickers.
   * @param {string} frequency - The frequency of the statement ('annual' or 'quarterly').
   * @param {boolean} formatted - Whether to return the statement in a formatted manner.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated income statement data.
   * @throws {Error} If income statement retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).build();
   * const income = await tickers.getIncomeStatement('quarterly', true);
   * console.log(income);
   * tickers.free();
   */
  async getIncomeStatement(frequency, formatted) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_get_income_statement(this.handle, frequency, formatted ? 1 : 0, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get income statement: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the balance sheets for the tickers.
   * @param {string} frequency - The frequency of the statement ('annual' or 'quarterly').
   * @param {boolean} formatted - Whether to return the statement in a formatted manner.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated balance sheet data.
   * @throws {Error} If balance sheet retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).build();
   * const balance = await tickers.getBalanceSheet('quarterly', true);
   * console.log(balance);
   * tickers.free();
   */
  async getBalanceSheet(frequency, formatted) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_get_balance_sheet(this.handle, frequency, formatted ? 1 : 0, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get balance sheet: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the cash flow statements for the tickers.
   * @param {string} frequency - The frequency of the statement ('annual' or 'quarterly').
   * @param {boolean} formatted - Whether to return the statement in a formatted manner.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated cash flow statement data.
   * @throws {Error} If cash flow statement retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).build();
   * const cashflow = await tickers.getCashflowStatement('quarterly', true);
   * console.log(cashflow);
   * tickers.free();
   */
  async getCashflowStatement(frequency, formatted) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_get_cashflow_statement(this.handle, frequency, formatted ? 1 : 0, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get cash flow statement: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves financial ratios for the tickers.
   * @param {string} frequency - The frequency of the ratios ('annual' or 'quarterly').
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated financial ratios.
   * @throws {Error} If financial ratios retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).build();
   * const ratios = await tickers.getFinancialRatios('quarterly');
   * console.log(ratios);
   * tickers.free();
   */
  async getFinancialRatios(frequency) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_get_financial_ratios(this.handle, frequency, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get financial ratios: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves returns data for the tickers.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing returns data.
   * @throws {Error} If returns retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).startDate('2023-01-01').endDate('2023-12-31').build();
   * const returns = await tickers.returns();
   * console.log(returns);
   * tickers.free();
   */
  async returns() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_returns(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get returns: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves performance statistics for the tickers.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing aggregated performance statistics.
   * @throws {Error} If performance stats retrieval fails.
   * @example
   * const tickers = await new TickersBuilder()
   *   .symbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * const stats = await tickers.performanceStats();
   * console.log(stats);
   * tickers.free();
   */
  async performanceStats() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_performance_stats(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get performance stats: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the returns chart for the tickers.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the returns chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const tickers = await new TickersBuilder()
   *   .symbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .build();
   * const chart = await tickers.returnsChart(600, 800);
   * chart.show();
   * tickers.free();
   */
  async returnsChart(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_returns_chart(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get returns chart: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves the returns correlation matrix for the tickers.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the returns correlation matrix.
   * @throws {Error} If matrix retrieval fails.
   * @example
   * const tickers = await new TickersBuilder()
   *   .symbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * const matrix = await tickers.returnsMatrix(600, 800);
   * matrix.show();
   * tickers.free();
   */
  async returnsMatrix(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_returns_matrix(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get returns matrix: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves a comprehensive analytics report for the tickers.
   * @param {string} reportType - The type of report to display (e.g., 'performance', 'financials', 'options', 'news').
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the report.
   * @throws {Error} If report retrieval fails.
   * @example
   * const tickers = await new TickersBuilder()
   *   .symbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * const report = await tickers.report('performance');
   * report.show();
   * tickers.free();
   */
  async report(reportType) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_tickers_report(this.handle, reportType, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get report: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves a Ticker instance for a specific symbol from the Tickers collection.
   * @param {string} symbol - The ticker symbol to retrieve (e.g., 'AAPL').
   * @returns {Promise<Ticker>} A promise resolving to the Ticker object for the specified symbol.
   * @throws {Error} If Ticker retrieval fails.
   * @example
   * const tickers = await new TickersBuilder().symbols(['AAPL', 'MSFT']).build();
   * const ticker = await tickers.getTicker('AAPL');
   * console.log(await ticker.getQuote());
   * ticker.free();
   * tickers.free();
   */
  async getTicker(symbol) {
    return new Promise((resolve, reject) => {
      const handle = lib.finalytics_tickers_get_ticker(this.handle, symbol);
      if (!handle || handle.isNull()) {
        return reject(new Error('Failed to get Ticker'));
      }
      resolve(new Ticker(handle));
    });
  }

  /**
   * Optimizes the portfolio of tickers based on the specified objective and constraints.
   * @param {string} objectiveFunction - The objective function for optimization (e.g., 'max_sharpe').
   * @param {string} assetConstraints - JSON string defining asset-level constraints (e.g., '[[0,1],[0,1]]').
   * @param {string} categoricalConstraints - JSON string defining categorical constraints (e.g., '[{"Name":"AssetClass","Categories":["EQUITY","EQUITY"],"Constraints":[["EQUITY",0.0,0.8]]}]').
   * @param {string} weights - JSON string defining portfolio-level constraints (e.g., '{}').
   * @returns {Promise<Portfolio>} A promise resolving to the optimized Portfolio object.
   * @throws {Error} If portfolio optimization fails.
   * @example
   * const tickers = await new TickersBuilder()
   *   .symbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * const portfolio = await tickers.optimize('max_sharpe', '[[0,1],[0,1]]', '[{"Name":"AssetClass","Categories":["EQUITY","EQUITY"],"Constraints":[["EQUITY",0.0,0.8]]}]', '{}');
   * console.log(await portfolio.optimizationResults());
   * portfolio.free();
   * tickers.free();
   */
  async optimize(objectiveFunction, assetConstraints, categoricalConstraints, weights) {
    return new Promise((resolve, reject) => {
      const handle = lib.finalytics_tickers_optimize(this.handle, objectiveFunction, assetConstraints, categoricalConstraints, weights);
      if (!handle || handle.isNull()) {
        return reject(new Error('Failed to optimize portfolio'));
      }
      resolve(new Portfolio(handle));
    });
  }

  /**
   * Releases resources associated with the Tickers.
   * Should be called when the Tickers is no longer needed to prevent memory leaks.
   */
  free() {
    if (this.handle) {
      lib.finalytics_tickers_free(this.handle);
      this.handle = null;
    }
  }
}

/**
 * TickersBuilder class for constructing Tickers instances using the builder pattern.
 */
class TickersBuilder {
  /**
   * Initializes a new TickersBuilder with default values.
   * Defaults:
   * - symbols: []
   * - startDate: ''
   * - endDate: ''
   * - interval: '1d'
   * - benchmarkSymbol: ''
   * - confidenceLevel: 0.95
   * - riskFreeRate: 0.02
   * - tickersData: null
   * - benchmarkData: null
   */
  constructor() {
    this.symbolsValue = [];
    this.startDateValue = '';
    this.endDateValue = '';
    this.intervalValue = '1d';
    this.benchmarkSymbolValue = '';
    this.confidenceLevelValue = 0.95;
    this.riskFreeRateValue = 0.02;
    this.tickersDataValue = null;
    this.benchmarkDataValue = null;
  }

  /**
   * Sets the ticker symbols.
   * @param {string[]} value - Array of ticker symbols (e.g., ['AAPL', 'MSFT']).
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  symbols(value) { this.symbolsValue = value; return this; }

  /**
   * Sets the start date for the data period.
   * @param {string} value - The start date in YYYY-MM-DD format.
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  startDate(value) { this.startDateValue = value; return this; }

  /**
   * Sets the end date for the data period.
   * @param {string} value - The end date in YYYY-MM-DD format.
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  endDate(value) { this.endDateValue = value; return this; }

  /**
   * Sets the data interval.
   * @param {string} value - The data interval (e.g., '2m', '5m', '15m', '30m', '1h', '1d', '1wk', '1mo', '3mo').
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  interval(value) { this.intervalValue = value; return this; }

  /**
   * Sets the benchmark symbol.
   * @param {string} value - The benchmark symbol (e.g., '^GSPC').
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  benchmarkSymbol(value) { this.benchmarkSymbolValue = value; return this; }

  /**
   * Sets the confidence level for VaR and ES calculations.
   * @param {number} value - The confidence level (e.g., 0.95 for 95% confidence).
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  confidenceLevel(value) { this.confidenceLevelValue = value; return this; }

  /**
   * Sets the risk-free rate for calculations.
   * @param {number} value - The risk-free rate (e.g., 0.02 for 2%).
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  riskFreeRate(value) { this.riskFreeRateValue = value; return this; }

  /**
   * Sets custom ticker data.
   * @param {Polars.DataFrame[]|null} value - Array of Polars DataFrames containing custom ticker data (null if not used).
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  tickersData(value) { this.tickersDataValue = value; return this; }

  /**
   * Sets custom benchmark data.
   * @param {Polars.DataFrame|null} value - A Polars DataFrame containing custom benchmark data (null if not used).
   * @returns {TickersBuilder} The builder instance for method chaining.
   */
  benchmarkData(value) { this.benchmarkDataValue = value; return this; }

  /**
   * Constructs the Tickers instance with the configured parameters.
   * The symbols parameter is required; other parameters are optional and use defaults if not set.
   * @returns {Promise<Tickers>} A promise resolving to the initialized Tickers object.
   * @throws {Error} If Tickers creation fails or symbols is empty.
   * @example
   * const tickers = await new TickersBuilder()
   *   .symbols(['AAPL', 'MSFT'])
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * tickers.free();
   */
  async build() {
    if (!this.symbolsValue.length) throw new Error('Symbols is required and cannot be empty');
    const symbolsJson = JSON.stringify(this.symbolsValue);
    const tickersDataJson = this.tickersDataValue
      ? JSON.stringify(this.tickersDataValue.map(df => dfToJSON(df)))
      : '';
    const benchmarkDataJson = this.benchmarkDataValue ? dfToJSON(this.benchmarkDataValue) : '';
    return new Promise((resolve, reject) => {
      const handle = lib.finalytics_tickers_new(
        symbolsJson,
        this.startDateValue,
        this.endDateValue,
        this.intervalValue,
        this.benchmarkSymbolValue,
        this.confidenceLevelValue,
        this.riskFreeRateValue,
        tickersDataJson || null,
        benchmarkDataJson || null
      );
      if (!handle || handle.isNull()) {
        return reject(new Error('Failed to create Tickers'));
      }
      resolve(new Tickers(handle));
    });
  }
}

export { Tickers, TickersBuilder };