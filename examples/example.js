// Finalytics — JavaScript Examples
//
// Installation
// ────────────
//   npm install finalytics
//
//   # Download the required native binary:
//   curl -O https://raw.githubusercontent.com/Nnamdi-sys/finalytics/refs/heads/main/js/download_binaries.sh
//   bash download_binaries.sh
//
// Full docs: https://www.npmjs.com/package/finalytics
//
// Run this example (from the repo root)
// ───────────────────────────────────────
//   bash examples/example.sh js

import {
  TickerBuilder,
  TickersBuilder,
  PortfolioBuilder,
  ScreenerBuilder,
} from "finalytics";
import Polars from "nodejs-polars";
import fs from "fs";

// ── 1. Screener — Large-Cap NASDAQ Technology Stocks with ROE >= 15% ────────

async function screener() {
  console.log("=== 1. Screener ===");

  let s;
  try {
    s = await new ScreenerBuilder()
      .quoteType("EQUITY")
      .addFilter({ operator: "eq", operands: ["exchange", "NMS"] })
      .addFilter({ operator: "eq", operands: ["sector", "Technology"] })
      .addFilter({
        operator: "gte",
        operands: ["intradaymarketcap", 10000000000],
      })
      .addFilter({
        operator: "gte",
        operands: ["returnonequity.lasttwelvemonths", 0.15],
      })
      .sortField("intradaymarketcap")
      .sortDescending(true)
      .offset(0)
      .size(10)
      .build();
  } catch (err) {
    console.error("Error creating Screener:", err.message);
    return;
  }

  try {
    const symbols = await s.symbols();
    console.log("Symbols:", symbols);

    const overview = await s.overview();
    console.log("Overview:", overview.toString());

    const metrics = await s.metrics();
    console.log("Metrics:", metrics.toString());

    await s.display();
  } catch (err) {
    console.error("Error in Screener methods:", err.message);
  } finally {
    s.free();
  }
}

// ── 2. Ticker — Single security analysis with all report types ───────────────

async function ticker() {
  console.log("=== 2. Ticker ===");

  let t;
  try {
    t = await new TickerBuilder()
      .symbol("AAPL")
      .startDate("2023-01-01")
      .endDate("2024-12-31")
      .interval("1d")
      .benchmarkSymbol("^GSPC")
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .build();
  } catch (err) {
    console.error("Error creating Ticker:", err.message);
    return;
  }

  for (const reportType of ["performance", "financials", "options", "news"]) {
    try {
      const report = await t.report(reportType);
      console.log(`Ticker ${reportType} report: Opening in browser...`);
      await report.show();
    } catch (err) {
      console.error(`Error in ${reportType} report:`, err.message);
    }
  }

  t.free();
}

// ── 3. Tickers — Multiple securities analysis ────────────────────────────────

