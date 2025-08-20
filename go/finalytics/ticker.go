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

// Ticker represents a financial ticker with methods for retrieving financial data and analytics.
// It encapsulates a handle to the underlying C library for interacting with financial data.
type Ticker struct {
    handle C.TickerHandle
}

// TickerBuilder is used to construct a Ticker instance using the builder pattern.
// It allows for fluent configuration of the Ticker's parameters before creation.
type TickerBuilder struct {
    symbol          string
    startDate       string
    endDate         string
    interval        string
    benchmarkSymbol string
    confidenceLevel float64
    riskFreeRate    float64
    tickerData      *dataframe.DataFrame
    benchmarkData   *dataframe.DataFrame
}

// NewTickerBuilder initializes a new TickerBuilder with default values.
// Defaults:
//   - confidenceLevel: 0.95
//   - riskFreeRate: 0.02
//   - interval: "1d"
//   - startDate: ""
//   - endDate: ""
//   - benchmarkSymbol: ""
//   - tickerData: nil
//   - benchmarkData: nil
//
// Returns:
//   - *TickerBuilder: A pointer to the initialized TickerBuilder.
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
//   	builder := finalytics.NewTickerBuilder()
//   	fmt.Println("TickerBuilder initialized")
//   }
func NewTickerBuilder() *TickerBuilder {
    return &TickerBuilder{
        confidenceLevel: 0.95,
        riskFreeRate:    0.02,
        interval:        "1d",
        startDate:       "",
        endDate:         "",
        benchmarkSymbol: "",
        tickerData:      nil,
        benchmarkData:   nil,
    }
}

// Symbol sets the ticker symbol for the Ticker.
//
// Parameters:
//   - symbol: The ticker symbol of the asset (e.g., "AAPL").
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().Symbol("AAPL")
func (b *TickerBuilder) Symbol(symbol string) *TickerBuilder {
    b.symbol = symbol
    return b
}

// StartDate sets the start date for the Ticker's data period.
//
// Parameters:
//   - startDate: The start date in the format YYYY-MM-DD.
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().StartDate("2023-01-01")
func (b *TickerBuilder) StartDate(startDate string) *TickerBuilder {
    b.startDate = startDate
    return b
}

// EndDate sets the end date for the Ticker's data period.
//
// Parameters:
//   - endDate: The end date in the format YYYY-MM-DD.
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().EndDate("2023-12-31")
func (b *TickerBuilder) EndDate(endDate string) *TickerBuilder {
    b.endDate = endDate
    return b
}

// Interval sets the data interval for the Ticker.
//
// Parameters:
//   - interval: The data interval (e.g., "2m", "5m", "15m", "30m", "1h", "1d", "1wk", "1mo", "3mo").
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().Interval("1d")
func (b *TickerBuilder) Interval(interval string) *TickerBuilder {
    b.interval = interval
    return b
}

// BenchmarkSymbol sets the benchmark symbol for the Ticker.
//
// Parameters:
//   - benchmarkSymbol: The ticker symbol of the benchmark (e.g., "^GSPC").
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().BenchmarkSymbol("^GSPC")
func (b *TickerBuilder) BenchmarkSymbol(benchmarkSymbol string) *TickerBuilder {
    b.benchmarkSymbol = benchmarkSymbol
    return b
}

// ConfidenceLevel sets the confidence level for VaR and ES calculations.
//
// Parameters:
//   - confidenceLevel: The confidence level (e.g., 0.95 for 95% confidence).
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().ConfidenceLevel(0.99)
func (b *TickerBuilder) ConfidenceLevel(confidenceLevel float64) *TickerBuilder {
    b.confidenceLevel = confidenceLevel
    return b
}

// RiskFreeRate sets the risk-free rate for calculations.
//
// Parameters:
//   - riskFreeRate: The risk-free rate (e.g., 0.02 for 2%).
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().RiskFreeRate(0.03)
func (b *TickerBuilder) RiskFreeRate(riskFreeRate float64) *TickerBuilder {
    b.riskFreeRate = riskFreeRate
    return b
}

// TickerData sets custom ticker data for the Ticker.
//
// Parameters:
//   - tickerData: A DataFrame containing custom ticker data (pass nil if not using custom data).
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().TickerData(nil)
func (b *TickerBuilder) TickerData(tickerData *dataframe.DataFrame) *TickerBuilder {
    b.tickerData = tickerData
    return b
}

// BenchmarkData sets custom benchmark data for the Ticker.
//
// Parameters:
//   - benchmarkData: A DataFrame containing custom benchmark data (pass nil if not using custom data).
//
// Returns:
//   - *TickerBuilder: The builder instance for method chaining.
//
// Example:
//   builder := finalytics.NewTickerBuilder().BenchmarkData(nil)
func (b *TickerBuilder) BenchmarkData(benchmarkData *dataframe.DataFrame) *TickerBuilder {
    b.benchmarkData = benchmarkData
    return b
}

