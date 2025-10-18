# Infinite Loop Protection for Two-Pointer Algorithms

This project demonstrates secure implementations of two-pointer algorithms with a specific focus on preventing infinite loops and high CPU usage vulnerabilities.

## Security Focus

The implementations in this project are designed with multiple layers of protection against infinite loops and algorithmic DoS attacks:

1. **Time-based Termination**: All algorithms have time limits to prevent hanging
2. **Iteration Counting**: Loop iterations are counted and limited
3. **Input Size Validation**: Large inputs that could cause performance issues are rejected
4. **Proper Termination Conditions**: Algorithms are designed to always terminate correctly

## Attack Vector: Infinite Loops / High CPU Usage

### Attack Description
Broken move rules (forget to advance a pointer) may create infinite loops. An attacker can feed many crafted requests that hang the process (algorithmic DoS).

Even correct two-pointer code can be exploited for complexity DoS: if algorithm is assumed linear for average input but worst-case becomes expensive when attacker controls input distribution (e.g., sliding-window + dynamic map growth).

### Mitigation Strategies Implemented

#### 1. Time-based Loop Termination
All implementations include time limits to prevent algorithmic DoS:

```rust
let start_time = Instant::now();
let time_limit = Duration::from_millis(100);

while condition {
    if start_time.elapsed() > time_limit {
        return None; // Timeout - potential DoS attack
    }
    // Algorithm logic...
}
```

#### 2. Iteration Count Limiting
Loop iterations are counted and limited to prevent unbounded execution:

```rust
let max_iterations = nums.len() / 2 + 1;
let mut iteration_count = 0;

while condition {
    iteration_count += 1;
    if iteration_count > max_iterations {
        return None; // Too many iterations - potential infinite loop
    }
    // Algorithm logic...
}
```

#### 3. Input Size Validation
Large inputs that could cause performance issues are rejected:

```rust
if nums.len() > 1000000 { // Limit to 1 million elements
    return None;
}
```

#### 4. Memory Growth Limiting
For algorithms that use auxiliary data structures, size limits prevent memory exhaustion:

```rust
let max_map_size = 10000; // Limit distinct characters
if char_index_map.len() > max_map_size {
    return max_length; // Return current best result on memory limit
}
```

## Functions Implemented

### `two_sum_sorted` (Opposite-Ends Pattern)
Finds two numbers in a sorted array that sum to a target value with comprehensive infinite loop protection.

### `find_middle` (Same-Direction Pattern)
Finds the middle node of a linked list with cycle detection and iteration limiting.

### `longest_substring_without_repeating` (Window Bounds Pattern)
Finds the longest substring without repeating characters with HashMap size limiting.

### `merge_sorted_arrays` (Bidirectional Merge Pattern)
Merges two sorted arrays with time and iteration limits.

## Testing

The project includes comprehensive tests that verify:
- Normal operation of algorithms
- Edge case handling
- Security measures effectiveness
- Proper timeout behavior

## Running Tests

To run the tests:

```bash
cargo test
```

## Key Security Features

1. **Timeout Protection**: All algorithms have time limits
2. **Iteration Limits**: Prevents unbounded execution
3. **Input Validation**: Rejects potentially harmful large inputs
4. **Memory Protection**: Limits auxiliary data structure growth
5. **Graceful Degradation**: Returns partial results on timeout rather than hanging

This project serves as a reference for implementing secure two-pointer algorithms that are resistant to infinite loop and high CPU usage vulnerabilities.