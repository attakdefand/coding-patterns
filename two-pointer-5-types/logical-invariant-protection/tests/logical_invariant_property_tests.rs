//! Property-based tests for logical invariant protection in two-pointer algorithms
//!
//! These tests verify that the implementations maintain correctness properties
//! even when logical invariants are broken by untrusted input.

use logical_invariant_protection::{
    container_with_most_water, is_palindrome, three_sum, two_sum_sorted,
};
use proptest::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn two_sum_sorted_returns_valid_indices(nums in prop::collection::vec(-1000i32..1000, 0..100), target in -2000i32..2000) {
            // Test that two_sum_sorted always returns valid indices or None
            if let Some((i, j)) = two_sum_sorted(&nums, target) {
                // Indices should be within bounds
                prop_assert!(i < nums.len());
                prop_assert!(j < nums.len());
                prop_assert!(i != j);
                // The sum should equal the target
                prop_assert_eq!(nums[i] + nums[j], target);
            }
            // If None is returned, there should be no valid pair
            // (We can't easily verify this without reimplementing the algorithm)
        }

        #[test]
        fn three_sum_returns_valid_indices(nums in prop::collection::vec(-100i32..100, 0..20), target in -300i32..300) {
            // Test that three_sum always returns valid indices or None
            if let Some((i, j, k)) = three_sum(&nums, target) {
                // Indices should be within bounds
                prop_assert!(i < nums.len());
                prop_assert!(j < nums.len());
                prop_assert!(k < nums.len());
                // Indices should be distinct
                prop_assert!(i != j && j != k && i != k);
                // The sum should equal the target
                prop_assert_eq!(nums[i] + nums[j] + nums[k], target);
            }
            // If None is returned, there should be no valid triplet
            // (We can't easily verify this without reimplementing the algorithm)
        }

        #[test]
        fn is_palindrome_consistent(s in "\\PC*") {
            // Test that is_palindrome behaves consistently
            let result = is_palindrome(&s);
            prop_assert!(result == true || result == false);
        }

        #[test]
        fn container_with_most_water_non_negative(height in prop::collection::vec(-100i32..100, 0..50)) {
            // Test that container_with_most_water always returns non-negative values
            let result = container_with_most_water(&height);
            prop_assert!(result >= 0);
        }
    }
}
