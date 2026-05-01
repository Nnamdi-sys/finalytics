#!/bin/bash
# Finalytics — Run all tests
#
# Builds each language environment exactly as used in development,
# then runs the corresponding test suite.
#
# Usage (from repo root):
#   bash examples/test.sh           # run all languages
#   bash examples/test.sh rust      # run only Rust
#   bash examples/test.sh python    # run only Python
#   bash examples/test.sh go        # run only Go
#   bash examples/test.sh js        # run only JavaScript
#
# Prerequisites:
#   Rust   — cargo, rustup
#   Python — python3, maturin  (pip install maturin)
#   Go     — go, cbindgen      (cargo install cbindgen)
#   JS     — node, npm, cbindgen

set -e

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LANG="${1:-all}"

# ── Helpers ───────────────────────────────────────────────────────────────────

banner() {
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  $1"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
}

# Finds any root-owned files inside a directory and fixes their ownership back
# to the current user. Needed when the script was previously run with sudo,
# which leaves files that the normal user cannot overwrite.
fix_root_owned() {
  local dir="$1"
  local found
  found=$(find "$dir" -maxdepth 5 -user root 2>/dev/null || true)
  if [ -n "$found" ]; then
    echo "  ⚠  Root-owned files detected in $dir (previous sudo run)."
    echo "     Fixing ownership back to $(id -un)..."
    sudo chown -R "$(id -un)" "$dir"
  fi
}

# ── Rust ──────────────────────────────────────────────────────────────────────

run_rust() {
  banner "🦀  Rust — cargo test"
  fix_root_owned "$REPO_ROOT/target"
  cd "$REPO_ROOT"
  cargo test --manifest-path rust/Cargo.toml
}

# ── Python ────────────────────────────────────────────────────────────────────

run_python() {
  banner "🐍  Python — venv + maturin develop + test.py"
  fix_root_owned "$REPO_ROOT/python/.venv"
  cd "$REPO_ROOT/python"

  if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv .venv
  fi

  # shellcheck disable=SC1091
  source .venv/bin/activate
  maturin develop

  python test.py

  deactivate
}

# ── Go ────────────────────────────────────────────────────────────────────────

run_go() {
  banner "🐹  Go — dev_ffi.sh + go run main.go"
  fix_root_owned "$REPO_ROOT/target"
  fix_root_owned "$REPO_ROOT/go"
  cd "$REPO_ROOT/go"
  bash dev_ffi.sh
  go run -tags dev main.go
}

# ── JavaScript ────────────────────────────────────────────────────────────────
# Builds the FFI native library via dev_ffi.sh, installs dependencies in js/,
# then runs js/index.js directly (all imports are relative, so no
# examples/package.json wiring is needed here).

run_js() {
  banner "🟨  JavaScript — dev_ffi.sh + npm install + node index.js"
  fix_root_owned "$REPO_ROOT/target"
  fix_root_owned "$REPO_ROOT/js"
  cd "$REPO_ROOT/js"
  bash dev_ffi.sh
  npm install
  node index.js
}

# ── Dispatch ──────────────────────────────────────────────────────────────────

case "$LANG" in
  rust)   run_rust   ;;
  python) run_python ;;
  go)     run_go     ;;
  js)     run_js     ;;
  all)
    run_rust
    run_python
    run_go
    run_js
    ;;
  *)
    echo "Unknown target: '$LANG'"
    echo "Usage: $0 [rust|python|go|js|all]"
    exit 1
    ;;
esac

echo ""
echo "✅  Done!"
