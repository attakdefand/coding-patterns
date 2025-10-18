//! Implementations of opposite-ends (converging) two-pointer algorithms with infinite loop protection
//!
//! This module demonstrates secure implementations that prevent infinite loops
//! and high CPU usage through multiple layers of protection.

use std::time::{Duration, Instant};

/// Finds two numbers in a sorted array that sum to a target value with infinite loop protection
///
/// # Security Measures Against Infinite Loops
/// 1. Time-based loop termination to prevent algorithmic DoS
/// 2. Iteration count limiting to prevent unbounded execution
/// 3. Proper pointer advancement validation
/// 4. Input size validation to prevent complexity attacks
///
/// # Arguments
/// * `nums` - A sorted slice of integers
/// * `target` - The target sum to find
///
/// # Returns
/// * `Some((i, j))` - Indices of the two numbers that sum to target
/// * `None` - If no such pair exists or if input is invalid
///
/// # Examples
/// ```
/// use infinite_loop_protection::two_pointer::opposite_ends::two_sum_sorted;
///
/// let nums = vec![2, 7, 11, 15];
/// let result = two_sum_sorted(&nums, 9);
/// assert_eq!(result, Some((0, 1)));
/// ```
pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Input validation - prevent complexity attacks with large inputs
    if nums.len() > 1000000 {
        // Limit to 1 million elements
        return None;
    }

    // Safety check for minimum array size
    if nums.len() < 2 {
        return None;
    }

    // Initialize pointers with bounds checking
    let mut left = 0;
    let mut right = nums.len() - 1;

    // Protection against infinite loops:
    // 1. Time limit (100ms should be more than enough for any reasonable input)
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(100);

    // 2. Iteration limit (worst case is n/2 iterations)
    let max_iterations = nums.len() / 2 + 1;
    let mut iteration_count = 0;

    // Main loop with comprehensive infinite loop protection
    while left < right {
        // Check time limit to prevent algorithmic DoS
        if start_time.elapsed() > time_limit {
            // Log or handle timeout as needed
            return None; // Timeout - potential DoS attack or extremely slow execution
        }

        // Check iteration limit to prevent unbounded execution
        iteration_count += 1;
        if iteration_count > max_iterations {
            // This should never happen with correct logic, but protects against bugs
            return None; // Too many iterations - potential infinite loop
        }

        // Safe addition with overflow checking
        match nums[left].checked_add(nums[right]) {
            Some(sum) => {
                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => return Some((left, right)),
                    std::cmp::Ordering::Less => {
                        left += 1;
                        // Additional bounds check after increment
                        if left >= nums.len() {
                            return None;
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        // Safe decrement with underflow protection
                        if right == 0 {
                            return None;
                        }
                        right -= 1;
                    }
                }
            }
            None => {
                // Handle overflow case - move the pointer that's more likely to help
                if nums[left] > 0 && nums[right] > 0 {
                    // Both positive, sum overflowed, so decrease right
                    if right == 0 {
                        return None;
                    }
                    right -= 1;
                } else {
                    // At least one negative, so increase left
                    left += 1;
                    if left >= nums.len() {
                        return None;
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_sorted_normal_case() {
        let nums = vec![2, 7, 11, 15];
        let result = two_sum_sorted(&nums, 9);
        assert_eq!(result, Some((0, 1)));

        let nums = vec![2, 3, 4];
        let result = two_sum_sorted(&nums, 6);
        assert_eq!(result, Some((0, 2)));
    }

    #[test]
    fn test_two_sum_sorted_edge_cases() {
        // Empty array
        let nums = vec![];
        let result = two_sum_sorted(&nums, 0);
        assert_eq!(result, None);

        // Single element
        let nums = vec![1];
        let result = two_sum_sorted(&nums, 1);
        assert_eq!(result, None);

        // Two elements
        let nums = vec![1, 2];
        let result = two_sum_sorted(&nums, 3);
        assert_eq!(result, Some((0, 1)));

        // No valid pair
        let nums = vec![1, 2, 3, 4, 5];
        let result = two_sum_sorted(&nums, 10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_two_sum_sorted_large_input_protection() {
        // Test with input that's at the limit
        let nums: Vec<i32> = (0..1000000).collect();
        let result = two_sum_sorted(&nums, 999999);
        // Should work fine (finds 0 + 999999)
        assert_eq!(result, Some((0, 999999)));

        // Test with input that exceeds the limit
        let nums: Vec<i32> = (0..1000001).collect();
        let result = two_sum_sorted(&nums, 1000000);
        // Should be rejected due to size limit
        assert_eq!(result, None);
    }
}
