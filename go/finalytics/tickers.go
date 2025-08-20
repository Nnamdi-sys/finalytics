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

// Tickers represents a collection of financial tickers with methods for retrieving aggregated data and analytics.
// It encapsulates a handle to the underlying C library for interacting with multiple tickers.
type Tickers struct {
    handle C.TickersHandle
}

// TickersBuilder is used to construct a Tickers instance using the builder pattern.
// It allows for fluent configuration of the Tickers' parameters before creation.
type TickersBuilder struct {
    symbols          []string
    startDate        string
    endDate          string
    interval         string
    benchmarkSymbol  string
    confidenceLevel  float64
    riskFreeRate     float64
    tickersData      []dataframe.DataFrame
    benchmarkData    *dataframe.DataFrame
}

// NewTickersBuilder initializes a new TickersBuilder with default values.
// Defaults:
//   - symbols: nil
//   - startDate: ""
//   - endDate: ""
//   - interval: "1d"
//   - benchmarkSymbol: ""
//   - confidenceLevel: 0.95
//   - riskFreeRate: 0.02
//   - tickersData: nil
//   - benchmarkData: nil
//
// Returns:
//   - *TickersBuilder: A pointer to the initialized TickersBuilder.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	builder := finalytics.NewTickersBuilder()
//   	fmt.Println("TickersBuilder initialized")
//   }
func NewTickersBuilder() *TickersBuilder {
    return &TickersBuilder{
        symbols:          nil,
        startDate:        "",
        endDate:          "",
        interval:         "1d",
        benchmarkSymbol:  "",
        confidenceLevel:  0.95,
        riskFreeRate:     0.02,
        tickersData:      nil,
        benchmarkData:    nil,
    }
}

// Symbols sets the ticker symbols for the Tickers.
//
// Parameters:
//   - symbols: A string slice of ticker symbols (e.g., []string{"AAPL", "MSFT"}).
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().Symbols([]string{"AAPL", "MSFT"})
func (b *TickersBuilder) Symbols(symbols []string) *TickersBuilder {
    b.symbols = symbols
    return b
}

// StartDate sets the start date for the Tickers' data period.
//
// Parameters:
//   - startDate: The start date in the format YYYY-MM-DD.
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().StartDate("2023-01-01")
func (b *TickersBuilder) StartDate(startDate string) *TickersBuilder {
    b.startDate = startDate
    return b
}

// EndDate sets the end date for the Tickers' data period.
//
// Parameters:
//   - endDate: The end date in the format YYYY-MM-DD.
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().EndDate("2023-12-31")
func (b *TickersBuilder) EndDate(endDate string) *TickersBuilder {
    b.endDate = endDate
    return b
}

// Interval sets the data interval for the Tickers.
//
// Parameters:
//   - interval: The data interval (e.g., "2m", "5m", "15m", "30m", "1h", "1d", "1wk", "1mo", "3mo").
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().Interval("1d")
func (b *TickersBuilder) Interval(interval string) *TickersBuilder {
    b.interval = interval
    return b
}

// BenchmarkSymbol sets the benchmark symbol for the Tickers.
//
// Parameters:
//   - benchmarkSymbol: The ticker symbol of the benchmark (e.g., "^GSPC").
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().BenchmarkSymbol("^GSPC")
func (b *TickersBuilder) BenchmarkSymbol(benchmarkSymbol string) *TickersBuilder {
    b.benchmarkSymbol = benchmarkSymbol
    return b
}

// ConfidenceLevel sets the confidence level for VaR and ES calculations.
//
// Parameters:
//   - confidenceLevel: The confidence level (e.g., 0.95 for 95% confidence).
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().ConfidenceLevel(0.99)
func (b *TickersBuilder) ConfidenceLevel(confidenceLevel float64) *TickersBuilder {
    b.confidenceLevel = confidenceLevel
    return b
}

