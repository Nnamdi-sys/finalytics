#!/bin/bash
# post_commit.sh — Run after committing to GitHub.
#
# Does three things in order:
#   1. Builds and publishes the C-FFI GitHub release  (ffi/release_ffi.sh)
#   2. Creates and pushes the Python package git tag  (python/vX.Y.Z)
#   3. Creates and pushes the Go module git tag       (go/vX.Y.Z)
#
# Usage (from repo root):
#   bash post_commit.sh

set -e

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"

# ── Helpers ───────────────────────────────────────────────────────────────────

ask() {
  local prompt="$1"
  local var_name="$2"
  while true; do
    printf "%s: " "$prompt"
    read -r value
    if [ -z "$value" ]; then
      echo "  ⚠  Value cannot be empty. Please try again."
    elif ! echo "$value" | grep -qE '^v[0-9]+\.[0-9]+\.[0-9]+$'; then
      echo "  ⚠  Invalid format. Expected vX.Y.Z with a lowercase 'v' (e.g. v0.2.0)."
    else
      eval "$var_name=\"$value\""
      break
    fi
  done
}

banner() {
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  $1"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
}

# ── Collect versions ──────────────────────────────────────────────────────────

echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║          Finalytics Post-Commit Release          ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""
echo "Please provide the version numbers for each release."
echo "FFI and Go/Python versions may differ."
echo ""

ask "FFI release version    (e.g. v0.2.0)" FFI_VERSION
ask "Python package version (e.g. v0.9.0)" PYTHON_VERSION
ask "Go module version      (e.g. v0.2.0)" GO_VERSION

PYTHON_TAG="python/${PYTHON_VERSION}"
GO_TAG="go/${GO_VERSION}"

echo ""
echo "About to perform the following:"
echo "  • Run ffi/release_ffi.sh with FFI version : $FFI_VERSION"
echo "  • Create and push git tag                 : $PYTHON_TAG"
echo "  • Create and push git tag                 : $GO_TAG"
echo ""
printf "Proceed? [y/N] "
read -r confirm
if [ "$confirm" != "y" ] && [ "$confirm" != "Y" ]; then
  echo "Aborted."
  exit 0
fi

# ── 1. FFI Release ────────────────────────────────────────────────────────────

banner "🔧  FFI Release — ffi/release_ffi.sh $FFI_VERSION"

cd "$REPO_ROOT/ffi"
bash release_ffi.sh "$FFI_VERSION"

# ── 2. Python Git Tag ─────────────────────────────────────────────────────────

banner "🐍  Python Git Tag — $PYTHON_TAG"

cd "$REPO_ROOT"
git tag "$PYTHON_TAG"
git push origin "$PYTHON_TAG"
echo "✅  Pushed tag: $PYTHON_TAG"

# ── 3. Go Git Tag ─────────────────────────────────────────────────────────────

banner "🐹  Go Git Tag — $GO_TAG"

git tag "$GO_TAG"
git push origin "$GO_TAG"
echo "✅  Pushed tag: $GO_TAG"

# ── Done ──────────────────────────────────────────────────────────────────────

echo ""
echo "🎉  All done!"
echo "    FFI release : $FFI_VERSION"
echo "    Python tag  : $PYTHON_TAG"
echo "    Go tag      : $GO_TAG"
echo ""
