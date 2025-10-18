# Timing Side-Channel Protection for Two-Pointer Algorithms

This project provides implementations of common two-pointer algorithms with protections against timing side-channel attacks. These implementations ensure that execution time does not leak information about the data being processed.

## Overview

Two-pointer algorithms are efficient techniques for solving array and string problems. However, traditional implementations can be vulnerable to timing side-channel attacks where attackers measure execution times to infer information about secret data.

This project addresses these vulnerabilities by implementing constant-time operations and ensuring consistent execution patterns regardless of input data.

## Protection Mechanisms

1. **Constant-time comparisons** using the `subtle` crate
2. **Avoiding early exits** based on secret data
3. **Consistent iteration counts** regardless of input
4. **Dummy operations** to mask actual computation times

## Implemented Algorithms

### 1. Secure Two Sum
Finds two numbers in a sorted array that sum to a target value without leaking timing information.

### 2. Secure String Comparison
Compares two strings in constant time to prevent timing attacks.

### 3. Secure Array Search
Searches for a target value in a sorted array with consistent timing.

### 4. Secure Sorted Intersection
Finds the intersection of two sorted arrays without timing leaks.

## Security Analysis

### Information Leakage / Timing Side-Channels

Two-pointer comparisons and early exits may leak timing differences if the data being compared is secret (e.g., comparing secrets byte-by-byte). Attackers measure times to infer info.

Our implementations prevent this by:
- Using constant-time comparison operations
- Ensuring all execution paths take the same amount of time
- Avoiding early exits based on secret data comparisons
- Performing consistent numbers of iterations regardless of input

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
timing-sidechannel-protection = { path = "./timing-sidechannel-protection" }
```

Then use the functions:

```rust
use timing_sidechannel_protection::{
    secure_two_sum,
    secure_string_compare,
    secure_array_search,
    secure_sorted_intersection,
};

// Secure two sum
let nums = [2, 7, 11, 15];
let result = secure_two_sum(&nums, 9);

// Secure string comparison
let equal = secure_string_compare("secret123", "secret123");

// Secure array search
let index = secure_array_search(&[1, 2, 3, 4, 5], 3);

// Secure sorted intersection
let intersection = secure_sorted_intersection(&[1, 2, 3], &[2, 3, 4]);
```

## Testing

The project includes comprehensive tests:

1. **Unit Tests** - Basic functionality verification
2. **Property Tests** - Testing mathematical properties and edge cases
3. **Attack-Defense Tests** - Simulating timing attacks and verifying resistance

Run tests with:

```bash
cargo test
```

## Benchmarking

The project includes benchmarks to verify that timing protection doesn't significantly impact performance:

```bash
cargo bench
```

## License

MIT