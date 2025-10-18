//! Secure implementations of common two-pointer algorithms with focus on infinite loop protection
//!
//! This module contains implementations of various two-pointer algorithms with
//! specific security considerations to prevent infinite loops and high CPU usage.

pub mod bidirectional_merge;
pub mod opposite_ends;
pub mod same_direction;
pub mod window_bounds;

// Re-export the main functions
pub use bidirectional_merge::merge_sorted_arrays;
pub use opposite_ends::two_sum_sorted;
pub use same_direction::find_middle;
pub use window_bounds::longest_substring_without_repeating;
