![Finalytics](https://github.com/Nnamdi-sys/finalytics/raw/main/logo-color.png)

[![Go Reference](https://pkg.go.dev/badge/github.com/Nnamdi-sys/finalytics/go/finalytics.svg)](https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-brightgreen)

This is a Go binding for the [Finalytics Rust Library](https://github.com/Nnamdi-sys/finalytics), designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

To install the Finalytics Go binding, add it to your Go project using:

```bash
go get github.com/Nnamdi-sys/finalytics/go/finalytics
```

After installing the Go module, **download the required native binaries** by running:
```bash
curl -O https://raw.githubusercontent.com/Nnamdi-sys/finalytics/refs/heads/main/go/download_binaries.sh
bash download_binaries.sh
```

## Example

View the [Go documentation](https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics) for more information.  
You can also check the [`main.go` file](https://github.com/Nnamdi-sys/finalytics/blob/main/go/main.go) for more usage examples.

```go
package main

import (
    "fmt"
    "github.com/Nnamdi-sys/finalytics/go/finalytics"
)

func main() {
	// Screen for Large Cap NASDAQ Stocks
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
        fmt.Printf("Error creating Screener: %v\n", err)
        return
    }
    defer screener.Free()

    // Get screened symbols
    symbols, err := screener.Symbols()
    if err != nil {
        fmt.Printf("Failed to get symbols: %v\n", err)
        return
    }
    fmt.Printf("Screened Symbols: %v\n", symbols)

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
        fmt.Printf("Failed to create Tickers: %v\n", err)
        return
    }
    defer tickers.Free()

    // Generate a Single Ticker Report
    if len(symbols) > 0 {
        ticker, err := tickers.GetTicker(symbols[0])
        if err != nil {
            fmt.Printf("Failed to get Ticker: %v\n", err)
            return
        }
        defer ticker.Free()

        for _, reportType := range []string{"performance", "financials", "options", "news"} {
            report, err := ticker.Report(reportType)
            if err != nil {
                fmt.Printf("Failed to get %s report: %v\n", reportType, err)
                continue
            }
            report.Show()
        }
    }

    // Generate a Multiple Ticker Report
    tickersReport, err := tickers.Report("performance")
    if err != nil {
        fmt.Printf("Failed to get Tickers report: %v\n", err)
        return
    }
    tickersReport.Show()

    // Perform Portfolio Optimization
    portfolio, err := tickers.Optimize("max_sharpe", "{}", "{}", "{}")
    if err != nil {
        fmt.Printf("Failed to optimize portfolio: %v\n", err)
        return
    }
    defer portfolio.Free()

    // Generate a Portfolio Report
    portfolioReport, err := portfolio.Report("performance")
    if err != nil {
        fmt.Printf("Failed to get Portfolio report: %v\n", err)
        return
    }
    portfolioReport.Show()
}
```