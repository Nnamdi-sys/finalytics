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

Finalytics JavaScript exposes four core modules for financial analytics:

### 1. Screener

Efficiently filter and rank securities (equities, crypto, etc.) using advanced metrics and custom filters.

**Usage Example:**
```javascript
import { Screener } from 'finalytics';

const screener = await Screener.new(
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

await screener.display();
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

const symbols = ['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'];
const tickers = await new TickersBuilder()
  .symbols(symbols)
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

**Usage Example:**
```javascript
const symbols = ['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'];
const portfolio = await new PortfolioBuilder()
  .symbols(symbols)
  .benchmarkSymbol('^GSPC')
  .startDate('2023-01-01')
  .endDate('2024-12-31')
  .interval('1d')
  .confidenceLevel(0.95)
  .riskFreeRate(0.02)
  .objectiveFunction('max_sharpe')
  .build();

const report = await portfolio.report('performance');
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
