//! Implementations of opposite-ends (converging) two-pointer algorithms with out-of-bounds protection
//!
//! This module demonstrates secure implementations that prevent out-of-bounds access
//! and memory corruption through multiple layers of protection.

/// Finds two numbers in a sorted array that sum to a target value
///
/// # Security Measures Against Out-of-Bounds Access
/// 1. Bounds checking before every array access
/// 2. Proper initialization of pointer indices
/// 3. Safe decrement operations with underflow protection
/// 4. Validation of input parameters
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
/// use out_of_bounds_protection::two_pointer::opposite_ends::two_sum_sorted;
///
/// let nums = vec![2, 7, 11, 15];
/// let result = two_sum_sorted(&nums, 9);
/// assert_eq!(result, Some((0, 1)));
/// ```
pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Input validation - prevent out-of-bounds by ensuring minimum array size
    if nums.len() < 2 {
        return None;
    }

    // Safe initialization of pointers with bounds checking
    let mut left = 0;
    let mut right = match nums.len().checked_sub(1) {
        Some(index) => index,
        None => return None, // Handle potential underflow
    };

    // Main loop with comprehensive bounds checking
    while left < right {
        // Double-check bounds before accessing array elements
        if left >= nums.len() || right >= nums.len() {
            return None;
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

/// Checks if a string is a palindrome, ignoring non-alphanumeric characters
///
/// # Security Measures Against Out-of-Bounds Access
/// 1. Bounds checking before every character access
/// 2. Safe conversion from string to character vector
/// 3. Proper handling of empty strings
/// 4. Safe increment/decrement operations
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
/// use out_of_bounds_protection::two_pointer::opposite_ends::is_palindrome;
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

/// Reverses an array in-place using two pointers
///
/// # Security Measures Against Out-of-Bounds Access
/// 1. Bounds validation before swap operations
/// 2. Safe pointer initialization
/// 3. Proper termination conditions
/// 4. Validation of input parameters
///
/// # Arguments
/// * `nums` - A mutable slice of integers to reverse
///
/// # Examples
/// ```
/// use out_of_bounds_protection::two_pointer::opposite_ends::reverse_array;
///
/// let mut nums = vec![1, 2, 3, 4, 5];
/// reverse_array(&mut nums);
/// assert_eq!(nums, vec![5, 4, 3, 2, 1]);
/// ```
pub fn reverse_array(nums: &mut [i32]) {
    let len = nums.len();

    // Handle empty array case
    if len == 0 {
        return;
    }

    // Safe initialization with bounds checking
    let mut left = 0;
    let mut right = match len.checked_sub(1) {
        Some(index) => index,
        None => return, // Handle potential underflow
    };

    while left < right {
        // Double-check bounds before swap operation
        if left >= len || right >= len {
            return;
        }

        // Safe swap operation
        nums.swap(left, right);

        // Safe advancement with bounds checking
        left += 1;
        if left >= len {
            return;
        }

        if right > 0 {
            right -= 1;
        } else {
            return;
        }
    }
}

/// Finds three numbers in a sorted array that sum to a target value
///
/// # Security Measures Against Out-of-Bounds Access
/// 1. Comprehensive bounds checking in nested loops
/// 2. Safe pointer initialization and advancement
/// 3. Overflow protection in arithmetic operations
/// 4. Validation of intermediate calculations
///
/// # Arguments
/// * `nums` - A sorted slice of integers
/// * `target` - The target sum to find
///
/// # Returns
/// * `Some((i, j, k))` - Indices of the three numbers that sum to target
/// * `None` - If no such triplet exists or if input is invalid
///
/// # Examples
/// ```
/// use out_of_bounds_protection::two_pointer::opposite_ends::three_sum;
///
/// let mut nums = vec![-1, 0, 1, 2, -1, -4];
/// nums.sort(); // Ensure the array is sorted
/// let result = three_sum(&nums, 0);
/// assert!(result.is_some());
/// ```
pub fn three_sum(nums: &[i32], target: i32) -> Option<(usize, usize, usize)> {
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
                        }
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
    fn test_reverse_array_normal_case() {
        let mut nums = vec![1, 2, 3, 4, 5];
        reverse_array(&mut nums);
        assert_eq!(nums, vec![5, 4, 3, 2, 1]);

        let mut nums = vec![1, 2, 3, 4];
        reverse_array(&mut nums);
        assert_eq!(nums, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_array_edge_cases() {
        // Empty array
        let mut nums = vec![];
        reverse_array(&mut nums);
        assert_eq!(nums, vec![]);

        // Single element
        let mut nums = vec![42];
        reverse_array(&mut nums);
        assert_eq!(nums, vec![42]);
    }

    #[test]
    fn test_three_sum_normal_case() {
        // The three_sum function expects a sorted array
        let mut nums = vec![-1, 0, 1, 2, -1, -4];
        nums.sort(); // Sort the array first: [-4, -1, -1, 0, 1, 2]
        let result = three_sum(&nums, 0);
        assert!(result.is_some());

        let nums = vec![0, 0, 0];
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
