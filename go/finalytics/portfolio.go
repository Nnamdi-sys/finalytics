package finalytics

/*
#include <finalytics.h>
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"fmt"
	"unsafe"

	"github.com/go-gota/gota/dataframe"
)

// Portfolio represents a portfolio of assets with methods for retrieving optimization results and analytics.
// It encapsulates a handle to the underlying C library for interacting with portfolio data.
type Portfolio struct {
	handle C.PortfolioHandle
}

// PortfolioBuilder is used to construct a Portfolio instance using the builder pattern.
// It allows for fluent configuration of the Portfolio's parameters before creation.
type PortfolioBuilder struct {
	tickerSymbols          []string
	benchmarkSymbol        string
	startDate              string
	endDate                string
	interval               string
	confidenceLevel        float64
	riskFreeRate           float64
	objectiveFunction      string
	assetConstraints       string
	categoricalConstraints string
	weights                string
	tickersData            []dataframe.DataFrame
	benchmarkData          *dataframe.DataFrame
	transactions           string
	rebalanceStrategy      string
	scheduledCashFlows     string
}

// NewPortfolioBuilder initializes a new PortfolioBuilder with default values.
// Defaults:
//   - tickerSymbols: nil
//   - benchmarkSymbol: ""
//   - startDate: ""
//   - endDate: ""
//   - interval: "1d"
//   - confidenceLevel: 0.95
//   - riskFreeRate: 0.02
//   - objectiveFunction: "max_sharpe"
//   - assetConstraints: "{}"
//   - categoricalConstraints: "{}"
//   - weights: "{}"
//   - tickersData: nil
//   - benchmarkData: nil
//
// Returns:
//   - *PortfolioBuilder: A pointer to the initialized PortfolioBuilder.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		builder := finalytics.NewPortfolioBuilder()
//		fmt.Println("PortfolioBuilder initialized")
//	}
func NewPortfolioBuilder() *PortfolioBuilder {
	return &PortfolioBuilder{
		tickerSymbols:          nil,
		benchmarkSymbol:        "",
		startDate:              "",
		endDate:                "",
		interval:               "1d",
		confidenceLevel:        0.95,
		riskFreeRate:           0.02,
		objectiveFunction:      "max_sharpe",
		assetConstraints:       "{}",
		categoricalConstraints: "{}",
		weights:                "{}",
		tickersData:            nil,
		benchmarkData:          nil,
		transactions:           "[]",
		rebalanceStrategy:      "{}",
		scheduledCashFlows:     "[]",
	}
}

// TickerSymbols sets the ticker symbols for the Portfolio.
//
// Parameters:
//   - tickerSymbols: A string slice of ticker symbols (e.g., []string{"AAPL", "MSFT"}).
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().TickerSymbols([]string{"AAPL", "MSFT"})
func (b *PortfolioBuilder) TickerSymbols(tickerSymbols []string) *PortfolioBuilder {
	b.tickerSymbols = tickerSymbols
	return b
}

// BenchmarkSymbol sets the benchmark symbol for the Portfolio.
//
// Parameters:
//   - benchmarkSymbol: The ticker symbol of the benchmark (e.g., "^GSPC").
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().BenchmarkSymbol("^GSPC")
func (b *PortfolioBuilder) BenchmarkSymbol(benchmarkSymbol string) *PortfolioBuilder {
	b.benchmarkSymbol = benchmarkSymbol
	return b
}

// StartDate sets the start date for the Portfolio's data period.
//
// Parameters:
//   - startDate: The start date in the format YYYY-MM-DD.
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().StartDate("2023-01-01")
func (b *PortfolioBuilder) StartDate(startDate string) *PortfolioBuilder {
	b.startDate = startDate
	return b
}

// EndDate sets the end date for the Portfolio's data period.
//
// Parameters:
//   - endDate: The end date in the format YYYY-MM-DD.
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().EndDate("2023-12-31")
func (b *PortfolioBuilder) EndDate(endDate string) *PortfolioBuilder {
	b.endDate = endDate
	return b
}

// Interval sets the data interval for the Portfolio.
//
// Parameters:
//   - interval: The data interval (e.g., "2m", "5m", "15m", "30m", "1h", "1d", "1wk", "1mo", "3mo").
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().Interval("1d")
func (b *PortfolioBuilder) Interval(interval string) *PortfolioBuilder {
	b.interval = interval
	return b
}

// ConfidenceLevel sets the confidence level for VaR and ES calculations.
//
// Parameters:
//   - confidenceLevel: The confidence level (e.g., 0.95 for 95% confidence).
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().ConfidenceLevel(0.99)
func (b *PortfolioBuilder) ConfidenceLevel(confidenceLevel float64) *PortfolioBuilder {
	b.confidenceLevel = confidenceLevel
	return b
}

// RiskFreeRate sets the risk-free rate for calculations.
//
// Parameters:
//   - riskFreeRate: The risk-free rate (e.g., 0.02 for 2%).
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().RiskFreeRate(0.03)
func (b *PortfolioBuilder) RiskFreeRate(riskFreeRate float64) *PortfolioBuilder {
	b.riskFreeRate = riskFreeRate
	return b
}

// ObjectiveFunction sets the objective function for optimization.
//
// Parameters:
//   - objectiveFunction: The objective function. Supported values:
//     "max_sharpe", "max_sortino", "max_return", "min_vol", "min_var",
//     "min_cvar", "min_drawdown", "risk_parity", "max_diversification",
//     "hierarchical_risk_parity"
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().ObjectiveFunction("max_sharpe")
func (b *PortfolioBuilder) ObjectiveFunction(objectiveFunction string) *PortfolioBuilder {
	b.objectiveFunction = objectiveFunction
	return b
}

// AssetConstraints sets the asset-level constraints for optimization.
//
// Parameters:
//   - assetConstraints: JSON string defining asset-level constraints (e.g., `[[0,1],[0,1]]` for min/max weights).
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().AssetConstraints(`[[0,1],[0,1]]`)
func (b *PortfolioBuilder) AssetConstraints(assetConstraints string) *PortfolioBuilder {
	b.assetConstraints = assetConstraints
	return b
}

// CategoricalConstraints sets the categorical constraints for optimization.
//
// Parameters:
//   - categoricalConstraints: JSON string defining categorical constraints (e.g., constraints on asset classes).
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().CategoricalConstraints(`[{"Name":"AssetClass","Categories":["EQUITY","EQUITY"],"Constraints":[["EQUITY",0.0,0.8]]}]`)
func (b *PortfolioBuilder) CategoricalConstraints(categoricalConstraints string) *PortfolioBuilder {
	b.categoricalConstraints = categoricalConstraints
	return b
}

// weights sets the portfolio-level constraints for optimization.
//
// Parameters:
//   - weights: JSON string defining portfolio-level constraints (e.g., "{}").
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().weights("{}")
func (b *PortfolioBuilder) Weights(weights string) *PortfolioBuilder {
	b.weights = weights
	return b
}

// TickersData sets custom ticker data for the Portfolio.
//
// Parameters:
//   - tickersData: A slice of DataFrames containing custom ticker data for each symbol (pass nil or empty slice if not using custom data).
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().TickersData(nil)
func (b *PortfolioBuilder) TickersData(tickersData []dataframe.DataFrame) *PortfolioBuilder {
	b.tickersData = tickersData
	return b
}

// BenchmarkData sets custom benchmark data for the Portfolio.
//
// Parameters:
//   - benchmarkData: A DataFrame containing custom benchmark data (pass nil if not using custom data).
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
//
// Example:
//
//	builder := finalytics.NewPortfolioBuilder().BenchmarkData(nil)
func (b *PortfolioBuilder) BenchmarkData(benchmarkData *dataframe.DataFrame) *PortfolioBuilder {
	b.benchmarkData = benchmarkData
	return b
}

// Transactions sets ad-hoc per-asset transactions (additions / withdrawals).
//
// Parameters:
//   - transactions: JSON string defining transactions. Format:
//     `[{"date":"2024-01-15","ticker":"AAPL","amount":5000},{"date":"2024-06-01","ticker":"MSFT","amount":-2000}]`
//     Positive amounts are additions, negative are withdrawals.
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
func (b *PortfolioBuilder) Transactions(transactions string) *PortfolioBuilder {
	b.transactions = transactions
	return b
}

// RebalanceStrategy sets the rebalancing strategy for the portfolio simulation.
//
// Parameters:
//   - rebalanceStrategy: JSON string defining the strategy. Formats:
//     `{"type":"calendar","frequency":"monthly"}` — rebalance on a fixed calendar schedule
//     `{"type":"threshold","threshold":0.05}` — rebalance when any weight drifts > threshold
//     `{"type":"calendar_or_threshold","frequency":"quarterly","threshold":0.05}` — either trigger
//     Frequency values: "monthly", "quarterly", "semi_annually", "annually"
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
func (b *PortfolioBuilder) RebalanceStrategy(rebalanceStrategy string) *PortfolioBuilder {
	b.rebalanceStrategy = rebalanceStrategy
	return b
}

// ScheduledCashFlows sets recurring cash flow schedules for the portfolio simulation.
//
// Parameters:
//   - scheduledCashFlows: JSON string defining the schedules. Format:
//     `[{"amount":2000,"frequency":"monthly","start_date":null,"end_date":null,"allocation":"pro_rata"}]`
//     Amount: positive = addition, negative = withdrawal.
//     Allocation: "pro_rata", "rebalance", or {"custom":[0.4,0.3,0.2,0.1]}
//
// Returns:
//   - *PortfolioBuilder: The builder instance for method chaining.
func (b *PortfolioBuilder) ScheduledCashFlows(scheduledCashFlows string) *PortfolioBuilder {
	b.scheduledCashFlows = scheduledCashFlows
	return b
}

// Build constructs the Portfolio instance with the configured parameters.
// The tickerSymbols parameter is required; other parameters are optional and use defaults if not set.
//
// Returns:
//   - *Portfolio: A pointer to the initialized Portfolio object.
//   - error: An error if the Portfolio creation fails or tickerSymbols is missing/empty.
//
// Example:
//
//	package main
//
//	import (
//		"encoding/json"
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		// Sample asset constraints: min/max weights for each asset
//		assetConstraints, err := json.Marshal([][2]float64{{0.1, 0.5}, {0.1, 0.5}, {0.1, 0.5}, {0.1, 0.5}})
//		if err != nil {
//			fmt.Printf("Failed to marshal assetConstraints: %v\n", err)
//			return
//		}
//
//		// Sample categorical constraints: limit EQUITY to 80% and CRYPTO to 20%
//		categoricalConstraints, err := json.Marshal([]map[string]interface{}{
//			{
//				"name": "AssetClass",
//				"category_per_symbol": []string{"EQUITY", "EQUITY", "EQUITY", "EQUITY", "CRYPTO"},
//				"weight_per_category": [][]interface{}{
//					{"EQUITY", 0.0, 0.8},
//					{"CRYPTO", 0.0, 0.2},
//				},
//			},
//		})
//		if err != nil {
//			fmt.Printf("Error marshaling categoricalConstraints: %v\n", err)
//			return
//		}
//
//		portfolio, err := finalytics.NewPortfolioBuilder().
//			TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//			BenchmarkSymbol("^GSPC").
//			StartDate("2023-01-01").
//			EndDate("2023-12-31").
//			Interval("1d").
//			ConfidenceLevel(0.95).
//			RiskFreeRate(0.02).
//			ObjectiveFunction("max_sharpe").
//			AssetConstraints(string(assetConstraints)).
//			CategoricalConstraints(string(categoricalConstraints)).
//			weights("{}").
//			Build()
//		if err != nil {
//			fmt.Printf("Failed to create Portfolio: %v\n", err)
//			return
//		}
//		defer portfolio.Free()
//		fmt.Println("Portfolio created successfully for AAPL, MSFT, NVDA, and BTC-USD")
//	}
func (b *PortfolioBuilder) Build() (*Portfolio, error) {
	// Validate required parameter
	if len(b.tickerSymbols) == 0 {
		return nil, errors.New("tickerSymbols is required and cannot be empty")
	}

	// Convert tickerSymbols to JSON
	symbolsString, err := StringSliceToJSON(b.tickerSymbols)
	if err != nil {
		return nil, fmt.Errorf("failed to convert symbols to JSON: %v", err)
	}
	cTickerSymbols := C.CString(symbolsString)
	defer C.free(unsafe.Pointer(cTickerSymbols))
	cBenchmarkSymbol := C.CString(b.benchmarkSymbol)
	defer C.free(unsafe.Pointer(cBenchmarkSymbol))
	cStartDate := C.CString(b.startDate)
	defer C.free(unsafe.Pointer(cStartDate))
	cEndDate := C.CString(b.endDate)
	defer C.free(unsafe.Pointer(cEndDate))
	cInterval := C.CString(b.interval)
	defer C.free(unsafe.Pointer(cInterval))
	cObjectiveFunction := C.CString(b.objectiveFunction)
	defer C.free(unsafe.Pointer(cObjectiveFunction))
	cAssetConstraints := C.CString(b.assetConstraints)
	defer C.free(unsafe.Pointer(cAssetConstraints))
	cCategoricalConstraints := C.CString(b.categoricalConstraints)
	defer C.free(unsafe.Pointer(cCategoricalConstraints))
	cweights := C.CString(b.weights)
	defer C.free(unsafe.Pointer(cweights))

	// Handle tickersData
	var cTickersData *C.char
	if len(b.tickersData) > 0 {
		jsonStr, err := dataFramesToJSONString(b.tickersData)
		if err != nil {
			return nil, fmt.Errorf("failed to convert tickersData to JSON: %v", err)
		}
		cTickersData = C.CString(jsonStr)
		defer C.free(unsafe.Pointer(cTickersData))
	} else {
		cTickersData = nil
	}

	// Handle benchmarkData
	var cBenchmarkData *C.char
	if b.benchmarkData != nil {
		jsonStr, err := dataFrameToJSONString(*b.benchmarkData)
		if err != nil {
			return nil, fmt.Errorf("failed to convert benchmarkData to JSON: %v", err)
		}
		cBenchmarkData = C.CString(jsonStr)
		defer C.free(unsafe.Pointer(cBenchmarkData))
	} else {
		cBenchmarkData = nil
	}

	// Handle transactions
	cTransactions := C.CString(b.transactions)
	defer C.free(unsafe.Pointer(cTransactions))

	// Handle rebalance strategy
	cRebalanceStrategy := C.CString(b.rebalanceStrategy)
	defer C.free(unsafe.Pointer(cRebalanceStrategy))

	// Handle scheduled cash flows
	cScheduledCashFlows := C.CString(b.scheduledCashFlows)
	defer C.free(unsafe.Pointer(cScheduledCashFlows))

	// Call the Rust function (or C FFI function)
	handle := C.finalytics_portfolio_new(
		cTickerSymbols,
		cBenchmarkSymbol,
		cStartDate,
		cEndDate,
		cInterval,
		C.double(b.confidenceLevel),
		C.double(b.riskFreeRate),
		cObjectiveFunction,
		cAssetConstraints,
		cCategoricalConstraints,
		cweights,
		cTickersData,
		cBenchmarkData,
		cTransactions,
		cRebalanceStrategy,
		cScheduledCashFlows,
	)
	if handle == nil {
		return nil, getLastError("failed to create Portfolio")
	}
	return &Portfolio{handle: handle}, nil
}

// Free releases the resources associated with the Portfolio.
// It should be called when the Portfolio is no longer needed to prevent memory leaks.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		portfolio, err := finalytics.NewPortfolioBuilder().
//			TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//			Build()
//		if err != nil {
//			fmt.Printf("Failed to create Portfolio: %v\n", err)
//			return
//		}
//		portfolio.Free()
//		fmt.Println("Portfolio resources freed successfully")
//	}
func (p *Portfolio) Free() {
	if p.handle != nil {
		C.finalytics_portfolio_free(p.handle)
		p.handle = nil
	}
}

// OptimizationResults retrieves portfolio optimization results.
//
// Returns:
//   - map[string]any: A map containing the optimization results (e.g., weights, expected return, volatility).
//   - error: An error if the optimization results retrieval fails.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		portfolio, err := finalytics.NewPortfolioBuilder().
//			TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//			BenchmarkSymbol("^GSPC").
//			StartDate("2023-01-01").
//			EndDate("2023-12-31").
//			Interval("1d").
//			ConfidenceLevel(0.95).
//			RiskFreeRate(0.02).
//			ObjectiveFunction("max_sharpe").
//			Build()
//		if err != nil {
//			fmt.Printf("Failed to create Portfolio: %v\n", err)
//			return
//		}
//		defer portfolio.Free()
//
//		results, err := portfolio.OptimizationResults()
//		if err != nil {
//			fmt.Printf("Failed to get optimization results: %v\n", err)
//			return
//		}
//		fmt.Printf("Optimization Results: %v\n", results)
//	}
func (p *Portfolio) OptimizationResults() (map[string]any, error) {
	var cOutput *C.char
	result := C.finalytics_portfolio_optimization_results(p.handle, &cOutput)
	if result != 0 {
		return nil, getLastError("failed to get optimization results")
	}
	return parseJSONResult(cOutput)
}

// OptimizationChart retrieves the portfolio optimization chart as an HTML object.
//
// Parameters:
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the optimization chart.
//   - error: An error if the chart retrieval fails.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		portfolio, err := finalytics.NewPortfolioBuilder().
//			TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//			BenchmarkSymbol("^GSPC").
//			StartDate("2023-01-01").
//			EndDate("2023-12-31").
//			Interval("1d").
//			ConfidenceLevel(0.95).
//			RiskFreeRate(0.02).
//			ObjectiveFunction("max_sharpe").
//			Build()
//		if err != nil {
//			fmt.Printf("Failed to create Portfolio: %v\n", err)
//			return
//		}
//		defer portfolio.Free()
//
//		optChart, err := portfolio.OptimizationChart(0, 0)
//		if err != nil {
//			fmt.Printf("Failed to get optimization chart: %v\n", err)
//			return
//		}
//		optChart.Show()
//	}
func (p *Portfolio) OptimizationChart(height, width uint) (HTML, error) {
	var cOutput *C.char
	result := C.finalytics_portfolio_optimization_chart(p.handle, C.uint(height), C.uint(width), &cOutput)
	if result != 0 {
		return HTML{}, getLastError("failed to get optimization chart")
	}
	defer C.finalytics_free_string(cOutput)
	htmlStr := C.GoString(cOutput)
	return HTML{Content: htmlStr}, nil
}

// PerformanceChart retrieves the portfolio performance chart as an HTML object.
//
// Parameters:
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the performance chart.
//   - error: An error if the chart retrieval fails.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		portfolio, err := finalytics.NewPortfolioBuilder().
//			TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//			BenchmarkSymbol("^GSPC").
//			StartDate("2023-01-01").
//			EndDate("2023-12-31").
//			Interval("1d").
//			ConfidenceLevel(0.95).
//			RiskFreeRate(0.02).
//			ObjectiveFunction("max_sharpe").
//			Build()
//		if err != nil {
//			fmt.Printf("Failed to create Portfolio: %v\n", err)
//			return
//		}
//		defer portfolio.Free()
//
//		perfChart, err := portfolio.PerformanceChart(0, 0)
//		if err != nil {
//			fmt.Printf("Failed to get performance chart: %v\n", err)
//			return
//		}
//		perfChart.Show()
//	}
func (p *Portfolio) PerformanceChart(height, width uint) (HTML, error) {
	var cOutput *C.char
	result := C.finalytics_portfolio_performance_chart(p.handle, C.uint(height), C.uint(width), &cOutput)
	if result != 0 {
		return HTML{}, getLastError("failed to get performance chart")
	}
	defer C.finalytics_free_string(cOutput)
	htmlStr := C.GoString(cOutput)
	return HTML{Content: htmlStr}, nil
}

// PerformanceStats retrieves performance statistics for the portfolio.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated performance statistics for the portfolio (e.g., returns, volatility, Sharpe ratio).
//   - error: An error if the performance statistics retrieval fails.
//
// Example:
//
//	  package main
//
//	  import (
//	  	"fmt"
//	  	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	  	"github.com/go-gota/gota/dataframe"
//	  )
//
//	  func main() {
//			portfolio, err := finalytics.NewPortfolioBuilder().
//				TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//				BenchmarkSymbol("^GSPC").
//				StartDate("2023-01-01").
//				EndDate("2023-12-31").
//				Interval("1d").
//				ConfidenceLevel(0.95).
//				RiskFreeRate(0.02).
//				ObjectiveFunction("max_sharpe").
//				Build()
//			if err != nil {
//				fmt.Printf("Failed to create Portfolio: %v\n", err)
//				return
//			}
//			defer portfolio.Free()
//
//	  	perfStats, err := portfolio.PerformanceStats()
//	  	if err != nil {
//	  		fmt.Printf("Failed to get performance stats: %v\n", err)
//	  		return
//	  	}
//	  	fmt.Printf("Performance Stats:\n%v\n", perfStats)
//	  }
func (t *Portfolio) PerformanceStats() (dataframe.DataFrame, error) {
	var cOutput *C.char
	result := C.finalytics_portfolio_performance_stats(t.handle, &cOutput)
	if result != 0 {
		return dataframe.DataFrame{}, getLastError("failed to get performance stats")
	}
	return parseJSONToDataFrame(cOutput)
}

// AssetReturnsChart retrieves the asset returns chart for the portfolio as an HTML object.
//
// Parameters:
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the asset returns chart.
//   - error: An error if the chart retrieval fails.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		portfolio, err := finalytics.NewPortfolioBuilder().
//			TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//			BenchmarkSymbol("^GSPC").
//			StartDate("2023-01-01").
//			EndDate("2023-12-31").
//			Interval("1d").
//			ConfidenceLevel(0.95).
//			RiskFreeRate(0.02).
//			ObjectiveFunction("max_sharpe").
//			Build()
//		if err != nil {
//			fmt.Printf("Failed to create Portfolio: %v\n", err)
//			return
//		}
//		defer portfolio.Free()
//
//		assetChart, err := portfolio.AssetReturnsChart(0, 0)
//		if err != nil {
//			fmt.Printf("Failed to get asset returns chart: %v\n", err)
//			return
//		}
//		assetChart.Show()
//	}
func (p *Portfolio) AssetReturnsChart(height, width uint) (HTML, error) {
	var cOutput *C.char
	result := C.finalytics_portfolio_asset_returns_chart(p.handle, C.uint(height), C.uint(width), &cOutput)
	if result != 0 {
		return HTML{}, getLastError("failed to get asset returns chart")
	}
	defer C.finalytics_free_string(cOutput)
	htmlStr := C.GoString(cOutput)
	return HTML{Content: htmlStr}, nil
}

// PortfolioValueChart retrieves the portfolio value over time chart as an HTML object.
//
// Parameters:
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the portfolio value chart.
//   - error: An error if the chart retrieval fails.
func (p *Portfolio) PortfolioValueChart(height, width uint) (HTML, error) {
	var cOutput *C.char
	result := C.finalytics_portfolio_value_chart(p.handle, C.uint(height), C.uint(width), &cOutput)
	if result != 0 {
		return HTML{}, getLastError("failed to get portfolio value chart")
	}
	defer C.finalytics_free_string(cOutput)
	htmlStr := C.GoString(cOutput)
	return HTML{Content: htmlStr}, nil
}

// ReturnsMatrix retrieves the returns correlation matrix for the portfolio as an HTML object.
//
// Parameters:
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the returns correlation matrix.
//   - error: An error if the matrix retrieval fails.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		portfolio, err := finalytics.NewPortfolioBuilder().
//			TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//			BenchmarkSymbol("^GSPC").
//			StartDate("2023-01-01").
//			EndDate("2023-12-31").
//			Interval("1d").
//			ConfidenceLevel(0.95).
//			RiskFreeRate(0.02).
//			ObjectiveFunction("max_sharpe").
//			Build()
//		if err != nil {
//			fmt.Printf("Failed to create Portfolio: %v\n", err)
//			return
//		}
//		defer portfolio.Free()
//
//		retMatrix, err := portfolio.ReturnsMatrix(0, 0)
//		if err != nil {
//			fmt.Printf("Failed to get returns matrix: %v\n", err)
//			return
//		}
//		retMatrix.Show()
//	}
func (p *Portfolio) ReturnsMatrix(height, width uint) (HTML, error) {
	var cOutput *C.char
	result := C.finalytics_portfolio_returns_matrix(p.handle, C.uint(height), C.uint(width), &cOutput)
	if result != 0 {
		return HTML{}, getLastError("failed to get returns matrix")
	}
	defer C.finalytics_free_string(cOutput)
	htmlStr := C.GoString(cOutput)
	return HTML{Content: htmlStr}, nil
}

// Report retrieves a comprehensive report for the portfolio as an HTML object.
//
// Parameters:
//   - reportType: The type of report to display (e.g., "performance").
//
// Returns:
//   - HTML: An HTML object containing the report.
//   - error: An error if the report retrieval fails.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"github.com/Nnamdi-sys/finalytics/go/finalytics"
//	)
//
//	func main() {
//		portfolio, err := finalytics.NewPortfolioBuilder().
//			TickerSymbols([]string{"AAPL", "MSFT", "NVDA", "BTC-USD"}).
//			BenchmarkSymbol("^GSPC").
//			StartDate("2023-01-01").
//			EndDate("2023-12-31").
//			Interval("1d").
//			ConfidenceLevel(0.95).
//			RiskFreeRate(0.02).
//			ObjectiveFunction("max_sharpe").
//			Build()
//		if err != nil {
//			fmt.Printf("Failed to create Portfolio: %v\n", err)
//			return
//		}
//		defer portfolio.Free()
//
//		report, err := portfolio.Report("performance")
//		if err != nil {
//			fmt.Printf("Failed to get report: %v\n", err)
//			return
//		}
//		report.Show()
//	}
//
// UpdateDates updates the portfolio's date range and re-fetches data for out-of-sample evaluation.
//
// This method is for portfolios built from Yahoo Finance data (not custom data).
// It rebuilds all underlying ticker and benchmark data for the new date range.
// The optimization result (weights) is preserved so they can be evaluated
// out-of-sample on the new period.
//
// After calling this method, call PerformanceStats() to evaluate the
// optimized weights on the new data (it recomputes automatically).
//
// Parameters:
//   - startDate: New start date (e.g., "2024-01-01").
//   - endDate: New end date (e.g., "2024-12-31").
//
// Returns:
//   - error: An error if the update fails.
//
// Example:
//
//	portfolio.UpdateDates("2024-01-01", "2024-12-31")
//	stats, _ := portfolio.PerformanceStats()
//	fmt.Println(stats)
func (p *Portfolio) UpdateDates(startDate, endDate string) error {
	cStartDate := C.CString(startDate)
	defer C.free(unsafe.Pointer(cStartDate))
	cEndDate := C.CString(endDate)
	defer C.free(unsafe.Pointer(cEndDate))

	result := C.finalytics_portfolio_update_dates(p.handle, cStartDate, cEndDate)
	if result != 0 {
		return getLastError("failed to update portfolio dates")
	}
	return nil
}

// TransactionHistory retrieves the transaction history table for the portfolio.
//
// Returns a table of all transaction events during the simulation, including
// rebalances, cash flows, and combined events. Each row includes portfolio
// value before/after, per-asset values, trade amounts, turnover, cumulative TWR and MWR.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing the transaction history.
//   - error: An error if the retrieval fails.
func (p *Portfolio) TransactionHistory() (dataframe.DataFrame, error) {
	var cOutput *C.char
	result := C.finalytics_portfolio_transaction_history(p.handle, &cOutput)
	if result != 0 {
		return dataframe.DataFrame{}, getLastError("failed to get transaction history")
	}
	return parseJSONToDataFrame(cOutput)
}

func (p *Portfolio) Report(reportType string) (HTML, error) {
	cReportType := C.CString(reportType)
	defer C.free(unsafe.Pointer(cReportType))
	var cOutput *C.char
	result := C.finalytics_portfolio_report(p.handle, cReportType, &cOutput)
	if result != 0 {
		return HTML{}, getLastError("failed to get report")
	}
	defer C.finalytics_free_string(cOutput)
	htmlStr := C.GoString(cOutput)
	return HTML{Content: htmlStr}, nil
}