// RiskFreeRate sets the risk-free rate for calculations.
//
// Parameters:
//   - riskFreeRate: The risk-free rate (e.g., 0.02 for 2%).
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().RiskFreeRate(0.03)
func (b *TickersBuilder) RiskFreeRate(riskFreeRate float64) *TickersBuilder {
    b.riskFreeRate = riskFreeRate
    return b
}

// TickersData sets custom ticker data for the Tickers.
//
// Parameters:
//   - tickersData: A slice of DataFrames containing custom ticker data for each symbol (pass nil or empty slice if not using custom data).
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().TickersData(nil)
func (b *TickersBuilder) TickersData(tickersData []dataframe.DataFrame) *TickersBuilder {
    b.tickersData = tickersData
    return b
}

// BenchmarkData sets custom benchmark data for the Tickers.
//
// Parameters:
//   - benchmarkData: A DataFrame containing custom benchmark data (pass nil if not using custom data).
//
// Returns:
//   - *TickersBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickersBuilder().BenchmarkData(nil)
func (b *TickersBuilder) BenchmarkData(benchmarkData *dataframe.DataFrame) *TickersBuilder {
    b.benchmarkData = benchmarkData
    return b
}

// Build constructs the Tickers instance with the configured parameters.
// The symbols parameter is required; other parameters are optional and use defaults if not set.
//
// Returns:
//   - *Tickers: A pointer to the initialized Tickers object.
//   - error: An error if the Tickers creation fails or symbols is missing/empty.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
//   		ConfidenceLevel(0.95).
//   		RiskFreeRate(0.02).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//   	fmt.Println("Tickers created successfully for AAPL and MSFT")
//   }
func (b *TickersBuilder) Build() (*Tickers, error) {
    // Validate required parameter
    if len(b.symbols) == 0 {
        return nil, errors.New("symbols is required and cannot be empty")
    }

    // Convert symbols to JSON
    symbolsString, err := StringSliceToJSON(b.symbols)
    if err != nil {
        return nil, fmt.Errorf("failed to convert symbols to JSON: %v", err)
    }
    cSymbols := C.CString(symbolsString)
    defer C.free(unsafe.Pointer(cSymbols))
    cStartDate := C.CString(b.startDate)
    defer C.free(unsafe.Pointer(cStartDate))
    cEndDate := C.CString(b.endDate)
    defer C.free(unsafe.Pointer(cEndDate))
    cInterval := C.CString(b.interval)
    defer C.free(unsafe.Pointer(cInterval))
    cBenchmarkSymbol := C.CString(b.benchmarkSymbol)
    defer C.free(unsafe.Pointer(cBenchmarkSymbol))

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

    // Call the Rust function (or C FFI function)
    handle := C.finalytics_tickers_new(
        cSymbols,
        cStartDate,
        cEndDate,
        cInterval,
        cBenchmarkSymbol,
        C.double(b.confidenceLevel),
        C.double(b.riskFreeRate),
        cTickersData,
        cBenchmarkData,
    )
    if handle == nil {
        return nil, errors.New("failed to create Tickers")
    }
    return &Tickers{handle: handle}, nil
}

// Free releases the resources associated with the Tickers.
// It should be called when the Tickers is no longer needed to prevent memory leaks.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	tickers.Free()
//   	fmt.Println("Tickers resources freed successfully")
//   }
func (t *Tickers) Free() {
    if t.handle != nil {
        C.finalytics_tickers_free(t.handle)
        t.handle = nil
    }
}

