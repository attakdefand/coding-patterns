//! Implementations of opposite-ends (converging) two-pointer algorithms with integer overflow protection
//!
//! This module demonstrates secure implementations that protect against integer overflow
//! and underflow when computing indices, sums, or sizes in two-pointer algorithms.

use std::collections::HashMap;

/// Finds two numbers in an array that sum to a target value with integer overflow protection
/// 
/// # Security Measures Against Integer Overflow/Underflow
/// 1. Uses checked arithmetic operations for sum calculations
/// 2. Validates array indices before access
/// 3. Handles extreme value cases safely
/// 4. Provides fallback mechanisms for edge cases
/// 
/// # Arguments
/// * `nums` - A slice of integers
/// * `target` - The target sum to find
/// 
/// # Returns
/// * `Some((i, j))` - Indices of the two numbers that sum to target
/// * `None` - If no such pair exists or if overflow would occur
/// 
/// # Examples
/// ```
/// use integer_overflow_protection::two_pointer::opposite_ends::two_sum_safe;
/// 
/// let nums = vec![2, 7, 11, 15];
/// let result = two_sum_safe(&nums, 9);
/// assert_eq!(result, Some((0, 1)));
/// ```
pub fn two_sum_safe(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Handle edge cases
    if nums.len() < 2 {
        return None;
    }

    // Use hash-based approach to avoid potential overflow in pointer arithmetic
    let mut num_to_index: HashMap<i32, usize> = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        // Check for potential overflow in subtraction
        let complement = match target.checked_sub(num) {
            Some(val) => val,
            None => {
                // Overflow in subtraction, skip this number
                num_to_index.insert(num, i);
                continue;
            }
        };
        
        if let Some(&j) = num_to_index.get(&complement) {
            // Return indices in sorted order
            if i < j {
                return Some((i, j));
            } else {
                return Some((j, i));
            }
        }
        
        num_to_index.insert(num, i);
    }
    
    None
}

/// Finds three numbers in an array that sum to a target value with integer overflow protection
/// 
/// # Security Measures Against Integer Overflow/Underflow
/// 1. Uses checked arithmetic operations for sum calculations
/// 2. Validates array indices before access
/// 3. Handles extreme value cases safely
/// 4. Provides fallback mechanisms for edge cases
/// 
/// # Arguments
/// * `nums` - A slice of integers
/// * `target` - The target sum to find
/// 
/// # Returns
/// * `Some((i, j, k))` - Indices of the three numbers that sum to target
/// * `None` - If no such triplet exists or if overflow would occur
/// 
/// # Examples
/// ```
/// use integer_overflow_protection::two_pointer::opposite_ends::three_sum_safe;
/// 
/// let nums = vec![-1, 0, 1, 2, -1, -4];
/// let result = three_sum_safe(&nums, 0);
/// assert!(result.is_some());
/// ```
pub fn three_sum_safe(nums: &[i32], target: i32) -> Option<(usize, usize, usize)> {
    let len = nums.len();
    
    // Input validation - need at least 3 elements
    if len < 3 {
        return None;
    }

    // Create a vector of (value, index) pairs and sort by value
    let mut indexed_nums: Vec<(i32, usize)> = nums.iter().enumerate().map(|(i, &val)| (val, i)).collect();
    indexed_nums.sort_by_key(|&(val, _)| val);

    // Outer loop
    for i in 0..len - 2 {
        // Double-check bounds for outer loop index
        if i >= len {
            return None;
        }
        
        // Safe subtraction with overflow checking
        let remaining_target = match target.checked_sub(indexed_nums[i].0) {
            Some(val) => val,
            None => continue, // Skip if subtraction would overflow
        };
        
        // Use two pointers for the remaining two elements
        let mut left = i + 1;
        let mut right = len - 1;
        
        while left < right {
            // Double-check bounds
            if left >= len || right >= len {
                break;
            }
            
            // Safe addition with overflow checking
            let sum = match indexed_nums[left].0.checked_add(indexed_nums[right].0) {
                Some(val) => val,
                None => {
                    // Overflow in addition, move pointers
                    if indexed_nums[left].0 > 0 && indexed_nums[right].0 > 0 {
                        // Both positive, sum overflowed, so decrease right
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    } else {
                        // At least one negative, so increase left
                        left += 1;
                        if left >= len {
                            break;
                        }
                    }
                    continue;
                }
            };
            
            match sum.cmp(&remaining_target) {
                std::cmp::Ordering::Equal => {
                    // Found a triplet, return the original indices
                    let indices = [
                        indexed_nums[i].1,
                        indexed_nums[left].1,
                        indexed_nums[right].1
                    ];
                    // Return indices in sorted order
                    let mut sorted_indices = indices;
                    sorted_indices.sort();
                    return Some((sorted_indices[0], sorted_indices[1], sorted_indices[2]));
                },
                std::cmp::Ordering::Less => {
                    left += 1;
                },
                std::cmp::Ordering::Greater => {
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                }
            }
        }
    }

    None
}

