# Coding Standards and Best Practices

This document outlines the coding standards and best practices for the logical-invariant-protection project.

## Rust Coding Standards

### Naming Conventions

1. **Variables and Functions**: Use `snake_case`
   ```rust
   let my_variable = 42;
   fn calculate_sum(a: i32, b: i32) -> i32 { a + b }
   ```

2. **Structs, Enums, and Traits**: Use `PascalCase`
   ```rust
   struct DataPoint { x: i32, y: i32 }
   enum HttpStatus { Ok, NotFound, ServerError }
   trait Drawable { fn draw(&self); }
   ```

3. **Constants**: Use `SCREAMING_SNAKE_CASE`
   ```rust
   const MAX_BUFFER_SIZE: usize = 1024;
   ```

4. **Modules**: Use `snake_case`
   ```rust
   mod two_pointer;
   ```

### Code Organization

1. **Module Structure**: Follow the established pattern of organizing by algorithm type
   ```rust
   // lib.rs
   pub mod two_pointer;
   
   // two_pointer/mod.rs
   pub mod opposite_ends;
   pub mod window_bounds;
   
   // two_pointer/opposite_ends.rs
   // Implementation of opposite-ends algorithms
   ```

2. **Function Documentation**: All public functions must have documentation comments
   ```rust
   /// Finds two numbers in a sorted array that sum to a target value
   /// 
   /// # Arguments
   /// * `nums` - A slice of integers (expected to be sorted)
   /// * `target` - The target sum to find
   /// 
   /// # Returns
   /// * `Some((i, j))` - Indices of the two numbers that sum to target
   /// * `None` - If no such pair exists
   pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
       // Implementation
   }
   ```

3. **Error Handling**: Use Rust's `Result` and `Option` types appropriately
   ```rust
   // Good: Return Option for potentially missing values
   fn find_element(arr: &[i32], target: i32) -> Option<usize> {
       arr.iter().position(|&x| x == target)
   }
   
   // Good: Return Result for operations that can fail
   fn parse_data(input: &str) -> Result<Data, ParseError> {
       // Implementation
   }
   ```

### Security Considerations

1. **Input Validation**: Always validate inputs, especially when security is a concern
   ```rust
   fn is_sorted_non_decreasing(nums: &[i32]) -> bool {
       nums.windows(2).all(|w| w[0] <= w[1])
   }
   ```

2. **Bounds Checking**: Explicitly check array bounds when necessary
   ```rust
   if left < nums.len() && right < nums.len() {
       // Safe to access nums[left] and nums[right]
   }
   ```

3. **Integer Overflow Protection**: Use checked arithmetic operations
   ```rust
   match a.checked_add(b) {
       Some(sum) => sum,
       None => return Err("Integer overflow occurred"),
   }
   ```

4. **Fallback Mechanisms**: Implement alternative algorithms for edge cases
   ```rust
   if !is_sorted_non_decreasing(nums) {
       // Handle unsorted input with alternative approach
       return two_sum_unsorted_fallback(nums, target);
   }
   ```

### Testing Standards

1. **Test Organization**: Group related tests in modules
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_normal_case() {
           // Test implementation
       }
       
       #[test]
       fn test_edge_case() {
           // Edge case test
       }
   }
   ```

2. **Property-Based Testing**: Use proptest for randomized testing
   ```rust
   proptest! {
       #[test]
       fn test_property(nums in prop::collection::vec(-1000i32..1000, 0..100)) {
           // Property-based test
       }
   }
   ```

3. **Security-Focused Tests**: Include tests for attack scenarios
   ```rust
   #[test]
   fn test_malicious_input() {
       // Test with deliberately malformed input
   }
   ```

### Documentation Standards

1. **Module Documentation**: Each module should have a descriptive comment
   ```rust
   //! Implementations of opposite-ends (converging) two-pointer algorithms with logical invariant protection
   ```

2. **Function Documentation**: Follow Rust documentation standards
   ```rust
   /// Brief description
   /// 
   /// Detailed description if needed
   /// 
   /// # Arguments
   /// * `param` - Description of parameter
   /// 
   /// # Returns
   /// Description of return value
   /// 
   /// # Examples
   /// ```
   /// use crate::function_name;
   /// 
   /// let result = function_name(param);
   /// assert_eq!(result, expected);
   /// ```
   ```

3. **Error Documentation**: Document possible error conditions
   ```rust
   /// # Errors
   /// This function will return an error if...
   ```

### Performance Considerations

1. **Algorithmic Complexity**: Be aware of time and space complexity
   ```rust
   // Document complexity in comments when non-obvious
   // Time complexity: O(n), Space complexity: O(1)
   ```

2. **Memory Usage**: Minimize unnecessary allocations
   ```rust
   // Prefer reusing existing data structures when possible
   let mut buffer = Vec::with_capacity(expected_size);
   ```

3. **Iterator Usage**: Use iterators for efficient processing
   ```rust
   // Good: Uses iterator chain
   let sum: i32 = nums.iter().sum();
   
   // Avoid: Manual loop when iterator is available
   let mut sum = 0;
   for num in nums {
       sum += num;
   }
   ```

### Code Review Guidelines

1. **Security Review**: Check for:
   - Input validation
   - Bounds checking
   - Integer overflow protection
   - Fallback mechanisms for edge cases

2. **Correctness Review**: Check for:
   - Proper error handling
   - Complete test coverage
   - Algorithmic correctness

3. **Maintainability Review**: Check for:
   - Clear documentation
   - Consistent naming
   - Appropriate code organization

## Tools and Automation

1. **Formatting**: All code must pass `cargo fmt -- --check`
2. **Linting**: All code must pass `cargo clippy -- -D warnings`
3. **Testing**: All tests must pass with `cargo test`
4. **Security**: All dependencies must pass `cargo audit`

## Contributing

When contributing to this project, please:

1. Follow these coding standards
2. Write comprehensive tests
3. Document all public APIs
4. Run all quality checks before submitting a pull request
5. Include security considerations in your implementation

By following these standards, we ensure that the codebase remains maintainable, secure, and of high quality.