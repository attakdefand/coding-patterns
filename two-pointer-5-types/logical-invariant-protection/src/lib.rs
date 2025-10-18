//! Logical Invariant Protection for Two-Pointer Algorithms
//!
//! This library demonstrates secure implementations of two-pointer algorithms
//! with specific focus on protecting against broken logical invariants from untrusted input.

pub mod two_pointer;

// Re-export the main functions for easier access
pub use two_pointer::{
    two_sum_sorted,
    is_palindrome,
    three_sum,
    container_with_most_water,
};