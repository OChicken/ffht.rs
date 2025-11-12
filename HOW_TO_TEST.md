# How to Run FFHT Tests

This guide covers all available tests in the FFHT library.

## Quick Start (All Tests)

```bash
cd /home/OChicken/pqc/bp.rs/FFHT

# Build and run all C tests
make clean
make all
./test_float
./test_double

# If on ARM/aarch64, also test NEON
make test_neon
./test_neon

# Test Python bindings (if installed)
python example.py
```

---

## C Tests (Standard Implementation)

### 1. Float Precision Test
Tests single-precision (float32) FHT implementation.

```bash
make test_float
./test_float
```

**What it does:**
- Tests all sizes from 2^1 to 2^30 elements
- Compares against naive reference implementation
- Reports error if accuracy > 1e-5
- Benchmarks performance for each size

**Expected output:**
```
1 4.5678901234e-06
2 5.1234567890e-06
3 6.7890123456e-06
...
30 1.2345678901e-02
```
Each line: `log_n time_per_transform`

### 2. Double Precision Test
Tests double-precision (float64) FHT implementation.

```bash
make test_double
./test_double
```

Same format as test_float but with higher precision.

---

## C Tests (Header-Only Implementation)

The header-only version includes all code in headers (no separate .o files needed).

### 3. Float Header-Only Test
```bash
make test_float_header_only
./test_float_header_only
```

### 4. Double Header-Only Test
```bash
make test_double_header_only
./test_double_header_only
```

**Use case:** When you want to include FFHT in a single-file compilation.

---

## ARM NEON Test (aarch64 only)

**Only run this on ARM64 systems (Apple Silicon, Graviton, etc.)**

```bash
make test_neon
./test_neon
```

**What it does:**
- Verifies NEON implementation correctness (log_n 1-10)
- Compares against naive reference implementation
- Benchmarks NEON performance on your ARM CPU
- Reports PASS/FAIL for each size

**Expected output:**
```
ARM NEON FHT Implementation Test
=================================

Correctness tests:
log_n= 1 (n=     2): max_error=0.00e+00 ... PASS
log_n= 2 (n=     4): max_error=0.00e+00 ... PASS
log_n= 3 (n=     8): max_error=0.00e+00 ... PASS
...
log_n=10 (n=  1024): max_error=2.38e-05 ... PASS

All correctness tests PASSED!

Performance benchmarks:
log_n= 8 (n=     256): 0.312 us per transform
log_n=10 (n=    1024): 1.453 us per transform
log_n=12 (n=    4096): 6.234 us per transform
log_n=16 (n=   65536): 112.567 us per transform
log_n=20 (n= 1048576): 2134.890 us per transform

All tests completed successfully!
```

---

## Python Tests

### 5. Python Example/Benchmark

First, install the Python module:
```bash
pip install numpy
python setup.py install
```

Then run the example:
```bash
python example.py
```

**What it does:**
- Creates a random array of 2^20 (1M) floats
- Performs FHT 1000 times
- Reports average time per transform

**Expected output:**
```
0.0012345  # Average time in seconds
```

---

## Understanding Test Output

### Test Success Criteria

| Test | Success Condition | Failure Indication |
|------|-------------------|-------------------|
| test_float | All errors < 1e-5 | Prints "ERROR: xxxxx" |
| test_double | All errors < 1e-10 | Prints "ERROR: xxxxx" |
| test_neon | All show "PASS" | Shows "FAIL" for any size |
| Header-only | Same as regular tests | Prints "ERROR: xxxxx" |

### Performance Expectations

Approximate times for 2^20 (1M) elements on different platforms:

| Platform | Float | Double |
|----------|-------|--------|
| Intel i7 (AVX2) | 0.7 ms | 1.4 ms |
| AMD Ryzen (AVX2) | 0.8 ms | 1.5 ms |
| Apple M1 (NEON) | 0.9 ms | 1.6 ms |
| Graviton3 (NEON) | 1.2 ms | 2.3 ms |
| Older x86 (SSE2) | 1.5 ms | 2.8 ms |

---

## Debugging Failed Tests

### If test_float or test_double fails:

1. **Check architecture detection:**
   ```bash
   gcc -march=native -Q --help=target | grep march
   ```

2. **Try with basic optimization:**
   ```bash
   make clean
   CFLAGS="-O2 -std=c99" make test_float
   ./test_float
   ```

3. **Check for alignment issues:**
   ```bash
   # Run with verbose output
   gcc -v -O3 -march=native test_float.c fht.c fast_copy.c -o test_float -lm
   ```

### If test_neon fails on ARM:

1. **Verify you're on aarch64:**
   ```bash
   uname -m  # Should output: aarch64
   ```

2. **Check NEON support:**
   ```bash
   cat /proc/cpuinfo | grep -i neon
   ```

3. **Try without -march=native:**
   ```bash
   make clean
   CC=gcc CFLAGS="-O3 -std=c99" make test_neon
   ./test_neon
   ```

### If Python tests fail:

1. **Check NumPy installation:**
   ```bash
   python -c "import numpy; print(numpy.__version__)"
   ```

2. **Rebuild with verbose output:**
   ```bash
   python setup.py build --verbose
   python setup.py install
   ```

3. **Test import:**
   ```python
   import ffht
   import numpy as np
   x = np.random.randn(256).astype(np.float32)
   ffht.fht(x)
   print("Success!")
   ```

---

## Advanced Testing

### Test Specific Size

Modify test files to focus on a specific size:

```c
// In test_float.c, change line 27:
for (int log_n = 10; log_n <= 10; ++log_n) {  // Only test 2^10
```

### Memory Profiling

```bash
# Linux
valgrind --leak-check=full ./test_float

# macOS
leaks --atExit -- ./test_float
```

### Performance Profiling

```bash
# Linux with perf
perf record ./test_float
perf report

# View cycles per element
perf stat -e cycles,instructions ./test_float
```

---

## Continuous Integration

To run all tests automatically:

```bash
#!/bin/bash
set -e  # Exit on any error

cd /home/OChicken/pqc/bp.rs/FFHT

echo "Building tests..."
make clean
make all

echo "Running float test..."
./test_float > /dev/null || exit 1

echo "Running double test..."
./test_double > /dev/null || exit 1

# On ARM only
if [[ $(uname -m) == "aarch64" ]]; then
    echo "Running NEON test..."
    make test_neon
    ./test_neon || exit 1
fi

# If Python is available
if command -v python &> /dev/null; then
    echo "Testing Python bindings..."
    python example.py > /dev/null || exit 1
fi

echo "All tests PASSED!"
```

---

## Summary

| Test Command | Platform | Purpose |
|--------------|----------|---------|
| `./test_float` | All | Correctness + benchmark (float32) |
| `./test_double` | All | Correctness + benchmark (float64) |
| `./test_neon` | ARM64 only | NEON implementation verification |
| `./test_*_header_only` | All | Header-only version tests |
| `python example.py` | All | Python bindings benchmark |

**Recommended**: Run `test_float`, `test_double`, and (on ARM) `test_neon` after any changes.
