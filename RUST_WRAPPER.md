# FFHT Rust Wrapper

Rust wrapper for the [FFHT library](https://github.com/FALCONN-LIB/FFHT) providing highly optimized Fast Hadamard Transform implementations.

## Features

- **SIMD optimized**: Uses AVX on x86_64, NEON on ARM64, or SSE2 fallback
- **In-place and out-of-place** transforms
- **f32 and f64** support
- **ndarray integration** for convenient array operations
- **Safe API** wrapping unsafe C FFI
- **Zero-cost abstractions**: Direct FFI bindings with minimal overhead

## Quick Start

### Basic Usage

```rust
use ffht::Fht;

let mut data = vec![1.0f64, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
f64::fht_inplace(&mut data).unwrap();
// Result: [0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
```

### Using ndarray

```rust
use ffht::FhtArray;
use ndarray::Array1;

let mut data = Array1::from(vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
data.fht_inplace().unwrap();
```

## Use Case: Clustered Belief Propagation

The FFHT library is particularly useful for **clustered belief propagation** in SASCA (Soft Analytical Side-Channel Attacks).

### Key Insight

**XOR in bit space = convolution in Walsh space**

This means XOR constraints between multi-bit variables can be computed efficiently:

```rust
use ffht::FhtArray;
use ndarray::Array1;

// Clustered nodes: a = b ⊕ c (8 bits each)
let mut msg_b = Array1::from(vec![/* 256 probabilities */]);
let mut msg_c = Array1::from(vec![/* 256 probabilities */]);

// Transform to Walsh domain
msg_b.fht_inplace().unwrap();
msg_c.fht_inplace().unwrap();

// XOR becomes pointwise multiplication
let mut msg_a = &msg_b * &msg_c;

// Transform back
msg_a.fht_inplace().unwrap();
msg_a /= 256.0; // Scale by 1/n

// msg_a now contains P(a | b, c, a = b ⊕ c)
```

### Keccak Theta Layer Example

For 5-way XOR (parity computation):

```rust
// Transform all messages to Walsh domain
let walsh_messages: Vec<_> = messages.iter()
    .map(|msg| {
        let mut w = msg.clone();
        w.fht_inplace().unwrap();
        w
    })
    .collect();

// Multiply all in Walsh space (= multi-way XOR)
let mut result = walsh_messages[0].clone();
for msg in &walsh_messages[1..] {
    result = &result * msg;
}

// Transform back
result.fht_inplace().unwrap();
result /= 256.0; // Scale
result /= result.sum(); // Normalize
```

## Performance

On x86_64 with AVX2:
- **2^16 elements**: ~150 µs
- **2^20 elements**: ~2.5 ms

This is significantly faster than naive O(n²) implementations.

## Examples

```bash
# Basic usage examples
cargo run --example basic

# Belief propagation use cases (XOR, parity)
cargo run --example cluster_bp
```

## Testing

```bash
cargo test
```

All 9 tests should pass (8 unit tests + 1 doc test).

## Implementation Notes

### Architecture Detection

The build script automatically detects your architecture and compiles the appropriate SIMD version:
- x86_64: AVX2/AVX/SSE2 (with `-march=native`)
- aarch64: NEON
- Others: Portable fallback

### Memcpy Fix

The original FFHT's `fast_copy` function has a bug for small arrays (< 32 bytes) when using AVX2. This wrapper uses `memcpy` for out-of-place operations, which is correct for all sizes and still very fast.

## API Reference

### Trait: `Fht`

Implemented for `f32` and `f64`:

```rust
fn fht_inplace(data: &mut [Self]) -> FhtResult<()>;
fn fht(input: &[Self], output: &mut [Self]) -> FhtResult<()>;
```

### Trait: `FhtArray`

Implemented for `Array1<f32>` and `Array1<f64>`:

```rust
fn fht_inplace(&mut self) -> FhtResult<()>;
fn fht(&self) -> FhtResult<Self>;
```

### Error Type

```rust
enum FhtError {
    InvalidSize(usize),     // Not a power of 2
    SizeTooLarge(usize),    // > 2^30
    InternalError(i32),     // C library error
}
```

## Integration with bp.rs

This wrapper is designed to integrate with the `bp.rs/common` belief propagation library:

- `ClusterNode`: Multi-bit variable nodes
- `ClusterFactorLinear`: XOR/linear factors using FHT
- Efficient message passing for 8-256 bit clusters

See `bp.rs/common/src/cluster_factor_linear.rs` for usage.

## License

MIT (same as original FFHT library)

## References

- [FFHT GitHub](https://github.com/FALCONN-LIB/FFHT)
- [SASCA Paper](https://eprint.iacr.org/2014/410)
