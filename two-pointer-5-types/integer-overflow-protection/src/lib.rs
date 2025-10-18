//! Integer Overflow Protection for Two-Pointer Algorithms
//!
//! This library demonstrates secure implementations of two-pointer algorithms
//! with specific focus on protecting against integer overflow and underflow.

pub mod two_pointer;

// Re-export the main functions for easier access
pub use two_pointer::{
    container_with_most_water_safe, find_subarray_sum_safe, three_sum_safe, two_sum_safe,
};
