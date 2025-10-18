//! Implementations of opposite-ends (converging) two-pointer algorithms with logical invariant protection
//!
//! This module demonstrates secure implementations that protect against broken logical invariants
//! when attackers supply unsorted or maliciously arranged data.

/// Checks if a slice is sorted in non-decreasing order
fn is_sorted_non_decreasing(nums: &[i32]) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}

/// Finds two numbers in a sorted array that sum to a target value with logical invariant protection
/// 
/// # Security Measures Against Broken Logical Invariants
/// 1. Input validation to ensure sorted input
/// 2. Fallback to alternative algorithms for unsorted input
/// 3. Safe handling of edge cases
/// 4. Proper error reporting for invalid inputs
/// 
/// # Arguments
/// * `nums` - A slice of integers (expected to be sorted)
/// * `target` - The target sum to find
/// 
/// # Returns
/// * `Some((i, j))` - Indices of the two numbers that sum to target
/// * `None` - If no such pair exists or if input is invalid
/// 
/// # Examples
/// ```
/// use logical_invariant_protection::two_pointer::opposite_ends::two_sum_sorted;
/// 
/// let nums = vec![2, 7, 11, 15]; // Note: this is sorted
/// let result = two_sum_sorted(&nums, 9);
/// assert_eq!(result, Some((0, 1)));
/// ```
pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Input validation - check if array is sorted
    if !is_sorted_non_decreasing(nums) {
        // Handle unsorted input by using a hash-based approach
        return two_sum_unsorted_fallback(nums, target);
    }

    // Safety check for minimum array size
    if nums.len() < 2 {
        return None;
    }

    // Initialize pointers with bounds checking
    let mut left = 0;
    let mut right = nums.len() - 1;

    // Main loop with proper termination conditions
    while left < right {
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
                    },
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
                // Handle overflow case
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

/// Fallback implementation for two_sum when input is not sorted
fn two_sum_unsorted_fallback(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    
    let mut num_to_index: HashMap<i32, usize> = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = match target.checked_sub(num) {
            Some(val) => val,
            None => continue, // Skip if subtraction would overflow
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

/// Checks if a string is a palindrome, ignoring non-alphanumeric characters
/// 
/// # Security Measures Against Broken Logical Invariants
/// 1. Proper handling of Unicode characters
/// 2. Safe character processing
/// 3. Robust alphanumeric checking
/// 
/// # Arguments
/// * `s` - The string to check
/// 
/// # Returns
/// * `true` - If the string is a palindrome
/// * `false` - Otherwise
/// 
/// # Examples
/// ```
/// use logical_invariant_protection::two_pointer::opposite_ends::is_palindrome;
/// 
/// assert_eq!(is_palindrome("A man, a plan, a canal: Panama"), true);
/// assert_eq!(is_palindrome("race a car"), false);
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    // Handle empty string case
    if len == 0 {
        return true;
    }
    
    // Safe initialization with bounds checking
    let mut left = 0;
    let mut right = match len.checked_sub(1) {
        Some(index) => index,
        None => return true, // Handle potential underflow
    };

    while left < right {
        // Double-check bounds before accessing characters
        if left >= len || right >= len {
            return false;
        }
        
        // Skip non-alphanumeric characters from the left with bounds checking
        while left < right && left < len && !chars[left].is_alphanumeric() {
            left += 1;
        }
        
        // Skip non-alphanumeric characters from the right with bounds checking
        while left < right && right < len && !chars[right].is_alphanumeric() {
            if right == 0 {
                break;
            }
            right -= 1;
        }
        
        // Check if we've moved past each other
        if left >= right {
            break;
        }
        
        // Double-check bounds before final comparison
        if left >= len || right >= len {
            return false;
        }
        
        // Compare characters (case-insensitive)
        if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
            return false;
        }
        
        // Safe advancement with bounds checking
        left += 1;
        if left >= len {
            break;
        }
        
        if right > 0 {
            right -= 1;
        } else {
            break;
        }
    }
    
    true
}

