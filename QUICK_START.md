# FFHT Rust Wrapper - Quick Start

## Your Questions Answered

### Q: How do I regenerate those test binaries?

```bash
./rebuild_debug_tests.sh
```

This creates:
- `test_defines` - Check SIMD macros (__AVX__, __SSE2__)
- `test_inplace` - Test in-place FHT
- `test_oop` - Test out-of-place FHT

### Q: Were they just for debugging?

**Yes!** They were created to identify and fix the `fast_copy` bug in the C library. They're not part of the core functionality, just debugging tools.

### Q: Where are the comprehensive tests (log_n 1-30 like Python)?

**They're in the Rust code!** The comprehensive test exists but is marked `#[ignore]` so it doesn't slow down normal testing:

```bash
# Run the full log_n 1-30 test (takes a few minutes)
cargo test --release test_all_sizes -- --ignored --nocapture
```

This tests all 30 sizes just like the Python tests!

## Common Test Commands

**Fast tests** (< 1 second):
```bash
cargo test                                  # 9 unit tests
cargo test test_sizes_subset -- --nocapture # 8 selected sizes
```

**Comprehensive test** (few minutes):
```bash
cargo test --release test_all_sizes -- --ignored --nocapture  # All 30 sizes
```

**Examples**:
```bash
cargo run --example basic        # 5 usage examples
cargo run --example cluster_bp   # Belief propagation use cases
```

**Debug binaries**:
```bash
./rebuild_debug_tests.sh  # Rebuild
./test_defines            # Check SIMD
./test_inplace            # Test in-place
./test_oop                # Test out-of-place
```

## What Got Fixed

The original FFHT library had a bug in `fast_copy.c`:
- AVX2 version fails for arrays < 32 bytes
- It does `n >>= 5` which gives 0 for small arrays
- Loop doesn't run, data isn't copied

**Solution:** Changed `fht_impl.h` to use `memcpy` directly instead of `fast_copy`.

This is why `test_fast_copy` is removed - we don't use that function anymore!

## File Guide

**Want to:** | **Use:**
------------|----------
Run quick tests | `cargo test`
Run all-sizes test | `cargo test --release test_all_sizes -- --ignored --nocapture`
See usage examples | `cargo run --example basic`
Rebuild debug binaries | `./rebuild_debug_tests.sh`
See detailed test docs | `TEST_SUMMARY.md`
See implementation docs | `RUST_WRAPPER.md`

## Test Output Examples

**Subset test:**
```
log_n= 1 (n=       2): max_error=0.00e0 ✓
log_n= 2 (n=       4): max_error=0.00e0 ✓
...
log_n=20 (n= 1048576): max_error=0.00e0 ✓
```

**Debug test:**
```
__AVX__ is defined
__AVX2__ is defined
__SSE2__ is defined
Done
```

**Example output:**
```
Example: Clustered XOR Factor (8 bits)
Expected peak: 0x66
Actual peak:   0x66
✓ Correct! XOR constraint satisfied
```

That's it! The wrapper is complete and fully tested.
