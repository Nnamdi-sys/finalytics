package main

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/Nnamdi-sys/finalytics/go/finalytics"
	"github.com/go-gota/gota/dataframe"
)

// testTicker tests all methods of the Ticker struct.
func testTicker() {
    fmt.Println("=== Testing Ticker ===")
    ticker, err := finalytics.NewTickerBuilder().
        Symbol("AAPL").
        StartDate("2023-01-01").
        EndDate("2023-12-31").
        Interval("1d").
        BenchmarkSymbol("^GSPC").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        Build()
    if err != nil {
        fmt.Printf("Error creating Ticker: %v\n", err)
        return
    }
    defer ticker.Free()

    // Test GetQuote
    quote, err := ticker.GetQuote()
    if err != nil {
        fmt.Printf("Error in GetQuote: %v\n", err)
    } else {
        fmt.Printf("GetQuote: %v\n", quote)
    }

    // Test GetSummaryStats
    summary, err := ticker.GetSummaryStats()
    if err != nil {
        fmt.Printf("Error in GetSummaryStats: %v\n", err)
    } else {
        fmt.Printf("GetSummaryStats: %v\n", summary)
    }

    // Test GetPriceHistory
    history, err := ticker.GetPriceHistory()
    if err != nil {
        fmt.Printf("Error in GetPriceHistory: %v\n", err)
    } else {
        fmt.Printf("GetPriceHistory: %v\n", history)
    }

    // Test GetOptionsChain
    options, err := ticker.GetOptionsChain()
    if err != nil {
        fmt.Printf("Error in GetOptionsChain: %v\n", err)
    } else {
        fmt.Printf("GetOptionsChain: %v\n", options)
    }

    // Test GetNews
    news, err := ticker.GetNews()
    if err != nil {
        fmt.Printf("Error in GetNews: %v\n", err)
    } else {
        fmt.Printf("GetNews: %v\n", news)
    }

    // Test GetIncomeStatement
    income, err := ticker.GetIncomeStatement("quarterly", true)
    if err != nil {
        fmt.Printf("Error in GetIncomeStatement: %v\n", err)
    } else {
        fmt.Printf("GetIncomeStatement: %v\n", income)
    }

    // Test GetBalanceSheet
    balance, err := ticker.GetBalanceSheet("quarterly", true)
    if err != nil {
        fmt.Printf("Error in GetBalanceSheet: %v\n", err)
    } else {
        fmt.Printf("GetBalanceSheet: %v\n", balance)
    }

    // Test GetCashflowStatement
    cashflow, err := ticker.GetCashflowStatement("quarterly", true)
    if err != nil {
        fmt.Printf("Error in GetCashflowStatement: %v\n", err)
    } else {
        fmt.Printf("GetCashflowStatement: %v\n", cashflow)
    }

    // Test GetFinancialRatios
    ratios, err := ticker.GetFinancialRatios("quarterly")
    if err != nil {
        fmt.Printf("Error in GetFinancialRatios: %v\n", err)
    } else {
        fmt.Printf("GetFinancialRatios: %v\n", ratios)
    }

    // Test VolatilitySurface
    volSurface, err := ticker.VolatilitySurface()
    if err != nil {
        fmt.Printf("Error in VolatilitySurface: %v\n", err)
    } else {
        fmt.Printf("VolatilitySurface: %v\n", volSurface)
    }

    // Test PerformanceStats
    perfStats, err := ticker.PerformanceStats()
    if err != nil {
        fmt.Printf("Error in PerformanceStats: %v\n", err)
    } else {
        fmt.Printf("PerformanceStats: %v\n", perfStats)
    }

    // Test PerformanceChart
    perfChart, err := ticker.PerformanceChart(0, 0)
    if err != nil {
        fmt.Printf("Error in PerformanceChart: %v\n", err)
    } else {
        perfChart.Show()
    }

    // Test CandlestickChart
    candleChart, err := ticker.CandlestickChart(0, 0)
    if err != nil {
        fmt.Printf("Error in CandlestickChart: %v\n", err)
    } else {
        candleChart.Show()
    }

    // Test OptionsChart
    optChart, err := ticker.OptionsChart("surface", 0, 0)
    if err != nil {
        fmt.Printf("Error in OptionsChart: %v\n", err)
    } else {
        optChart.Show()
    }

    // Test NewsSentimentChart
    newsChart, err := ticker.NewsSentimentChart(0, 0)
    if err != nil {
        fmt.Printf("Error in NewsSentimentChart: %v\n", err)
    } else {
        newsChart.Show()
    }

    // Test Report
    report, err := ticker.Report("full")
    if err != nil {
        fmt.Printf("Error in Report: %v\n", err)
    } else {
        report.Show()
    }
}

