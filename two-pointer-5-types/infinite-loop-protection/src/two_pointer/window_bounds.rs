//! Implementations of window bounds two-pointer algorithms with infinite loop protection
//!
//! This module demonstrates secure implementations that prevent infinite loops
//! and high CPU usage through multiple layers of protection, especially important
//! for sliding window algorithms that can be exploited for complexity DoS.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Finds the length of the longest substring without repeating characters with infinite loop protection
///
/// # Security Measures Against Infinite Loops
/// 1. Time-based loop termination to prevent algorithmic DoS
/// 2. Iteration count limiting to prevent unbounded execution
/// 3. HashMap size limiting to prevent memory exhaustion
/// 4. Input validation to prevent complexity attacks
///
/// # Arguments
/// * `s` - The input string
///
/// # Returns
/// * `usize` - Length of the longest substring without repeating characters
///
/// # Examples
/// ```
/// use infinite_loop_protection::two_pointer::window_bounds::longest_substring_without_repeating;
///
/// assert_eq!(longest_substring_without_repeating("abcabcbb"), 3);
/// assert_eq!(longest_substring_without_repeating("bbbbb"), 1);
/// ```
pub fn longest_substring_without_repeating(s: &str) -> usize {
    // Input validation - prevent complexity attacks with large inputs
    if s.len() > 100000 {
        // Limit to 100K characters
        return 0;
    }

    if s.is_empty() {
        return 0;
    }

    let chars: Vec<char> = s.chars().collect();

    // Protection against infinite loops:
    // 1. Time limit (100ms should be more than enough for any reasonable input)
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(100);

    // 2. Iteration limit (prevent unbounded execution)
    let max_iterations = s.len() * 2; // Reasonable upper bound
    let mut iteration_count = 0;

    // 3. HashMap size limit (prevent memory exhaustion)
    let max_map_size = 10000; // Limit distinct characters

    let mut char_index_map: HashMap<char, usize> = HashMap::new();
    let mut max_length = 0;
    let mut left = 0;

    for right in 0..chars.len() {
        // Check time limit to prevent algorithmic DoS
        if start_time.elapsed() > time_limit {
            return max_length; // Return current best result on timeout
        }

        // Check iteration limit
        iteration_count += 1;
        if iteration_count > max_iterations {
            return max_length; // Return current best result on iteration limit
        }

        let current_char = chars[right];

        // Check HashMap size to prevent memory exhaustion
        if char_index_map.len() > max_map_size {
            return max_length; // Return current best result on memory limit
        }

        // If character is already in the current window, move left pointer
        if let Some(&last_index) = char_index_map.get(&current_char) {
            if last_index >= left {
                left = last_index + 1;
            }
        }

        // Update character's latest index
        char_index_map.insert(current_char, right);

        // Update maximum length
        max_length = std::cmp::max(max_length, right - left + 1);
    }

    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_without_repeating_normal_case() {
        assert_eq!(longest_substring_without_repeating("abcabcbb"), 3);
        assert_eq!(longest_substring_without_repeating("bbbbb"), 1);
        assert_eq!(longest_substring_without_repeating("pwwkew"), 3);
    }

    #[test]
    fn test_longest_substring_without_repeating_edge_cases() {
        assert_eq!(longest_substring_without_repeating(""), 0);
        assert_eq!(longest_substring_without_repeating("a"), 1);
        assert_eq!(longest_substring_without_repeating("abcdef"), 6);
    }

    #[test]
    fn test_longest_substring_without_repeating_large_input_protection() {
        // Test with input that's at the limit
        let s: String = "a".repeat(100000);
        let result = longest_substring_without_repeating(&s);
        // Should work fine
        assert_eq!(result, 1);

        // Test with input that exceeds the limit
        let s: String = "a".repeat(100001);
        let result = longest_substring_without_repeating(&s);
        // Should be rejected due to size limit
        assert_eq!(result, 0);
    }
}
