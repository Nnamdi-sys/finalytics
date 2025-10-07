#!/bin/bash

set -e

export DOCKER_DEFAULT_PLATFORM=linux/amd64

# --- Version Handling ---
if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 v0.1.0"
  exit 1
fi

VERSION="$1"
RELEASE_TAG="$VERSION"
RELEASE_NAME="Finalytics C-FFI"

echo "🚀 Preparing release: $RELEASE_TAG"

# --- Build Artifacts ---

cbindgen --config cbindgen.toml --crate finalytics-ffi --output include/finalytics.h

# macOS builds (requires Mac host)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Ensure targets are added (optional, safe to re-run)
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

# Linux and Windows builds with nightly toolchain
cross +nightly build --release --target x86_64-unknown-linux-gnu --jobs 4
cross +nightly build --release --target x86_64-pc-windows-gnu --jobs 4

# Copy artifacts into Go lib structure
mkdir -p lib/macos
mkdir -p lib/linux
mkdir -p lib/windows

cp ../target/x86_64-apple-darwin/release/libfinalytics_ffi.dylib lib/macos/libfinalytics_ffi_x86_64.dylib
cp ../target/aarch64-apple-darwin/release/libfinalytics_ffi.dylib lib/macos/libfinalytics_ffi_aarch64.dylib
cp ../target/x86_64-unknown-linux-gnu/release/libfinalytics_ffi.so lib/linux/libfinalytics_ffi.so
cp ../target/x86_64-pc-windows-gnu/release/finalytics_ffi.dll lib/windows/finalytics_ffi.dll

# --- Fix install names for macOS dylibs ---
install_name_tool -id @rpath/libfinalytics_ffi_x86_64.dylib lib/macos/libfinalytics_ffi_x86_64.dylib
install_name_tool -id @rpath/libfinalytics_ffi_aarch64.dylib lib/macos/libfinalytics_ffi_aarch64.dylib

echo "✅ FFI artifacts generated and copied successfully!"

# --- GitHub Release ---
if gh release view "$RELEASE_TAG" >/dev/null 2>&1; then
  echo "ℹ️ Release $RELEASE_TAG already exists, uploading assets..."
else
  echo "📦 Creating release $RELEASE_TAG..."
  gh release create "$RELEASE_TAG" \
    --title "$RELEASE_NAME" \
    --notes "Automated release of Finalytics C-FFI binaries ($VERSION)"
fi

gh release upload "$RELEASE_TAG" \
    ./lib/macos/libfinalytics_ffi_x86_64.dylib \
    ./lib/macos/libfinalytics_ffi_aarch64.dylib \
    ./lib/linux/libfinalytics_ffi.so \
    ./lib/windows/finalytics_ffi.dll \
    ./include/finalytics.h \
    --clobber

echo "🎉 Release $RELEASE_TAG (Finalytics C-FFI) completed successfully!"
