//! Race Condition & TOCTOU Protection for Two-Pointer Algorithms
//!
//! This crate provides implementations of common two-pointer algorithms with protections
//! against race conditions and Time-of-Check to Time-of-Use (TOCTOU) vulnerabilities.
//! These implementations ensure thread-safe access to shared data and prevent
//! inconsistent states in concurrent environments.
//!
//! # Protection Mechanisms
//!
//! 1. Thread-safe data structures using RwLock and Mutex
//! 2. Atomic operations for shared counters and flags
//! 3. Proper synchronization primitives to prevent data races
//! 4. Lock ordering to prevent deadlocks
//! 5. Clone-on-write patterns for safe concurrent access

pub mod concurrent_utils;
pub mod race_protection;
pub mod two_pointer;

// Re-export the main functions for easier access
pub use race_protection::{ConcurrentTwoPointer, TwoPointerState};
pub use two_pointer::{
    concurrent_array_search, concurrent_sorted_intersection, concurrent_string_compare,
    concurrent_two_sum,
};
