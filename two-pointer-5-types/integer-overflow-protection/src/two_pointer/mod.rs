//! Secure implementations of common two-pointer algorithms with focus on integer overflow protection
//!
//! This module contains implementations of various two-pointer algorithms with
//! specific security considerations to prevent integer overflow and underflow issues.

pub mod opposite_ends;
pub mod window_bounds;

// Re-export the main functions
pub use opposite_ends::{find_subarray_sum_safe, three_sum_safe, two_sum_safe};
pub use window_bounds::container_with_most_water_safe;
