// Finalytics — Go Examples
//
// Installation
// ────────────
//   go get github.com/Nnamdi-sys/finalytics/go/finalytics
//
//   # Download the required native binary:
//   curl -O https://raw.githubusercontent.com/Nnamdi-sys/finalytics/refs/heads/main/go/download_binaries.sh
//   bash download_binaries.sh
//
// Full docs: https://pkg.go.dev/github.com/Nnamdi-sys/finalytics/go/finalytics
//
// Run this example (from the repo root)
// ───────────────────────────────────────
//   bash examples/example.sh go

package main

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/Nnamdi-sys/finalytics/go/finalytics"
	"github.com/go-gota/gota/dataframe"
)

func main() {
	screener()
	ticker()
	tickers()
	portfolioOptimizationOOS()
	portfolioOptimizationConstraints()
	portfolioAllocationRebalancingDCA()
	customData()
}

// ── 1. Screener — Large-Cap NASDAQ Technology Stocks with ROE >= 15% ────────

func screener() {
	fmt.Println("=== 1. Screener ===")

	s, err := finalytics.NewScreenerBuilder().
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
		fmt.Printf("Error creating Screener: %v\n", err)
		return
	}
	defer s.Free()

	symbols, _ := s.Symbols()
	fmt.Println("Symbols:", symbols)

	overview, _ := s.Overview()
	fmt.Println("Overview:", overview)

	metrics, _ := s.Metrics()
	fmt.Println("Metrics:", metrics)

	s.Display()
}

// ── 2. Ticker — Single security analysis with all report types ───────────────

func ticker() {
	fmt.Println("=== 2. Ticker ===")

	t, err := finalytics.NewTickerBuilder().
		Symbol("AAPL").
		StartDate("2023-01-01").
		EndDate("2024-12-31").
		Interval("1d").
		BenchmarkSymbol("^GSPC").
		ConfidenceLevel(0.95).
		RiskFreeRate(0.02).
		Build()
	if err != nil {
		fmt.Printf("Error creating Ticker: %v\n", err)
		return
	}
	defer t.Free()

	for _, reportType := range []string{"performance", "financials", "options", "news"} {
		report, err := t.Report(reportType)
		if err != nil {
			fmt.Printf("Error in %s report: %v\n", reportType, err)
			continue
		}
		report.Show()
	}
}

// ── 3. Tickers — Multiple securities analysis ────────────────────────────────

func tickers() {
	fmt.Println("=== 3. Tickers ===")

	ts, err := finalytics.NewTickersBuilder().
		Symbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
		StartDate("2023-01-01").
		EndDate("2024-12-31").
		Interval("1d").
		BenchmarkSymbol("^GSPC").
		ConfidenceLevel(0.95).
		RiskFreeRate(0.02).
		Build()
	if err != nil {
		fmt.Printf("Error creating Tickers: %v\n", err)
		return
	}
	defer ts.Free()

	report, err := ts.Report("performance")
	if err != nil {
		fmt.Printf("Error in Tickers report: %v\n", err)
		return
	}
	report.Show()
}

// ── 4. Portfolio — Optimization with Out-of-Sample Evaluation ───────────────

func portfolioOptimizationOOS() {
	fmt.Println("=== 4. Portfolio — Optimization with Out-of-Sample Evaluation ===")

	// Optimize on 2023-2024 data (in-sample)
	p, err := finalytics.NewPortfolioBuilder().
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
		fmt.Printf("Error creating Portfolio: %v\n", err)
		return
	}
	defer p.Free()

	report, err := p.Report("optimization")
	if err != nil {
		fmt.Printf("Error in optimization report: %v\n", err)
	} else {
		report.Show()
	}

	// Update to 2025 data for out-of-sample evaluation
	err = p.UpdateDates("2025-01-01", "2026-01-01")
	if err != nil {
		fmt.Printf("Error in UpdateDates: %v\n", err)
		return
	}

	_, err = p.PerformanceStats()
	if err != nil {
		fmt.Printf("Error in PerformanceStats: %v\n", err)
		return
	}

	report, err = p.Report("performance")
	if err != nil {
		fmt.Printf("Error in performance report: %v\n", err)
	} else {
		report.Show()
	}
}

// ── 5. Portfolio — Optimization with Weight & Categorical Constraints ─────────

