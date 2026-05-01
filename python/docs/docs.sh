#!/bin/bash
# docs.sh — Preview or publish the Finalytics Quarto documentation.
#
# Usage (from anywhere):
#   bash docs.sh            # prompts you to choose
#   bash docs.sh preview    # serve locally at http://localhost:4444
#   bash docs.sh publish    # publish to https://nnamdi.quarto.pub/finalytics

set -e

DOCS_DIR="$(cd "$(dirname "$0")" && pwd)"

# ── Helpers ───────────────────────────────────────────────────────────────────

banner() {
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  $1"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
}

check_quarto() {
  if ! command -v quarto >/dev/null 2>&1; then
    echo "❌  Quarto is not installed or not on PATH."
    echo "    Install it from: https://quarto.org/docs/get-started/"
    exit 1
  fi
}

check_maturin() {
  if ! command -v maturin >/dev/null 2>&1; then
    echo "❌  maturin is not installed or not on PATH."
    echo "    Install it with: pip install maturin"
    exit 1
  fi
}

setup_python() {
  banner "🐍  Python — venv + maturin develop"
  PYTHON_DIR="$(dirname "$DOCS_DIR")"
  cd "$PYTHON_DIR"

  if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv .venv
  fi

  # shellcheck disable=SC1091
  source .venv/bin/activate
  maturin develop

  # Quarto requires Jupyter and pyyaml to execute .qmd notebook cells
  pip install --quiet jupyter pyyaml
}

# ── Commands ─────────────────────────────────────────────────────────────────

do_preview() {
  banner "🔍  Preview — http://localhost:4444"
  cd "$DOCS_DIR"
  quarto preview --port 4444
}

do_publish() {
  banner "🚀  Publish — https://nnamdi.quarto.pub/finalytics"
  cd "$DOCS_DIR"
  quarto publish quarto-pub --no-prompt
  echo ""
  echo "✅  Published to https://nnamdi.quarto.pub/finalytics"
}

# ── Dispatch ─────────────────────────────────────────────────────────────────

check_quarto
check_maturin
setup_python

ACTION="${1:-}"

if [ -z "$ACTION" ]; then
  echo ""
  echo "What would you like to do?"
  echo "  1) Preview locally  (http://localhost:4444)"
  echo "  2) Publish          (https://nnamdi.quarto.pub/finalytics)"
  echo ""
  printf "Enter 1 or 2: "
  read -r choice
  case "$choice" in
    1) ACTION="preview" ;;
    2) ACTION="publish" ;;
    *)
      echo "Invalid choice. Please enter 1 or 2."
      exit 1
      ;;
  esac
fi

case "$ACTION" in
  preview) do_preview ;;
  publish) do_publish ;;
  *)
    echo "Unknown action: '$ACTION'"
    echo "Usage: $0 [preview|publish]"
    exit 1
    ;;
esac
