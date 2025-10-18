//! Implementations of bidirectional merge two-pointer algorithms with infinite loop protection
//!
//! This module demonstrates secure implementations that prevent infinite loops
//! and high CPU usage through multiple layers of protection.

use std::time::{Duration, Instant};

/// Merges two sorted arrays into one sorted array with infinite loop protection
/// 
/// # Security Measures Against Infinite Loops
/// 1. Time-based loop termination to prevent algorithmic DoS
/// 2. Iteration count limiting to prevent unbounded execution
/// 3. Input size validation to prevent complexity attacks
/// 4. Proper termination conditions
/// 
/// # Arguments
/// * `nums1` - First sorted slice of integers
/// * `nums2` - Second sorted slice of integers
/// 
/// # Returns
/// * `Vec<i32>` - Merged sorted array
/// 
/// # Examples
/// ```
/// use infinite_loop_protection::two_pointer::bidirectional_merge::merge_sorted_arrays;
/// 
/// let nums1 = vec![1, 2, 4];
/// let nums2 = vec![1, 3, 4];
/// let result = merge_sorted_arrays(&nums1, &nums2);
/// assert_eq!(result, vec![1, 1, 2, 3, 4, 4]);
/// ```
pub fn merge_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    // Input validation - prevent complexity attacks with large inputs
    if nums1.len() + nums2.len() > 2000000 { // Limit to 2 million total elements
        return vec![];
    }

    let mut result = Vec::with_capacity(nums1.len() + nums2.len());
    let mut i = 0;
    let mut j = 0;
    
    // Protection against infinite loops:
    // 1. Time limit (100ms should be more than enough for any reasonable input)
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(100);
    
    // 2. Iteration limit (prevent unbounded execution)
    let max_iterations = nums1.len() + nums2.len() + 1; // Worst case is m+n iterations
    let mut iteration_count = 0;

    // Merge elements while both arrays have elements
    while i < nums1.len() && j < nums2.len() {
        // Check time limit to prevent algorithmic DoS
        if start_time.elapsed() > time_limit {
            // Truncate and return partial result on timeout
            return result;
        }
        
        // Check iteration limit to prevent unbounded execution
        iteration_count += 1;
        if iteration_count > max_iterations {
            // This should never happen with correct logic, but protects against bugs
            return result; // Return partial result on iteration limit
        }

        if nums1[i] <= nums2[j] {
            result.push(nums1[i]);
            i += 1;
        } else {
            result.push(nums2[j]);
            j += 1;
        }
    }
    
    // Add remaining elements from nums1 with loop protection
    while i < nums1.len() {
        // Check time limit
        if start_time.elapsed() > time_limit {
            return result;
        }
        
        // Check iteration limit
        iteration_count += 1;
        if iteration_count > max_iterations {
            return result;
        }
        
        result.push(nums1[i]);
        i += 1;
    }
    
    // Add remaining elements from nums2 with loop protection
    while j < nums2.len() {
        // Check time limit
        if start_time.elapsed() > time_limit {
            return result;
        }
        
        // Check iteration limit
        iteration_count += 1;
        if iteration_count > max_iterations {
            return result;
        }
        
        result.push(nums2[j]);
        j += 1;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_arrays_normal_case() {
        let nums1 = vec![1, 2, 4];
        let nums2 = vec![1, 3, 4];
        let result = merge_sorted_arrays(&nums1, &nums2);
        assert_eq!(result, vec![1, 1, 2, 3, 4, 4]);
        
        let nums1 = vec![];
        let nums2 = vec![1, 2, 3];
        let result = merge_sorted_arrays(&nums1, &nums2);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sorted_arrays_edge_cases() {
        let nums1 = vec![];
        let nums2 = vec![];
        let result = merge_sorted_arrays(&nums1, &nums2);
        assert_eq!(result, vec![]);
        
        let nums1 = vec![1];
        let nums2 = vec![2];
        let result = merge_sorted_arrays(&nums1, &nums2);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_merge_sorted_arrays_large_input_protection() {
        // Test with inputs that exceed the limit
        let nums1: Vec<i32> = (0..1000001).collect();
        let nums2: Vec<i32> = (0..1000000).collect();
        let result = merge_sorted_arrays(&nums1, &nums2);
        // Should be rejected due to size limit
        assert_eq!(result, vec![]);
        
        // Test with reasonable size inputs
        let nums1: Vec<i32> = (0..1000).collect();
        let nums2: Vec<i32> = (0..1000).collect();
        let result = merge_sorted_arrays(&nums1, &nums2);
        // Should work fine
        assert_eq!(result.len(), 2000);
    }
}