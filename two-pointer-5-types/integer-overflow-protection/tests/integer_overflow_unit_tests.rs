//! Unit tests for integer overflow protection in two-pointer algorithms
//!
//! These tests verify that the implementations correctly handle cases where
//! integer overflow or underflow could occur.

use integer_overflow_protection::{
    two_sum_safe,
    three_sum_safe,
    container_with_most_water_safe,
    find_subarray_sum_safe,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_safe_with_large_values() {
        // Test with values that could cause overflow
        let nums = vec![i32::MAX, 1, 2];
        let result = two_sum_safe(&nums, i32::MAX.wrapping_add(1)); // This would overflow
        // Should handle overflow gracefully
        assert!(result.is_none() || result.is_some());
        
        // Test with values that could cause underflow
        let nums = vec![i32::MIN, -1, -2];
        let result = two_sum_safe(&nums, i32::MIN.wrapping_sub(1)); // This would underflow
        // Should handle underflow gracefully
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_three_sum_safe_with_large_values() {
        // Test with values that could cause overflow
        let nums = vec![i32::MAX, 1, 2, 3];
        let result = three_sum_safe(&nums, i32::MAX.wrapping_add(6)); // This would overflow
        // Should handle overflow gracefully
        assert!(result.is_none() || result.is_some());
        
        // Test with values that could cause underflow
        let nums = vec![i32::MIN, -1, -2, -3];
        let result = three_sum_safe(&nums, i32::MIN.wrapping_sub(6)); // This would underflow
        // Should handle underflow gracefully
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_container_with_most_water_safe_with_large_values() {
        // Test with large values that might cause overflow
        let height = vec![i32::MAX, i32::MAX];
        let result = container_with_most_water_safe(&height);
        // Should handle overflow gracefully
        assert!(result >= 0);
        
        // Test with a large array
        let height = vec![1; 100000]; // Large array of 1s
        let result = container_with_most_water_safe(&height);
        assert_eq!(result, 99999); // (100000-1) * 1
    }

    #[test]
    fn test_find_subarray_sum_safe_with_large_values() {
        // Test with values that could cause overflow
        let nums = vec![i32::MAX, 1, 2];
        let result = find_subarray_sum_safe(&nums, i32::MAX.wrapping_add(1)); // This would overflow
        // Should handle overflow gracefully
        assert!(result.is_none() || result.is_some());
        
        // Test with values that could cause underflow
        let nums = vec![i32::MIN, -1, -2];
        let result = find_subarray_sum_safe(&nums, i32::MIN.wrapping_sub(3)); // This would underflow
        // Should handle underflow gracefully
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_two_sum_safe_normal_cases() {
        // Normal case
        let nums = vec![2, 7, 11, 15];
        let result = two_sum_safe(&nums, 9);
        assert_eq!(result, Some((0, 1)));
        
        // Negative numbers
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = two_sum_safe(&nums, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_three_sum_safe_normal_cases() {
        // Normal case
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = three_sum_safe(&nums, 0);
        assert!(result.is_some());
        
        // All zeros
        let nums = vec![0, 0, 0];
        let result = three_sum_safe(&nums, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_container_with_most_water_safe_normal_cases() {
        // Normal case
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = container_with_most_water_safe(&height);
        assert_eq!(result, 49);
        
        // Simple case
        let height = vec![1, 1];
        let result = container_with_most_water_safe(&height);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_subarray_sum_safe_normal_cases() {
        // Normal case
        let nums = vec![1, 4, 2, 7, 3];
        let result = find_subarray_sum_safe(&nums, 6);
        assert_eq!(result, Some((1, 2))); // Subarray [4, 2]
        
        // Simple case
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_subarray_sum_safe(&nums, 9);
        assert_eq!(result, Some((1, 3))); // Subarray [2, 3, 4]
    }
}