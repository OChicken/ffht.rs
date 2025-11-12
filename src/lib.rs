//! # FFHT - Fast Fast Hadamard Transform
//!
//! Rust wrapper for the FFHT library providing highly optimized
//! Fast Hadamard Transform implementations.
//!
//! ## Features
//!
//! - **SIMD optimized**: Uses AVX on x86_64, NEON on ARM64
//! - **In-place and out-of-place** transforms
//! - **f32 and f64** support
//! - **ndarray integration** for convenient array operations
//! - **Safe API** wrapping unsafe C FFI
//!
//! ## Usage
//!
//! ```rust
//! use ffht::{Fht, FhtArray};
//! use ndarray::Array1;
//!
//! // Create data (length must be power of 2)
//! let mut data = Array1::from(vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0]);
//!
//! // Perform in-place FHT
//! data.fht_inplace().unwrap();
//!
//! println!("Transformed: {:?}", data);
//! ```

use ndarray::{Array1, ArrayViewMut1};
use std::os::raw::c_int;

/// Error types for FFHT operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FhtError {
    /// Input size is not a power of 2
    InvalidSize(usize),
    /// Input size is too large (log_n > 30)
    SizeTooLarge(usize),
    /// Internal FFT error
    InternalError(i32),
}

impl std::fmt::Display for FhtError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FhtError::InvalidSize(size) => {
                write!(f, "Input size {} is not a power of 2", size)
            }
            FhtError::SizeTooLarge(size) => {
                write!(f, "Input size {} is too large (max 2^30)", size)
            }
            FhtError::InternalError(code) => {
                write!(f, "FFHT internal error: {}", code)
            }
        }
    }
}

impl std::error::Error for FhtError {}

pub type FhtResult<T> = Result<T, FhtError>;

/// Raw FFI bindings to FFHT C library
mod ffi {
    use std::os::raw::c_int;

    extern "C" {
        /// In-place FHT for f32
        pub fn fht_float(buf: *mut f32, log_n: c_int) -> c_int;

        /// In-place FHT for f64
        pub fn fht_double(buf: *mut f64, log_n: c_int) -> c_int;

        /// Out-of-place FHT for f32
        pub fn fht_float_oop(input: *const f32, output: *mut f32, log_n: c_int) -> c_int;

        /// Out-of-place FHT for f64
        pub fn fht_double_oop(input: *const f64, output: *mut f64, log_n: c_int) -> c_int;
    }
}

/// Trait for types that support Fast Hadamard Transform
pub trait Fht: Sized {
    /// Perform in-place FHT on a mutable array
    fn fht_inplace(data: &mut [Self]) -> FhtResult<()>;

    /// Perform out-of-place FHT
    fn fht(input: &[Self], output: &mut [Self]) -> FhtResult<()>;
}

impl Fht for f32 {
    fn fht_inplace(data: &mut [Self]) -> FhtResult<()> {
        let n = data.len();
        let log_n = validate_size(n)?;

        let result = unsafe { ffi::fht_float(data.as_mut_ptr(), log_n as c_int) };

        if result != 0 {
            Err(FhtError::InternalError(result))
        } else {
            Ok(())
        }
    }

    fn fht(input: &[Self], output: &mut [Self]) -> FhtResult<()> {
        if input.len() != output.len() {
            return Err(FhtError::InvalidSize(output.len()));
        }

        let n = input.len();
        let log_n = validate_size(n)?;

        let result = unsafe {
            ffi::fht_float_oop(input.as_ptr(), output.as_mut_ptr(), log_n as c_int)
        };

        if result != 0 {
            Err(FhtError::InternalError(result))
        } else {
            Ok(())
        }
    }
}

impl Fht for f64 {
    fn fht_inplace(data: &mut [Self]) -> FhtResult<()> {
        let n = data.len();
        let log_n = validate_size(n)?;

        let result = unsafe { ffi::fht_double(data.as_mut_ptr(), log_n as c_int) };

        if result != 0 {
            Err(FhtError::InternalError(result))
        } else {
            Ok(())
        }
    }

