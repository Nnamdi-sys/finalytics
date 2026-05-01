import ffi from "@2060.io/ffi-napi";
import ref from "ref-napi";
import Polars from "nodejs-polars";
import { Chart, getNativeLibPath, getLastError } from "./utils.js";

// Define C types
const ScreenerHandle = ref.types.void; // Opaque pointer
const ScreenerHandlePtr = ref.refType(ScreenerHandle);
const CharPtr = ref.types.CString;
const CharPtrPtr = ref.refType(CharPtr);

// Load the finalytics library
const lib = ffi.Library(getNativeLibPath(), {
  finalytics_screener_new: [
    ScreenerHandlePtr,
    [CharPtr, CharPtr, CharPtr, "int", "int", "int"],
  ],
  finalytics_screener_free: ["void", [ScreenerHandlePtr]],
  finalytics_free_string: ["void", [CharPtr]],
  finalytics_screener_symbols: ["int", [ScreenerHandlePtr, CharPtrPtr]],
  finalytics_screener_overview: ["int", [ScreenerHandlePtr, CharPtrPtr]],
  finalytics_screener_metrics: ["int", [ScreenerHandlePtr, CharPtrPtr]],
  finalytics_screener_display: ["int", [ScreenerHandlePtr, CharPtrPtr]],
});

/**
 * Screener class for filtering securities based on asset type, conditions, and sorting criteria.
 */
class Screener {
  /**
   * Creates a new Screener instance.
   * @param {Buffer} handle - Opaque pointer to the underlying C ScreenerHandle.
   * @private
   */
  constructor(handle) {
    this.handle = handle;
  }

  /**
   * Creates a new Screener instance with the specified parameters.
   * @param {string} assetType - The type of asset to screen (e.g., 'EQUITY', 'ETF', 'CRYPTO').
   * @param {string[]} conditions - Array of JSON strings defining filter conditions (e.g., ['{"operator":"eq","operands":["exchange","NMS"]}', '{"operator":"gte","operands":["intradaymarketcap",10000000000]}']).
   * @param {string} sortBy - The field to sort results by (e.g., 'intradaymarketcap').
   * @param {boolean} descending - Whether to sort in ascending order (false) or descending order (true).
   * @param {number} offset - The starting index for pagination (e.g., 0).
   * @param {number} limit - The maximum number of results to return (e.g., 10).
   * @returns {Promise<Screener>} A promise resolving to the initialized Screener object.
   * @throws {Error} If Screener creation fails or assetType is missing.
   * @example
   * const screener = await Screener.new(
   *   'EQUITY',
   *   [
   *     JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] }),
   *     JSON.stringify({ operator: 'gte', operands: ['intradaymarketcap', 10000000000] })
   *   ],
   *   'intradaymarketcap',
   *   true,
   *   0,
   *   10
   * );
   * screener.free();
   */
  static async new(assetType, conditions, sortBy, descending, offset, limit) {
    if (!assetType) throw new Error("assetType is required");
    const conditionsJson = JSON.stringify(conditions);
    return new Promise((resolve, reject) => {
      const handle = lib.finalytics_screener_new(
        assetType,
        conditionsJson,
        sortBy,
        descending ? 1 : 0,
        offset,
        limit,
      );
      if (!handle || handle.isNull()) {
        return reject(getLastError("Failed to create Screener"));
      }
      resolve(new Screener(handle));
    });
  }

  /**
   * Retrieves the symbols matching the screener criteria.
   * @returns {Promise<Object>} A promise resolving to a JSON object containing the screened symbols.
   * @throws {Error} If symbols retrieval fails.
   * @example
   * const screener = await Screener.new(
   * 'EQUITY',
   * [
   *  JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] })
   * ],
   * 'intradaymarketcap',
   * true,
   * 0,
   * 10);
   * const symbols = await screener.symbols();
   * console.log(symbols);
   * screener.free();
   */
  async symbols() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_screener_symbols(this.handle, outputPtr);
      if (result !== 0) {
        return reject(getLastError("Failed to get symbols"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(JSON.parse(output));
    });
  }

  /**
   * Retrieves an overview of the screened securities.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing overview data.
   * @throws {Error} If overview retrieval fails.
   * @example
   * const screener = await Screener.new(
   * 'EQUITY',
   * [
   *  JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] })
   * ],
   * 'intradaymarketcap',
   * true,
   * 0,
   * 10);
   * const overview = await screener.overview();
   * console.log(overview);
   * screener.free();
   */
  async overview() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_screener_overview(this.handle, outputPtr);
      if (result !== 0) {
        return reject(getLastError("Failed to get overview"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Retrieves detailed metrics for the screened securities.
   * @returns {Promise<Polars.DataFrame>} A promise resolving to a Polars DataFrame containing screener metrics.
   * @throws {Error} If metrics retrieval fails.
   * @example
   * const screener = await Screener.new(
   * 'EQUITY',
   * [
   *  JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] })
   * ],
   * 'intradaymarketcap',
   * true,
   * 0,
   * 10);
   * const metrics = await screener.metrics();
   * console.log(metrics);
   * screener.free();
   */
  async metrics() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_screener_metrics(this.handle, outputPtr);
      if (result !== 0) {
        return reject(getLastError("Failed to get screener metrics"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      resolve(Polars.readJSON(Buffer.from(output)));
    });
  }

  /**
   * Releases resources associated with the Screener.
   * Should be called when the Screener is no longer needed to prevent memory leaks.
   * @example
   * const screener = await Screener.new(
   * 'EQUITY',
   * [
   *  JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] })
   * ],
   * 'intradaymarketcap',
   * true,
   * 0,
   * 10);
   * screener.free();
   */
  free() {
    if (this.handle) {
      lib.finalytics_screener_free(this.handle);
      this.handle = null;
    }
  }

