/// Basic usage example for FFHT Rust wrapper

use ffht::{Fht, FhtArray};
use ndarray::Array1;

fn main() {
    println!("=== FFHT Basic Usage Examples ===\n");

    // Example 1: In-place FHT with raw Vec
    example1_inplace();

    // Example 2: Out-of-place FHT
    example2_oop();

    // Example 3: ndarray integration
    example3_ndarray();

    // Example 4: Verify orthogonality
    example4_orthogonality();

    // Example 5: Large transform
    example5_large();
}

fn example1_inplace() {
    println!("Example 1: In-place FHT with Vec<f64>");
    println!("---------------------------------------");

    let mut data = vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
    println!("Input:  {:?}", data);

    f64::fht_inplace(&mut data).unwrap();
    println!("Output: {:?}", data);
    println!();
}

fn example2_oop() {
    println!("Example 2: Out-of-place FHT");
    println!("---------------------------");

    let input = vec![1.0f32, 2.0, 3.0, 4.0];
    let mut output = vec![0.0f32; 4];

    println!("Input:  {:?}", input);

    f32::fht(&input, &mut output).unwrap();
    println!("Output: {:?}", output);
    println!();
}

fn example3_ndarray() {
    println!("Example 3: ndarray integration");
    println!("-------------------------------");

    // Create array
    let data = Array1::from(vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
    println!("Input array:  {}", data);

    // Out-of-place transform
    let transformed = data.fht().unwrap();
    println!("Transformed:  {}", transformed);

    // In-place transform
    let mut data2 = data.clone();
    data2.fht_inplace().unwrap();
    println!("In-place:     {}", data2);
    println!();
}

fn example4_orthogonality() {
    println!("Example 4: Verify FHT is orthogonal");
    println!("-----------------------------------");

    let original = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut data = original.clone();

    println!("Original: {:?}", original);

    // Forward FHT
    f64::fht_inplace(&mut data).unwrap();
    println!("Forward:  {:?}", data);

    // Inverse FHT (same as forward, just needs scaling)
    f64::fht_inplace(&mut data).unwrap();

    // Scale by 1/n to get inverse
    let n = data.len() as f64;
    for x in &mut data {
        *x /= n;
    }

    println!("Inverse:  {:?}", data);

    // Check error
    let max_error = original
        .iter()
        .zip(data.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max);

    println!("Max error: {:.2e}", max_error);
    println!();
}

fn example5_large() {
    println!("Example 5: Large transform (2^16 = 65536 elements)");
    println!("---------------------------------------------------");

    let n = 1 << 16;
    let mut data = vec![0.0; n];

    // Fill with some pattern
    for i in 0..n {
        data[i] = if i % 2 == 0 { 1.0 } else { -1.0 };
    }

    println!("Input size: {}", n);
    println!("First 8 elements: {:?}", &data[0..8]);

    // Measure time
    let start = std::time::Instant::now();
    f64::fht_inplace(&mut data).unwrap();
    let duration = start.elapsed();

    println!("FHT completed in: {:?}", duration);
    println!("First 8 elements: {:?}", &data[0..8]);

    // Check some properties
    let max_val = data.iter().map(|x| x.abs()).fold(0.0, f64::max);
    let sum = data.iter().sum::<f64>();

    println!("Max value: {:.2e}", max_val);
    println!("Sum: {:.2e}", sum);
}
