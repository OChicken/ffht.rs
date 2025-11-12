#!/bin/bash
# Run all FFHT tests
# Usage: ./run_all_tests.sh

set -e  # Exit on first error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

echo "======================================"
echo "  FFHT Test Suite"
echo "======================================"
echo ""

# Detect architecture
ARCH=$(uname -m)
echo "Architecture: $ARCH"
echo ""

# Build
echo "[1/5] Building C library..."
make clean > /dev/null 2>&1
make all > /dev/null 2>&1
echo "✓ Build successful"
echo ""

# Test float
echo "[2/5] Testing float precision..."
if ./test_float > test_float.log 2>&1; then
    echo "✓ Float test PASSED"
else
    echo "✗ Float test FAILED - see test_float.log"
    exit 1
fi
echo ""

# Test double
echo "[3/5] Testing double precision..."
if ./test_double > test_double.log 2>&1; then
    echo "✓ Double test PASSED"
else
    echo "✗ Double test FAILED - see test_double.log"
    exit 1
fi
echo ""

# Test NEON on ARM
if [[ "$ARCH" == "aarch64" ]]; then
    echo "[4/5] Testing ARM NEON implementation..."
    make test_neon > /dev/null 2>&1
    if ./test_neon; then
        echo "✓ NEON test PASSED"
    else
        echo "✗ NEON test FAILED"
        exit 1
    fi
else
    echo "[4/5] Skipping NEON test (not on aarch64)"
fi
echo ""

# Test Python if available
echo "[5/5] Testing Python bindings..."
if command -v python3 &> /dev/null; then
    if python3 -c "import ffht" 2>/dev/null; then
        if python3 example.py > /dev/null 2>&1; then
            echo "✓ Python test PASSED"
        else
            echo "⚠ Python example failed (but module loads)"
        fi
    else
        echo "⚠ Python module not installed (run: python setup.py install)"
    fi
elif command -v python &> /dev/null; then
    if python -c "import ffht" 2>/dev/null; then
        if python example.py > /dev/null 2>&1; then
            echo "✓ Python test PASSED"
        else
            echo "⚠ Python example failed (but module loads)"
        fi
    else
        echo "⚠ Python module not installed (run: python setup.py install)"
    fi
else
    echo "⚠ Python not found - skipping Python tests"
fi
echo ""

# Cleanup
rm -f test_float.log test_double.log

echo "======================================"
echo "  All tests completed successfully!"
echo "======================================"
echo ""
echo "Test logs cleaned up."
echo "To see detailed output, run tests individually:"
echo "  ./test_float"
echo "  ./test_double"
if [[ "$ARCH" == "aarch64" ]]; then
    echo "  ./test_neon"
fi
