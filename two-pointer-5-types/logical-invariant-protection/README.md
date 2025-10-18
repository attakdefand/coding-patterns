# Logical Invariant Protection for Two-Pointer Algorithms

This project demonstrates secure implementations of two-pointer algorithms with a specific focus on protecting against broken logical invariants caused by untrusted input.

## Security Focus

The implementations in this project are designed with multiple layers of protection against logical invariants being broken by untrusted input:

1. **Input Validation**: Checking assumptions about input data
2. **Fallback Mechanisms**: Alternative algorithms for invalid inputs
3. **Safe Handling**: Graceful degradation when invariants are broken
4. **Error Reporting**: Clear feedback when inputs violate assumptions

## Attack Vector: Logical Invariants Broken by Untrusted Input

### Attack Description
Many two-pointer solutions assume sorted input or monotonic property. If the attacker supplies unsorted or maliciously arranged data, the algorithm can return incorrect results or behave unpredictably.

### Example Vulnerable Code
```rust
// Vulnerable implementation without logical invariant protection
fn vulnerable_two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Assumes nums is sorted, but what if it's not?
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;  // This only works if array is sorted!
        } else {
            right -= 1; // This only works if array is sorted!
        }
    }
    None
}
```

## Mitigation Strategies Implemented

### 1. Input Validation
All implementations check their assumptions about input data:

```rust
fn is_sorted_non_decreasing(nums: &[i32]) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}
```

### 2. Fallback Mechanisms
When assumptions are violated, alternative algorithms are used:

```rust
if !is_sorted_non_decreasing(nums) {
    // Handle unsorted input by using a hash-based approach
    return two_sum_unsorted_fallback(nums, target);
}
```

### 3. Safe Handling of Edge Cases
Proper handling of negative values, empty inputs, and boundary conditions:

```rust
if left_height < 0 || right_height < 0 {
    // Skip negative heights or handle them appropriately
    // ...
}
```

### 4. Clear Error Reporting
Functions provide clear feedback when inputs violate assumptions.

## Functions Implemented

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

## Testing

The project includes comprehensive tests that verify:
- Normal operation with valid inputs
- Proper handling of unsorted inputs
- Edge case handling
- Security measures effectiveness

Additional documentation:
- [Logical Invariant Protection Summary](LOGICAL_INVARIANT_PROTECTION_SUMMARY.md)
- [Attack Scenarios and Protection Mechanisms](ATTACK_SCENARIOS.md)

## Running Tests

To run the tests:

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

## Key Security Features

1. **Assumption Validation**: All functions check their input assumptions
2. **Graceful Degradation**: Alternative algorithms when assumptions are violated
3. **Safe Error Handling**: Proper handling of invalid inputs
4. **Clear Feedback**: Informative error reporting
5. **Predictable Behavior**: Consistent results regardless of input arrangement

This project serves as a reference for implementing secure two-pointer algorithms that are resistant to logical invariant violations from untrusted input.