async function tickers() {
  console.log("=== 3. Tickers ===");

  let ts;
  try {
    ts = await new TickersBuilder()
      .symbols(["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
      .startDate("2023-01-01")
      .endDate("2024-12-31")
      .interval("1d")
      .benchmarkSymbol("^GSPC")
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .build();
  } catch (err) {
    console.error("Error creating Tickers:", err.message);
    return;
  }

  try {
    const report = await ts.report("performance");
    console.log("Tickers performance report: Opening in browser...");
    await report.show();
  } catch (err) {
    console.error("Error in Tickers report:", err.message);
  }

  ts.free();
}

// ── 4. Portfolio — Optimization with Out-of-Sample Evaluation ───────────────

async function portfolioOptimizationOOS() {
  console.log(
    "=== 4. Portfolio — Optimization with Out-of-Sample Evaluation ===",
  );

  let p;
  try {
    // Optimize on 2023-2024 data (in-sample)
    p = await new PortfolioBuilder()
      .tickerSymbols(["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
      .benchmarkSymbol("^GSPC")
      .startDate("2023-01-01")
      .endDate("2024-12-31")
      .interval("1d")
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .objectiveFunction("max_sharpe")
      .build();
  } catch (err) {
    console.error("Error creating Portfolio:", err.message);
    return;
  }

  try {
    let report = await p.report("optimization");
    console.log("Optimization report: Opening in browser...");
    await report.show();

    // Update to 2025 data for out-of-sample evaluation
    await p.updateDates("2025-01-01", "2026-01-01");
    await p.performanceStats();

    report = await p.report("performance");
    console.log("Out-of-sample performance report: Opening in browser...");
    await report.show();
  } catch (err) {
    console.error("Error in Portfolio OOS evaluation:", err.message);
  }

  p.free();
}

// ── 5. Portfolio — Optimization with Weight & Categorical Constraints ─────────

async function portfolioOptimizationConstraints() {
  console.log(
    "=== 5. Portfolio — Optimization with Weight & Categorical Constraints ===",
  );

  const assetConstraints = JSON.stringify([
    [0.05, 0.4], // AAPL
    [0.05, 0.4], // MSFT
    [0.05, 0.4], // NVDA
    [0.05, 0.3], // JPM
    [0.05, 0.2], // XOM
    [0.05, 0.25], // BTC-USD
  ]);

  const categoricalConstraints = JSON.stringify([
    {
      name: "Sector",
      category_per_symbol: [
        "Tech",
        "Tech",
        "Tech",
        "Finance",
        "Energy",
        "Crypto",
      ],
      weight_per_category: [
        ["Tech", 0.3, 0.6],
        ["Finance", 0.05, 0.3],
        ["Energy", 0.05, 0.2],
        ["Crypto", 0.05, 0.25],
      ],
    },
    {
      name: "Asset Class",
      category_per_symbol: [
        "Equity",
        "Equity",
        "Equity",
        "Equity",
        "Equity",
        "Crypto",
      ],
      weight_per_category: [
        ["Equity", 0.7, 0.95],
        ["Crypto", 0.05, 0.3],
      ],
    },
  ]);

  let portfolio;
  try {
    portfolio = await new PortfolioBuilder()
      .tickerSymbols(["AAPL", "MSFT", "NVDA", "JPM", "XOM", "BTC-USD"])
      .benchmarkSymbol("^GSPC")
      .startDate("2023-01-01")
      .endDate("2024-12-31")
      .interval("1d")
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .objectiveFunction("max_sharpe")
      .assetConstraints(assetConstraints)
      .categoricalConstraints(categoricalConstraints)
      .build();
  } catch (err) {
    console.error("Error creating Portfolio:", err.message);
    return;
  }

  try {
    const report = await portfolio.report("optimization");
    console.log("Constrained optimization report: Opening in browser...");
    await report.show();
  } catch (err) {
    console.error("Error in optimization report:", err.message);
  }

  portfolio.free();
}

// ── 6. Portfolio — Explicit Allocation with Rebalancing and DCA ─────────────

async function portfolioAllocationRebalancingDCA() {
  console.log(
    "=== 6. Portfolio — Explicit Allocation with Rebalancing and DCA ===",
  );

  let p;
  try {
    p = await new PortfolioBuilder()
      .tickerSymbols(["AAPL", "MSFT", "NVDA", "BTC-USD"])
      .benchmarkSymbol("^GSPC")
      .startDate("2023-01-01")
      .endDate("2024-12-31")
      .interval("1d")
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .weights(JSON.stringify([25000.0, 25000.0, 25000.0, 25000.0]))
      .rebalanceStrategy(
        JSON.stringify({ type: "calendar", frequency: "quarterly" }),
      )
      .scheduledCashFlows(
        JSON.stringify([
          {
            amount: 2000.0,
            frequency: "monthly",
            start_date: null,
            end_date: null,
            allocation: "pro_rata",
          },
        ]),
      )
      .build();
  } catch (err) {
    console.error("Error creating Portfolio:", err.message);
    return;
  }

  try {
    const report = await p.report("performance");
    console.log("Performance report: Opening in browser...");
    await report.show();
  } catch (err) {
    console.error("Error in Portfolio report:", err.message);
  }

  p.free();
}

// ── 7. Custom Data (KLINE) — Load CSV data and use with Ticker, Tickers, Portfolio ──

async function customData() {
  console.log("=== 7. Custom Data (KLINE) ===");

  // Load data from CSV files
  const files = {
    aapl: "examples/datasets/aapl.csv",
    msft: "examples/datasets/msft.csv",
    nvda: "examples/datasets/nvda.csv",
    goog: "examples/datasets/goog.csv",
    btcusd: "examples/datasets/btcusd.csv",
    gspc: "examples/datasets/gspc.csv",
  };

  const dataFrames = {};
  for (const [name, path] of Object.entries(files)) {
    try {
      const csvData = fs.readFileSync(path, "utf8");
      dataFrames[name] = Polars.readCSV(csvData);
    } catch (err) {
      console.error(`Error reading ${path}:`, err.message);
      return;
    }
  }

  // Single Ticker from custom data
  console.log("--- Custom Ticker ---");
  let t;
  try {
    t = await new TickerBuilder()
      .symbol("AAPL")
      .benchmarkSymbol("^GSPC")
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .tickerData(dataFrames["aapl"])
      .benchmarkData(dataFrames["gspc"])
      .build();

    const report = await t.report("performance");
    console.log("Custom Ticker performance report: Opening in browser...");
    await report.show();
  } catch (err) {
    console.error("Error in custom Ticker:", err.message);
  } finally {
    if (t) t.free();
  }

  // Multiple Tickers from custom data
  console.log("--- Custom Tickers ---");
  const tickersData = [
    dataFrames["nvda"],
    dataFrames["goog"],
    dataFrames["aapl"],
    dataFrames["msft"],
    dataFrames["btcusd"],
  ];

  let ts;
  try {
    ts = await new TickersBuilder()
      .symbols(["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
      .benchmarkSymbol("^GSPC")
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .tickersData(tickersData)
      .benchmarkData(dataFrames["gspc"])
      .build();

    const report = await ts.report("performance");
    console.log("Custom Tickers performance report: Opening in browser...");
    await report.show();
  } catch (err) {
    console.error("Error in custom Tickers:", err.message);
  } finally {
    if (ts) ts.free();
  }

  // Portfolio optimization from custom data
  console.log("--- Custom Portfolio ---");
  let p;
  try {
    p = await new PortfolioBuilder()
      .tickerSymbols(["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
      .benchmarkSymbol("^GSPC")
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .objectiveFunction("max_sharpe")
      .tickersData(tickersData)
      .benchmarkData(dataFrames["gspc"])
      .build();

    const report = await p.report("optimization");
    console.log("Custom Portfolio optimization report: Opening in browser...");
    await report.show();
  } catch (err) {
    console.error("Error in custom Portfolio:", err.message);
  } finally {
    if (p) p.free();
  }
}

// Run all examples
async function main() {
  await screener();
  await ticker();
  await tickers();
  await portfolioOptimizationOOS();
  await portfolioOptimizationConstraints();
  await portfolioAllocationRebalancingDCA();
  await customData();
}

main().catch(console.error);
