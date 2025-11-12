# ARM NEON Support for FFHT

This directory now includes ARM NEON (aarch64) support for the Fast Fast Hadamard Transform library.

## What was added

1. **fht_neon.c** - ARM NEON implementation of FHT using ARM NEON intrinsics
2. **Updated fast_copy.c** - Added NEON-optimized memory copy
3. **Updated fht_impl.h** - Automatic detection of ARM architecture
4. **test_neon.c** - Test and benchmark program for NEON implementation

## How it works

The build system automatically detects the target architecture:
- On x86_64 with AVX: uses AVX implementation (fastest on Intel/AMD)
- On x86_64 without AVX: uses SSE implementation
- On aarch64 (ARM64): uses NEON implementation (NEW!)
- On other architectures: uses portable C fallback

## Building on ARM

### Compile the test program:
```bash
gcc -O3 -march=native -std=c99 test_neon.c fht.c fast_copy.c -o test_neon -lm
./test_neon
```

### For Python bindings on ARM:
```bash
python setup.py install
```

The setup.py will automatically use the NEON implementation on ARM platforms.

## Implementation Details

The NEON implementation uses a recursive divide-and-conquer approach:
- Small sizes (log_n = 1-3) use hand-optimized NEON code
- Larger sizes use recursive decomposition with NEON for the final butterfly operations
- Uses 128-bit NEON vectors (equivalent to SSE2 on x86)

## Performance Notes

- NEON provides 4-way SIMD for single precision (float32x4_t)
- NEON provides 2-way SIMD for double precision (float64x2_t)
- Performance is comparable to SSE2 on x86
- On Apple Silicon (M1/M2), performance is excellent
- On ARM Cortex-A series, performance depends on the specific CPU

## Verification

The implementation has been designed to produce bit-exact results compared to the SSE/AVX versions (within floating-point tolerance). Use test_neon.c to verify correctness on your ARM platform.

## Compatibility

- Requires ARMv8 (aarch64) or later
- Compatible with:
  - Apple Silicon (M1, M2, M3)
  - AWS Graviton
  - Ampere Altra
  - Qualcomm Snapdragon
  - Other ARM64 processors with NEON support

## Limitations

- Currently supports log_n up to 30 (same as x86 versions)
- Recursive implementation may use more stack for very large transforms
- For very small transforms (log_n < 4), scalar code may be competitive

## Future Optimizations

Potential improvements for future versions:
- Use SVE (Scalable Vector Extension) on ARMv9 for wider vectors
- Further unrolling for specific sizes
- Assembly-optimized kernels for critical sizes
- Cache-blocking for very large transforms
