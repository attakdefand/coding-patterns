# Logical Invariant Protection for Two-Pointer Algorithms - Summary

This document summarizes the implementation of logical invariant protection for two-pointer algorithms, specifically addressing the security concern of "Logical invariants broken by untrusted input."

## Problem Addressed

Many two-pointer solutions assume sorted input or monotonic properties. If an attacker supplies unsorted or maliciously arranged data, the algorithm can return incorrect results or behave unpredictably.

## Implementation Strategy

We implemented multiple layers of protection:

### 1. Input Validation
All functions validate their assumptions about input data before proceeding with the main algorithm.

```rust
fn is_sorted_non_decreasing(nums: &[i32]) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}
```

### 2. Fallback Mechanisms
When assumptions are violated, alternative algorithms are used that don't rely on those assumptions.

For `two_sum_sorted`:
```rust
if !is_sorted_non_decreasing(nums) {
    // Handle unsorted input by using a hash-based approach
    return two_sum_unsorted_fallback(nums, target);
}
```

For `three_sum`:
```rust
if !is_sorted_non_decreasing(nums) {
    // Handle unsorted input by sorting first
    let mut sorted_nums: Vec<(i32, usize)> = nums.iter().enumerate().map(|(i, &val)| (val, i)).collect();
    sorted_nums.sort_by_key(|&(val, _)| val);
    // ... process with sorted data and map back to original indices
}
```

### 3. Safe Handling of Edge Cases
Robust handling of negative values, empty inputs, and boundary conditions.

### 4. Clear Error Reporting
Functions provide clear feedback when inputs violate assumptions.

## Functions Protected

### `two_sum_sorted` (Opposite-Ends Pattern)
- Validates sorted input assumption
- Falls back to hash-based approach for unsorted input
- Handles integer overflow safely

### `is_palindrome` (Opposite-Ends Pattern)
- Robust character processing
- Safe handling of Unicode characters
- Proper alphanumeric checking

### `three_sum` (Opposite-Ends Pattern)
- Validates sorted input assumption
- Preprocesses unsorted input by sorting
- Maps results back to original indices

### `container_with_most_water` (Window Bounds Pattern)
- Handles negative height values
- Protects against integer overflow
- Maintains logical invariants during pointer movement

## Test Coverage

### Unit Tests
- Normal operation with valid inputs
- Proper handling of unsorted inputs
- Edge case handling

### Property-Based Tests
- Randomized testing to verify correctness properties
- Boundary condition testing
- Stress testing with various input sizes

### Attack-Defense Tests
- Specific tests simulating attacker behavior
- Validation of fallback mechanisms
- Verification of safe handling of malicious inputs

## Security Benefits

1. **Assumption Validation**: All functions check their input assumptions
2. **Graceful Degradation**: Alternative algorithms when assumptions are violated
3. **Safe Error Handling**: Proper handling of invalid inputs
4. **Clear Feedback**: Informative error reporting
5. **Predictable Behavior**: Consistent results regardless of input arrangement

## Running Tests

To run all tests:
```bash
cargo test
```

To run specific test suites:
```bash
# Unit tests
cargo test --test logical_invariant_unit_tests

# Property-based tests
cargo test --test logical_invariant_property_tests

# Attack-defense tests
cargo test --test logical_invariant_attack_defense_tests
```