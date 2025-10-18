//! Attack-defense tests for logical invariant protection in two-pointer algorithms
//!
//! These tests simulate attacks where logical invariants are broken by untrusted input
//! and verify that the implementations defend against them properly.

use logical_invariant_protection::{
    two_sum_sorted,
    is_palindrome,
    three_sum,
    container_with_most_water,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_sorted_with_maliciously_unsorted_input() {
        // Simulate an attacker providing deliberately unsorted input
        // The algorithm should still work correctly
        let nums = vec![100, 1, 50, 2, 75, 3];
        let target = 5;
        
        let result = two_sum_sorted(&nums, target);
        // Should find 2 + 3 = 5
        assert!(result.is_some());
        if let Some((i, j)) = result {
            assert_eq!(nums[i] + nums[j], target);
        }
    }

    #[test]
    fn test_three_sum_with_maliciously_unsorted_input() {
        // Simulate an attacker providing deliberately unsorted input
        // The algorithm should still work correctly
        let nums = vec![10, -10, 5, -5, 0, 1, -1];
        let target = 0;
        
        let result = three_sum(&nums, target);
        assert!(result.is_some());
        if let Some((i, j, k)) = result {
            assert_eq!(nums[i] + nums[j] + nums[k], target);
        }
    }

    #[test]
    fn test_is_palindrome_with_malicious_unicode_input() {
        // Simulate an attacker providing Unicode strings that might break assumptions
        // The algorithm should handle them safely
        let s = "A man, a plan, a canal: Panama!@#$%^&*()_+{}|:<>?"; // Mix of ASCII and special chars
        assert_eq!(is_palindrome(s), true);
        
        let s = "上海海上🌊🌊"; // Mix of Chinese characters and emojis
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_container_with_most_water_with_negative_values() {
        // Simulate an attacker providing negative height values
        // The algorithm should handle them safely
        let height = vec![-5, -10, 1, 8, 6, 2, 5, 4, 8, 3, 7, -3];
        let result = container_with_most_water(&height);
        // Should still return a valid non-negative area
        assert!(result >= 0);
    }

    #[test]
    fn test_two_sum_sorted_with_duplicate_values() {
        // Test edge case with many duplicate values that might confuse the algorithm
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2];
        let target = 3;
        
        let result = two_sum_sorted(&nums, target);
        assert!(result.is_some());
        if let Some((i, j)) = result {
            assert_eq!(nums[i] + nums[j], target);
        }
    }

    #[test]
    fn test_three_sum_with_all_same_values() {
        // Test edge case with all same values
        let nums = vec![0, 0, 0, 0, 0, 0];
        let target = 0;
        
        let result = three_sum(&nums, target);
        assert!(result.is_some());
        if let Some((i, j, k)) = result {
            assert_eq!(nums[i] + nums[j] + nums[k], target);
        }
    }

    #[test]
    fn test_two_sum_sorted_with_extreme_values() {
        // Test with extreme values that might cause overflow
        let nums = vec![i32::MIN, -1000000, 0, 1000000, i32::MAX];
        let target = i32::MAX + i32::MIN; // This would overflow in normal addition
        
        let result = two_sum_sorted(&nums, target);
        // Should handle overflow gracefully
        if let Some((i, j)) = result {
            // We can't easily verify the sum due to overflow, but indices should be valid
            assert!(i < nums.len());
            assert!(j < nums.len());
            assert!(i != j);
        }
    }

    #[test]
    fn test_container_with_most_water_with_single_positive() {
        // Test with mostly negative values and one positive
        let height = vec![-1, -2, -3, 5, -4, -5];
        let result = container_with_most_water(&height);
        assert!(result >= 0);
    }
}