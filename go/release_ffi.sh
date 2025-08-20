#!/bin/bash

set -e

export DOCKER_DEFAULT_PLATFORM=linux/amd64

cd ../ffi

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

mkdir -p ../go/finalytics/lib/macos
mkdir -p ../go/finalytics/lib/linux
mkdir -p ../go/finalytics/lib/windows

cp include/finalytics.h ../go/finalytics/
cp ../target/x86_64-apple-darwin/release/libfinalytics_ffi.dylib ../go/finalytics/lib/macos/libfinalytics_ffi_x86_64.dylib
cp ../target/aarch64-apple-darwin/release/libfinalytics_ffi.dylib ../go/finalytics/lib/macos/libfinalytics_ffi_aarch64.dylib
cp ../target/x86_64-unknown-linux-gnu/release/libfinalytics_ffi.so ../go/finalytics/lib/linux/libfinalytics_ffi.so
cp ../target/x86_64-pc-windows-gnu/release/finalytics_ffi.dll ../go/finalytics/lib/windows/finalytics_ffi.dll

echo "FFI artifacts (dynamic libraries and header) generated and copied successfully!"

# --- Upload to GitHub Release ---

# Get the latest tag or commit hash for the release
RELEASE_TAG=$(git describe --tags --abbrev=0 2>/dev/null || git rev-parse --short HEAD)
RELEASE_NAME="FFI Artifacts $RELEASE_TAG"

# Create the release if it doesn't exist
gh release view "$RELEASE_TAG" || gh release create "$RELEASE_TAG" --title "$RELEASE_NAME" --notes "Automated release of FFI binaries"

# Upload the binaries and header to the release
gh release upload "$RELEASE_TAG" \
    ../go/finalytics/lib/macos/libfinalytics_ffi_x86_64.dylib \
    ../go/finalytics/lib/macos/libfinalytics_ffi_aarch64.dylib \
    ../go/finalytics/lib/linux/libfinalytics_ffi.so \
    ../go/finalytics/lib/windows/finalytics_ffi.dll \
    ../go/finalytics/finalytics.h \
    --clobber

echo "FFI artifacts uploaded to GitHub release $RELEASE_TAG!"
