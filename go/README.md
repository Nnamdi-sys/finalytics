![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![Go Reference](https://pkg.go.dev/badge/github.com/Nnamdi-sys/finalytics/go/finalytics.svg)](https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)

---

# Finalytics Go Binding

**Finalytics** is a high-performance Go binding for the Finalytics Rust library, designed for retrieving financial data, security analysis, and portfolio optimization.
It provides a fast, modular interface for advanced analytics, and powers dashboards and applications across platforms.

---

## 🚀 Installation

To install the Finalytics Go binding, add it to your Go project using:

```bash
go get github.com/Nnamdi-sys/finalytics/go/finalytics
```

After installing the Go module, **download the required native binary** by running:
```bash
curl -O https://raw.githubusercontent.com/Nnamdi-sys/finalytics/refs/heads/main/go/download_binaries.sh
bash download_binaries.sh
```

---

## 🐹 Main Modules

Finalytics Go exposes five core modules for financial analytics:

### 1. Screener

Efficiently filter and rank securities using advanced metrics and custom filters.

**Usage Example:**
```go
screener, err := finalytics.NewScreenerBuilder().
    QuoteType("EQUITY").
    AddFilter(`{"operator":"eq","operands":["exchange","NMS"]}`).
    AddFilter(`{"operator":"eq","operands":["sector","Technology"]}`).
    AddFilter(`{"operator":"gte","operands":["intradaymarketcap",10000000000]}`).
    AddFilter(`{"operator":"gte","operands":["returnonequity.lasttwelvemonths",0.15]}`).
    SortField("intradaymarketcap").
    SortDescending(true).
    Offset(0).
    Size(10).
    Build()
if err != nil {
    panic(err)
}
defer screener.Free()

screener.Display()
symbols, _ := screener.Symbols()
fmt.Println("Symbols:", symbols)
```

---

### 2. Ticker

Analyze a single security in depth: performance, financials, options, news, and more.

**Usage Example:**
```go
ticker, err := finalytics.NewTickerBuilder().
    Symbol("AAPL").
    StartDate("2023-01-01").
    EndDate("2024-12-31").
    Interval("1d").
    BenchmarkSymbol("^GSPC").
    ConfidenceLevel(0.95).
    RiskFreeRate(0.02).
    Build()
if err != nil {
    panic(err)
}
defer ticker.Free()

for _, reportType := range []string{"performance", "financials", "options", "news"} {
    report, err := ticker.Report(reportType)
    if err == nil {
        report.Show()
    }
}
```

---

### 3. Tickers

Work with multiple securities at once—aggregate reports, batch analytics, and portfolio construction.

**Usage Example:**
```go
tickers, err := finalytics.NewTickersBuilder().
    Symbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
    StartDate("2023-01-01").
    EndDate("2024-12-31").
    Interval("1d").
    BenchmarkSymbol("^GSPC").
    ConfidenceLevel(0.95).
    RiskFreeRate(0.02).
    Build()
if err != nil {
    panic(err)
}
defer tickers.Free()

report, err := tickers.Report("performance")
if err == nil {
    report.Show()
}
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
```go
import "encoding/json"

// Optimize on 2023 - 2024 data (in-sample)
portfolio, err := finalytics.NewPortfolioBuilder().
    TickerSymbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
    BenchmarkSymbol("^GSPC").
    StartDate("2023-01-01").
    EndDate("2024-12-31").
    Interval("1d").
    ConfidenceLevel(0.95).
    RiskFreeRate(0.02).
    ObjectiveFunction("max_sharpe").
    Build()
if err != nil {
    panic(err)
}
defer portfolio.Free()

report, _ := portfolio.Report("optimization")
report.Show()

// Update to 2025 data for out-of-sample evaluation
portfolio.UpdateDates("2025-01-01", "2026-01-01")
portfolio.PerformanceStats()
report, _ = portfolio.Report("performance")
report.Show()
```

**Usage Example: Explicit Allocation with Rebalancing and DCA**
```go
import "encoding/json"

weights, _ := json.Marshal([]float64{25000.0, 25000.0, 25000.0, 25000.0})
rebalance, _ := json.Marshal(map[string]interface{}{
    "type":      "calendar",
    "frequency": "quarterly",
})
cashFlows, _ := json.Marshal([]map[string]interface{}{
    {
        "amount":     2000.0,
        "frequency":  "monthly",
        "start_date": nil,
        "end_date":   nil,
        "allocation": "pro_rata",
    },
})

portfolio, err := finalytics.NewPortfolioBuilder().
    TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
    BenchmarkSymbol("^GSPC").
    StartDate("2023-01-01").
    EndDate("2024-12-31").
    Interval("1d").
    ConfidenceLevel(0.95).
    RiskFreeRate(0.02).
    Weights(string(weights)).
    RebalanceStrategy(string(rebalance)).
    ScheduledCashFlows(string(cashFlows)).
    Build()
if err != nil {
    panic(err)
}
defer portfolio.Free()

report, _ := portfolio.Report("performance")
report.Show()
```

**Usage Example: Optimization with Weight & Categorical Constraints**
```go
import "encoding/json"

// Per-asset bounds: [lower, upper] in the same order as ticker_symbols
assetConstraints, _ := json.Marshal([][]float64{
    {0.05, 0.40}, // AAPL
    {0.05, 0.40}, // MSFT
    {0.05, 0.40}, // NVDA
    {0.05, 0.30}, // JPM
    {0.05, 0.20}, // XOM
    {0.05, 0.25}, // BTC-USD
})

categoricalConstraints, _ := json.Marshal([]map[string]interface{}{
    {
        "name":               "Sector",
        "category_per_symbol": []string{"Tech", "Tech", "Tech", "Finance", "Energy", "Crypto"},
        "weight_per_category": [][]interface{}{
            {"Tech", 0.30, 0.60},
            {"Finance", 0.05, 0.30},
            {"Energy", 0.05, 0.20},
            {"Crypto", 0.05, 0.25},
        },
    },
    {
        "name":               "Asset Class",
        "category_per_symbol": []string{"Equity", "Equity", "Equity", "Equity", "Equity", "Crypto"},
        "weight_per_category": [][]interface{}{
            {"Equity", 0.70, 0.95},
            {"Crypto", 0.05, 0.30},
        },
    },
})

portfolio, err := finalytics.NewPortfolioBuilder().
    TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "JPM", "XOM", "BTC-USD"}).
    BenchmarkSymbol("^GSPC").
    StartDate("2023-01-01").
    EndDate("2024-12-31").
    Interval("1d").
    ConfidenceLevel(0.95).
    RiskFreeRate(0.02).
    ObjectiveFunction("max_sharpe").
    AssetConstraints(string(assetConstraints)).
    CategoricalConstraints(string(categoricalConstraints)).
    Build()