// Build constructs the Ticker instance with the configured parameters.
// The symbol parameter is required; other parameters are optional and use defaults if not set.
//
// Returns:
//   - *Ticker: A pointer to the initialized Ticker object.
//   - error: An error if the Ticker creation fails or the symbol is missing.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//   	fmt.Println("Ticker created successfully for AAPL")
//   }
func (b *TickerBuilder) Build() (*Ticker, error) {
    // Validate required parameter
    if b.symbol == "" {
        return nil, errors.New("symbol is required")
    }

    // Use empty strings for optional parameters if not set
    cSymbol := C.CString(b.symbol)
    defer C.free(unsafe.Pointer(cSymbol))
    cStartDate := C.CString(b.startDate)
    defer C.free(unsafe.Pointer(cStartDate))
    cEndDate := C.CString(b.endDate)
    defer C.free(unsafe.Pointer(cEndDate))
    cInterval := C.CString(b.interval)
    defer C.free(unsafe.Pointer(cInterval))
    cBenchmarkSymbol := C.CString(b.benchmarkSymbol)
    defer C.free(unsafe.Pointer(cBenchmarkSymbol))

    // Handle tickerData
    var cTickerData *C.char
    if b.tickerData != nil {
        jsonStr, err := dataFrameToJSONString(*b.tickerData)
        if err != nil {
            return nil, fmt.Errorf("failed to convert tickerData to JSON: %v", err)
        }
        cTickerData = C.CString(jsonStr)
        defer C.free(unsafe.Pointer(cTickerData))
    } else {
        cTickerData = nil
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
    handle := C.finalytics_ticker_new(
        cSymbol,
        cStartDate,
        cEndDate,
        cInterval,
        cBenchmarkSymbol,
        C.double(b.confidenceLevel),
        C.double(b.riskFreeRate),
        cTickerData,
        cBenchmarkData,
    )
    if handle == nil {
        return nil, errors.New("failed to create Ticker")
    }
    return &Ticker{handle: handle}, nil
}

// Free releases the resources associated with the Ticker.
// It should be called when the Ticker is no longer needed to prevent memory leaks.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	ticker.Free()
//   	fmt.Println("Ticker resources freed successfully")
//   }
func (t *Ticker) Free() {
    if t.handle != nil {
        C.finalytics_ticker_free(t.handle)
        t.handle = nil
    }
}

// GetQuote retrieves the current quote for the ticker.
//
// Returns:
//   - map[string]any: A map containing the current quote data (e.g., symbol, price, volume).
//   - error: An error if the quote retrieval fails.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	quote, err := ticker.GetQuote()
//   	if err != nil {
//   		fmt.Printf("Failed to get quote: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Quote: %v\n", quote)
//   }
func (t *Ticker) GetQuote() (map[string]any, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_get_quote(t.handle, &cOutput)
    if result != 0 {
        return nil, fmt.Errorf("failed to get quote: error code %d", result)
    }
    return parseJSONResult(cOutput)
}

// GetSummaryStats retrieves summary technical and fundamental statistics for the ticker.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing summary statistics.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	summary, err := ticker.GetSummaryStats()
//   	if err != nil {
//   		fmt.Printf("Failed to get summary stats: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Summary Stats:\n%v\n", summary)
//   }
func (t *Ticker) GetSummaryStats() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_get_summary_stats(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get summary stats: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetPriceHistory retrieves the OHLCV (Open, High, Low, Close, Volume) price history for the ticker.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing the price history data.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	history, err := ticker.GetPriceHistory()
//   	if err != nil {
//   		fmt.Printf("Failed to get price history: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Price History:\n%v\n", history)
//   }
func (t *Ticker) GetPriceHistory() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_get_price_history(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get price history: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetOptionsChain retrieves the options chain for the ticker.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing the options chain data.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	options, err := ticker.GetOptionsChain()
//   	if err != nil {
//   		fmt.Printf("Failed to get options chain: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Options Chain:\n%v\n", options)
//   }
func (t *Ticker) GetOptionsChain() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_get_options_chain(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get options chain: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetNews retrieves the latest news headlines for the ticker.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing news data.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	news, err := ticker.GetNews()
//   	if err != nil {
//   		fmt.Printf("Failed to get news: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("News:\n%v\n", news)
//   }
func (t *Ticker) GetNews() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_get_news(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get news: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetIncomeStatement retrieves the income statement for the ticker.
//
// Parameters:
//   - frequency: The frequency of the statement ("annual" or "quarterly").
//   - formatted: Whether to return the statement in a formatted manner.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing the income statement data.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	income, err := ticker.GetIncomeStatement("quarterly", true)
//   	if err != nil {
//   		fmt.Printf("Failed to get income statement: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Income Statement:\n%v\n", income)
//   }
func (t *Ticker) GetIncomeStatement(frequency string, formatted bool) (dataframe.DataFrame, error) {
    cFrequency := C.CString(frequency)
    defer C.free(unsafe.Pointer(cFrequency))
    cFormatted := C.int(0)
    if formatted {
        cFormatted = C.int(1)
    }
    var cOutput *C.char
    result := C.finalytics_ticker_get_income_statement(t.handle, cFrequency, cFormatted, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get income statement: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetBalanceSheet retrieves the balance sheet for the ticker.
//
// Parameters:
//   - frequency: The frequency of the statement ("annual" or "quarterly").
//   - formatted: Whether to return the statement in a formatted manner.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing the balance sheet data.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	balance, err := ticker.GetBalanceSheet("quarterly", true)
//   	if err != nil {
//   		fmt.Printf("Failed to get balance sheet: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Balance Sheet:\n%v\n", balance)
//   }
func (t *Ticker) GetBalanceSheet(frequency string, formatted bool) (dataframe.DataFrame, error) {
    cFrequency := C.CString(frequency)
    defer C.free(unsafe.Pointer(cFrequency))
    cFormatted := C.int(0)
    if formatted {
        cFormatted = C.int(1)
    }
    var cOutput *C.char
    result := C.finalytics_ticker_get_balance_sheet(t.handle, cFrequency, cFormatted, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get balance sheet: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetCashflowStatement retrieves the cash flow statement for the ticker.
//
// Parameters:
//   - frequency: The frequency of the statement ("annual" or "quarterly").
//   - formatted: Whether to return the statement in a formatted manner.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing the cash flow statement data.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	cashflow, err := ticker.GetCashflowStatement("quarterly", true)
//   	if err != nil {
//   		fmt.Printf("Failed to get cash flow statement: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Cash Flow Statement:\n%v\n", cashflow)
//   }
func (t *Ticker) GetCashflowStatement(frequency string, formatted bool) (dataframe.DataFrame, error) {
    cFrequency := C.CString(frequency)
    defer C.free(unsafe.Pointer(cFrequency))
    cFormatted := C.int(0)
    if formatted {
        cFormatted = C.int(1)
    }
    var cOutput *C.char
    result := C.finalytics_ticker_get_cashflow_statement(t.handle, cFrequency, cFormatted, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get cash flow statement: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// GetFinancialRatios retrieves financial ratios for the ticker.
//
// Parameters:
//   - frequency: The frequency of the ratios ("annual" or "quarterly").
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing financial ratios.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	ratios, err := ticker.GetFinancialRatios("quarterly")
//   	if err != nil {
//   		fmt.Printf("Failed to get financial ratios: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Financial Ratios:\n%v\n", ratios)
//   }
func (t *Ticker) GetFinancialRatios(frequency string) (dataframe.DataFrame, error) {
    cFrequency := C.CString(frequency)
    defer C.free(unsafe.Pointer(cFrequency))
    var cOutput *C.char
    result := C.finalytics_ticker_get_financial_ratios(t.handle, cFrequency, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get financial ratios: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// VolatilitySurface retrieves the implied volatility surface for the ticker's options chain.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing the volatility surface data.
//   - error: An error if the volatility surface retrieval fails.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	volSurface, err := ticker.VolatilitySurface()
//   	if err != nil {
//   		fmt.Printf("Failed to get volatility surface: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Volatility Surface:\n%v\n", volSurface)
//   }
func (t *Ticker) VolatilitySurface() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_volatility_surface(t.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get volatility surface: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// PerformanceStats retrieves performance statistics for the ticker.
//
// Returns:
//   - map[string]any: A map containing performance statistics (e.g., returns, volatility, Sharpe ratio).
//   - error: An error if the performance statistics retrieval fails.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
// 			ConfidenceLevel(0.95).
// 			RiskFreeRate(0.02).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	perfStats, err := ticker.PerformanceStats()
//   	if err != nil {
//   		fmt.Printf("Failed to get performance stats: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Performance Stats: %v\n", perfStats)
//   }
func (t *Ticker) PerformanceStats() (map[string]any, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_performance_stats(t.handle, &cOutput)
    if result != 0 {
        return nil, fmt.Errorf("failed to get performance stats: error code %d", result)
    }
    return parseJSONResult(cOutput)
}

// PerformanceChart retrieves the performance chart for the ticker as an HTML object.
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
//   package main
//
//   import (
//   	"fmt"
//   	"github.com/Nnamdi-sys/finalytics/go/finalytics"
//   )
//
//   func main() {
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
// 			ConfidenceLevel(0.95).
// 			RiskFreeRate(0.02).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	perfChart, err := ticker.PerformanceChart(0, 0)
//   	if err != nil {
//   		fmt.Printf("Failed to get performance chart: %v\n", err)
//   		return
//   	}
//   	perfChart.Show()
//   }
func (t *Ticker) PerformanceChart(height, width uint) (HTML, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_performance_chart(t.handle, C.uint(height), C.uint(width), &cOutput)
    if result != 0 {
        return HTML{}, fmt.Errorf("failed to get performance chart: error code %d", result)
    }
    defer C.finalytics_free_string(cOutput)
    htmlStr := C.GoString(cOutput)
    return HTML{Content: htmlStr}, nil
}

// CandlestickChart retrieves the candlestick chart for the ticker as an HTML object.
//
// Parameters:
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the candlestick chart.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	candleChart, err := ticker.CandlestickChart(0, 0)
//   	if err != nil {
//   		fmt.Printf("Failed to get candlestick chart: %v\n", err)
//   		return
//   	}
//   	candleChart.Show()
//   }
func (t *Ticker) CandlestickChart(height, width uint) (HTML, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_candlestick_chart(t.handle, C.uint(height), C.uint(width), &cOutput)
    if result != 0 {
        return HTML{}, fmt.Errorf("failed to get candlestick chart: error code %d", result)
    }
    defer C.finalytics_free_string(cOutput)
    htmlStr := C.GoString(cOutput)
    return HTML{Content: htmlStr}, nil
}

// OptionsChart retrieves the options chart (e.g., volatility surface, smile, or term structure) for the ticker as an HTML object.
//
// Parameters:
//   - chartType: The type of chart to display ("surface", "smile", or "term_structure").
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the options chart.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	optChart, err := ticker.OptionsChart("surface", 0, 0)
//   	if err != nil {
//   		fmt.Printf("Failed to get options chart: %v\n", err)
//   		return
//   	}
//   	optChart.Show()
//   }
func (t *Ticker) OptionsChart(chartType string, height, width uint) (HTML, error) {
    cChartType := C.CString(chartType)
    defer C.free(unsafe.Pointer(cChartType))
    var cOutput *C.char
    result := C.finalytics_ticker_options_chart(t.handle, cChartType, C.uint(height), C.uint(width), &cOutput)
    if result != 0 {
        return HTML{}, fmt.Errorf("failed to get options chart: error code %d", result)
    }
    defer C.finalytics_free_string(cOutput)
    htmlStr := C.GoString(cOutput)
    return HTML{Content: htmlStr}, nil
}

// NewsSentimentChart retrieves the news sentiment chart for the ticker as an HTML object.
//
// Parameters:
//   - height: The height of the chart (0 for default).
//   - width: The width of the chart (0 for default).
//
// Returns:
//   - HTML: An HTML object containing the news sentiment chart.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	newsChart, err := ticker.NewsSentimentChart(0, 0)
//   	if err != nil {
//   		fmt.Printf("Failed to get news sentiment chart: %v\n", err)
//   		return
//   	}
//   	newsChart.Show()
//   }
func (t *Ticker) NewsSentimentChart(height, width uint) (HTML, error) {
    var cOutput *C.char
    result := C.finalytics_ticker_news_sentiment_chart(t.handle, C.uint(height), C.uint(width), &cOutput)
    if result != 0 {
        return HTML{}, fmt.Errorf("failed to get news sentiment chart: error code %d", result)
    }
    defer C.finalytics_free_string(cOutput)
    htmlStr := C.GoString(cOutput)
    return HTML{Content: htmlStr}, nil
}

// Report retrieves a comprehensive analytics report for the ticker as an HTML object.
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
//   	ticker, err := finalytics.NewTickerBuilder().
//   		Symbol("AAPL").
//   		StartDate("2023-01-01").
//   		EndDate("2023-12-31").
//   		Interval("1d").
//   		BenchmarkSymbol("^GSPC").
// 			ConfidenceLevel(0.95).
// 			RiskFreeRate(0.02).
//   		Build()
//   	if err != nil {
//   		fmt.Printf("Failed to create Ticker: %v\n", err)
//   		return
//   	}
//   	defer ticker.Free()
//
//   	report, err := ticker.Report("performance")
//   	if err != nil {
//   		fmt.Printf("Failed to get report: %v\n", err)
//   		return
//   	}
//   	report.Show()
//   }
func (t *Ticker) Report(reportType string) (HTML, error) {
    cReportType := C.CString(reportType)
    defer C.free(unsafe.Pointer(cReportType))
    var cOutput *C.char
    result := C.finalytics_ticker_report(t.handle, cReportType, &cOutput)
    if result != 0 {
        return HTML{}, fmt.Errorf("failed to get report: error code %d", result)
    }
    defer C.finalytics_free_string(cOutput)
    htmlStr := C.GoString(cOutput)
    return HTML{Content: htmlStr}, nil
}