    fn fht(input: &[Self], output: &mut [Self]) -> FhtResult<()> {
        if input.len() != output.len() {
            return Err(FhtError::InvalidSize(output.len()));
        }

        let n = input.len();
        let log_n = validate_size(n)?;

        let result = unsafe {
            ffi::fht_double_oop(input.as_ptr(), output.as_mut_ptr(), log_n as c_int)
        };

        if result != 0 {
            Err(FhtError::InternalError(result))
        } else {
            Ok(())
        }
    }
}

/// Validate that size is a power of 2 and return log_2(size)
fn validate_size(size: usize) -> FhtResult<usize> {
    if size == 0 || !size.is_power_of_two() {
        return Err(FhtError::InvalidSize(size));
    }

    let log_n = size.trailing_zeros() as usize;

    if log_n > 30 {
        return Err(FhtError::SizeTooLarge(size));
    }

    Ok(log_n)
}

/// Extension trait for ndarray integration
pub trait FhtArray {
    /// Perform in-place FHT on this array
    fn fht_inplace(&mut self) -> FhtResult<()>;

    /// Perform FHT and return new array
    fn fht(&self) -> FhtResult<Self>
    where
        Self: Sized;
}

impl FhtArray for Array1<f32> {
    fn fht_inplace(&mut self) -> FhtResult<()> {
        let slice = self.as_slice_mut().unwrap();
        f32::fht_inplace(slice)
    }

    fn fht(&self) -> FhtResult<Self> {
        let mut output = Array1::zeros(self.len());
        f32::fht(self.as_slice().unwrap(), output.as_slice_mut().unwrap())?;
        Ok(output)
    }
}

impl FhtArray for Array1<f64> {
    fn fht_inplace(&mut self) -> FhtResult<()> {
        let slice = self.as_slice_mut().unwrap();
        f64::fht_inplace(slice)
    }

    fn fht(&self) -> FhtResult<Self> {
        let mut output = Array1::zeros(self.len());
        f64::fht(self.as_slice().unwrap(), output.as_slice_mut().unwrap())?;
        Ok(output)
    }
}

/// Extension for mutable array views (only in-place operations)
impl<'a> FhtArray for ArrayViewMut1<'a, f32> {
    fn fht_inplace(&mut self) -> FhtResult<()> {
        let slice = self.as_slice_mut().unwrap();
        f32::fht_inplace(slice)
    }

    fn fht(&self) -> FhtResult<Self> {
        // Views cannot create owned arrays, use Array1::fht() instead
        unimplemented!("Use fht_inplace() for views, or convert to Array1 first")
    }
}

impl<'a> FhtArray for ArrayViewMut1<'a, f64> {
    fn fht_inplace(&mut self) -> FhtResult<()> {
        let slice = self.as_slice_mut().unwrap();
        f64::fht_inplace(slice)
    }