// testTickers tests all methods of the Tickers struct.
func testTickers() {
    fmt.Println("=== Testing Tickers ===")
    tickers, err := finalytics.NewTickersBuilder().
        Symbols([]string{"AAPL", "MSFT"}).
        StartDate("2023-01-01").
        EndDate("2023-12-31").
        Interval("1d").
        BenchmarkSymbol("^GSPC").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        Build()
    if err != nil {
        fmt.Printf("Error creating Tickers: %v\n", err)
        return
    }
    defer tickers.Free()

    // Test GetSummaryStats
    summary, err := tickers.GetSummaryStats()
    if err != nil {
        fmt.Printf("Error in GetSummaryStats: %v\n", err)
    } else {
        fmt.Printf("GetSummaryStats: %v\n", summary)
    }

    // Test GetPriceHistory
    history, err := tickers.GetPriceHistory()
    if err != nil {
        fmt.Printf("Error in GetPriceHistory: %v\n", err)
    } else {
        fmt.Printf("GetPriceHistory: %v\n", history)
    }

    // Test GetOptionsChain
    options, err := tickers.GetOptionsChain()
    if err != nil {
        fmt.Printf("Error in GetOptionsChain: %v\n", err)
    } else {
        fmt.Printf("GetOptionsChain: %v\n", options)
    }

    // Test GetIncomeStatement
    income, err := tickers.GetIncomeStatement("quarterly", true)
    if err != nil {
        fmt.Printf("Error in GetIncomeStatement: %v\n", err)
    } else {
        fmt.Printf("GetIncomeStatement: %v\n", income)
    }

    // Test GetBalanceSheet
    balance, err := tickers.GetBalanceSheet("quarterly", true)
    if err != nil {
        fmt.Printf("Error in GetBalanceSheet: %v\n", err)
    } else {
        fmt.Printf("GetBalanceSheet: %v\n", balance)
    }

    // Test GetCashflowStatement
    cashflow, err := tickers.GetCashflowStatement("quarterly", true)
    if err != nil {
        fmt.Printf("Error in GetCashflowStatement: %v\n", err)
    } else {
        fmt.Printf("GetCashflowStatement: %v\n", cashflow)
    }

    // Test GetFinancialRatios
    ratios, err := tickers.GetFinancialRatios("quarterly")
    if err != nil {
        fmt.Printf("Error in GetFinancialRatios: %v\n", err)
    } else {
        fmt.Printf("GetFinancialRatios: %v\n", ratios)
    }

    // Test Returns
    returns, err := tickers.Returns()
    if err != nil {
        fmt.Printf("Error in Returns: %v\n", err)
    } else {
        fmt.Printf("Returns: %v\n", returns)
    }

    // Test PerformanceStats
    perfStats, err := tickers.PerformanceStats()
    if err != nil {
        fmt.Printf("Error in PerformanceStats: %v\n", err)
    } else {
        fmt.Printf("PerformanceStats: %v\n", perfStats)
    }

    // Test ReturnsChart
    retChart, err := tickers.ReturnsChart(0, 0)
    if err != nil {
        fmt.Printf("Error in ReturnsChart: %v\n", err)
    } else {
        retChart.Show()
    }

    // Test ReturnsMatrix
    retMatrix, err := tickers.ReturnsMatrix(0, 0)
    if err != nil {
        fmt.Printf("Error in ReturnsMatrix: %v\n", err)
    } else {
        retMatrix.Show()
    }

    // Test Report
    report, err := tickers.Report("performance")
    if err != nil {
        fmt.Printf("Error in Report: %v\n", err)
    } else {
        report.Show()
    }

    // Test GetTicker
    ticker, err := tickers.GetTicker("AAPL")
    if err != nil {
        fmt.Printf("Error in GetTicker: %v\n", err)
    } else {
        defer ticker.Free()
        fmt.Printf("GetTicker: Successfully retrieved ticker for AAPL\n")
    }

    // Test Optimize
    portfolio, err := tickers.Optimize("max_sharpe", "{}", "{}", "{}")
    if err != nil {
        fmt.Printf("Error in Optimize: %v\n", err)
    } else {
        defer portfolio.Free()
        fmt.Printf("Optimize: Successfully created portfolio\n")
    }
}

