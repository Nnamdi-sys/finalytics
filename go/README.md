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

Finalytics Go exposes four core modules for financial analytics:

### 1. Screener

Efficiently filter and rank securities (equities, crypto, etc.) using advanced metrics and custom filters.

**Usage Example:**
```go
screener, err := finalytics.NewScreener(
    "EQUITY",
    []string{
        `{"operator":"eq","operands":["exchange","NMS"]}`,
        `{"operator":"gte","operands":["intradaymarketcap",10000000000]}`,
    },
    "intradaymarketcap",
    true,
    0,
    10,
)
if err != nil {
    panic(err)
}
defer screener.Free()

screener.Display()
symbols, err := screener.Symbols()
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
symbols := []string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}
tickers, err := finalytics.NewTickersBuilder().
    Symbols(symbols).
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

**Usage Example:**
```go
symbols := []string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}
portfolio, err := finalytics.NewPortfolioBuilder().
    Symbols(symbols).
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

report, err := portfolio.Report("performance")
if err == nil {
    report.Show()
}
```

---

## 📚 More Documentation

- See the [Go API documentation](https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics) for full details.

---

## 🗂️ Multi-language Bindings

Finalytics is also available in:
- [Rust](../../rust/README.md)
- [Python](../../python/README.md)
- [Node.js](../../js/README.md)
- [Web Application](../../web/README.md)

---

**Finalytics** — Modular, high-performance financial analytics for Go.