    fn fht(&self) -> FhtResult<Self> {
        // Views cannot create owned arrays, use Array1::fht() instead
        unimplemented!("Use fht_inplace() for views, or convert to Array1 first")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_validate_size() {
        assert!(validate_size(8).is_ok());
        assert_eq!(validate_size(8).unwrap(), 3);

        assert!(validate_size(7).is_err());
        assert!(validate_size(0).is_err());
    }

    #[test]
    fn test_fht_f32_inplace() {
        let mut data = vec![1.0f32, -1.0, 1.0, -1.0];
        f32::fht_inplace(&mut data).unwrap();

        // FHT of [1, -1, 1, -1] should be [0, 4, 0, 0]
        assert_abs_diff_eq!(data[0], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(data[1], 4.0, epsilon = 1e-6);
        assert_abs_diff_eq!(data[2], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(data[3], 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_fht_f64_inplace() {
        let mut data = vec![1.0f64, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        f64::fht_inplace(&mut data).unwrap();

        // Check that transform happened (non-trivial result)
        let sum: f64 = data.iter().map(|x| x.abs()).sum();
        assert!(sum > 0.0);
    }

    #[test]
    fn test_fht_oop() {
        let input = vec![1.0f32, -1.0, 1.0, -1.0];
        let mut output = vec![0.0f32; 4];

        println!("Input: {:?}", input);
        println!("Output before: {:?}", output);

        f32::fht(&input, &mut output).unwrap();

        println!("Output after: {:?}", output);

        // Should match in-place result
        assert_abs_diff_eq!(output[0], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(output[1], 4.0, epsilon = 1e-6);
    }

    #[test]
    fn test_ndarray_integration() {
        let mut data = Array1::from(vec![1.0f64, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0]);
        data.fht_inplace().unwrap();

        // Verify transformation occurred
        assert!(data.iter().any(|&x| x.abs() > 1e-6));
    }

    #[test]
    fn test_ndarray_oop() {
        let data = Array1::from(vec![1.0f32, -1.0, 1.0, -1.0]);
        let result = data.fht().unwrap();

        assert_abs_diff_eq!(result[0], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(result[1], 4.0, epsilon = 1e-6);
    }

    #[test]
    fn test_invalid_size() {
        let mut data = vec![1.0f32, 2.0, 3.0]; // Not power of 2
        let result = f32::fht_inplace(&mut data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FhtError::InvalidSize(3));
    }

    #[test]
    fn test_orthogonality() {
        // FHT is orthogonal: FHT(FHT(x)) / n = x
        let original = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let mut data = original.clone();

        // Forward transform
        f64::fht_inplace(&mut data).unwrap();

        // Inverse transform (FHT is self-inverse up to scaling)
        f64::fht_inplace(&mut data).unwrap();

        // Scale by 1/n
        let n = data.len() as f64;
        for x in &mut data {
            *x /= n;
        }

        // Should recover original
        for (&result, &expected) in data.iter().zip(original.iter()) {
            assert_abs_diff_eq!(result, expected, epsilon = 1e-10);
        }
    }

    /// Comprehensive test: All sizes from 2^1 to 2^30
    /// This matches the original C test (test_float.c and test_double.c)
    ///
    /// Note: Run with --release for reasonable performance:
    ///   cargo test --release test_all_sizes -- --nocapture --test-threads=1
    #[test]
    #[ignore] // Ignore by default (takes time and memory)
    fn test_all_sizes() {
        println!("\nComprehensive FHT Test: log_n 1 to 30");
        println!("=====================================\n");

        use std::time::Instant;

        for log_n in 1..=30 {
            let n = 1_usize << log_n;

            print!("log_n={:2} (n={:10}): ", log_n, n);
            std::io::Write::flush(&mut std::io::stdout()).ok();

            // Create random-ish data
            let mut data: Vec<f64> = (0..n)
                .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
                .collect();
            let original = data.clone();

            // Test orthogonality: FHT(FHT(x)) / n = x
            let start = Instant::now();

            // Forward
            f64::fht_inplace(&mut data).unwrap();

            // Inverse
            f64::fht_inplace(&mut data).unwrap();

            let duration = start.elapsed();

            // Scale
            for x in &mut data {
                *x /= n as f64;
            }

            // Check error
            let max_error = data.iter()
                .zip(original.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0, f64::max);

            print!("max_error={:.2e}, time={:?}", max_error, duration);

            if max_error > 1e-6 {
                println!(" ✗ FAILED");
                panic!("Test failed for log_n={}: max_error={}", log_n, max_error);
            } else {
                println!(" ✓");
            }

            // For large sizes (> 2^25), give warning about memory
            if log_n == 25 {
                println!("\nNote: Sizes > 2^25 require significant memory (>256MB)");
            }
        }

        println!("\n=====================================");
        println!("All 30 sizes passed!");
    }

    /// Subset test: Test selected sizes that are fast
    /// Run with: cargo test test_sizes_subset -- --nocapture
    #[test]
    fn test_sizes_subset() {
        let test_sizes = vec![1, 2, 4, 8, 10, 12, 16, 20];

        println!("\nFHT Subset Test (selected sizes)");
        println!("=================================\n");

        for log_n in test_sizes {
            let n = 1_usize << log_n;

            let mut data: Vec<f64> = (0..n)
                .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
                .collect();
            let original = data.clone();

            // Test orthogonality
            f64::fht_inplace(&mut data).unwrap();
            f64::fht_inplace(&mut data).unwrap();

            for x in &mut data {
                *x /= n as f64;
            }

            let max_error = data.iter()
                .zip(original.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0, f64::max);

            println!("log_n={:2} (n={:8}): max_error={:.2e} ✓", log_n, n, max_error);
            assert!(max_error < 1e-6);
        }

        println!("\nAll subset tests passed!");
    }
}
