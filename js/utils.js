import fs from 'fs';
import os from 'os';
import path from "path";
import open from "open";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

/**
 * Chart class for rendering HTML content in the default web browser.
 */
class Chart {
  /**
   * Creates a new Chart instance.
   * @param {string} contents - HTML string to be displayed.
   */
  constructor(contents) {
    this.contents = contents;
  }

  /**
   * Displays the HTML content in the default web browser.
   * Writes the HTML to a temporary file and opens it.
   * @throws {Error} If writing the file or opening the browser fails.
   */
  async show() {
    const tempFile = path.join(__dirname, `temp_${Date.now()}.html`);
    try {
      await fs.promises.writeFile(tempFile, this.contents);
      await open(tempFile);
    } catch (err) {
      throw new Error(`Failed to display HTML: ${err.message}`);
    } finally {
      // Clean up the temporary file after a delay
      setTimeout(async () => {
        try {
          await fs.promises.unlink(tempFile);
        } catch (err) {
          console.warn(
            `Failed to delete temporary file ${tempFile}: ${err.message}`,
          );
        }
      }, 5000); // 5-second delay to ensure browser opens
    }
  }
}

/**
 * Ensures all values in float columns are serialized with one decimal place if they have no decimal.
 * @param {Object} data - Column-oriented JS object from df.toObject().
 * @param {Object} columnTypes - Object mapping column names to their types (e.g., 'Float64', 'Int64').
 * @returns {Object} - New object with float columns coerced.
 */
 function ensureFloatColumnsWithDecimal(data, columnTypes) {
   const result = {};
   for (const col of Object.keys(data)) {
     if (columnTypes[col] && columnTypes[col].toString().toLowerCase().includes('float')) {
       result[col] = data[col].map(v => {
         if (typeof v === 'number' && Number.isInteger(v)) {
           // Value is a float column but has no decimal, so force decimal
           return v.toFixed(1);
         }
         return v;
       });
     } else {
       result[col] = data[col];
     }
   }
   return result;
 }

/**
 * Converts a Polars DataFrame to a column-oriented JSON string.
 * @param {DataFrame} df - The Polars DataFrame.
 * @returns {string} - The column-oriented JSON string.
 */
 function dfToJSON(df) {
   const columnOrientedData = df.toObject();
   const columnTypes = {};
   df.columns.forEach((col, idx) => {
     columnTypes[col] = df.dtypes[idx];
   });
   const fixedData = ensureFloatColumnsWithDecimal(columnOrientedData, columnTypes);
   return JSON.stringify(fixedData, null, 2);
 }
 
 function getNativeLibPath() {
   const platform = os.platform();
   const arch = os.arch();
 
   let candidatePaths = [];
 
   if (platform === 'darwin') {
     const dylib = arch === 'arm64'
       ? 'libfinalytics_ffi_aarch64.dylib'
       : 'libfinalytics_ffi_x86_64.dylib';
     candidatePaths = [
       path.join(__dirname, 'lib', 'macos', dylib),
       path.join(__dirname, dylib),
       path.join(process.cwd(), dylib)
     ];
   } else if (platform === 'win32') {
     candidatePaths = [
       path.join(__dirname, 'lib', 'windows', 'finalytics_ffi.dll'),
       path.join(__dirname, 'finalytics_ffi.dll'),
       path.join(process.cwd(), 'finalytics_ffi.dll')
     ];
   } else if (platform === 'linux') {
     candidatePaths = [
       path.join(__dirname, 'lib', 'linux', 'libfinalytics_ffi.so'),
       path.join(__dirname, 'libfinalytics_ffi.so'),
       path.join(process.cwd(), 'libfinalytics_ffi.so')
     ];
   } else {
     throw new Error(`Unsupported platform: ${platform}`);
   }
 
   // Return the first candidate that exists
   for (const candidate of candidatePaths) {
     if (fs.existsSync(candidate)) {
       return candidate;
     }
   }
 
   // Fallback: let ffi-napi search by name (if in system path)
   if (platform === 'darwin') {
     return 'libfinalytics_ffi.dylib';
   } else if (platform === 'win32') {
     return 'finalytics_ffi.dll';
   } else if (platform === 'linux') {
     return 'libfinalytics_ffi.so';
   }
   
   throw new Error('Native library not found for platform: ' + platform);
 }

export { Chart, dfToJSON, getNativeLibPath };
