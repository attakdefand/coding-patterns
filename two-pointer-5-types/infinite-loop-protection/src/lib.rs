//! Infinite Loop Protection for Two-Pointer Algorithms
//!
//! This library demonstrates secure implementations of two-pointer algorithms
//! with specific focus on preventing infinite loops and high CPU usage.

pub mod two_pointer;

// Re-export the main functions for easier access
pub use two_pointer::{
    find_middle, longest_substring_without_repeating, merge_sorted_arrays, two_sum_sorted,
};
