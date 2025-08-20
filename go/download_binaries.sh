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

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

case "$OS" in
    linux)
        LIBDIR="$SCRIPT_DIR/lib/linux"
        LIBNAME="libfinalytics_ffi.so"
        ;;
    darwin)
        LIBDIR="$SCRIPT_DIR/lib/macos"
        if [ "$ARCH" = "arm64" ]; then
            LIBNAME="libfinalytics_ffi_aarch64.dylib"
        else
            LIBNAME="libfinalytics_ffi_x86_64.dylib"
        fi
        ;;
    msys*|mingw*|cygwin*|windows*)
        LIBDIR="$SCRIPT_DIR/lib/windows"
        LIBNAME="finalytics_ffi.dll"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

URL="https://github.com/$REPO/releases/download/$TAG/$LIBNAME"

mkdir -p "$LIBDIR"

if [ -f "$LIBDIR/$LIBNAME" ]; then
    echo "Native library already present: $LIBDIR/$LIBNAME"
    exit 0
fi

echo "Downloading native library for $OS/$ARCH from $URL ..."
curl -L -o "$LIBDIR/$LIBNAME" "$URL"

if [ -f "$LIBDIR/$LIBNAME" ]; then
    echo "Downloaded library to $LIBDIR/$LIBNAME"
else
    echo "Failed to download library."
    exit 1
fi