  /**
   * Displays the screener overview and metrics as an HTML report in the default browser.
   * @returns {Promise<void>} A promise that resolves when the report has been opened.
   * @throws {Error} If the display retrieval fails.
   * @example
   * const screener = await new ScreenerBuilder()
   *   .quoteType('EQUITY')
   *   .addFilter({ operator: 'eq', operands: ['exchange', 'NMS'] })
   *   .sortField('intradaymarketcap')
   *   .size(10)
   *   .build();
   * await screener.display();
   * screener.free();
   */
  async display() {
    return new Promise((resolve, reject) => {
      const outputPtr = ref.alloc(CharPtrPtr);
      const result = lib.finalytics_screener_display(this.handle, outputPtr);
      if (result !== 0) {
        return reject(getLastError("Failed to get screener display"));
      }
      const output = ref.readCString(outputPtr.deref(), 0);
      lib.finalytics_free_string(outputPtr.deref());
      const parsed = JSON.parse(output);
      const html = `<html><body>${parsed.overview_html || ""}${parsed.metrics_html || ""}</body></html>`;
      const chart = new Chart(html);
      chart.show().then(resolve).catch(reject);
    });
  }
}

/**
 * Builder class for constructing Screener instances with a fluent, chainable API.
 *
 * Defaults:
 * - quoteType: 'EQUITY'
 * - filters: []
 * - sortField: ''
 * - sortDescending: true
 * - offset: 0
 * - size: 250
 *
 * @example
 * const screener = await new ScreenerBuilder()
 *   .quoteType('EQUITY')
 *   .addFilter({ operator: 'eq', operands: ['exchange', 'NMS'] })
 *   .addFilter({ operator: 'gte', operands: ['intradaymarketcap', 10000000000] })
 *   .sortField('intradaymarketcap')
 *   .sortDescending(true)
 *   .offset(0)
 *   .size(10)
 *   .build();
 * screener.free();
 */
class ScreenerBuilder {
  /**
   * Initializes a new ScreenerBuilder with default values.
   */
  constructor() {
    this.quoteTypeValue = "EQUITY";
    this.filtersValue = [];
    this.sortFieldValue = "";
    this.sortDescendingValue = true;
    this.offsetValue = 0;
    this.sizeValue = 250;
  }

  /**
   * Sets the asset/quote type to screen.
   * @param {string} value - The quote type (e.g., 'EQUITY', 'ETF', 'CRYPTO', 'MUTUALFUND', 'INDEX', 'FUTURE').
   * @returns {ScreenerBuilder} The builder instance for method chaining.
   */
  quoteType(value) {
    this.quoteTypeValue = value;
    return this;
  }

  /**
   * Adds a filter condition to the screener.
   * Accepts either a plain JavaScript object or a JSON string.
   * Can be called multiple times to add multiple filters.
   * @param {Object|string} value - A filter object (e.g., { operator: 'eq', operands: ['exchange', 'NMS'] }) or its JSON string representation.
   * @returns {ScreenerBuilder} The builder instance for method chaining.
   */
  addFilter(value) {
    if (typeof value === "string") {
      this.filtersValue.push(value);
    } else {
      this.filtersValue.push(JSON.stringify(value));
    }
    return this;
  }

  /**
   * Sets the field to sort results by.
   * @param {string} value - The sort field metric (e.g., 'intradaymarketcap').
   * @returns {ScreenerBuilder} The builder instance for method chaining.
   */
  sortField(value) {
    this.sortFieldValue = value;
    return this;
  }

  /**
   * Sets whether to sort in descending order.
   * @param {boolean} value - true for descending, false for ascending.
   * @returns {ScreenerBuilder} The builder instance for method chaining.
   */
  sortDescending(value) {
    this.sortDescendingValue = value;
    return this;
  }

  /**
   * Sets the starting index for pagination.
   * @param {number} value - The offset (e.g., 0).
   * @returns {ScreenerBuilder} The builder instance for method chaining.
   */
  offset(value) {
    this.offsetValue = value;
    return this;
  }

  /**
   * Sets the maximum number of results to return.
   * @param {number} value - The result limit (e.g., 10).
   * @returns {ScreenerBuilder} The builder instance for method chaining.
   */
  size(value) {
    this.sizeValue = value;
    return this;
  }

  /**
   * Constructs the Screener instance with the configured parameters.
   * @returns {Promise<Screener>} A promise resolving to the initialized Screener object.
   * @throws {Error} If Screener creation fails.
   * @example
   * const screener = await new ScreenerBuilder()
   *   .quoteType('EQUITY')
   *   .addFilter({ operator: 'eq', operands: ['exchange', 'NMS'] })
   *   .addFilter({ operator: 'eq', operands: ['sector', 'Technology'] })
   *   .addFilter({ operator: 'gte', operands: ['intradaymarketcap', 10000000000] })
   *   .addFilter({ operator: 'gte', operands: ['returnonequity.lasttwelvemonths', 0.15] })
   *   .sortField('intradaymarketcap')
   *   .sortDescending(true)
   *   .offset(0)
   *   .size(10)
   *   .build();
   * screener.free();
   */
  async build() {
    return Screener.new(
      this.quoteTypeValue,
      this.filtersValue,
      this.sortFieldValue,
      this.sortDescendingValue,
      this.offsetValue,
      this.sizeValue,
    );
  }
}

export { Screener, ScreenerBuilder };
