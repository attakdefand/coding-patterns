# Integer Overflow Protection for Two-Pointer Algorithms

This project demonstrates secure implementations of two-pointer algorithms with a specific focus on protecting against integer overflow and underflow caused by using signed integers or wrong types when computing indices, sums, or sizes.

## Security Focus

The implementations in this project are designed with multiple layers of protection against integer overflow and underflow:

1. **Checked Arithmetic Operations**: Using Rust's `checked_add`, `checked_sub`, and `checked_mul` methods
2. **Larger Intermediate Types**: Using `i64` for intermediate calculations to prevent overflow
3. **Safe Error Handling**: Graceful degradation when overflow/underflow occurs
4. **Input Validation**: Checking assumptions about input data
5. **Boundary Checking**: Preventing out-of-bounds access

## Attack Vector: Integer Overflow/Underflow

### Attack Description
Using signed integers or wrong types causes overflow when computing indices, sums, or sizes (e.g., sum comparisons) — attacker can cause wraparound and bypass checks.

### Example Vulnerable Code
```rust
// Vulnerable implementation without integer overflow protection
fn vulnerable_two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let sum = nums[left] + nums[right]; // Vulnerable to overflow!
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}
```

## Mitigation Strategies Implemented

### 1. Checked Arithmetic Operations
All implementations use checked arithmetic to detect overflow:

```rust
let complement = match target.checked_sub(num) {
    Some(val) => val,
    None => continue, // Handle overflow gracefully
};
```

### 2. Larger Intermediate Types
Using larger types for intermediate calculations:

```rust
let mut current_sum: i64 = 0; // Use larger type to prevent overflow
```

### 3. Safe Error Handling
Graceful handling of overflow conditions:

```rust
let current_area = match width.checked_mul(current_height) {
    Some(area) => area,
    None => i64::MAX // Handle overflow by using a large value
};
```

### 4. Input Validation
Checking input assumptions:

```rust
if nums.len() < 2 {
    return None;
}
```

## Functions Implemented

### `two_sum_safe` (Opposite-Ends Pattern)
- Uses hash-based approach to avoid pointer arithmetic overflow
- Checked arithmetic for sum calculations
- Handles extreme value cases safely

### `three_sum_safe` (Opposite-Ends Pattern)
- Combination of sorting and hash-based approach
- Checked arithmetic for sum calculations
- Handles extreme value cases safely

### `container_with_most_water_safe` (Window Bounds Pattern)
- Uses `i64` for area calculations to prevent overflow
- Checked arithmetic for width and area calculations
- Safe handling of large height values

### `find_subarray_sum_safe` (Window Bounds Pattern)
- Uses `i64` for sum calculations to prevent overflow
- Checked arithmetic for sum operations
- Safe handling of extreme values

## Testing

The project includes comprehensive tests that verify:
- Normal operation with valid inputs
- Proper handling of extreme values
- Edge case handling
- Security measures effectiveness

## Running Tests

To run the tests:

```bash
cargo test
```

## Key Security Features

1. **Checked Arithmetic**: All arithmetic operations use checked variants
2. **Larger Types**: Intermediate calculations use larger types to prevent overflow
3. **Graceful Degradation**: Functions handle overflow conditions gracefully
4. **Clear Feedback**: Informative error reporting
5. **Predictable Behavior**: Consistent results regardless of input values

This project serves as a reference for implementing secure two-pointer algorithms that are resistant to integer overflow and underflow attacks.