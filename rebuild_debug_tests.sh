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

FLAGS="-march=native"

# Test 1: Check compiler defines
echo "[1/3] Compiling test_defines..."
gcc test_defines.c -o test_defines $FLAGS
./test_defines
echo "✓ test_defines"
echo

# Test 2: Test in-place FHT
echo "[2/3] Compiling test_inplace..."
gcc test_inplace.c "$LIBFFHT" -o test_inplace $FLAGS
./test_inplace
echo "✓ test_inplace"
echo

# Test 3: Test out-of-place FHT
echo "[3/3] Compiling test_oop..."
gcc test_oop.c "$LIBFFHT" -o test_oop $FLAGS
./test_oop
echo "✓ test_oop"

# Note: test_fast_copy is no longer relevant (we use memcpy now)

echo
echo "================== Done! =================="
echo
echo "Run tests:"
echo "  ./test_defines  - Check SIMD macros (__AVX__, __SSE2__, etc.)"
echo "  ./test_inplace  - Test in-place FHT"
echo "  ./test_oop      - Test out-of-place FHT"
echo
echo "Note: test_fast_copy removed (using memcpy directly now)"
echo
rm test_defines test_inplace test_oop
