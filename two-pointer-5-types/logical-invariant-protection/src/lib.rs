//! Logical Invariant Protection for Two-Pointer Algorithms
//!
//! This library demonstrates secure implementations of two-pointer algorithms
//! with specific focus on protecting against broken logical invariants from untrusted input.

pub mod two_pointer;

// Re-export the main functions for easier access
pub use two_pointer::{container_with_most_water, is_palindrome, three_sum, two_sum_sorted};