/// Finds a contiguous subarray with a given sum with integer overflow protection
/// 
/// # Security Measures Against Integer Overflow/Underflow
/// 1. Uses checked arithmetic operations for sum calculations
/// 2. Validates array indices before access
/// 3. Handles extreme value cases safely
/// 4. Provides fallback mechanisms for edge cases
/// 
/// # Arguments
/// * `nums` - A slice of integers
/// * `target` - The target sum to find
/// 
/// # Returns
/// * `Some((start, end))` - Start and end indices of the subarray
/// * `None` - If no such subarray exists or if overflow would occur
/// 
/// # Examples
/// ```
/// use integer_overflow_protection::two_pointer::opposite_ends::find_subarray_sum_safe;
/// 
/// let nums = vec![1, 4, 2, 7, 3];
/// let result = find_subarray_sum_safe(&nums, 6);
/// assert_eq!(result, Some((1, 2))); // Subarray [4, 2]
/// ```
pub fn find_subarray_sum_safe(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let len = nums.len();
    
    if len == 0 {
        return None;
    }
    
    let mut left = 0;
    let mut current_sum: i64 = 0; // Use larger type to prevent overflow
    
    for right in 0..len {
        // Double-check bounds
        if right >= len {
            break;
        }
        
        // Add current element to sum using checked arithmetic
        current_sum = match current_sum.checked_add(nums[right] as i64) {
            Some(sum) => sum,
            None => {
                // Overflow occurred, reset and start from next position
                left = right + 1;
                current_sum = 0;
                continue;
            }
        };
        
        // Shrink window from left while sum is greater than target
        while current_sum > target as i64 && left <= right {
            current_sum = match current_sum.checked_sub(nums[left] as i64) {
                Some(sum) => sum,
                None => {
                    // Underflow occurred, reset
                    left = right + 1;
                    current_sum = 0;
                    break;
                }
            };
            left += 1;
        }
        
        // Check if we found the target sum
        if current_sum == target as i64 {
            return Some((left, right));
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_safe_normal_case() {
        let nums = vec![2, 7, 11, 15];
        let result = two_sum_safe(&nums, 9);
        assert_eq!(result, Some((0, 1)));
        
        let nums = vec![3, 2, 4];
        let result = two_sum_safe(&nums, 6);
        assert_eq!(result, Some((1, 2)));
    }

    #[test]
    fn test_two_sum_safe_edge_cases() {
        // Empty array
        let nums = vec![];
        let result = two_sum_safe(&nums, 0);
        assert_eq!(result, None);
        
        // Single element
        let nums = vec![1];
        let result = two_sum_safe(&nums, 1);
        assert_eq!(result, None);
        
        // Two elements
        let nums = vec![1, 2];
        let result = two_sum_safe(&nums, 3);
        assert_eq!(result, Some((0, 1)));
        
        // No valid pair
        let nums = vec![1, 2, 3];
        let result = two_sum_safe(&nums, 7);
        assert_eq!(result, None);
    }

    #[test]
    fn test_three_sum_safe_normal_case() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = three_sum_safe(&nums, 0);
        assert!(result.is_some());
        
        let nums = vec![0, 0, 0];
        let result = three_sum_safe(&nums, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_three_sum_safe_edge_cases() {
        // Too few elements
        let nums = vec![1, 2];
        let result = three_sum_safe(&nums, 3);
        assert_eq!(result, None);
        
        // No valid triplet
        let nums = vec![1, 2, 3];
        let result = three_sum_safe(&nums, 10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_subarray_sum_safe_normal_case() {
        let nums = vec![1, 4, 2, 7, 3];
        let result = find_subarray_sum_safe(&nums, 6);
        assert_eq!(result, Some((1, 2))); // Subarray [4, 2]
        
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_subarray_sum_safe(&nums, 9);
        assert_eq!(result, Some((1, 3))); // Subarray [2, 3, 4]
    }

    #[test]
    fn test_find_subarray_sum_safe_edge_cases() {
        // Empty array
        let nums = vec![];
        let result = find_subarray_sum_safe(&nums, 0);
        assert_eq!(result, None);
        
        // Single element match
        let nums = vec![5];
        let result = find_subarray_sum_safe(&nums, 5);
        assert_eq!(result, Some((0, 0)));
        
        // Single element no match
        let nums = vec![5];
        let result = find_subarray_sum_safe(&nums, 3);
        assert_eq!(result, None);
        
        // No valid subarray
        let nums = vec![1, 2, 3];
        let result = find_subarray_sum_safe(&nums, 10);
        assert_eq!(result, None);
    }
}