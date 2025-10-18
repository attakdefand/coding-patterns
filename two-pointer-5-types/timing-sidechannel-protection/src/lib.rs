//! Timing Side-Channel Protection for Two-Pointer Algorithms
//!
//! This crate provides implementations of common two-pointer algorithms with protections
//! against timing side-channel attacks. These implementations ensure that execution time
//! does not leak information about the data being processed.
//!
//! # Protection Mechanisms
//!
//! 1. Constant-time comparisons using the `subtle` crate
//! 2. Avoiding early exits based on secret data
//! 3. Ensuring consistent iteration counts regardless of input
//! 4. Using dummy operations to mask actual computation

pub mod two_pointer;
pub mod timing_utils;

// Re-export the main functions for easier access
pub use two_pointer::{
    secure_two_sum,
    secure_string_compare,
    secure_array_search,
    secure_sorted_intersection,
};