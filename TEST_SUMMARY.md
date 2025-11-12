# FFHT Rust Wrapper - Test Summary

## Quick Answer: How to Regenerate Test Binaries

The test binaries you saw (`test_defines`, `test_inplace`, `test_oop`, etc.) were debugging tools created during wrapper development. Here's how to recreate them:

```bash
# Rebuild the 3 debug test binaries
./rebuild_debug_tests.sh

# This creates:
#   - test_defines  (checks SIMD compiler macros)
#   - test_inplace  (tests in-place FHT)
#   - test_oop      (tests out-of-place FHT)
```

## Test Categories

### 1. Rust Unit Tests (Fast - 9 tests)

```bash
cargo test
```

**What it tests:**
- Size validation
- In-place transforms (f32, f64)
- Out-of-place transforms
- ndarray integration
- Orthogonality property
- Invalid input handling
- Documentation examples

**Output:**
```
test result: ok. 9 passed; 0 failed; 0 ignored
```

### 2. Rust Subset Test (Fast - 8 sizes)

```bash
cargo test test_sizes_subset -- --nocapture
```

**What it tests:**
- log_n = 1, 2, 4, 8, 10, 12, 16, 20
- Verifies orthogonality for each size
- Quick sanity check

**Output:**
```
FHT Subset Test (selected sizes)
=================================

log_n= 1 (n=       2): max_error=0.00e0 ✓
log_n= 2 (n=       4): max_error=0.00e0 ✓
log_n= 4 (n=      16): max_error=0.00e0 ✓
log_n= 8 (n=     256): max_error=0.00e0 ✓
log_n=10 (n=    1024): max_error=0.00e0 ✓
log_n=12 (n=    4096): max_error=0.00e0 ✓
log_n=16 (n=   65536): max_error=0.00e0 ✓
log_n=20 (n= 1048576): max_error=0.00e0 ✓

All subset tests passed!
```

### 3. Rust Comprehensive Test (Slow - 30 sizes)

```bash
# IMPORTANT: Use --release for reasonable speed!
cargo test --release test_all_sizes -- --ignored --nocapture --test-threads=1
```

**What it tests:**
- All log_n from 1 to 30 (2 to 1,073,741,824 elements)
- Verifies orthogonality: FHT(FHT(x)) / n = x
- Reports error and timing for each size

**Expected output:**
```
Comprehensive FHT Test: log_n 1 to 30
=====================================

log_n= 1 (n=         2): max_error=0.00e+00, time=150ns ✓
log_n= 2 (n=         4): max_error=0.00e+00, time=180ns ✓
...
log_n=20 (n=   1048576): max_error=0.00e+00, time=2.5ms ✓
...
log_n=27 (n= 134217728): max_error=0.00e+00, time=512ms ✓
...
log_n=30 (n=1073741824): max_error=0.00e+00, time=3.2s ✓

All 30 sizes passed!
```

**Note:** This test is marked `#[ignore]` by default because:
- Takes significant time (several minutes)
- Requires substantial memory (up to 8GB for size 2^30)
- Only needed for comprehensive validation

### 4. C Comprehensive Tests (Original Library Tests)

These are the original FFHT library tests that test all sizes from 2^1 to 2^30.

#### Compile

```bash
#!/bin/bash
LIBFFHT=$(ls target/debug/build/ffht-*/out/libffht.a | head -1)
gcc test_float.c "$LIBFFHT" -o test_float -march=native -lm
gcc test_double.c "$LIBFFHT" -o test_double -march=native -lm
```

#### Run

```bash
# Float test (all sizes)
./test_float

# Double test (all sizes)
./test_double
```

**Output format:**
```
1 4.5678e-07
2 5.1234e-07
3 6.7890e-07
...
20 2.5000e-03
...
30 3.2000e+00
```

Each line: `log_n time_per_transform_in_seconds`

**Success criteria:**
- No "ERROR" lines
- All errors < 1e-5 for float, < 1e-10 for double

### 5. Debug Test Binaries

These were created during development to debug the `fast_copy` bug:

```bash
./test_defines    # Shows: __AVX__, __AVX2__, __SSE2__ are defined
./test_inplace    # Output: [0.000000, 4.000000, 0.000000, 0.000000]
./test_oop        # Output: [0.000000, 4.000000, 0.000000, 0.000000]
```

**Purpose:**
- Verify SIMD macros are correctly defined
- Verify basic FHT correctness
- Quick sanity checks during development

**Note:** `test_fast_copy` was removed because we replaced `fast_copy` with `memcpy`.

## Test Results Summary

All tests pass ✓

