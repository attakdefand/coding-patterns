//! Out-of-Bounds Protection for Two-Pointer Algorithms
//!
//! This library demonstrates secure implementations of two-pointer algorithms
//! with specific focus on preventing out-of-bounds access and memory corruption.

pub mod two_pointer;

// Re-export the main functions for easier access
pub use two_pointer::{
    two_sum_sorted,
    is_palindrome,
    reverse_array,
    three_sum,
};