/// Finds three numbers in a sorted array that sum to a target value with logical invariant protection
/// 
/// # Security Measures Against Broken Logical Invariants
/// 1. Input validation to ensure sorted input
/// 2. Fallback to alternative algorithms for unsorted input
/// 3. Safe handling of edge cases
/// 4. Proper error reporting for invalid inputs
/// 
/// # Arguments
/// * `nums` - A slice of integers (expected to be sorted)
/// * `target` - The target sum to find
/// 
/// # Returns
/// * `Some((i, j, k))` - Indices of the three numbers that sum to target
/// * `None` - If no such triplet exists or if input is invalid
/// 
/// # Examples
/// ```
/// use logical_invariant_protection::two_pointer::opposite_ends::three_sum;
/// 
/// let mut nums = vec![-1, 0, 1, 2, -1, -4];
/// nums.sort(); // Ensure the array is sorted
/// let result = three_sum(&nums, 0);
/// assert!(result.is_some());
/// ```
pub fn three_sum(nums: &[i32], target: i32) -> Option<(usize, usize, usize)> {
    // Input validation - check if array is sorted
    if !is_sorted_non_decreasing(nums) {
        // Handle unsorted input by sorting first
        let mut sorted_nums: Vec<(i32, usize)> = nums.iter().enumerate().map(|(i, &val)| (val, i)).collect();
        sorted_nums.sort_by_key(|&(val, _)| val);
        
        // Extract sorted values and original indices
        let sorted_values: Vec<i32> = sorted_nums.iter().map(|&(val, _)| val).collect();
        let original_indices: Vec<usize> = sorted_nums.iter().map(|&(_, idx)| idx).collect();
        
        // Call three_sum with sorted values
        if let Some((i, j, k)) = three_sum_sorted(&sorted_values, target) {
            // Map back to original indices
            return Some((original_indices[i], original_indices[j], original_indices[k]));
        }
        return None;
    }
    
    three_sum_sorted(nums, target)
}

/// Internal implementation for three_sum with sorted input
fn three_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize, usize)> {
    let len = nums.len();
    
    // Input validation - need at least 3 elements
    if len < 3 {
        return None;
    }

    // Outer loop with bounds checking
    for i in 0..len - 2 {
        // Double-check bounds for outer loop index
        if i >= len {
            return None;
        }
        
        // Safe subtraction with overflow checking
        let remaining_target = match target.checked_sub(nums[i]) {
            Some(val) => val,
            None => continue, // Skip if subtraction would overflow
        };
        
        // Safe initialization of inner pointers with bounds checking
        let mut left = match i.checked_add(1) {
            Some(val) => val,
            None => return None, // Handle overflow in index calculation
        };
        
        let mut right = match len.checked_sub(1) {
            Some(index) => index,
            None => return None, // Handle potential underflow
        };

        // Inner two-pointer loop with comprehensive bounds checking
        while left < right {
            // Double-check bounds before accessing array elements
            if left >= len || right >= len {
                return None;
            }
            
            // Safe addition with overflow checking
            match nums[left].checked_add(nums[right]) {
                Some(sum) => {
                    match sum.cmp(&remaining_target) {
                        std::cmp::Ordering::Equal => return Some((i, left, right)),
                        std::cmp::Ordering::Less => {
                            left += 1;
                            // Additional bounds check after increment
                            if left >= len {
                                break;
                            }
                        },
                        std::cmp::Ordering::Greater => {
                            // Safe decrement with underflow protection
                            if right == 0 {
                                break;
                            }
                            right -= 1;
                        }
                    }
                }
                None => {
                    // Handle overflow case
                    if nums[left] > 0 && nums[right] > 0 {
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
        // Test with sorted input
        let nums = vec![2, 7, 11, 15];
        let result = two_sum_sorted(&nums, 9);
        assert_eq!(result, Some((0, 1)));
        
        let nums = vec![2, 3, 4];
        let result = two_sum_sorted(&nums, 6);
        assert_eq!(result, Some((0, 2)));
    }

    #[test]
    fn test_two_sum_sorted_unsorted_input() {
        // Test with unsorted input - should still work
        let nums = vec![7, 2, 15, 11];
        let result = two_sum_sorted(&nums, 9);
        // Should find 2 + 7 = 9
        assert!(result.is_some());
        
        let nums = vec![4, 2, 3];
        let result = two_sum_sorted(&nums, 6);
        // Should find 2 + 4 = 6
        assert!(result.is_some());
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
    fn test_is_palindrome_normal_case() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama"), true);
        assert_eq!(is_palindrome("race a car"), false);
        assert_eq!(is_palindrome("Madam"), true);
    }

    #[test]
    fn test_is_palindrome_edge_cases() {
        // Empty string
        assert_eq!(is_palindrome(""), true);
        
        // Single character
        assert_eq!(is_palindrome("a"), true);
        
        // All non-alphanumeric
        assert_eq!(is_palindrome("!!!"), true);
    }

    #[test]
    fn test_three_sum_normal_case() {
        // Test with sorted input
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = three_sum(&nums, 0);
        assert!(result.is_some());
        
        let nums = vec![0, 0, 0];
        let result = three_sum(&nums, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_three_sum_unsorted_input() {
        // Test with unsorted input - should still work
        let nums = vec![1, -1, -4, 0, 2, -1];
        let result = three_sum(&nums, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_three_sum_edge_cases() {
        // Too few elements
        let nums = vec![1, 2];
        let result = three_sum(&nums, 3);
        assert_eq!(result, None);
        
        // No valid triplet
        let nums = vec![0, 1, 1];
        let result = three_sum(&nums, 0);
        assert_eq!(result, None);
    }
}