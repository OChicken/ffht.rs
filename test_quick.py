#!/usr/bin/env python3
"""
Python test corresponding to test_quick.c
Tests the FFHT Python wrapper to verify C implementation is correct
"""

import numpy as np
import ffht

def test_inplace():
    """Test in-place FHT (corresponds to test_inplace() in test_quick.c)"""
    print("\ntest_inplace")

    # Same test data as C: {1.0, -1.0, 1.0, -1.0}
    data = np.array([1.0, -1.0, 1.0, -1.0], dtype=np.float32)

    print(f"Input:  {data}")

    # In-place transform (modifies data)
    ffht.fht(data)

    print(f"Output: {data}")

    # Verify the results
    # For input [1, -1, 1, -1] with log_n=2, what should the output be?
    # Let's see what the C test outputs
    return data

def test_inplace_copy():
    """
    Test that mimics out-of-place by copying first
    (Python wrapper only has in-place, so we simulate OOP)
    """
    print("\ntest_inplace_copy (simulating OOP)")

    input_data = np.array([1.0, -1.0, 1.0, -1.0], dtype=np.float32)
    output = input_data.copy()  # Create a copy

    print(f"Input:  {input_data}")
    print(f"Output before: {output}")

    # Transform the copy (simulates out-of-place)
    ffht.fht(output)

    print(f"Output after:  {output}")

    # Verify input unchanged
    assert np.array_equal(input_data, [1.0, -1.0, 1.0, -1.0])

    return output

def test_double():
    """Test with float64 (double precision)"""
    print("\ntest_double")

    data = np.array([1.0, -1.0, 1.0, -1.0], dtype=np.float64)

    print(f"Input:  {data}")

    ffht.fht(data)

    print(f"Output: {data}")

    return data

def test_larger_size():
    """Test with size 8"""
    print("\ntest_larger_size")

    data = np.array([1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0], dtype=np.float32)

    print(f"Input:  {data}")

    ffht.fht(data)

    print(f"Output: {data}")

    return data

def main():
    print("=" * 60)
    print("FFHT Python Test (corresponding to test_quick.c)")
    print("=" * 60)

    result1 = test_inplace()
    result2 = test_inplace_copy()
    result3 = test_double()
    result4 = test_larger_size()

    print("\n" + "=" * 60)
    print("Summary:")
    print("=" * 60)
    print(f"test_inplace result:      {result1}")
    print(f"test_inplace_copy result: {result2}")
    print(f"test_double result:       {result3}")
    print(f"test_larger_size result:  {result4}")

    # Check that float32 and float64 give same results (within precision)
    print("\nVerifying float32 vs float64 consistency:")
    if np.allclose(result1, result3, rtol=1e-6):
        print("✓ Results match within tolerance")
    else:
        print("✗ Results differ!")
        print(f"  Difference: {result1 - result3}")

if __name__ == "__main__":
    main()
