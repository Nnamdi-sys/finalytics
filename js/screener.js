import ffi from '@2060.io/ffi-napi';
import ref from 'ref-napi';
import Polars from 'nodejs-polars';
import { Chart, getNativeLibPath } from './utils.js';

// Define C types
const ScreenerHandle = ref.types.void; // Opaque pointer
const ScreenerHandlePtr = ref.refType(ScreenerHandle);
const CharPtr = ref.types.CString;
const CharPtrPtr = ref.refType(CharPtr);

// Load the finalytics library
const lib = ffi.Library(getNativeLibPath(), {
  finalytics_screener_new: [ScreenerHandlePtr, [CharPtr, CharPtr, CharPtr, 'int', 'int', 'int']],
  finalytics_screener_free: ['void', [ScreenerHandlePtr]],
  finalytics_free_string: ['void', [CharPtr]],
  finalytics_screener_symbols: ['int', [ScreenerHandlePtr, CharPtrPtr]],
  finalytics_screener_overview: ['int', [ScreenerHandlePtr, CharPtrPtr]],
  finalytics_screener_metrics: ['int', [ScreenerHandlePtr, CharPtrPtr]],
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
    if (!assetType) throw new Error('assetType is required');
    const conditionsJson = JSON.stringify(conditions);
    return new Promise((resolve, reject) => {
      const handle = lib.finalytics_screener_new(
        assetType,
        conditionsJson,
        sortBy,
        descending ? 1 : 0,
        offset,
        limit
      );
      if (!handle || handle.isNull()) {
        return reject(new Error('Failed to create Screener'));
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
        return reject(new Error(`Failed to get symbols: error code ${result}`));
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
        return reject(new Error(`Failed to get overview: error code ${result}`));
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
        return reject(new Error(`Failed to get screener metrics: error code ${result}`));
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
}

export { Screener };