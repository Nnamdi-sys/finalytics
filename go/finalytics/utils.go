package finalytics

// v0.1.2 release

/*
// #cgo CFLAGS: -I${SRCDIR}
// #cgo LDFLAGS: -L${SRCDIR} -lfinalytics_ffi
#cgo darwin,amd64 CFLAGS: -I${SRCDIR}
#cgo darwin,amd64 LDFLAGS: -L${SRCDIR}/lib/macos -lfinalytics_ffi_x86_64
#cgo darwin,arm64 CFLAGS: -I${SRCDIR}
#cgo darwin,arm64 LDFLAGS: -L${SRCDIR}/lib/macos -lfinalytics_ffi_aarch64
#cgo linux,amd64 CFLAGS: -I${SRCDIR}
#cgo linux,amd64 LDFLAGS: -L${SRCDIR}/lib/linux -lfinalytics_ffi
#cgo windows,amd64 CFLAGS: -I${SRCDIR}
#cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib/windows -lfinalytics_ffi
#include <finalytics.h>
#include <stdlib.h>
*/
import "C"
import (
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"os"
	"os/exec"
	"regexp"
	"runtime"
	"strconv"
	"strings"

	"github.com/go-gota/gota/dataframe"
	"github.com/go-gota/gota/series"
)

// StringSliceToJSON converts a []string to a JSON string array.
func StringSliceToJSON(slice []string) (string, error) {
    jsonBytes, err := json.Marshal(slice)
    if err != nil {
        return "", err
    }
    return string(jsonBytes), nil
}

// parseJSONResult parses a C string containing JSON data and frees the string.
func parseJSONResult(cResult *C.char) (map[string]interface{}, error) {
	defer C.finalytics_free_string(cResult)
	if cResult == nil {
		return nil, errors.New("failed to get result")
	}
	result := C.GoString(cResult)
	var data map[string]interface{}
	if err := json.Unmarshal([]byte(result), &data); err != nil {
		return nil, fmt.Errorf("failed to parse JSON: %v", err)
	}
	return data, nil
}

func parseJSONResultArray(cOutput *C.char) ([]string, error) {
	var result []string
	// Convert C string to Go string
	goStr := C.GoString(cOutput)
	// Unmarshal into a slice of strings
	err := json.Unmarshal([]byte(goStr), &result)
	if err != nil {
		return nil, fmt.Errorf("failed to parse JSON: %w", err)
	}
	return result, nil
}

// parseJSONToDataFrame parses a C string containing JSON data and loads it into a dataframe,
// ensuring the column order matches the order in the JSON string.
func parseJSONToDataFrame(cResult *C.char) (dataframe.DataFrame, error) {
    defer C.finalytics_free_string(cResult)
    if cResult == nil {
        return dataframe.DataFrame{}, errors.New("failed to get result")
    }
    result := C.GoString(cResult)

    // Step 1: Extract the first object from the JSON array
    start := strings.Index(result, "{")
    end := strings.Index(result, "}") + 1
    if start == -1 || end == -1 || end <= start {
        return dataframe.DataFrame{}, errors.New("could not find first object in JSON array")
    }
    firstObjRaw := result[start:end]

    // Step 2: Use regex to extract keys in order
    re := regexp.MustCompile(`"([^"]+)"\s*:`)
    matches := re.FindAllStringSubmatch(firstObjRaw, -1)
    keyOrder := []string{}
    for _, match := range matches {
        keyOrder = append(keyOrder, match[1])
    }

    // Step 3: Load the dataframe
    df := dataframe.ReadJSON(strings.NewReader(result))
    if df.Err != nil {
        return dataframe.DataFrame{}, fmt.Errorf("failed to parse JSON into dataframe: %v", df.Err)
    }

    // Step 4: Reorder columns
    if len(keyOrder) > 0 {
        df = df.Select(keyOrder)
    }

    return df, nil
}

// dataFrameToJSONString converts a DataFrame to a JSON string in columnar format,
// ensuring all float columns are marshaled as strings with decimal points for whole numbers,
// and full precision for non-whole numbers.
func dataFrameToJSONString(df dataframe.DataFrame) (string, error) {
    columnar := make(map[string]interface{})
    nRows := df.Nrow()

    for _, colName := range df.Names() {
        col := df.Col(colName)
        values := make([]interface{}, nRows)

        switch col.Type() {
        case series.Float:
            for i := 0; i < nRows; i++ {
                v := col.Elem(i).Float()
                values[i] = v
                if math.Mod(v, 1) == 0 {
                    // Whole number: format with one decimal
                    values[i] = strconv.FormatFloat(v, 'f', 1, 64)
                } else {
                    // Non-whole: format with full precision
                    values[i] = strconv.FormatFloat(v, 'f', -1, 64)
                }
            }
        default:
            for i := 0; i < nRows; i++ {
                values[i] = col.Elem(i).Val()
            }
        }
        columnar[colName] = values
    }

    jsonData, err := json.Marshal(columnar)
    if err != nil {
        return "", fmt.Errorf("failed to marshal DataFrame to JSON: %v", err)
    }
    return string(jsonData), nil
}

// dataFramesToJSONString converts a slice of DataFrames to a JSON string array of JSON strings.
func dataFramesToJSONString(dfs []dataframe.DataFrame) (string, error) {
    jsonStrings := make([]string, len(dfs))
    for i, df := range dfs {
        jsonStr, err := dataFrameToJSONString(df)
        if err != nil {
            return "", fmt.Errorf("failed to convert DataFrame[%d] to JSON: %v", i, err)
        }
        jsonStrings[i] = jsonStr
    }
    jsonData, err := json.Marshal(jsonStrings)
    if err != nil {
        return "", fmt.Errorf("failed to marshal DataFrames to JSON string array: %v", err)
    }
    return string(jsonData), nil
}

// HTML stores the HTML string for a chart, table or report.
type HTML struct {
	Content string
}

// Show writes the HTML to a temporary file and opens it in the default browser.
func (c *HTML) Show() error {
	tmpFile, err := os.CreateTemp("", "chart-*.html")
	if err != nil {
		return fmt.Errorf("failed to create temp file: %w", err)
	}
	defer tmpFile.Close()

	_, err = tmpFile.WriteString(c.Content)
	if err != nil {
		return fmt.Errorf("failed to write HTML to temp file: %w", err)
	}

	// Open the file in the default browser
	var cmd *exec.Cmd
	switch runtime.GOOS {
	case "darwin":
		cmd = exec.Command("open", tmpFile.Name())
	case "windows":
		cmd = exec.Command("rundll32", "url.dll,FileProtocolHandler", tmpFile.Name())
	default: // linux, freebsd, etc.
		cmd = exec.Command("xdg-open", tmpFile.Name())
	}
	return cmd.Start()
}
