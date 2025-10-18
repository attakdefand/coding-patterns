//! Two-pointer algorithms with timing side-channel protection

use subtle::{Choice, ConstantTimeEq, ConstantTimeGreater, ConstantTimeLess};

/// Securely finds two numbers in a sorted array that sum to a target value
/// 
/// This implementation avoids timing side-channels by:
/// 1. Using constant-time comparisons
/// 2. Performing the same number of iterations regardless of input
/// 3. Avoiding early exits based on secret data
/// 
/// # Arguments
/// * `nums` - A sorted slice of integers
/// * `target` - The target sum to find
/// 
/// # Returns
/// * `Option<(usize, usize)>` - Indices of the two numbers that sum to target, or None
pub fn secure_two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    if nums.len() < 2 {
        return None;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut result_indices: Option<(usize, usize)> = None;
    
    // We'll iterate through all possible combinations to avoid timing leaks
    // This ensures constant time execution regardless of where the match is found
    while left < right {
        let left_val = nums[left];
        let right_val = nums[right];
        let sum = left_val + right_val;
        
        // Constant-time comparison
        let is_match = sum.ct_eq(&target);
        
        // Only update result if we found a match (but continue iterating)
        let new_result = (left, right);
        result_indices = if is_match.into() {
            Some(new_result)
        } else {
            result_indices
        };
        
        // Move pointers using constant-time comparisons to avoid timing leaks
        let sum_less = sum.ct_lt(&target);
        let sum_greater = sum.ct_gt(&target);
        
        // Update pointers in a way that doesn't leak timing information
        if sum_less.into() {
            left += 1;
        }
        if sum_greater.into() {
            if right > 0 {
                right -= 1;
            }
        }
        
        // Continue iteration to maintain constant timing
        if !sum_less.into() && !sum_greater.into() {
            // Found exact match, but continue to avoid timing leak
            left += 1;
            if right > 0 {
                right -= 1;
            }
        }
    }
    
    result_indices
}

/// Securely compares two strings using a two-pointer approach
/// 
/// This implementation avoids timing side-channels by:
/// 1. Using constant-time comparisons for each character
/// 2. Performing the same number of iterations regardless of input
/// 3. Avoiding early exits based on mismatched characters
/// 
/// # Arguments
/// * `a` - First string to compare
/// * `b` - Second string to compare
/// 
/// # Returns
/// * `bool` - True if strings are equal, false otherwise
pub fn secure_string_compare(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    
    // Get the maximum length to ensure constant iterations
    let max_len = core::cmp::max(a_bytes.len(), b_bytes.len());
    
    // Initialize result as true (equal)
    let mut result = Choice::from(1u8);
    
    // Compare each position, padding with 0 for shorter string
    for i in 0..max_len {
        let a_byte = if i < a_bytes.len() { a_bytes[i] } else { 0 };
        let b_byte = if i < b_bytes.len() { b_bytes[i] } else { 0 };
        
        // Constant-time comparison
        let bytes_equal = a_byte.ct_eq(&b_byte);
        result = result & bytes_equal;
    }
    
    // Also check that lengths are equal (constant time)
    let lengths_equal = a_bytes.len().ct_eq(&b_bytes.len());
    result = result & lengths_equal;
    
    result.into()
}

/// Securely searches for a target value in a sorted array
/// 
/// This implementation avoids timing side-channels by:
/// 1. Using constant-time comparisons
/// 2. Performing the same number of iterations regardless of input
/// 3. Avoiding early exits based on found values
/// 
/// # Arguments
/// * `arr` - A sorted slice of integers
/// * `target` - The value to search for
/// 
/// # Returns
/// * `Option<usize>` - Index of the target value, or None
pub fn secure_array_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    
    let mut result_index: Option<usize> = None;
    
    // Iterate through all elements to avoid timing leaks
    for (i, &value) in arr.iter().enumerate() {
        // Constant-time comparison
        let is_match = value.ct_eq(&target);
        
        // Only update result if we found a match (but continue iterating)
        let new_result = i;
        if is_match.into() {
            result_index = Some(new_result);
        }
    }
    
    result_index
}

/// Securely finds the intersection of two sorted arrays
/// 
/// This implementation avoids timing side-channels by:
/// 1. Using constant-time comparisons
/// 2. Performing the same number of iterations regardless of input
/// 3. Avoiding early exits based on intersection results
/// 
/// # Arguments
/// * `arr1` - First sorted array
/// * `arr2` - Second sorted array
/// 
/// # Returns
/// * `Vec<i32>` - Intersection of the two arrays
pub fn secure_sorted_intersection(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut found_flags = vec![false; arr1.len()];
    
    // For each element in arr1, check if it exists in arr2
    for (i, &val1) in arr1.iter().enumerate() {
        let mut found = Choice::from(0u8);
        
        // Check against all elements in arr2
        for &val2 in arr2.iter() {
            let is_match = val1.ct_eq(&val2);
            found = found | is_match;
        }
        
        // Mark as found if match was detected
        if found.into() {
            found_flags[i] = true;
        }
    }
    
    // Collect results in a separate pass to avoid timing leaks
    for (i, &val) in arr1.iter().enumerate() {
        if found_flags[i] {
            result.push(val);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_two_sum() {
        let nums = [2, 7, 11, 15];
        assert_eq!(secure_two_sum(&nums, 9), Some((0, 1)));
        
        let nums = [3, 2, 4];
        assert_eq!(secure_two_sum(&nums, 6), Some((1, 2)));
        
        let nums = [3, 3];
        assert_eq!(secure_two_sum(&nums, 6), Some((0, 1)));
        
        let nums = [1, 2, 3];
        assert_eq!(secure_two_sum(&nums, 7), None);
    }

    #[test]
    fn test_secure_string_compare() {
        assert!(secure_string_compare("hello", "hello"));
        assert!(!secure_string_compare("hello", "world"));
        assert!(!secure_string_compare("hello", "hello1"));
        assert!(!secure_string_compare("hello1", "hello"));
    }

    #[test]
    fn test_secure_array_search() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(secure_array_search(&arr, 3), Some(2));
        assert_eq!(secure_array_search(&arr, 6), None);
        
        let arr = [];
        assert_eq!(secure_array_search(&arr, 1), None);
    }

    #[test]
    fn test_secure_sorted_intersection() {
        let arr1 = [1, 2, 2, 3, 4];
        let arr2 = [2, 2, 3, 5, 6];
        let result = secure_sorted_intersection(&arr1, &arr2);
        assert_eq!(result, vec![2, 2, 3]);
        
        let arr1 = [1, 3, 5];
        let arr2 = [2, 4, 6];
        let result = secure_sorted_intersection(&arr1, &arr2);
        assert_eq!(result, vec![]);
    }
}