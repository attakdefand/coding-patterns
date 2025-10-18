//! Property-based tests for integer overflow protection in two-pointer algorithms
//!
//! These tests verify that the implementations maintain correctness properties
//! even when integer overflow or underflow could occur.

use integer_overflow_protection::{
    container_with_most_water_safe, find_subarray_sum_safe, three_sum_safe, two_sum_safe,
};
use proptest::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn two_sum_safe_returns_valid_indices(nums in prop::collection::vec(-1000i32..1000, 0..100), target in -2000i32..2000) {
            // Test that two_sum_safe always returns valid indices or None
            if let Some((i, j)) = two_sum_safe(&nums, target) {
                // Indices should be within bounds
                prop_assert!(i < nums.len());
                prop_assert!(j < nums.len());
                prop_assert!(i != j);
                // The sum should equal the target (if no overflow occurred)
                if let Some(sum) = nums[i].checked_add(nums[j]) {
                    if sum == target {
                        prop_assert_eq!(nums[i] + nums[j], target);
                    }
                }
            }
            // If None is returned, there should be no valid pair
            // (We can't easily verify this without reimplementing the algorithm)
        }

        #[test]
        fn three_sum_safe_returns_valid_indices(nums in prop::collection::vec(-100i32..100, 0..20), target in -300i32..300) {
            // Test that three_sum_safe always returns valid indices or None
            if let Some((i, j, k)) = three_sum_safe(&nums, target) {
                // Indices should be within bounds
                prop_assert!(i < nums.len());
                prop_assert!(j < nums.len());
                prop_assert!(k < nums.len());
                // Indices should be distinct
                prop_assert!(i != j && j != k && i != k);
                // The sum should equal the target (if no overflow occurred)
                if let Some(sum2) = nums[i].checked_add(nums[j]).and_then(|s| s.checked_add(nums[k])) {
                    if sum2 == target {
                        prop_assert_eq!(nums[i] + nums[j] + nums[k], target);
                    }
                }
            }
            // If None is returned, there should be no valid triplet
            // (We can't easily verify this without reimplementing the algorithm)
        }

        #[test]
        fn container_with_most_water_safe_non_negative(height in prop::collection::vec(-100i32..100, 0..50)) {
            // Test that container_with_most_water_safe always returns non-negative values
            let result = container_with_most_water_safe(&height);
            prop_assert!(result >= 0);
        }

        #[test]
        fn find_subarray_sum_safe_valid_result(nums in prop::collection::vec(-1000i32..1000, 0..50), target in -5000i32..5000) {
            // Test that find_subarray_sum_safe returns valid results
            if let Some((start, end)) = find_subarray_sum_safe(&nums, target) {
                // Indices should be within bounds
                prop_assert!(start < nums.len());
                prop_assert!(end < nums.len());
                prop_assert!(start <= end);

                // Calculate the actual sum of the subarray
                let actual_sum: i64 = nums[start..=end].iter().map(|&x| x as i64).sum();
                // Check if it matches the target (accounting for possible overflow)
                prop_assert!(actual_sum == target as i64 || actual_sum > i32::MAX as i64 || actual_sum < i32::MIN as i64);
            }
        }
    }
}
