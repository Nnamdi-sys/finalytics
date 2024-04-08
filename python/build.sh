#!/bin/bash

# Install Python Build Dependencies
pip install maturin twine

# Add targets for Linux
rustup target add x86_64-unknown-linux-gnu
rustup target add i686-unknown-linux-gnu

# Add targets for Windows
rustup target add x86_64-pc-windows-msvc
#rustup target add i686-pc-windows-msvc

# Add targets for macOS
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Add targets for Linux
linux_targets=("x86_64-unknown-linux-gnu" "i686-unknown-linux-gnu")

# Add targets for Windows
windows_targets=("x86_64-pc-windows-msvc") # "i686-pc-windows-msvc"

# Add targets for macOS
macos_targets=("x86_64-apple-darwin" "aarch64-apple-darwin")

# Python versions to build
python_versions=("3.7" "3.8" "3.9" "3.10" "3.11" "3.12")

for target in "${linux_targets[@]}"; do
    for version in "${python_versions[@]}"; do
        OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
        OPENSSL_INCLUDE_DIR="/usr/include/openssl"
        maturin build --release --strip --out dist --target "$target" -i "python$version"
    done
done

for target in "${windows_targets[@]}"; do
    for version in "${python_versions[@]}"; do
        maturin build --release --strip --out dist --target "$target" -i "python$version"
    done
done

for target in "${macos_targets[@]}"; do
    for version in "${python_versions[@]}"; do
        maturin build --release --strip --out dist --target "$target" -i "python$version"
    done
done

# Build the source distribution (sdist)
maturin build --sdist --release --strip --out dist

# Publish to PyPI using Twine
#twine upload dist/*
