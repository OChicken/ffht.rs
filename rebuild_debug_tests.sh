#!/bin/bash
# Rebuild debug test binaries created during FFHT wrapper development

set -e
cd "$(dirname "$0")"

echo "Rebuilding debug test binaries..."
echo

# Find the compiled library
LIBFFHT=$(ls target/debug/build/ffht-*/out/libffht.a 2>/dev/null | head -1)

if [ -z "$LIBFFHT" ]; then
    echo "Error: libffht.a not found!"
    echo "Run 'cargo build' first to generate the C library."
    exit 1
fi

echo "Using library: $LIBFFHT"
echo
