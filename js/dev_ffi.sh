#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status

# Step 1: Go to ffi directory
cd ../ffi

# Step 2: Build the Rust library
cargo build --release

# Step 3: Generate the C header
cbindgen --config cbindgen.toml --crate finalytics-ffi --output include/finalytics.h

# Step 4: Copy artifacts to js/ directory
# Use the arch-specific dylib name that getNativeLibPath() in utils.js looks for
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
  DYLIB_DEST="libfinalytics_ffi_aarch64.dylib"
else
  DYLIB_DEST="libfinalytics_ffi_x86_64.dylib"
fi
cp include/finalytics.h ../js
cp ../target/release/libfinalytics_ffi.dylib "../js/$DYLIB_DEST"

echo "FFI artifacts generated and copied successfully!"
