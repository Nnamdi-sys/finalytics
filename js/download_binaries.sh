#!/bin/bash
set -e

REPO="Nnamdi-sys/finalytics"
TAG="${1:-latest}"

# Find the installed package directory (relative to current working directory)
PKGDIR="$(npm root)/finalytics"

# Resolve latest tag if needed
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

MODPATH="$PKGDIR/lib/$MOD_OS"
mkdir -p "$MODPATH"

if [ -f "$MODPATH/$LIBNAME" ]; then
    echo "Native library already present: $MODPATH/$LIBNAME"
else
    echo "Downloading native library for $OS/$ARCH from $URL ..."
    curl -L -o "$MODPATH/$LIBNAME" "$URL"
    if [ -f "$MODPATH/$LIBNAME" ]; then
        echo "Downloaded library to $MODPATH/$LIBNAME"
    else
        echo "Failed to download library."
        exit 1
    fi
fi

# Download the header file as well
HEADER_URL="https://github.com/$REPO/releases/download/$TAG/finalytics.h"
HEADER_PATH="$PKGDIR/lib/finalytics.h"
if [ ! -f "$HEADER_PATH" ]; then
    echo "Downloading header file from $HEADER_URL ..."
    curl -L -o "$HEADER_PATH" "$HEADER_URL"
    if [ -f "$HEADER_PATH" ]; then
        echo "Downloaded header to $HEADER_PATH"
    else
        echo "Failed to download header file."
        exit 1
    fi
fi