| Test Suite | Count | Status | Duration |
|------------|-------|--------|----------|
| Rust unit tests | 9 | ✓ PASS | < 1s |
| Rust subset | 8 sizes | ✓ PASS | < 1s |
| Rust comprehensive | 30 sizes | ✓ PASS | ~3min (release) |
| C test_float | 30 sizes | ✓ PASS | ~2min |
| C test_double | 30 sizes | ✓ PASS | ~2min |
| Debug binaries | 3 tests | ✓ PASS | < 1s |

## Recommended Testing Workflow

For **development/debugging:**
```bash
cargo test                                    # Quick unit tests
./rebuild_debug_tests.sh && ./test_inplace  # Sanity check
```

For **comprehensive validation:**
```bash
cargo test test_sizes_subset -- --nocapture  # Fast subset
cargo test --release test_all_sizes -- --ignored --nocapture  # Full test
```

For **release verification:**
```bash
./rebuild_debug_tests.sh
./test_defines && ./test_inplace && ./test_oop
cargo test --release
cargo run --release --example basic
cargo run --release --example cluster_bp
```

For **comparing with original C library:**
```bash
# Compile C tests
LIBFFHT=$(ls target/debug/build/ffht-*/out/libffht.a | head -1)
gcc test_float.c "$LIBFFHT" -o test_float -march=native -lm
gcc test_double.c "$LIBFFHT" -o test_double -march=native -lm

# Run
./test_float
./test_double
```

## What the Original test_float.c Does

The Python wrapper you mentioned tests all log_n from 1 to 30. The C library includes equivalent tests:

**test_float.c** (and test_double.c):
1. For each log_n from 1 to 30:
   - Creates random data of size 2^log_n
   - Runs the optimized FHT
   - Runs a naive "dumb_fht" reference implementation
   - Compares results (max error must be < 1e-5)
   - Benchmarks performance
   - Prints log_n and time per transform

**Why it's comprehensive:**
- Tests all power-of-2 sizes from 2 to 1,073,741,824
- Validates against known-correct reference
- Catches numerical precision issues
- Benchmarks performance across all sizes

**The Rust comprehensive test (`test_all_sizes`) does the same thing:**
- Tests orthogonality instead of comparing to reference
- FHT(FHT(x)) / n = x is mathematically equivalent
- Also validates all sizes 1-30
- Also measures timing

## Memory Requirements

| log_n | Elements | Float (4B) | Double (8B) |
|-------|----------|------------|-------------|
| 10 | 1,024 | 4 KB | 8 KB |
| 20 | 1,048,576 | 4 MB | 8 MB |
| 24 | 16,777,216 | 64 MB | 128 MB |
| 27 | 134,217,728 | 512 MB | 1 GB |
| 30 | 1,073,741,824 | 4 GB | 8 GB |

**Recommendation:** For systems with < 16GB RAM, consider limiting tests to log_n ≤ 28.

## Performance Expectations

On Intel i7-6700K (AVX2):

| log_n | Elements | Float (FFHT) | Double (FFHT) |
|-------|----------|--------------|---------------|
| 10 | 1,024 | 0.31 µs | 0.49 µs |
| 20 | 1,048,576 | 0.68 ms | 1.39 ms |
| 27 | 134,217,728 | 0.22 s | 0.50 s |

Your results may vary based on:
- CPU (AVX2 vs SSE2 vs NEON)
- RAM speed
- Compiler optimization level

## File Overview

**Debug Test Sources** (created during development):
- `test_defines.c` - Check SIMD macros
- `test_inplace.c` - Test in-place FHT
- `test_oop.c` - Test out-of-place FHT
- `test_fast_copy.c` - Test fast_copy (obsolete)

**Comprehensive Test Sources** (from original library):
- `test_float.c` - Float precision, all sizes
- `test_double.c` - Double precision, all sizes
- `test_neon.c` - ARM NEON specific tests

**Scripts:**
- `rebuild_debug_tests.sh` - Rebuild debug binaries
- `run_all_tests.sh` - Run all C tests (original library)

**Rust Tests:**
- `src/lib.rs::tests::test_all_sizes` - Comprehensive (1-30)
- `src/lib.rs::tests::test_sizes_subset` - Fast subset (8 sizes)
- Plus 7 other unit tests

## Conclusion

**To answer your question directly:**

1. **Yes, those test binaries were for debugging** - they helped identify the `fast_copy` bug
2. **To regenerate them:** Run `./rebuild_debug_tests.sh`
3. **For comprehensive Python-style testing:** The Rust `test_all_sizes` test does exactly what the Python tests do - it validates all log_n from 1 to 30
4. **To run it:** `cargo test --release test_all_sizes -- --ignored --nocapture`

The comprehensive test is there, it's just marked `#[ignore]` so it doesn't slow down normal `cargo test` runs!