// testPortfolio tests all methods of the Portfolio struct.
func testPortfolio() {
    fmt.Println("=== Testing Portfolio ===")
    portfolio, err := finalytics.NewPortfolioBuilder().
        TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
        BenchmarkSymbol("SPY").
        StartDate("2023-01-01").
        EndDate("2023-12-31").
        Interval("1d").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        ObjectiveFunction("max_sharpe").
        AssetConstraints("{}").
        CategoricalConstraints("{}").
        Weights("{}").
        Build()
    if err != nil {
        fmt.Printf("Error creating Portfolio: %v\n", err)
        return
    }
    defer portfolio.Free()

    // Test OptimizationResults
    results, err := portfolio.OptimizationResults()
    if err != nil {
        fmt.Printf("Error in OptimizationResults: %v\n", err)
    } else {
        fmt.Printf("OptimizationResults: %v\n", results)
    }

    // Test OptimizationChart
    optChart, err := portfolio.OptimizationChart(0, 0)
    if err != nil {
        fmt.Printf("Error in OptimizationChart: %v\n", err)
    } else {
        optChart.Show()
    }

    // Test PerformanceChart
    perfChart, err := portfolio.PerformanceChart(0, 0)
    if err != nil {
        fmt.Printf("Error in PerformanceChart: %v\n", err)
    } else {
        perfChart.Show()
    }

    // Test AssetReturnsChart
    assetChart, err := portfolio.AssetReturnsChart(0, 0)
    if err != nil {
        fmt.Printf("Error in AssetReturnsChart: %v\n", err)
    } else {
        assetChart.Show()
    }

    // Test ReturnsMatrix
    retMatrix, err := portfolio.ReturnsMatrix(0, 0)
    if err != nil {
        fmt.Printf("Error in ReturnsMatrix: %v\n", err)
    } else {
        retMatrix.Show()
    }

    // Test Report
    report, err := portfolio.Report("performance")
    if err != nil {
        fmt.Printf("Error in Report: %v\n", err)
    } else {
        report.Show()
    }
}

// testScreener tests all methods of the Screener struct.
func testScreener() {
    fmt.Println("=== Testing Screener ===")
    
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

    // Test Symbols
    symbols, err := screener.Symbols()
    if err != nil {
        fmt.Printf("Error in Symbols: %v\n", err)
    } else {
        fmt.Printf("Symbols: %v\n", symbols)
    }

    // Test Overview
    overview, err := screener.Overview()
    if err != nil {
        fmt.Printf("Error in Overview: %v\n", err)
    } else {
        fmt.Printf("Overview: %v\n", overview)
    }

    // Test Metrics
    metrics, err := screener.Metrics()
    if err != nil {
        fmt.Printf("Error in Metrics: %v\n", err)
    } else {
        fmt.Printf("Metrics: %v\n", metrics)
    }
}

