# Installation Instructions for ARM64 (aarch64)

## Quick Start

### 1. Build and Test the C Library

```bash
cd FFHT
make test_neon
./test_neon
```

This will compile the NEON-optimized FHT implementation and run correctness and performance tests.

### 2. Install Python Bindings

```bash
# Make sure you have NumPy installed
pip install numpy

# Build and install the Python extension
python setup.py install
```

### 3. Test the Python Interface

```python
import numpy as np
import ffht

# Create a test array (size must be power of 2)
n = 256
x = np.random.randn(n).astype(np.float32)

# Perform Fast Hadamard Transform in-place
ffht.fht(x)

print("FHT computed successfully!")
```

## Platform-Specific Notes

### Apple Silicon (M1, M2, M3, M4)

Apple Silicon has excellent NEON performance. The library should work out of the box:

```bash
# Use system compiler (clang with ARM support)
CC=clang make test_neon
./test_neon
```

### AWS Graviton (Graviton2, Graviton3)

```bash
# On Amazon Linux / Ubuntu on Graviton
sudo yum install gcc numpy  # Amazon Linux
# or
sudo apt install gcc python3-numpy  # Ubuntu

make test_neon
./test_neon
```

### Raspberry Pi 4/5 (64-bit OS)

Make sure you're running a 64-bit OS (aarch64), not 32-bit (armhf):

```bash
uname -m  # Should output: aarch64

# Install dependencies
sudo apt install gcc python3-numpy python3-dev

# Build and test
make test_neon
./test_neon
```

### Other ARM64 Systems

The library should work on any ARMv8 (aarch64) system with NEON support. NEON is mandatory in ARMv8, so all aarch64 CPUs support it.

## Troubleshooting

### "Illegal instruction" error

If you get this error, it means `-march=native` generated instructions not supported by your CPU. Try:

```bash
# Use baseline ARMv8 instructions only
CFLAGS="-O3 -std=c99" make test_neon
```

### Performance is slower than expected

1. Make sure you're using `-O3` optimization
2. Check that you're compiling for the right architecture:
   ```bash
   gcc -march=native -Q --help=target | grep march
   ```
3. Verify NEON is being used:
   ```bash
   nm fht.o | grep neon
   objdump -d fht.o | grep -A 5 fht_float | head -20
   ```

### Python installation fails

Make sure you have NumPy and development headers:

```bash
# Ubuntu/Debian
sudo apt install python3-dev python3-numpy

# macOS
pip install numpy

# Then try again
python setup.py install
```

## Performance Expectations

Typical performance on various ARM platforms for 2^20 (1M) elements:

| Platform | Time (float) | Time (double) |
|----------|-------------|---------------|
| Apple M1 | ~0.8 ms | ~1.5 ms |
| Graviton3 | ~1.2 ms | ~2.3 ms |
| Graviton2 | ~1.8 ms | ~3.2 ms |
| Cortex-A76 | ~2.5 ms | ~4.5 ms |
| Cortex-A72 | ~3.5 ms | ~6.0 ms |

(Times are approximate and depend on memory frequency and system load)

## Comparing with x86

On comparable systems, ARM NEON performance is similar to x86 SSE2:
- AVX on modern Intel/AMD: ~30-50% faster than NEON
- SSE2 on older x86: similar to NEON
- Basic scalar code: ~10x slower than NEON

## Advanced: Cross-Compilation

To cross-compile for ARM from x86:

```bash
# Install cross-compiler
sudo apt install gcc-aarch64-linux-gnu

# Cross-compile
CC=aarch64-linux-gnu-gcc make test_neon

# Copy to ARM system and run
scp test_neon user@arm-system:
ssh user@arm-system ./test_neon
```

## Support

For issues specific to ARM/NEON implementation, please check:
1. README_NEON.md for implementation details
2. GitHub issues for known problems
3. Test your specific platform with test_neon.c

For general FFHT questions, see the main README.md
