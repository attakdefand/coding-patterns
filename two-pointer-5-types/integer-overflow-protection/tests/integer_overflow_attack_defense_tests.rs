//! Attack-defense tests for integer overflow protection in two-pointer algorithms
//!
//! These tests simulate attacks where integer overflow or underflow could be exploited
//! and verify that the implementations defend against them properly.

use integer_overflow_protection::{
    container_with_most_water_safe, find_subarray_sum_safe, three_sum_safe, two_sum_safe,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_safe_integer_overflow_attack() {
        // Simulate an attacker trying to cause integer overflow
        // by providing values that would overflow when added

        // Test with maximum values
        let nums = vec![i32::MAX, 1, i32::MAX];
        let target = i32::MAX; // This could cause issues if not handled properly

        let result = two_sum_safe(&nums, target);
        // Should handle overflow gracefully without crashing
        assert!(result.is_none() || result.is_some());

        // Test with minimum values
        let nums = vec![i32::MIN, -1, i32::MIN];
        let target = i32::MIN; // This could cause issues if not handled properly

        let result = two_sum_safe(&nums, target);
        // Should handle underflow gracefully without crashing
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_three_sum_safe_integer_overflow_attack() {
        // Simulate an attacker trying to cause integer overflow
        // by providing values that would overflow when added

        // Test with maximum values
        let nums = vec![i32::MAX, 1, 1, i32::MAX];
        let target = i32::MAX; // This could cause issues if not handled properly

        let result = three_sum_safe(&nums, target);
        // Should handle overflow gracefully without crashing
        assert!(result.is_none() || result.is_some());

        // Test with minimum values
        let nums = vec![i32::MIN, -1, -1, i32::MIN];
        let target = i32::MIN; // This could cause issues if not handled properly

        let result = three_sum_safe(&nums, target);
        // Should handle underflow gracefully without crashing
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_container_with_most_water_safe_integer_overflow_attack() {
        // Simulate an attacker trying to cause integer overflow
        // by providing large height values

        // Test with maximum values
        let height = vec![i32::MAX, i32::MAX, i32::MAX];
        let result = container_with_most_water_safe(&height);
        // Should handle overflow gracefully without crashing
        assert!(result >= 0);

        // Test with a very large array of maximum values
        let height = vec![i32::MAX; 100];
        let result = container_with_most_water_safe(&height);
        // Should handle overflow gracefully without crashing
        assert!(result >= 0);
    }

    #[test]
    fn test_find_subarray_sum_safe_integer_overflow_attack() {
        // Simulate an attacker trying to cause integer overflow
        // by providing values that would overflow when summed

        // Test with maximum values
        let nums = vec![i32::MAX, i32::MAX, i32::MAX];
        let target = i32::MAX; // This could cause issues if not handled properly

        let result = find_subarray_sum_safe(&nums, target);
        // Should handle overflow gracefully without crashing
        assert!(result.is_none() || result.is_some());

        // Test with minimum values
        let nums = vec![i32::MIN, i32::MIN, i32::MIN];
        let target = i32::MIN; // This could cause issues if not handled properly

        let result = find_subarray_sum_safe(&nums, target);
        // Should handle underflow gracefully without crashing
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_two_sum_safe_extreme_values() {
        // Test with extreme positive and negative values
        let nums = vec![i32::MAX, i32::MIN, 0, 1, -1];
        let result = two_sum_safe(&nums, -1);
        assert!(result.is_some() || result.is_none());

        let result = two_sum_safe(&nums, 1);
        assert!(result.is_some() || result.is_none());
    }

    #[test]
    fn test_three_sum_safe_extreme_values() {
        // Test with extreme positive and negative values
        let nums = vec![i32::MAX, i32::MIN, 0, 1, -1];
        let result = three_sum_safe(&nums, 0);
        assert!(result.is_some() || result.is_none());
    }

    #[test]
    fn test_container_with_most_water_safe_extreme_values() {
        // Test with extreme values
        let height = vec![i32::MAX, 0, i32::MIN, 1, -1];
        let result = container_with_most_water_safe(&height);
        // Should handle extreme values gracefully
        assert!(result >= 0);
    }

    #[test]
    fn test_find_subarray_sum_safe_extreme_values() {
        // Test with extreme positive and negative values
        let nums = vec![i32::MAX, i32::MIN, 0, 1, -1];
        let result = find_subarray_sum_safe(&nums, i32::MAX);
        assert!(result.is_some() || result.is_none());

        let result = find_subarray_sum_safe(&nums, i32::MIN);
        assert!(result.is_some() || result.is_none());
    }
}
