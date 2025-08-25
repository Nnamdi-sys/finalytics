#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status

# Step 1: Go to ffi directory
cd ../ffi

# Step 2: Build the Rust library
cargo build --release

# Step 3: Generate the C header
cbindgen --config cbindgen.toml --crate finalytics-ffi --output include/finalytics.h

# Step 4: Copy artifacts to go/ directory
cp include/finalytics.h ../js
cp ../target/release/libfinalytics_ffi.dylib ../js

echo "FFI artifacts generated and copied successfully!"
