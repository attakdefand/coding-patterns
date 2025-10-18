//! Implementations of window bounds two-pointer algorithms with integer overflow protection
//!
//! This module demonstrates secure implementations that protect against integer overflow
//! and underflow when computing indices, sums, or sizes in two-pointer algorithms.

/// Finds the maximum area of water that can be stored between two lines with integer overflow protection
/// 
/// # Security Measures Against Integer Overflow/Underflow
/// 1. Uses checked arithmetic operations for area calculations
/// 2. Validates array indices before access
/// 3. Handles extreme value cases safely
/// 4. Provides fallback mechanisms for edge cases
/// 
/// # Arguments
/// * `height` - A slice of integers representing heights of vertical lines
/// 
/// # Returns
/// * `i32` - Maximum area of water that can be stored
/// 
/// # Examples
/// ```
/// use integer_overflow_protection::two_pointer::window_bounds::container_with_most_water_safe;
/// 
/// let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
/// let result = container_with_most_water_safe(&height);
/// assert_eq!(result, 49);
/// ```
pub fn container_with_most_water_safe(height: &[i32]) -> i32 {
    // Handle edge cases
    if height.len() < 2 {
        return 0;
    }
    
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_area: i64 = 0; // Use larger type to prevent overflow
    
    // Main loop with proper termination conditions
    while left < right {
        // Double-check bounds before accessing array elements
        if left >= height.len() || right >= height.len() {
            break;
        }
        
        // Get heights with bounds checking
        let left_height = height[left];
        let right_height = height[right];
        
        // Calculate width safely using checked arithmetic
        let width = match (right as i64).checked_sub(left as i64) {
            Some(w) => w,
            None => {
                // This shouldn't happen with proper bounds, but handle gracefully
                break;
            }
        };
        
        // Calculate height as minimum of two lines
        let current_height = std::cmp::min(left_height, right_height) as i64;
        
        // Calculate area safely with overflow protection
        let current_area = match width.checked_mul(current_height) {
            Some(area) => area,
            None => {
                // Handle overflow by using a large value that won't be exceeded
                i64::MAX
            }
        };
        
        // Update maximum area
        max_area = std::cmp::max(max_area, current_area);
        
        // Move the pointer pointing to the shorter line
        if left_height <= right_height {
            left += 1;
        } else {
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }
    
    // Convert back to i32, saturating if necessary
    max_area.min(i32::MAX as i64) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_with_most_water_safe_normal_case() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = container_with_most_water_safe(&height);
        assert_eq!(result, 49);
        
        let height = vec![1, 1];
        let result = container_with_most_water_safe(&height);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_container_with_most_water_safe_edge_cases() {
        // Empty array
        let height = vec![];
        let result = container_with_most_water_safe(&height);
        assert_eq!(result, 0);
        
        // Single element
        let height = vec![1];
        let result = container_with_most_water_safe(&height);
        assert_eq!(result, 0);
        
        // Two elements
        let height = vec![1, 2];
        let result = container_with_most_water_safe(&height);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_container_with_most_water_safe_large_values() {
        // Test with large values that might cause overflow
        let height = vec![i32::MAX, i32::MAX];
        let result = container_with_most_water_safe(&height);
        // Should handle overflow gracefully
        assert!(result >= 0);
    }
}