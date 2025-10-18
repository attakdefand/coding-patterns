# Attack Scenarios and Protection Mechanisms

This document details specific attack scenarios related to "Integer overflow / underflow" in two-pointer algorithms and how our implementation protects against them.

## Attack Scenario 1: Integer Overflow in Sum Calculations

### Vulnerable Code Example
```rust
// Vulnerable implementation without integer overflow protection
fn vulnerable_two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let sum = nums[left] + nums[right]; // Vulnerable to overflow!
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}
```

### Attack Vector
An attacker provides large values like `i32::MAX` and `1` that when added together cause integer overflow, wrapping around to a negative value and bypassing validation checks.

### Protection Mechanism
Our implementation uses checked arithmetic operations:

```rust
pub fn two_sum_safe(nums: &[i32], target: i32) -> Option<(usize, usize)> {
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
```

## Attack Scenario 2: Integer Overflow in Index Calculations

### Attack Vector
An attacker provides inputs that cause index calculations to overflow, potentially leading to out-of-bounds access or incorrect algorithm behavior.

### Protection Mechanism
Our implementation validates array indices before access and uses checked arithmetic for calculations:

```rust
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
        // ... rest of implementation
    }
    
    None
}
```

## Attack Scenario 3: Integer Overflow in Area Calculations

### Attack Vector
An attacker provides large height values that when multiplied together cause integer overflow in area calculations.

### Protection Mechanism
Our implementation uses larger intermediate types and checked arithmetic:

```rust
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
```

## Attack Scenario 4: Integer Underflow in Subtraction Operations

### Attack Vector
An attacker provides values that cause subtraction operations to underflow, wrapping around to large positive values.

### Protection Mechanism
Our implementation uses checked arithmetic operations for all calculations:

```rust
// Safe subtraction with underflow checking
let remaining_target = match target.checked_sub(nums[i]) {
    Some(val) => val,
    None => continue, // Skip if subtraction would overflow
};
```

## Summary of Protection Strategies

1. **Checked Arithmetic Operations**: All arithmetic operations use checked variants
2. **Larger Intermediate Types**: Using `i64` for intermediate calculations to prevent overflow
3. **Safe Error Handling**: Graceful handling of overflow conditions without panicking
4. **Input Validation**: Checking assumptions about input data
5. **Boundary Checking**: Preventing out-of-bounds access
6. **Clear Error Reporting**: Informative error reporting

These protection mechanisms ensure that our two-pointer algorithms work correctly and securely regardless of the input values provided by potential attackers.