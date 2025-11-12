/// Example: Using FFHT for Clustered Belief Propagation
///
/// This demonstrates how FFHT is used in ClusterFactorLinear
/// for efficient message passing in belief propagation.

use ffht::FhtArray;
use ndarray::Array1;

fn main() {
    println!("=== FFHT for Clustered Belief Propagation ===\n");

    // Simulate clustered BP scenario
    cluster_xor_example();
    cluster_parity_example();
}

/// Example: XOR constraint with clustered nodes
///
/// For a = b ⊕ c where each variable is 8 bits (cluster size = 8),
/// we use WHT (Walsh-Hadamard Transform = FHT) for efficient message passing.
///
/// The key insight: XOR in bit space = convolution in Walsh space
fn cluster_xor_example() {
    println!("Example: Clustered XOR Factor (8 bits)");
    println!("=======================================");
    println!("Constraint: a = b ⊕ c (8-bit clusters)\n");

    let clustersize = 8;
    let numvalues = 1 << clustersize; // 256

    // Incoming messages (probability distributions)
    // These would come from neighboring nodes in the factor graph
    let mut msg_b = Array1::from(vec![0.0; numvalues]);
    let mut msg_c = Array1::from(vec![0.0; numvalues]);

    // Initialize with some example distributions
    // msg_b: slightly biased towards 0x42
    for i in 0..numvalues {
        msg_b[i] = if i == 0x42 { 2.0 } else { 1.0 };
    }
    msg_b /= msg_b.sum(); // Normalize

    // msg_c: slightly biased towards 0x24
    for i in 0..numvalues {
        msg_c[i] = if i == 0x24 { 2.0 } else { 1.0 };
    }
    msg_c /= msg_c.sum(); // Normalize

    println!("Input message b: peaked at 0x{:02X}", 0x42);
    println!("Input message c: peaked at 0x{:02X}", 0x24);
    println!();

    // Transform to Walsh domain
    let mut msg_b_walsh = msg_b.clone();
    let mut msg_c_walsh = msg_c.clone();

    msg_b_walsh.fht_inplace().unwrap();
    msg_c_walsh.fht_inplace().unwrap();

    println!("Transformed to Walsh domain (WHT applied)");

    // In Walsh domain, XOR becomes pointwise multiplication
    let mut msg_a_walsh = &msg_b_walsh * &msg_c_walsh;

    println!("Computed XOR via pointwise multiplication");

    // Transform back to probability domain
    msg_a_walsh.fht_inplace().unwrap();

    // Scale (FHT is orthogonal but not normalized)
    msg_a_walsh /= numvalues as f64;

    // Normalize to probability
    msg_a_walsh /= msg_a_walsh.sum();

    println!("Transformed back to probability domain");
    println!();

    // The result should be peaked at 0x42 ⊕ 0x24 = 0x66
    let expected = 0x42 ^ 0x24;
    let peak_idx = msg_a_walsh
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap();

    println!("Expected peak: 0x{:02X}", expected);
    println!("Actual peak:   0x{:02X}", peak_idx);
    println!("Peak probability: {:.4}", msg_a_walsh[peak_idx]);

    if peak_idx == expected {
        println!("✓ Correct! XOR constraint satisfied");
    }

    println!("\n{}\n", "=".repeat(60));
}

/// Example: Parity computation
///
/// Computing parity of 5 values (each 8 bits) using WHT
/// This is used in Keccak theta layer
fn cluster_parity_example() {
    println!("Example: Clustered Parity Factor (Keccak Theta)");
    println!("================================================");
    println!("Constraint: p = a₀ ⊕ a₁ ⊕ a₂ ⊕ a₃ ⊕ a₄ (5-way XOR)\n");

    let clustersize = 8;
    let numvalues = 1 << clustersize; // 256

    // 5 incoming messages (one from each column in Keccak)
    let mut messages = Vec::new();
    let values = vec![0x12, 0x34, 0x56, 0x78, 0x9A];

    for &val in &values {
        let mut msg = Array1::from(vec![1.0; numvalues]);
        msg[val] = 10.0; // Peak at specific value
        msg /= msg.sum();
        messages.push(msg);
    }

    println!("Input messages peaked at:");
    for (i, &val) in values.iter().enumerate() {
        println!("  a{} = 0x{:02X}", i, val);
    }
    println!();

    // Transform all to Walsh domain
    let walsh_messages: Vec<Array1<f64>> = messages
        .iter()
        .map(|msg| {
            let mut w = msg.clone();
            w.fht_inplace().unwrap();
            w
        })
        .collect();

    println!("Transformed all {} messages to Walsh domain", walsh_messages.len());

    // Multiply all Walsh transforms (= multi-way XOR)
    let mut result_walsh = walsh_messages[0].clone();
    for msg in &walsh_messages[1..] {
        result_walsh = &result_walsh * msg;
    }

    println!("Computed 5-way XOR via pointwise multiplication");

    // Transform back
    result_walsh.fht_inplace().unwrap();
    result_walsh /= numvalues as f64;
    result_walsh /= result_walsh.sum();

    println!("Transformed back to probability domain");
    println!();

    // Expected: 0x12 ⊕ 0x34 ⊕ 0x56 ⊕ 0x78 ⊕ 0x9A
    let expected = values.iter().fold(0, |acc, &x| acc ^ x);
    let peak_idx = result_walsh
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap();

    println!("Expected parity: 0x{:02X}", expected);
    println!("Actual peak:     0x{:02X}", peak_idx);
    println!("Peak probability: {:.4}", result_walsh[peak_idx]);

    if peak_idx == expected {
        println!("✓ Correct! Parity constraint satisfied");
    }

    println!("\n{}\n", "=".repeat(60));
}

/// This is how it would be used in ClusterFactorLinear::f2n()
#[allow(dead_code)]
fn cluster_factor_linear_f2n_example() {
    // Pseudocode for ClusterFactorLinear::f2n()
    //
    // fn f2n(&mut self) {
    //     let incoming = self.gather_incoming();
    //     let num_edges = self.edges.len();
    //     let n = 2^clustersize;
    //
    //     // Transform all to Walsh domain
    //     let mut walsh: Vec<Array1<f64>> = incoming.iter()
    //         .map(|msg| {
    //             let mut w = msg.clone();
    //             w.fht_inplace().unwrap();
    //             w
    //         })
    //         .collect();
    //
    //     // For each outgoing edge i
    //     for i in 0..num_edges {
    //         // Product of all EXCEPT i-th message
    //         let mut out_walsh = Array1::ones(n);
    //         for (j, msg) in walsh.iter().enumerate() {
    //             if j != i {
    //                 out_walsh = &out_walsh * msg;
    //             }
    //         }
    //
    //         // Transform back
    //         out_walsh.fht_inplace().unwrap();
    //         out_walsh /= n as f64;
    //
    //         // Normalize and store
    //         self.edges[i].m2n = out_walsh / out_walsh.sum();
    //     }
    // }
}
