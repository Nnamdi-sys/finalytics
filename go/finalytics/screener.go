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

// Screener represents a stock screener for filtering financial instruments based on specified criteria.
// It encapsulates a handle to the underlying C library for interacting with screener data.
type Screener struct {
    handle C.ScreenerHandle
}

// NewScreener creates a new Screener instance with the given parameters.
//
// Parameters:
//   - quoteType: The type of financial instrument to screen (e.g., "EQUITY", "MUTUALFUND", "ETF", "INDEX", "FUTURE", "CRYPTO").
//   - filters: A string slice, where each element is a JSON object representing a filter criterion.
//     Each filter should have the format:
//     {
//         "operator": "<op>",
//         "operands": ["<metric>", <value>[, <value2>]]
//     }
//     Supported operators are "eq" (equal), "gte" (greater than or equal), "lte" (less than or equal), "gt" (greater than), "lt" (less than), or "btwn" (between, requiring two values). The `<metric>` must be a valid screener metric for the quote type (e.g., "intradaymarketcap" for EQUITY). A full list of metrics is available at: https://github.com/Nnamdi-sys/finalytics/tree/main/rust/src/data/yahoo/screeners/screeners.json.
//   - sortField: The metric to sort by (e.g., "intradaymarketcap" for EQUITY). If empty, no sorting is applied.
//   - sortDescending: Whether to sort in descending order (true) or ascending order (false).
//   - offset: The starting index of results to return (e.g., 0 to start from the beginning).
//   - size: The maximum number of results to return (e.g., 10 for the top 10 results).
//
// Returns:
//   - *Screener: A pointer to the initialized Screener object.
//   - error: An error if the Screener creation fails.
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
//   	// Sample filters: select equities on the NMS exchange with market cap >= 10B
//   	filters := []string{
//		    `{"operator":"eq","operands":["exchange","NMS"]}`,
//		    `{"operator":"gte","operands":["intradaymarketcap",10000000000]}`
//		}
//
//   	screener, err := finalytics.NewScreener("EQUITY", filters, "intradaymarketcap", true, 0, 10)
//   	if err != nil {
//   		fmt.Printf("Failed to create Screener: %v\n", err)
//   		return
//   	}
//   	defer screener.Free()
//   	fmt.Println("Screener created successfully for EQUITY on NMS with market cap >= 10B")
//   }
func NewScreener(quoteType string, filters []string, sortField string, sortDescending bool, offset, size uint) (*Screener, error) {
    cQuoteType := C.CString(quoteType)
    defer C.free(unsafe.Pointer(cQuoteType))
    filters_string, err := StringSliceToJSON(filters)
    if err != nil {
		return nil, fmt.Errorf("failed to convert filters to JSON: %v", err)
	}
    cFilters := C.CString(filters_string)
    defer C.free(unsafe.Pointer(cFilters))
    cSortField := C.CString(sortField)
    defer C.free(unsafe.Pointer(cSortField))
    cSortDescending := C.int(0)
    if sortDescending {
        cSortDescending = C.int(1)
    }

    handle := C.finalytics_screener_new(cQuoteType, cFilters, cSortField, cSortDescending, C.uint(offset), C.uint(size))
    if handle == nil {
        return nil, errors.New("failed to create Screener")
    }
    return &Screener{handle: handle}, nil
}

// Free releases the resources associated with the Screener.
// It should be called when the Screener is no longer needed to prevent memory leaks.
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
//   	screener, err := finalytics.NewScreener("EQUITY", `[{"operator": "eq", "operands": ["exchange", "NMS"]}]`, "intradaymarketcap", true, 0, 10)
//   	if err != nil {
//   		fmt.Printf("Failed to create Screener: %v\n", err)
//   		return
//   	}
//   	screener.Free()
//   	fmt.Println("Screener resources freed successfully")
//   }
func (s *Screener) Free() {
    if s.handle != nil {
        C.finalytics_screener_free(s.handle)
        s.handle = nil
    }
}

// Symbols retrieves the list of ticker symbols matching the screener criteria.
//
// Returns:
//   - []string: A slice of ticker symbols (e.g., ["AAPL", "MSFT", "GOOGL"]).
//   - error: An error if the symbols retrieval fails.
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
//   	screener, err := finalytics.NewScreener("EQUITY", `[{"operator": "eq", "operands": ["exchange", "NMS"]}]`, "intradaymarketcap", true, 0, 10)
//   	if err != nil {
//   		fmt.Printf("Failed to create Screener: %v\n", err)
//   		return
//   	}
//   	defer screener.Free()
//
//   	symbols, err := screener.Symbols()
//   	if err != nil {
//   		fmt.Printf("Failed to get symbols: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Symbols: %v\n", symbols)
//   }
func (s *Screener) Symbols() ([]string, error) {
    var cOutput *C.char
    result := C.finalytics_screener_symbols(s.handle, &cOutput)
    if result != 0 {
        return nil, fmt.Errorf("failed to get symbols: error code %d", result)
    }
    return parseJSONResultArray(cOutput)
}

// Overview retrieves overview data for the screened instruments.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing overview data for the screened instruments.
//   - error: An error if the overview retrieval fails.
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
//   	screener, err := finalytics.NewScreener("EQUITY", `[{"operator": "eq", "operands": ["exchange", "NMS"]}]`, "intradaymarketcap", true, 0, 10)
//   	if err != nil {
//   		fmt.Printf("Failed to create Screener: %v\n", err)
//   		return
//   	}
//   	defer screener.Free()
//
//   	overview, err := screener.Overview()
//   	if err != nil {
//   		fmt.Printf("Failed to get overview: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Overview:\n%v\n", overview)
//   }
func (s *Screener) Overview() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_screener_overview(s.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get overview: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}

// Metrics retrieves detailed metrics for the screened instruments.
//
// Returns:
//   - dataframe.DataFrame: A DataFrame containing detailed metrics for the screened instruments.
//   - error: An error if the metrics retrieval fails.
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
//   	screener, err := finalytics.NewScreener("EQUITY", `[{"operator": "eq", "operands": ["exchange", "NMS"]}]`, "intradaymarketcap", true, 0, 10)
//   	if err != nil {
//   		fmt.Printf("Failed to create Screener: %v\n", err)
//   		return
//   	}
//   	defer screener.Free()
//
//   	metrics, err := screener.Metrics()
//   	if err != nil {
//   		fmt.Printf("Failed to get metrics: %v\n", err)
//   		return
//   	}
//   	fmt.Printf("Metrics:\n%v\n", metrics)
//   }
func (s *Screener) Metrics() (dataframe.DataFrame, error) {
    var cOutput *C.char
    result := C.finalytics_screener_metrics(s.handle, &cOutput)
    if result != 0 {
        return dataframe.DataFrame{}, fmt.Errorf("failed to get metrics: error code %d", result)
    }
    return parseJSONToDataFrame(cOutput)
}