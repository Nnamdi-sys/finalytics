#!/bin/bash
set -e

REPO="Nnamdi-sys/finalytics"
TAG="${1:-latest}"

# Resolve latest tag if needed (use awk for compatibility)
if [ "$TAG" = "latest" ]; then
    TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | awk -F'"' '/tag_name/ {print $4; exit}')
    if [ -z "$TAG" ]; then
        echo "Could not resolve latest release tag."
        exit 1
    fi
fi

OS=$(uname | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    linux)
        LIBNAME="libfinalytics_ffi.so"
        MOD_OS="linux"
        ;;
    darwin)
        MOD_OS="macos"
        if [ "$ARCH" = "arm64" ]; then
            LIBNAME="libfinalytics_ffi_aarch64.dylib"
        else
            LIBNAME="libfinalytics_ffi_x86_64.dylib"
        fi
        ;;
    msys*|mingw*|cygwin*|windows*)
        MOD_OS="windows"
        LIBNAME="finalytics_ffi.dll"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

URL="https://github.com/$REPO/releases/download/$TAG/$LIBNAME"

# Get the actual Go module cache directory for finalytics/go (with encoding)
MODDIR=$(go list -m -f '{{.Dir}}' github.com/Nnamdi-sys/finalytics/go)
MODPATH="$MODDIR/finalytics/lib/$MOD_OS"

# Make the module directory writable
chmod -R u+w "$MODDIR"

mkdir -p "$MODPATH"

if [ -f "$MODPATH/$LIBNAME" ]; then
    echo "Native library already present: $MODPATH/$LIBNAME"
    exit 0
fi

echo "Downloading native library for $OS/$ARCH from $URL ..."
curl -L -o "$MODPATH/$LIBNAME" "$URL"

if [ -f "$MODPATH/$LIBNAME" ]; then
    echo "Downloaded library to $MODPATH/$LIBNAME"
    # Optionally, set the directory back to read-only
    chmod -R a-w "$MODDIR"
else
    echo "Failed to download library."
    exit 1
fi
