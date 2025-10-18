# Attack Scenarios and Protection Mechanisms

This document details specific attack scenarios related to "Logical invariants broken by untrusted input" in two-pointer algorithms and how our implementation protects against them.

## Attack Scenario 1: Unsorted Input to Sorted-Dependent Algorithm

### Vulnerable Code Example
```rust
// Vulnerable implementation without logical invariant protection
fn vulnerable_two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Assumes nums is sorted, but what if it's not?
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;  // This only works if array is sorted!
        } else {
            right -= 1; // This only works if array is sorted!
        }
    }
    None
}
```

### Attack Vector
An attacker provides an unsorted array like `[7, 2, 15, 11]` with target `9`. The vulnerable algorithm would fail to find the valid pair `2 + 7 = 9` because it incorrectly moves the pointers based on the assumption that the array is sorted.

### Protection Mechanism
Our implementation validates the sorted assumption and uses a fallback mechanism:

```rust
pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Input validation - check if array is sorted
    if !is_sorted_non_decreasing(nums) {
        // Handle unsorted input by using a hash-based approach
        return two_sum_unsorted_fallback(nums, target);
    }
    // ... continue with normal two-pointer algorithm
}
```

The fallback uses a hash-based approach that doesn't depend on sorted input:
```rust
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
```

## Attack Scenario 2: Unsorted Input to Three-Sum Algorithm

### Attack Vector
An attacker provides an unsorted array to the three-sum algorithm, which typically requires sorted input to work correctly.

### Protection Mechanism
Our implementation preprocesses unsorted input:

```rust
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
```

## Attack Scenario 3: Negative Values in Container-With-Most-Water Algorithm

### Attack Vector
An attacker provides negative height values to the container-with-most-water algorithm, which doesn't make logical sense but could cause unexpected behavior.

### Protection Mechanism
Our implementation safely handles negative values:

```rust
pub fn container_with_most_water(height: &[i32]) -> i32 {
    // ... initialization code ...
    
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
        // ... continue with normal algorithm ...
    }
    
    max_area
}
```

## Attack Scenario 4: Unicode Characters in Palindrome Detection

### Attack Vector
An attacker provides Unicode strings with special characters that might break assumptions in palindrome detection.

### Protection Mechanism
Our implementation uses robust character processing:

```rust
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
```

## Summary of Protection Strategies

1. **Input Validation**: Check assumptions before proceeding
2. **Fallback Mechanisms**: Use alternative algorithms when assumptions are violated
3. **Safe Handling**: Gracefully handle edge cases and invalid inputs
4. **Boundary Checking**: Prevent out-of-bounds access
5. **Overflow Protection**: Handle integer overflow scenarios
6. **Clear Error Reporting**: Provide informative feedback

These protection mechanisms ensure that our two-pointer algorithms work correctly and securely regardless of the input provided by potential attackers.