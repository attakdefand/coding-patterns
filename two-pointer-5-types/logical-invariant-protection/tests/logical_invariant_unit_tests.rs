//! Unit tests for logical invariant protection in two-pointer algorithms
//!
//! These tests verify that the implementations correctly handle cases where
//! logical invariants are broken by untrusted input.

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
    fn test_two_sum_sorted_with_unsorted_input() {
        // Test that two_sum_sorted works correctly with unsorted input
        // This should trigger the fallback mechanism
        let nums = vec![7, 2, 15, 11];
        let result = two_sum_sorted(&nums, 9);
        // Should find 2 + 7 = 9
        assert!(result.is_some());
        if let Some((i, j)) = result {
            assert_eq!(nums[i] + nums[j], 9);
        }
    }

    #[test]
    fn test_two_sum_sorted_with_sorted_input() {
        // Test that two_sum_sorted works correctly with sorted input
        let nums = vec![2, 7, 11, 15];
        let result = two_sum_sorted(&nums, 9);
        assert_eq!(result, Some((0, 1)));
    }

    #[test]
    fn test_three_sum_with_unsorted_input() {
        // Test that three_sum works correctly with unsorted input
        // This should trigger the preprocessing mechanism
        let nums = vec![1, -1, -4, 0, 2, -1];
        let result = three_sum(&nums, 0);
        assert!(result.is_some());
        if let Some((i, j, k)) = result {
            assert_eq!(nums[i] + nums[j] + nums[k], 0);
        }
    }

    #[test]
    fn test_three_sum_with_sorted_input() {
        // Test that three_sum works correctly with sorted input
        let mut nums = vec![-1, 0, 1, 2, -1, -4];
        nums.sort();
        let result = three_sum(&nums, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_is_palindrome_with_unicode() {
        // Test robust handling of Unicode characters
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama"), true);
        assert_eq!(is_palindrome("race a car"), false);
        assert_eq!(is_palindrome("Madam"), true);
        assert_eq!(is_palindrome("上海海上"), true); // Chinese palindrome
    }

    #[test]
    fn test_container_with_most_water_negative_values() {
        // Test handling of negative height values
        let height = vec![-1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = container_with_most_water(&height);
        // Should skip the negative value and still find a valid solution
        assert!(result > 0);
    }

    #[test]
    fn test_container_with_most_water_normal_case() {
        // Test normal case
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = container_with_most_water(&height);
        assert_eq!(result, 49);
    }
}