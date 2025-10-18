# Race Condition & TOCTOU Protection for Two-Pointer Algorithms

This project provides implementations of common two-pointer algorithms with protections against race conditions and Time-of-Check to Time-of-Use (TOCTOU) vulnerabilities. These implementations ensure thread-safe access to shared data and prevent inconsistent states in concurrent environments.

## Overview

Two-pointer algorithms are efficient techniques for solving array and string problems. However, in concurrent environments, traditional implementations can be vulnerable to race conditions and TOCTOU attacks where multiple threads access and modify shared data simultaneously, leading to inconsistent states and potential security vulnerabilities.

This project addresses these vulnerabilities by implementing thread-safe operations and ensuring consistent execution patterns regardless of concurrent access.

## Protection Mechanisms

1. **Thread-safe data structures** using RwLock and Mutex
2. **Atomic operations** for shared counters and flags
3. **Proper synchronization primitives** to prevent data races
4. **Lock ordering** to prevent deadlocks
5. **Clone-on-write patterns** for safe concurrent access
6. **TOCTOU protection** through validation on each access

## Implemented Algorithms

### 1. Concurrent Two Sum
Finds two numbers in a sorted array that sum to a target value in a thread-safe manner.

### 2. Concurrent String Comparison
Compares two strings concurrently without race conditions.

### 3. Concurrent Array Search
Searches for a target value in a sorted array with thread safety.

### 4. Concurrent Sorted Intersection
Finds the intersection of two sorted arrays without data races.

## Security Analysis

### Race Conditions & TOCTOU (Concurrent Code)

**Description**: If two-pointer state is mutated concurrently without synchronization, attacker-controlled events (I/O, signals, other threads) may cause inconsistent reads or memory errors.

**Impact**: Data corruption, inconsistent states, potential security vulnerabilities.

**Attack Vector**: 
1. Multiple threads accessing shared two-pointer state
2. Modification of data between check and use operations
3. Inconsistent reads due to lack of synchronization

Our implementations prevent this by:
- Using thread-safe data structures (RwLock, Mutex)
- Employing atomic operations for shared state
- Validating data consistency on each access (TOCTOU protection)
- Properly synchronizing access to shared resources

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
race-condition-protection = { path = "./race-condition-protection" }
```

Then use the functions:

```rust
use race_condition_protection::{
    concurrent_two_sum,
    concurrent_string_compare,
    concurrent_array_search,
    concurrent_sorted_intersection,
    ConcurrentTwoPointer,
};

// Concurrent two sum
let nums = [2, 7, 11, 15];
let result = concurrent_two_sum(&nums, 9);

// Concurrent string comparison
let equal = concurrent_string_compare("secret", "secret");

// Concurrent array search
let index = concurrent_array_search(&[1, 2, 3, 4, 5], 3);

// Concurrent sorted intersection
let intersection = concurrent_sorted_intersection(&[1, 2, 3], &[2, 3, 4]);

// Advanced concurrent two-pointer
let data = vec![2, 7, 11, 15];
let algo = ConcurrentTwoPointer::new(data);
let result = algo.find_sum(9);
```

## Testing

The project includes comprehensive tests:

1. **Unit Tests** - Basic functionality verification
2. **Property Tests** - Testing mathematical properties and edge cases
3. **Attack-Defense Tests** - Simulating race conditions and verifying resistance

Run tests with:

```bash
cargo test
```

## Benchmarking

The project includes benchmarks to verify that race condition protection doesn't significantly impact performance:

```bash
cargo bench
```

## Dependencies

- `parking_lot = "0.12"` - For efficient synchronization primitives
- `crossbeam = "0.8"` - For additional concurrent utilities
- `rayon = "1.7"` - For parallel processing capabilities
- `criterion = "0.5"` - For performance benchmarking

## License

MIT