if err != nil {
    panic(err)
}
defer portfolio.Free()

report, _ := portfolio.Report("optimization")
report.Show()
```

---

### 5. Custom Data

Load your own price data from CSV files as DataFrames and use it with any Finalytics module.
CSV files must have columns: `timestamp` (unix epoch), `open`, `high`, `low`, `close`, `volume`, `adjclose`.

**Usage Example:**
```go
import (
    "os"
    "github.com/go-gota/gota/dataframe"
)

// Load data from CSV files
files := map[string]string{
    "aapl": "examples/datasets/aapl.csv",
    "msft": "examples/datasets/msft.csv",
    "nvda": "examples/datasets/nvda.csv",
    "goog": "examples/datasets/goog.csv",
    "btcusd": "examples/datasets/btcusd.csv",
    "gspc": "examples/datasets/gspc.csv",
}
dataFrames := make(map[string]dataframe.DataFrame)
for name, path := range files {
    file, _ := os.Open(path)
    defer file.Close()
    dataFrames[name] = dataframe.ReadCSV(file)
}

gspc := dataFrames["gspc"]

// Single Ticker from custom data
aaplDF := dataFrames["aapl"]
ticker, err := finalytics.NewTickerBuilder().
    Symbol("AAPL").
    BenchmarkSymbol("^GSPC").
    ConfidenceLevel(0.95).
    RiskFreeRate(0.02).
    TickerData(&aaplDF).
    BenchmarkData(&gspc).
    Build()
if err != nil {
    panic(err)
}
defer ticker.Free()

report, _ := ticker.Report("performance")
report.Show()

// Multiple Tickers from custom data
tickersData := []dataframe.DataFrame{
    dataFrames["nvda"], dataFrames["goog"], dataFrames["aapl"],
    dataFrames["msft"], dataFrames["btcusd"],
}
tickers, err := finalytics.NewTickersBuilder().
    Symbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
    BenchmarkSymbol("^GSPC").
    ConfidenceLevel(0.95).
    RiskFreeRate(0.02).
    TickersData(tickersData).
    BenchmarkData(&gspc).
    Build()
if err != nil {
    panic(err)
}
defer tickers.Free()

report, _ = tickers.Report("performance")
report.Show()

// Portfolio optimization from custom data
portfolio, err := finalytics.NewPortfolioBuilder().
    TickerSymbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
    BenchmarkSymbol("^GSPC").
    ConfidenceLevel(0.95).
    RiskFreeRate(0.02).
    ObjectiveFunction("max_sharpe").
    TickersData(tickersData).
    BenchmarkData(&gspc).
    Build()
if err != nil {
    panic(err)
}
defer portfolio.Free()

report, _ = portfolio.Report("optimization")
report.Show()
```

---

## 📚 More Documentation

- See the [Go API documentation](https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics) for full details.

---

## 🗂️ Multi-language Bindings

Finalytics is also available in:
- [Rust](../rust/README.md)
- [Python](../python/README.md)
- [Node.js](../js/README.md)
- [Web Application](../web/README.md)

---

**Finalytics** — Modular, high-performance financial analytics for Go.
