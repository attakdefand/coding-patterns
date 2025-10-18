# coding-patterns

This repository contains implementations of various coding patterns with a focus on security and attack-defense strategies. The project demonstrates secure implementations of algorithms and data structures while showing how they can be attacked and defended against.

## Repository Structure

- `two-pointer-project`: Main implementation of two-pointer algorithms with security considerations
- `two-pointer-5-types`: Documentation and specialized implementations of 5 types of two-pointer patterns
- `fast-n-slow-pointer`: Fast and slow pointer patterns (currently empty)

## Two-Pointer Patterns Covered

1. **Opposite-Ends (Converging)**
2. **Same-Direction (Fast/Slow; "Tortoise–Hare")**
3. **Window Bounds (Left/Right)**
4. **Partitioning (Read/Write or Hoare/Lomuto-style)**
5. **Bidirectional Merge / Set Ops**

## Security Focus

Each implementation includes security considerations to prevent:
- Out-of-bounds access
- Integer overflow/underflow
- Timing attacks
- Resource exhaustion

## Out-of-Bounds Protection

The `two-pointer-5-types/out-of-bounds-protection` directory contains specialized implementations focused on preventing out-of-bounds access and memory corruption vulnerabilities.

## Testing

Comprehensive test suites including:
- Unit tests
- Property-based tests
- Attack-defense tests