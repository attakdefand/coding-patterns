# Out-of-Bounds Protection for Two-Pointer Algorithms

This project demonstrates secure implementations of two-pointer algorithms with a specific focus on preventing out-of-bounds access and memory corruption vulnerabilities.

## Security Focus

The implementations in this project are designed with multiple layers of protection against out-of-bounds access:

1. **Bounds Checking**: Every array access is preceded by bounds validation
2. **Safe Pointer Operations**: All pointer movements use safe arithmetic with overflow/underflow protection
3. **Input Validation**: Comprehensive validation of input parameters
4. **Edge Case Handling**: Special handling for empty arrays, single elements, and boundary conditions
5. **Double-Check Pattern**: Critical operations use redundant bounds checking

## Attack Vector: Out-of-Bounds / Memory Corruption

### Attack Description
Improper index checking can lead to accessing memory outside valid bounds, potentially causing:
- Memory corruption
- Use-after-free vulnerabilities
- Crashes
- Information disclosure

### Mitigation Strategies Implemented

#### 1. Proper Bounds Checking
All implementations include comprehensive bounds checking before accessing array elements:

```rust
// Example from two_sum_sorted function
if left >= nums.len() || right >= nums.len() {
    return None;
}
```

#### 2. Safe Pointer Initialization
Pointers are initialized with proper bounds checking:

```rust
let mut right = match nums.len().checked_sub(1) {
    Some(index) => index,
    None => return None, // Handle potential underflow
};
```

#### 3. Safe Pointer Movement
All pointer movements include underflow/overflow protection:

```rust
if right > 0 {
    right -= 1;
} else {
    return None;
}
```

#### 4. Input Validation
Functions validate input parameters to prevent invalid operations:

```rust
if nums.len() < 2 {
    return None;
}
```

## Testing

The project includes comprehensive tests that verify:
- Normal operation of algorithms
- Edge case handling (empty arrays, single elements, etc.)
- Security measures effectiveness
- Proper error handling

## Running Tests

To run the tests:

```bash
cargo test
```

## Key Security Features

1. **Double-Check Pattern**: Critical array accesses are validated twice
2. **Safe Arithmetic**: All index calculations use checked arithmetic operations
3. **Boundary Condition Handling**: Special cases for array boundaries are explicitly handled
4. **Early Termination**: Functions terminate safely when invalid states are detected
5. **Comprehensive Error Handling**: All possible error conditions are handled gracefully

This project serves as a reference for implementing secure two-pointer algorithms that are resistant to out-of-bounds access vulnerabilities.