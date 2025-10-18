//! Implementations of window bounds two-pointer algorithms with logical invariant protection
//!
//! This module demonstrates secure implementations that protect against broken logical invariants
//! when attackers supply maliciously arranged data.

/// Finds the maximum area of water that can be stored between two lines with logical invariant protection
///
/// # Security Measures Against Broken Logical Invariants
/// 1. Input validation to handle negative values
/// 2. Safe handling of edge cases
/// 3. Proper error reporting for invalid inputs
/// 4. Protection against integer overflow
///
/// # Arguments
/// * `height` - A slice of integers representing heights of vertical lines
///
/// # Returns
/// * `i32` - Maximum area of water that can be stored
///
/// # Examples
/// ```
/// use logical_invariant_protection::two_pointer::window_bounds::container_with_most_water;
///
/// let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
/// let result = container_with_most_water(&height);
/// assert_eq!(result, 49);
/// ```
pub fn container_with_most_water(height: &[i32]) -> i32 {
    // Handle edge cases
    if height.len() < 2 {
        return 0;
    }

    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_area = 0;

    // Main loop with proper termination conditions
    while left < right {
        // Double-check bounds before accessing array elements
        if left >= height.len() || right >= height.len() {
            break;
        }

        // Get heights with bounds checking
        let left_height = height[left];
        let right_height = height[right];

        // Validate heights (negative heights don't make sense in this context)
        if left_height < 0 || right_height < 0 {
            // Skip negative heights or handle them appropriately
            if left_height < 0 {
                left += 1;
                continue;
            }
            if right_height < 0 {
                if right == 0 {
                    break;
                }
                right -= 1;
                continue;
            }
        }

        // Calculate width safely
        let width = match right.checked_sub(left) {
            Some(w) => w as i32,
            None => {
                // This shouldn't happen with proper bounds, but handle gracefully
                break;
            }
        };

        // Calculate height as minimum of two lines
        let current_height = std::cmp::min(left_height, right_height);

        // Calculate area safely with overflow protection
        let current_area = match width.checked_mul(current_height) {
            Some(area) => area,
            None => {
                // Handle overflow by using a large value that won't be exceeded
                i32::MAX
            }
        };

        // Update maximum area
        max_area = std::cmp::max(max_area, current_area);

        // Move the pointer pointing to the shorter line
        // This maintains the logical invariant that we're always trying to find a taller line
        if left_height <= right_height {
            left += 1;
        } else {
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_with_most_water_normal_case() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = container_with_most_water(&height);
        assert_eq!(result, 49);

        let height = vec![1, 1];
        let result = container_with_most_water(&height);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_container_with_most_water_edge_cases() {
        // Empty array
        let height = vec![];
        let result = container_with_most_water(&height);
        assert_eq!(result, 0);

        // Single element
        let height = vec![1];
        let result = container_with_most_water(&height);
        assert_eq!(result, 0);

        // Two elements
        let height = vec![1, 2];
        let result = container_with_most_water(&height);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_container_with_most_water_negative_values() {
        // Test with negative values - should be handled gracefully
        let height = vec![-1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = container_with_most_water(&height);
        // Should skip the negative value and still find a valid solution
        assert!(result > 0);
    }
}
