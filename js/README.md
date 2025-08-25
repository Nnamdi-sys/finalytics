![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)
[![NPM Version](https://img.shields.io/npm/v/finalytics)](https://www.npmjs.com/package/finalytics)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)

This is a JavaScript (ESM) binding for the [Finalytics Rust Library](https://github.com/Nnamdi-sys/finalytics), designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

To install the Finalytics JavaScript binding, add it to your Node.js project using:

```bash
npm install finalytics
```

## Example

View the [npm package documentation](https://www.npmjs.com/package/finalytics) for more information. You can also check the [index.js file](https://github.com/Nnamdi-sys/finalytics/blob/main/js/index.js) for more usage examples.

```javascript
import { Screener, TickersBuilder } from 'finalytics';

async function main() {
  console.log('=== Finalytics Example ===');

  let screener;
  try {
    screener = await Screener.new(
      'EQUITY',
      [
        JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] }),
        JSON.stringify({ operator: 'gte', operands: ['intradaymarketcap', 10000000000] }),
      ],
      'intradaymarketcap',
      true,
      0,
      10
    );
  } catch (err) {
    console.error('Error creating Screener:', err.message);
    return;
  }

  let symbols;
  try {
    symbols = await screener.symbols();
    console.log('Screened Symbols:', symbols);
  } catch (err) {
    console.error('Failed to get symbols:', err.message);
    screener.free();
    return;
  }

  let tickers;
  try {
    tickers = await new TickersBuilder()
      .symbols(symbols)
      .startDate('2023-01-01')
      .endDate('2024-12-31')
      .interval('1d')
      .benchmarkSymbol('^GSPC')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .build();
  } catch (err) {
    console.error('Failed to create Tickers:', err.message);
    screener.free();
    return;
  }

  if (symbols.length > 0) {
    let ticker;
    try {
      ticker = await tickers.getTicker(symbols[0]);
      for (const reportType of ['performance', 'financials', 'options', 'news']) {
        try {
          const report = await ticker.report(reportType);
          console.log(`Ticker ${reportType} report: Opening in browser...`);
          await report.show();
        } catch (err) {
          console.error(`Failed to get ${reportType} report:`, err.message);
        }
      }
    } catch (err) {
      console.error('Failed to get Ticker:', err.message);
    } finally {
      if (ticker) ticker.free();
    }
  }

  try {
    const tickersReport = await tickers.report('performance');
    console.log('Tickers report: Opening in browser...');
    await tickersReport.show();
  } catch (err) {
    console.error('Failed to get Tickers report:', err.message);
  }

  let portfolio;
  try {
    portfolio = await tickers.optimize('max_sharpe', '{}', '{}', '{}');
    const portfolioReport = await portfolio.report('performance');
    console.log('Portfolio report: Opening in browser...');
    await portfolioReport.show();
  } catch (err) {
    console.error('Failed to optimize portfolio or get report:', err.message);
  } finally {
    if (portfolio) portfolio.free();
  }

  tickers.free();
  screener.free();
}

main();