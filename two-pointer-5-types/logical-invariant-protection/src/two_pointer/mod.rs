//! Secure implementations of common two-pointer algorithms with focus on logical invariant protection
//!
//! This module contains implementations of various two-pointer algorithms with
//! specific security considerations to prevent issues when logical invariants are broken by untrusted input.

pub mod opposite_ends;
pub mod window_bounds;

// Re-export the main functions
pub use opposite_ends::{is_palindrome, three_sum, two_sum_sorted};
pub use window_bounds::container_with_most_water;
