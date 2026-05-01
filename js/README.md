![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![NPM Version](https://img.shields.io/npm/v/finalytics)](https://www.npmjs.com/package/finalytics)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)

---

# Finalytics JavaScript Binding

**Finalytics** is a high-performance JavaScript (ESM) binding for the Finalytics Rust library, designed for retrieving financial data, security analysis, and portfolio optimization.
It provides a fast, modular interface for advanced analytics, and powers dashboards and applications across platforms.

---

## 🚀 Installation

To install the Finalytics JavaScript binding, add it to your Node.js project using:

```bash
npm install finalytics
```

After installing the library, **download the required native binary** by running:
```bash
curl -O https://raw.githubusercontent.com/Nnamdi-sys/finalytics/refs/heads/main/js/download_binaries.sh
bash download_binaries.sh
```

---

## 📦 Main Modules

Finalytics JavaScript exposes five core modules for financial analytics:

### 1. Screener

Efficiently filter and rank securities using advanced metrics and custom filters.

**Usage Example:**
```javascript
import { ScreenerBuilder } from 'finalytics';

const screener = await new ScreenerBuilder()
  .quoteType('EQUITY')
  .addFilter({ operator: 'eq', operands: ['exchange', 'NMS'] })
  .addFilter({ operator: 'eq', operands: ['sector', 'Technology'] })
  .addFilter({ operator: 'gte', operands: ['intradaymarketcap', 10000000000] })
  .addFilter({ operator: 'gte', operands: ['returnonequity.lasttwelvemonths', 0.15] })
  .sortField('intradaymarketcap')
  .sortDescending(true)
  .offset(0)
  .size(10)
  .build();

const overview = await screener.overview();
console.log(overview.toString());
const metrics = await screener.metrics();
console.log(metrics.toString());
await screener.display();
screener.free();
```

---

### 2. Ticker

Analyze a single security in depth: performance, financials, options, news, and more.

**Usage Example:**
```javascript
import { TickerBuilder } from 'finalytics';

const ticker = await new TickerBuilder()
  .symbol('AAPL')
  .startDate('2023-01-01')
  .endDate('2024-12-31')
  .interval('1d')
  .benchmarkSymbol('^GSPC')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .build();

for (const reportType of ['performance', 'financials', 'options', 'news']) {
  const report = await ticker.report(reportType);
  await report.show();
}
ticker.free();
```

---

### 3. Tickers

Work with multiple securities at once—aggregate reports, batch analytics, and portfolio construction.

**Usage Example:**
```javascript
import { TickersBuilder } from 'finalytics';

const tickers = await new TickersBuilder()
  .symbols(['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'])
  .startDate('2023-01-01')
  .endDate('2024-12-31')
  .interval('1d')
  .benchmarkSymbol('^GSPC')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .build();

const report = await tickers.report('performance');
await report.show();
tickers.free();
```

---

### 4. Portfolio

Optimize and analyze portfolios using advanced objective functions and constraints.
Supports rebalancing strategies, scheduled cash flows (DCA), ad-hoc transactions,
and out-of-sample evaluation.

**Objective Functions:**
`max_sharpe`, `max_sortino`, `max_return`, `min_vol`, `min_var`, `min_cvar`, `min_drawdown`,
`risk_parity`, `max_diversification`, `hierarchical_risk_parity`

**Usage Example: Optimization with Out-of-Sample Evaluation**
```javascript
import { PortfolioBuilder } from 'finalytics';

// Optimize on 2023 - 2024 data (in-sample)
const portfolio = await new PortfolioBuilder()
  .tickerSymbols(['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'])
  .benchmarkSymbol('^GSPC')
  .startDate('2023-01-01')
  .endDate('2024-12-31')
  .interval('1d')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .objectiveFunction('max_sharpe')
  .build();

let report = await portfolio.report('optimization');
await report.show();

// Update to 2025 data for out-of-sample evaluation
await portfolio.updateDates('2025-01-01', '2026-01-01');
await portfolio.performanceStats();
report = await portfolio.report('performance');
await report.show();

portfolio.free();
```

**Usage Example: Explicit Allocation with Rebalancing and DCA**
```javascript
import { PortfolioBuilder } from 'finalytics';

const portfolio = await new PortfolioBuilder()
  .tickerSymbols(['AAPL', 'MSFT', 'NVDA', 'BTC-USD'])
  .benchmarkSymbol('^GSPC')
  .startDate('2023-01-01')
  .endDate('2024-12-31')
  .interval('1d')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .weights(JSON.stringify([25000.0, 25000.0, 25000.0, 25000.0]))
  .rebalanceStrategy(JSON.stringify({ type: 'calendar', frequency: 'quarterly' }))
  .scheduledCashFlows(JSON.stringify([
    {
      amount: 2000.0,
      frequency: 'monthly',
      start_date: null,
      end_date: null,
      allocation: 'pro_rata',
    },
  ]))
  .build();

const report = await portfolio.report('performance');
await report.show();
portfolio.free();
```

**Usage Example: Optimization with Weight & Categorical Constraints**
```javascript
import { PortfolioBuilder } from 'finalytics';

const assetConstraints = JSON.stringify([
  [0.05, 0.40], // AAPL
  [0.05, 0.40], // MSFT
  [0.05, 0.40], // NVDA
  [0.05, 0.30], // JPM
  [0.05, 0.20], // XOM
  [0.05, 0.25], // BTC-USD
]);

const categoricalConstraints = JSON.stringify([
  {
    name: 'Sector',
    category_per_symbol: ['Tech', 'Tech', 'Tech', 'Finance', 'Energy', 'Crypto'],
    weight_per_category: [
      ['Tech',    0.30, 0.60],
      ['Finance', 0.05, 0.30],
      ['Energy',  0.05, 0.20],
      ['Crypto',  0.05, 0.25],
    ],
  },
  {
    name: 'Asset Class',
    category_per_symbol: ['Equity', 'Equity', 'Equity', 'Equity', 'Equity', 'Crypto'],
    weight_per_category: [
      ['Equity', 0.70, 0.95],
      ['Crypto', 0.05, 0.30],
    ],
  },
]);

const portfolio = await new PortfolioBuilder()
  .tickerSymbols(['AAPL', 'MSFT', 'NVDA', 'JPM', 'XOM', 'BTC-USD'])
  .benchmarkSymbol('^GSPC')
  .startDate('2023-01-01')
  .endDate('2024-12-31')
  .interval('1d')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .objectiveFunction('max_sharpe')
  .assetConstraints(assetConstraints)
  .categoricalConstraints(categoricalConstraints)
  .build();

const report = await portfolio.report('optimization');
await report.show();
portfolio.free();
```

---

### 5. Custom Data

Load your own price data from CSV files as DataFrames and use it with any Finalytics module.
CSV files must have columns: `timestamp` (unix epoch), `open`, `high`, `low`, `close`, `volume`, `adjclose`.

**Usage Example:**
```javascript
import fs from 'fs';
import Polars from 'nodejs-polars';
import { TickerBuilder, TickersBuilder, PortfolioBuilder } from 'finalytics';

// Load data from CSV files
const files = {
  aapl: 'examples/datasets/aapl.csv',
  msft: 'examples/datasets/msft.csv',
  nvda: 'examples/datasets/nvda.csv',
  goog: 'examples/datasets/goog.csv',
  btcusd: 'examples/datasets/btcusd.csv',
  gspc: 'examples/datasets/gspc.csv',
};
const dataFrames = {};
for (const [name, path] of Object.entries(files)) {
  dataFrames[name] = Polars.readCSV(fs.readFileSync(path, 'utf8'));
}

// Single Ticker from custom data
const ticker = await new TickerBuilder()
  .symbol('AAPL')
  .benchmarkSymbol('^GSPC')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .tickerData(dataFrames['aapl'])
  .benchmarkData(dataFrames['gspc'])
  .build();

let report = await ticker.report('performance');
await report.show();
ticker.free();

// Multiple Tickers from custom data
const tickersData = [
  dataFrames['nvda'], dataFrames['goog'], dataFrames['aapl'],
  dataFrames['msft'], dataFrames['btcusd'],
];
const tickers = await new TickersBuilder()
  .symbols(['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'])
  .benchmarkSymbol('^GSPC')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .tickersData(tickersData)
  .benchmarkData(dataFrames['gspc'])
  .build();

report = await tickers.report('performance');
await report.show();
tickers.free();

// Portfolio optimization from custom data
const portfolio = await new PortfolioBuilder()
  .tickerSymbols(['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'])
  .benchmarkSymbol('^GSPC')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .objectiveFunction('max_sharpe')
  .tickersData(tickersData)
  .benchmarkData(dataFrames['gspc'])
  .build();

report = await portfolio.report('optimization');
await report.show();
portfolio.free();
```

---

## 📚 Documentation

- See the [npm package documentation](https://www.npmjs.com/package/finalytics) for full details.

---

## 🗂️ Multi-language Bindings

Finalytics is also available in:
- [Rust](../rust/README.md)
- [Python](../python/README.md)
- [Go](../go/README.md)
- [Web Application](../web/README.md)

---

**Finalytics** — Modular, high-performance financial analytics for JavaScript.
