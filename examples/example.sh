#!/bin/bash
# Finalytics — Run all examples
#
# Builds each language environment exactly as used in development,
# then runs the corresponding example.
#
# Usage (from repo root):
#   bash examples/example.sh           # run all languages
#   bash examples/example.sh rust      # run only Rust
#   bash examples/example.sh python    # run only Python
#   bash examples/example.sh go        # run only Go
#   bash examples/example.sh js        # run only JavaScript
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
# Copies examples/example.rs into rust/examples/ (the standard Cargo location
# for runnable examples) and runs it with cargo run --example example.

run_rust() {
  banner "🦀  Rust — cargo run --example example"
  fix_root_owned "$REPO_ROOT/target"
  mkdir -p "$REPO_ROOT/rust/examples"
  fix_root_owned "$REPO_ROOT/rust/examples"
  cp "$REPO_ROOT/examples/example.rs" "$REPO_ROOT/rust/examples/example.rs"
  # Run from the repo root via --manifest-path so that relative paths in the
  # example (e.g. examples/datasets/aapl.csv) resolve against the repo root.
  cd "$REPO_ROOT"
  cargo run --manifest-path rust/Cargo.toml --example example
}

# ── Python ────────────────────────────────────────────────────────────────────
# Creates a venv (if absent), builds the extension with maturin,
# then runs examples/example.py from the repo root.

run_python() {
  banner "🐍  Python — venv + maturin develop + example.py"
  fix_root_owned "$REPO_ROOT/python/.venv"
  cd "$REPO_ROOT/python"

  if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv .venv
  fi

  # shellcheck disable=SC1091
  source .venv/bin/activate
  maturin develop

  cd "$REPO_ROOT"
  python examples/example.py

  deactivate
}

# ── Go ────────────────────────────────────────────────────────────────────────
# Builds the FFI native library via dev_ffi.sh, then runs examples/example.go
# from within the go/ directory so that go.mod module resolution works.

run_go() {
  banner "🐹  Go — dev_ffi.sh + go run ../examples/example.go"
  fix_root_owned "$REPO_ROOT/target"
  fix_root_owned "$REPO_ROOT/go"
  cd "$REPO_ROOT/go"
  bash dev_ffi.sh
  go run -tags dev ../examples/example.go
}

# ── JavaScript ────────────────────────────────────────────────────────────────
# Builds the FFI native library via dev_ffi.sh, then installs dependencies
# in examples/ (which wires the local js/ package via file: reference),
# and runs examples/example.js directly.

run_js() {
  banner "🟨  JavaScript — dev_ffi.sh + npm install + node examples/example.js"
  fix_root_owned "$REPO_ROOT/target"
  fix_root_owned "$REPO_ROOT/js"
  cd "$REPO_ROOT/js"
  bash dev_ffi.sh
  cd "$REPO_ROOT/examples"
  npm install
  cd "$REPO_ROOT"
  node examples/example.js
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