// GetSummaryStats retrieves summary technical and fundamental statistics for the tickers.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated summary statistics for all tickers.
//   - error: An error if the statistics retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	summary, err := tickers.GetSummaryStats()
//   	if err != nil {
//   		fmt.Printf("Failed to get summary stats: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Summary Stats:\n%v\n", summary)
//   }
func (t *Tickers) GetSummaryStats() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_tickers_get_summary_stats(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get summary stats: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetPriceHistory retrieves the OHLCV (Open, High, Low, Close, Volume) price history for the tickers.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated price history data for all tickers.
//   - error: An error if the price history retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	history, err := tickers.GetPriceHistory()
//   	if err != nil {
//   		fmt.Printf("Failed to get price history: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Price History:\n%v\n", history)
//   }
func (t *Tickers) GetPriceHistory() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_tickers_get_price_history(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get price history: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetOptionsChain retrieves the options chain for the tickers.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated options chain data for all tickers.
//   - error: An error if the options chain retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	options, err := tickers.GetOptionsChain()
//   	if err != nil {
//   		fmt.Printf("Failed to get options chain: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Options Chain:\n%v\n", options)
//   }
func (t *Tickers) GetOptionsChain() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_tickers_get_options_chain(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get options chain: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetNews retrieves the latest news headlines for the tickers.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated news data for all tickers.
//   - error: An error if the news retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	news, err := tickers.GetNews()
//   	if err != nil {
//   		fmt.Printf("Failed to get news: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("News:\n%v\n", news)
//   }
func (t *Tickers) GetNews() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_tickers_get_news(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get news: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetIncomeStatement retrieves the income statements for the tickers.
//
// Parameters:
//   - frequency: The frequency of the statement ("annual" or "quarterly").
//   - formatted: Whether to return the statement in a formatted manner.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated income statement data for all tickers.
//   - error: An error if the income statement retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	income, err := tickers.GetIncomeStatement("quarterly", true)
//   	if err != nil {
//   		fmt.Printf("Failed to get income statement: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Income Statement:\n%v\n", income)
//   }
func (t *Tickers) GetIncomeStatement(frequency string, formatted bool) (dataframe.DataFrame, error) {
    cFrequency := C.CString(frequency)
    defer C.free(unsafe.Pointer(cFrequency))
    cFormatted := C.int(0)
    if formatted {
        cFormatted = C.int(1)
    }
    var cOutput *C.char
    result := C.finalytics_tickers_get_income_statement(t.handle, cFrequency, cFormatted, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get income statement: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetBalanceSheet retrieves the balance sheets for the tickers.
//
// Parameters:
//   - frequency: The frequency of the statement ("annual" or "quarterly").
//   - formatted: Whether to return the statement in a formatted manner.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated balance sheet data for all tickers.
//   - error: An error if the balance sheet retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	balance, err := tickers.GetBalanceSheet("quarterly", true)
//   	if err != nil {
//   		fmt.Printf("Failed to get balance sheet: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Balance Sheet:\n%v\n", balance)
//   }
func (t *Tickers) GetBalanceSheet(frequency string, formatted bool) (dataframe.DataFrame, error) {
    cFrequency := C.CString(frequency)
    defer C.free(unsafe.Pointer(cFrequency))
    cFormatted := C.int(0)
    if formatted {
        cFormatted = C.int(1)
    }
    var cOutput *C.char
    result := C.finalytics_tickers_get_balance_sheet(t.handle, cFrequency, cFormatted, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get balance sheet: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetCashflowStatement retrieves the cash flow statements for the tickers.
//
// Parameters:
//   - frequency: The frequency of the statement ("annual" or "quarterly").
//   - formatted: Whether to return the statement in a formatted manner.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated cash flow statement data for all tickers.
//   - error: An error if the cash flow statement retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	cashflow, err := tickers.GetCashflowStatement("quarterly", true)
//   	if err != nil {
//   		fmt.Printf("Failed to get cash flow statement: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Cash Flow Statement:\n%v\n", cashflow)
//   }
func (t *Tickers) GetCashflowStatement(frequency string, formatted bool) (dataframe.DataFrame, error) {
    cFrequency := C.CString(frequency)
    defer C.free(unsafe.Pointer(cFrequency))
    cFormatted := C.int(0)
    if formatted {
        cFormatted = C.int(1)
    }
    var cOutput *C.char
    result := C.finalytics_tickers_get_cashflow_statement(t.handle, cFrequency, cFormatted, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get cash flow statement: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetFinancialRatios retrieves financial ratios for the tickers.
//
// Parameters:
//   - frequency: The frequency of the ratios ("annual" or "quarterly").
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated financial ratios for all tickers.
//   - error: An error if the financial ratios retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	ratios, err := tickers.GetFinancialRatios("quarterly")
//   	if err != nil {
//   		fmt.Printf("Failed to get financial ratios: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Financial Ratios:\n%v\n", ratios)
//   }
func (t *Tickers) GetFinancialRatios(frequency string) (dataframe.DataFrame, error) {
    cFrequency := C.CString(frequency)
    defer C.free(unsafe.Pointer(cFrequency))
    var cOutput *C.char
    result := C.finalytics_tickers_get_financial_ratios(t.handle, cFrequency, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get financial ratios: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// Returns retrieves returns data for the tickers.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing returns data for all tickers.
//   - error: An error if the returns retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	returns, err := tickers.Returns()
//   	if err != nil {
//   		fmt.Printf("Failed to get returns: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Returns:\n%v\n", returns)
//   }
func (t *Tickers) Returns() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_tickers_returns(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get returns: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// PerformanceStats retrieves performance statistics for the tickers.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing aggregated performance statistics for all tickers (e.g., returns, volatility, Sharpe ratio).
//   - error: An error if the performance statistics retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   	"github.com/go-gota/gota/dataframe"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
//   		ConfidenceLevel(0.95).
//   		RiskFreeRate(0.02).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	perfStats, err := tickers.PerformanceStats()
//   	if err != nil {
//   		fmt.Printf("Failed to get performance stats: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Performance Stats:\n%v\n", perfStats)
//   }
func (t *Tickers) PerformanceStats() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_tickers_performance_stats(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get performance stats: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// ReturnsChart retrieves the returns chart for the tickers as an HTML object.
//
// Parameters:
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the returns chart.
//   - error: An error if the chart retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	retChart, err := tickers.ReturnsChart(0, 0)
//   	if err != nil {
//   		fmt.Printf("Failed to get returns chart: %v\n", err)
//   		return
//   	}
//   	retChart.Show()
//   }
func (t *Tickers) ReturnsChart(height, width uint) (HTML, error) {
    var cOutput *C.char
    result := C.finalytics_tickers_returns_chart(t.handle, C.uint(height), C.uint(width), &cOutput)
    if result != 0 {
        return HTML{}, fmt.Errorf("failed to get returns chart: error code %d", result)
    }
    defer C.finalytics_free_string(cOutput)
    htmlStr := C.GoString(cOutput)
    return HTML{Content: htmlStr}, nil
}

// ReturnsMatrix retrieves the returns correlation matrix for the tickers as an HTML object.
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
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	retMatrix, err := tickers.ReturnsMatrix(0, 0)
//   	if err != nil {
//   		fmt.Printf("Failed to get returns matrix: %v\n", err)
//   		return
//   	}
//   	retMatrix.Show()
//   }
func (t *Tickers) ReturnsMatrix(height, width uint) (HTML, error) {
    var cOutput *C.char
    result := C.finalytics_tickers_returns_matrix(t.handle, C.uint(height), C.uint(width), &cOutput)
    if result != 0 {
        return HTML{}, fmt.Errorf("failed to get returns matrix: error code %d", result)
    }
    defer C.finalytics_free_string(cOutput)
    htmlStr := C.GoString(cOutput)
    return HTML{Content: htmlStr}, nil
}

// Report retrieves a comprehensive analytics report for the tickers as an HTML object.
//
// Parameters:
//   - reportType: The type of report to display (e.g., "performance", "financials", "options", "news").
//
// Returns:
//   - HTML: An HTML object containing the report.
//   - error: An error if the report retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
//   		ConfidenceLevel(0.95).
//   		RiskFreeRate(0.02).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	report, err := tickers.Report("performance")
//   	if err != nil {
//   		fmt.Printf("Failed to get report: %v\n", err)
//   		return
//   	}
//   	report.Show()
//   }
func (t *Tickers) Report(reportType string) (HTML, error) {
    cReportType := C.CString(reportType)
    defer C.free(unsafe.Pointer(cReportType))
    var cOutput *C.char
    result := C.finalytics_tickers_report(t.handle, cReportType, &cOutput)
    if result != 0 {
        return HTML{}, fmt.Errorf("failed to get report: error code %d", result)
    }
    defer C.finalytics_free_string(cOutput)
    htmlStr := C.GoString(cOutput)
    return HTML{Content: htmlStr}, nil
}

// GetTicker retrieves a Ticker instance for a specific symbol from the Tickers collection.
//
// Parameters:
//   - symbol: The ticker symbol to retrieve (e.g., "AAPL").
//
// Returns:
//   - *Ticker: A pointer to the Ticker object for the specified symbol.
//   - error: An error if the Ticker retrieval fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	ticker, err := tickers.GetTicker("AAPL")
//   	if err != nil {
//   		fmt.Printf("Failed to get Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//   	fmt.Println("Successfully retrieved Ticker for AAPL")
//   }
func (t *Tickers) GetTicker(symbol string) (*Ticker, error) {
    cSymbol := C.CString(symbol)
    defer C.free(unsafe.Pointer(cSymbol))
    handle := C.finalytics_tickers_get_ticker(t.handle, cSymbol)
    if handle == nil {
        return nil, errors.New("failed to get Ticker")
    }
    return &Ticker{handle: handle}, nil
}

// Optimize optimizes the portfolio of tickers based on the specified objective and constraints.
//
// Parameters:
//   - objectiveFunction: The objective function for optimization (e.g., "max_sharpe").
//   - assetConstraints: JSON string defining asset-level constraints (e.g., "{}").
//   - categoricalConstraints: JSON string defining categorical constraints (e.g., "{}").
//   - weights: JSON string defining portfolio-level constraints (e.g., "{}").
//
// Returns:
//   - *Portfolio: A pointer to the optimized Portfolio object.
//   - error: An error if the portfolio optimization fails.
//
// Example:
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	tickers, err := finalytics.NewTickersBuilder().
//   		Symbols([]string{"AAPL", "MSFT"}).
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
//   		ConfidenceLevel(0.95).
//   		RiskFreeRate(0.02).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Tickers: %v\n", err)
//   		return
//   	}
//   	defer tickers.Free()
//
//   	portfolio, err := tickers.Optimize("max_sharpe", "{}", "{}", "{}")
//   	if err != nil {
//   		fmt.Printf("Failed to optimize portfolio: %v\n", err)
//   		return
//   	}
//   	defer portfolio.Free()
//   	fmt.Println("Successfully optimized portfolio")
//   }
func (t *Tickers) Optimize(objectiveFunction, assetConstraints, categoricalConstraints, weights string) (*Portfolio, error) {
    cObjectiveFunction := C.CString(objectiveFunction)
    defer C.free(unsafe.Pointer(cObjectiveFunction))
    cAssetConstraints := C.CString(assetConstraints)
    defer C.free(unsafe.Pointer(cAssetConstraints))
    cCategoricalConstraints := C.CString(categoricalConstraints)
    defer C.free(unsafe.Pointer(cCategoricalConstraints))
    cWeights := C.CString(weights)
    defer C.free(unsafe.Pointer(cWeights))

    handle := C.finalytics_tickers_optimize(t.handle, cObjectiveFunction, cAssetConstraints, cCategoricalConstraints, cWeights)
    if handle == nil {
        return nil, errors.New("failed to optimize portfolio")
    }
    return &Portfolio{handle: handle}, nil
}