func portfolioOptimizationConstraints() {
	fmt.Println("=== 5. Portfolio — Optimization with Weight & Categorical Constraints ===")

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
			"name":                "Sector",
			"category_per_symbol": []string{"Tech", "Tech", "Tech", "Finance", "Energy", "Crypto"},
			"weight_per_category": [][]interface{}{
				{"Tech", 0.30, 0.60},
				{"Finance", 0.05, 0.30},
				{"Energy", 0.05, 0.20},
				{"Crypto", 0.05, 0.25},
			},
		},
		{
			"name":                "Asset Class",
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
		fmt.Printf("Error creating Portfolio: %v\n", err)
		return
	}
	defer portfolio.Free()

	report, err := portfolio.Report("optimization")
	if err != nil {
		fmt.Printf("Error in optimization report: %v\n", err)
		return
	}
	report.Show()
}

// ── 6. Portfolio — Explicit Allocation with Rebalancing and DCA ─────────────

func portfolioAllocationRebalancingDCA() {
	fmt.Println("=== 6. Portfolio — Explicit Allocation with Rebalancing and DCA ===")

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

	p, err := finalytics.NewPortfolioBuilder().
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
		fmt.Printf("Error creating Portfolio: %v\n", err)
		return
	}
	defer p.Free()

	report, err := p.Report("performance")
	if err != nil {
		fmt.Printf("Error in performance report: %v\n", err)
	} else {
		report.Show()
	}
}

// ── 7. Custom Data (KLINE) — Load CSV data and use with Ticker, Tickers, Portfolio ──

func customData() {
	fmt.Println("=== 7. Custom Data (KLINE) ===")

	// Load data from CSV files
	// Paths are relative to the go/ directory (the working directory when
	// running via example.sh: cd go && go run ../examples/example.go)
	files := map[string]string{
		"aapl":   "../examples/datasets/aapl.csv",
		"msft":   "../examples/datasets/msft.csv",
		"nvda":   "../examples/datasets/nvda.csv",
		"goog":   "../examples/datasets/goog.csv",
		"btcusd": "../examples/datasets/btcusd.csv",
		"gspc":   "../examples/datasets/gspc.csv",
	}

	dataFrames := make(map[string]dataframe.DataFrame)
	for name, path := range files {
		file, err := os.Open(path)
		if err != nil {
			fmt.Printf("Error opening %s: %v\n", path, err)
			return
		}
		defer file.Close()
		dataFrames[name] = dataframe.ReadCSV(file)
	}

	gspc := dataFrames["gspc"]

	// Single Ticker from custom data
	fmt.Println("--- Custom Ticker ---")
	aaplDF := dataFrames["aapl"]
	t, err := finalytics.NewTickerBuilder().
		Symbol("AAPL").
		BenchmarkSymbol("^GSPC").
		ConfidenceLevel(0.95).
		RiskFreeRate(0.02).
		TickerData(&aaplDF).
		BenchmarkData(&gspc).
		Build()
	if err != nil {
		fmt.Printf("Error creating Ticker: %v\n", err)
		return
	}
	defer t.Free()

	report, err := t.Report("performance")
	if err != nil {
		fmt.Printf("Error in Ticker report: %v\n", err)
	} else {
		report.Show()
	}

	// Multiple Tickers from custom data
	fmt.Println("--- Custom Tickers ---")
	tickersData := []dataframe.DataFrame{
		dataFrames["nvda"],
		dataFrames["goog"],
		dataFrames["aapl"],
		dataFrames["msft"],
		dataFrames["btcusd"],
	}
	ts, err := finalytics.NewTickersBuilder().
		Symbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
		BenchmarkSymbol("^GSPC").
		ConfidenceLevel(0.95).
		RiskFreeRate(0.02).
		TickersData(tickersData).
		BenchmarkData(&gspc).
		Build()
	if err != nil {
		fmt.Printf("Error creating Tickers: %v\n", err)
		return
	}
	defer ts.Free()

	report, err = ts.Report("performance")
	if err != nil {
		fmt.Printf("Error in Tickers report: %v\n", err)
	} else {
		report.Show()
	}

	// Portfolio optimization from custom data
	fmt.Println("--- Custom Portfolio ---")
	p, err := finalytics.NewPortfolioBuilder().
		TickerSymbols([]string{"NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"}).
		BenchmarkSymbol("^GSPC").
		ConfidenceLevel(0.95).
		RiskFreeRate(0.02).
		ObjectiveFunction("max_sharpe").
		TickersData(tickersData).
		BenchmarkData(&gspc).
		Build()
	if err != nil {
		fmt.Printf("Error creating Portfolio: %v\n", err)
		return
	}
	defer p.Free()

	report, err = p.Report("optimization")
	if err != nil {
		fmt.Printf("Error in Portfolio report: %v\n", err)
	} else {
		report.Show()
	}
}
