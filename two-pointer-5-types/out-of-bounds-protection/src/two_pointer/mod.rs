//! Secure implementations of common two-pointer algorithms with focus on out-of-bounds protection
//!
//! This module contains implementations of various two-pointer algorithms with
//! specific security considerations to prevent out-of-bounds access and memory corruption.

pub mod opposite_ends;

// Re-export the main functions
pub use opposite_ends::{
    two_sum_sorted,
    is_palindrome,
    reverse_array,
    three_sum,
};