// io_test reads CSV data and tests Ticker, Tickers, and Portfolio structs.
func io_test() {
    fmt.Println("=== IO Test ===")

    // Define CSV file paths
    files := map[string]string{
        "nvda":   "../examples/datasets/nvda.csv",
        "goog":   "../examples/datasets/goog.csv",
        "aapl":   "../examples/datasets/aapl.csv",
        "msft":   "../examples/datasets/msft.csv",
        "btcusd": "../examples/datasets/btcusd.csv",
        "gspc":   "../examples/datasets/gspc.csv",
    }

    // Read CSV files into DataFrames
    dataFrames := make(map[string]dataframe.DataFrame)
    for name, path := range files {
        file, err := os.Open(path)
        if err != nil {
            fmt.Printf("Error opening %s: %v\n", path, err)
            return
        }
        defer file.Close()
        df := dataframe.ReadCSV(file)
        dataFrames[name] = df
    }

    // Test Ticker
    fmt.Println("--- Testing Ticker ---")
    aaplDF := dataFrames["aapl"] // Store in variable to make addressable
    gspcDF := dataFrames["gspc"] // Store in variable to make addressable
    ticker, err := finalytics.NewTickerBuilder().
        Symbol("AAPL").
        StartDate("2023-01-01").
        EndDate("2023-12-31").
        Interval("1d").
        BenchmarkSymbol("^GSPC").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        TickerData(&aaplDF).
        BenchmarkData(&gspcDF).
        Build()
    if err != nil {
        fmt.Printf("Error creating Ticker: %v\n", err)
        return
    }
    defer ticker.Free()
    report, err := ticker.Report("performance")
    if err != nil {
        fmt.Printf("Error in Ticker Report: %v\n", err)
    } else {
        report.Show()
    }

    // Test Tickers
    fmt.Println("--- Testing Tickers ---")
    tickersData := []dataframe.DataFrame{
        dataFrames["nvda"],
        dataFrames["goog"],
        dataFrames["aapl"],
        dataFrames["msft"],
        dataFrames["btcusd"],
    }
    tickers, err := finalytics.NewTickersBuilder().
        Symbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
        StartDate("2023-01-01").
        EndDate("2023-12-31").
        Interval("1d").
        BenchmarkSymbol("^GSPC").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        TickersData(tickersData).
        BenchmarkData(&gspcDF).
        Build()
    if err != nil {
        fmt.Printf("Error creating Tickers: %v\n", err)
        return
    }
    defer tickers.Free()
    report, err = tickers.Report("full")
    if err != nil {
        fmt.Printf("Error in Tickers Report: %v\n", err)
    } else {
        report.Show()
    }

    // Test Portfolio
    fmt.Println("--- Testing Portfolio ---")
    assetConstraints, err := json.Marshal([][2]float64{{0, 1}, {0, 1}, {0, 1}, {0, 1}, {0, 1}})
    if err != nil {
        fmt.Printf("Error marshaling assetConstraints: %v\n", err)
        return
    }
    categoricalConstraints, err := json.Marshal([]struct {
        Name        string
        Categories  []string
        Constraints [][3]any
    }{{
        Name:       "AssetClass",
        Categories: []string{"EQUITY", "EQUITY", "EQUITY", "EQUITY", "CRYPTO"},
        Constraints: [][3]any{
            {"EQUITY", 0.0, 0.8},
            {"CRYPTO", 0.0, 0.2},
        },
    }})
    if err != nil {
        fmt.Printf("Error marshaling categoricalConstraints: %v\n", err)
        return
    }
    portfolio, err := finalytics.NewPortfolioBuilder().
        TickerSymbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
        BenchmarkSymbol("^GSPC").
        StartDate("2023-01-01").
        EndDate("2023-12-31").
        Interval("1d").
        ConfidenceLevel(0.95).
        RiskFreeRate(0.02).
        ObjectiveFunction("max_sharpe").
        AssetConstraints(string(assetConstraints)).
        CategoricalConstraints(string(categoricalConstraints)).
        Weights("{}").
        TickersData(tickersData).
        BenchmarkData(&gspcDF).
        Build()
    if err != nil {
        fmt.Printf("Error creating Portfolio: %v\n", err)
        return
    }
    defer portfolio.Free()
    report, err = portfolio.Report("full")
    if err != nil {
        fmt.Printf("Error in Portfolio Report: %v\n", err)
    } else {
        report.Show()
    }
}

func readme_example() {
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

func main() {
	// curl -O https://raw.githubusercontent.com/Nnamdi-sys/finalytics/main/go/finalytics/download_binaries.sh
	// bash download_binaries.sh
    testTicker()
    testTickers()
    testPortfolio()
    testScreener()
    io_test()
    readme_example()
}