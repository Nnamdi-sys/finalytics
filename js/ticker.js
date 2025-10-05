import ffi from '@2060.io/ffi-napi';
import ref from 'ref-napi';
import Polars from 'nodejs-polars';
import { Chart, dfToJSON, getNativeLibPath } from './utils.js';

// Define C types
const TickerHandle = ref.types.void; // Opaque pointer
const TickerHandlePtr = ref.refType(TickerHandle);
const CharPtr = ref.types.CString;
const CharPtrPtr = ref.refType(CharPtr);

// Load the finalytics library
const lib = ffi.Library(getNativeLibPath(), {
  finalytics_ticker_new: [TickerHandlePtr, [
    CharPtr, CharPtr, CharPtr, CharPtr, CharPtr,
    'double', 'double', CharPtr, CharPtr
  ]],
  finalytics_ticker_free: ['void', [TickerHandlePtr]],
  finalytics_free_string: ['void', [CharPtr]],
  finalytics_ticker_get_quote: ['int', [TickerHandlePtr, CharPtrPtr]],
  finalytics_ticker_get_summary_stats: ['int', [TickerHandlePtr, CharPtrPtr]],
  finalytics_ticker_get_price_history: ['int', [TickerHandlePtr, CharPtrPtr]],
  finalytics_ticker_get_options_chain: ['int', [TickerHandlePtr, CharPtrPtr]],
  finalytics_ticker_get_news: ['int', [TickerHandlePtr, CharPtrPtr]],
  finalytics_ticker_get_income_statement: ['int', [TickerHandlePtr, CharPtr, 'int', CharPtrPtr]],
  finalytics_ticker_get_balance_sheet: ['int', [TickerHandlePtr, CharPtr, 'int', CharPtrPtr]],
  finalytics_ticker_get_cashflow_statement: ['int', [TickerHandlePtr, CharPtr, 'int', CharPtrPtr]],
  finalytics_ticker_get_financial_ratios: ['int', [TickerHandlePtr, CharPtr, CharPtrPtr]],
  finalytics_ticker_volatility_surface: ['int', [TickerHandlePtr, CharPtrPtr]],
  finalytics_ticker_performance_stats: ['int', [TickerHandlePtr, CharPtrPtr]],
  finalytics_ticker_performance_chart: ['int', [TickerHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_ticker_candlestick_chart: ['int', [TickerHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_ticker_options_chart: ['int', [TickerHandlePtr, CharPtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_ticker_news_sentiment_chart: ['int', [TickerHandlePtr, 'uint', 'uint', CharPtrPtr]],
  finalytics_ticker_report: ['int', [TickerHandlePtr, CharPtr, CharPtrPtr]],
});

/**
 * Ticker class representing a single financial ticker with methods for retrieving data and analytics.
 */
class Ticker {
  /**
   * Creates a new Ticker instance.
   * @param {Buffer} handle - Opaque pointer to the underlying C TickerHandle.
   * @private
   */
  constructor(handle) {
    this.handle = handle;
  }

  /**
   * Retrieves the latest quote for the ticker.
   * @returns {Promise<Object>} A promise resolving to the quote data as a JSON object.
   * @throws {Error} If quote retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const quote = await ticker.getQuote();
   * console.log(quote);
   * ticker.free();
   */
  async getQuote() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_quote(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get quote: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(JSON.parse(output));
    });
  }

  /**
   * Retrieves summary technical and fundamental statistics for the ticker.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing summary statistics.
   * @throws {Error} If stats retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const summary = await ticker.getSummaryStats();
   * console.log(summary);
   * ticker.free();
   */
  async getSummaryStats() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_summary_stats(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get summary stats: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves OHLCV price history for the ticker.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing price history data.
   * @throws {Error} If price history retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').startDate('2023-01-01').endDate('2023-12-31').build();
   * const history = await ticker.getPriceHistory();
   * console.log(history);
   * ticker.free();
   */
  async getPriceHistory() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_price_history(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get price history: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the options chain for the ticker.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing options chain data.
   * @throws {Error} If options chain retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const options = await ticker.getOptionsChain();
   * console.log(options);
   * ticker.free();
   */
  async getOptionsChain() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_options_chain(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get options chain: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the latest news headlines for the ticker.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing news data.
   * @throws {Error} If news retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const news = await ticker.getNews();
   * console.log(news);
   * ticker.free();
   */
  async getNews() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_news(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get news: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the income statement for the ticker.
   * @param {string} frequency - The frequency of the statement ('annual' or 'quarterly').
   * @param {boolean} formatted - Whether to return the statement in a formatted manner.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing income statement data.
   * @throws {Error} If income statement retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const income = await ticker.getIncomeStatement('quarterly', true);
   * console.log(income);
   * ticker.free();
   */
  async getIncomeStatement(frequency, formatted) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_income_statement(this.handle, frequency, formatted ? 1 : 0, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get income statement: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the balance sheet for the ticker.
   * @param {string} frequency - The frequency of the statement ('annual' or 'quarterly').
   * @param {boolean} formatted - Whether to return the statement in a formatted manner.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing balance sheet data.
   * @throws {Error} If balance sheet retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const balance = await ticker.getBalanceSheet('quarterly', true);
   * console.log(balance);
   * ticker.free();
   */
  async getBalanceSheet(frequency, formatted) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_balance_sheet(this.handle, frequency, formatted ? 1 : 0, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get balance sheet: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the cash flow statement for the ticker.
   * @param {string} frequency - The frequency of the statement ('annual' or 'quarterly').
   * @param {boolean} formatted - Whether to return the statement in a formatted manner.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing cash flow statement data.
   * @throws {Error} If cash flow statement retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const cashflow = await ticker.getCashflowStatement('quarterly', true);
   * console.log(cashflow);
   * ticker.free();
   */
  async getCashflowStatement(frequency, formatted) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_cashflow_statement(this.handle, frequency, formatted ? 1 : 0, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get cash flow statement: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves financial ratios for the ticker.
   * @param {string} frequency - The frequency of the ratios ('annual' or 'quarterly').
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing financial ratios.
   * @throws {Error} If financial ratios retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const ratios = await ticker.getFinancialRatios('quarterly');
   * console.log(ratios);
   * ticker.free();
   */
  async getFinancialRatios(frequency) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_get_financial_ratios(this.handle, frequency, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get financial ratios: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves the volatility surface for the ticker.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing volatility surface data.
   * @throws {Error} If volatility surface retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const surface = await ticker.volatilitySurface();
   * console.log(surface);
   * ticker.free();
   */
  async volatilitySurface() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_volatility_surface(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get volatility surface: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves performance statistics for the ticker.
   * @returns {Promise<Object>} A promise resolving to a JSON object containing performance statistics.
   * @throws {Error} If performance stats retrieval fails.
   * @example
   * const ticker = await new TickerBuilder()
   *   .symbol('AAPL')
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * const stats = await ticker.performanceStats();
   * console.log(stats);
   * ticker.free();
   */
  async performanceStats() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_performance_stats(this.handle, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get performance stats: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(JSON.parse(output));
    });
  }

  /**
   * Retrieves the performance chart for the ticker.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the performance chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const ticker = await new TickerBuilder()
   *   .symbol('AAPL')
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * const chart = await ticker.performanceChart(600, 800);
   * chart.show();
   * ticker.free();
   */
  async performanceChart(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_performance_chart(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get performance chart: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves the candlestick chart for the ticker.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the candlestick chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const ticker = await new TickerBuilder()
   *   .symbol('AAPL')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .build();
   * const chart = await ticker.candlestickChart(600, 800);
   * chart.show();
   * ticker.free();
   */
  async candlestickChart(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_candlestick_chart(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get candlestick chart: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves the options chart for the ticker.
   * @param {string} chartType - The type of options chart to display.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the options chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const chart = await ticker.optionsChart('volatility', 600, 800);
   * chart.show();
   * ticker.free();
   */
  async optionsChart(chartType, height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_options_chart(this.handle, chartType, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get options chart: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves the news sentiment chart for the ticker.
   * @param {number} [height=0] - The height of the chart (0 for default).
   * @param {number} [width=0] - The width of the chart (0 for default).
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the news sentiment chart.
   * @throws {Error} If chart retrieval fails.
   * @example
   * const ticker = await new TickerBuilder().symbol('AAPL').build();
   * const chart = await ticker.newsSentimentChart(600, 800);
   * chart.show();
   * ticker.free();
   */
  async newsSentimentChart(height = 0, width = 0) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_news_sentiment_chart(this.handle, height, width, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get news sentiment chart: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Retrieves a comprehensive analytics report for the ticker.
   * @param {string} reportType - The type of report to display (e.g., 'performance', 'financials', 'options', 'news').
   * @returns {Promise<Chart>} A promise resolving to a Chart instance containing the report.
   * @throws {Error} If report retrieval fails.
   * @example
   * const ticker = await new TickerBuilder()
   *   .symbol('AAPL')
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * const report = await ticker.report('performance');
   * report.show();
   * ticker.free();
   */
  async report(reportType) {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_ticker_report(this.handle, reportType, outputPtr);
      if (result !== 0) {
        return reject(new Error(`Failed to get report: error code ${result}`));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(new Chart(output));
    });
  }

  /**
   * Releases resources associated with the Ticker.
   * Should be called when the Ticker is no longer needed to prevent memory leaks.
   */
  free() {
    if (this.handle) {
      lib.finalytics_ticker_free(this.handle);
      this.handle = null;
    }
  }
}

/**
 * TickerBuilder class for constructing Ticker instances using the builder pattern.
 */
class TickerBuilder {
  /**
   * Initializes a new TickerBuilder with default values.
   * Defaults:
   * - symbol: ''
   * - startDate: ''
   * - endDate: ''
   * - interval: '1d'
   * - benchmarkSymbol: ''
   * - confidenceLevel: 0.95
   * - riskFreeRate: 0.02
   * - tickerData: null
   * - benchmarkData: null
   */
  constructor() {
    this.symbolValue = '';
    this.startDateValue = '';
    this.endDateValue = '';
    this.intervalValue = '1d';
    this.benchmarkSymbolValue = '';
    this.confidenceLevelValue = 0.95;
    this.riskFreeRateValue = 0.02;
    this.tickerDataValue = null;
    this.benchmarkDataValue = null;
  }

  /**
   * Sets the ticker symbol.
   * @param {string} value - The ticker symbol (e.g., 'AAPL').
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  symbol(value) { this.symbolValue = value; return this; }

  /**
   * Sets the start date for the data period.
   * @param {string} value - The start date in YYYY-MM-DD format.
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  startDate(value) { this.startDateValue = value; return this; }

  /**
   * Sets the end date for the data period.
   * @param {string} value - The end date in YYYY-MM-DD format.
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  endDate(value) { this.endDateValue = value; return this; }

  /**
   * Sets the data interval.
   * @param {string} value - The data interval (e.g., '2m', '5m', '15m', '30m', '1h', '1d', '1wk', '1mo', '3mo').
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  interval(value) { this.intervalValue = value; return this; }

  /**
   * Sets the benchmark symbol.
   * @param {string} value - The benchmark symbol (e.g., '^GSPC').
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  benchmarkSymbol(value) { this.benchmarkSymbolValue = value; return this; }

  /**
   * Sets the confidence level for VaR and ES calculations.
   * @param {number} value - The confidence level (e.g., 0.95 for 95% confidence).
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  confidenceLevel(value) { this.confidenceLevelValue = value; return this; }

  /**
   * Sets the risk-free rate for calculations.
   * @param {number} value - The risk-free rate (e.g., 0.02 for 2%).
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  riskFreeRate(value) { this.riskFreeRateValue = value; return this; }

  /**
   * Sets custom ticker data.
   * @param {Polars.DataFrame|null} value - A Polars DataFrame containing custom ticker data (null if not used).
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  tickerData(value) { this.tickerDataValue = value; return this; }

  /**
   * Sets custom benchmark data.
   * @param {Polars.DataFrame|null} value - A Polars DataFrame containing custom benchmark data (null if not used).
   * @returns {TickerBuilder} The builder instance for method chaining.
   */
  benchmarkData(value) { this.benchmarkDataValue = value; return this; }

  /**
   * Constructs the Ticker instance with the configured parameters.
   * The symbol parameter is required; other parameters are optional and use defaults if not set.
   * @returns {Promise<Ticker>} A promise resolving to the initialized Ticker object.
   * @throws {Error} If Ticker creation fails or symbol is missing.
   * @example
   * const ticker = await new TickerBuilder()
   *   .symbol('AAPL')
   *   .benchmarkSymbol('^GSPC')
   *   .startDate('2023-01-01')
   *   .endDate('2023-12-31')
   *   .interval('1d')
   *   .riskFreeRate(0.02)
   *   .build();
   * ticker.free();
   */
  async build() {
    if (!this.symbolValue) throw new Error('Symbol is required');
    const tickerDataJson = this.tickerDataValue ? dfToJSON(this.tickerDataValue) : '';
    const benchmarkDataJson = this.benchmarkDataValue ? dfToJSON(this.benchmarkDataValue) : '';
    return new Promise((resolve, reject) => {
      const handle = lib.finalytics_ticker_new(
        this.symbolValue,
        this.startDateValue,
        this.endDateValue,
        this.intervalValue,
        this.benchmarkSymbolValue,
        this.confidenceLevelValue,
        this.riskFreeRateValue,
        tickerDataJson || null,
        benchmarkDataJson || null
      );
      if (!handle || handle.isNull()) {
        return reject(new Error('Failed to create Ticker'));
      }
      resolve(new Ticker(handle));
    });
  }
}

export { Ticker, TickerBuilder };
