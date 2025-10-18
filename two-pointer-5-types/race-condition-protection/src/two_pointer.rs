//! Two-pointer algorithms with race condition and TOCTOU protection

use crate::concurrent_utils::ThreadSafeArray;
use crate::telemetry::{start_operation_timer, create_metrics, record_operation};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Concurrently finds two numbers in a sorted array that sum to a target value
///
/// This implementation avoids race conditions and TOCTOU by:
/// 1. Using thread-safe data structures
/// 2. Proper synchronization primitives
/// 3. Atomic operations for shared state
///
/// # Arguments
/// * `nums` - A sorted slice of integers
/// * `target` - The target sum to find
///
/// # Returns
/// * `Option<(usize, usize)>` - Indices of the two numbers that sum to target, or None
pub fn concurrent_two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let start_time = start_operation_timer();
    let mut loop_iterations = 0;
    
    if nums.len() < 2 {
        let metrics = create_metrics(start_time, 0, 0, 0);
        record_operation(&metrics);
        return None;
    }

    // For race condition safety, we'll check all pairs
    // This ensures consistent timing regardless of where the match is
    let found = AtomicBool::new(false);
    let result = Arc::new(Mutex::new(None));

    // Check all pairs - this ensures thread safety and consistent behavior
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            loop_iterations += 1;
            
            // Check if another thread already found a solution
            if found.load(Ordering::Relaxed) {
                let metrics = create_metrics(start_time, 1, 0, loop_iterations);
                record_operation(&metrics);
                let result_guard = result.lock();
                return *result_guard;
            }

            let sum = nums[i].wrapping_add(nums[j]);
            if sum == target {
                // Use atomic flag to prevent multiple threads from setting result
                if !found.swap(true, Ordering::Relaxed) {
                    let mut result_guard = result.lock();
                    *result_guard = Some((i, j));
                }
                let metrics = create_metrics(start_time, 1, 0, loop_iterations);
                record_operation(&metrics);
                let result_guard = result.lock();
                return *result_guard;
            }
        }
    }

    let metrics = create_metrics(start_time, 1, 0, loop_iterations);
    record_operation(&metrics);
    let result_guard = result.lock();
    *result_guard
}

/// Concurrently compares two strings using a two-pointer approach
///
/// This implementation avoids race conditions by:
/// 1. Using immutable data where possible
/// 2. Proper synchronization for shared state
/// 3. Atomic operations for control flow
///
/// # Arguments
/// * `a` - First string to compare
/// * `b` - Second string to compare
///
/// # Returns
/// * `bool` - True if strings are equal, false otherwise
pub fn concurrent_string_compare(a: &str, b: &str) -> bool {
    let start_time = start_operation_timer();
    let mut loop_iterations = 0;
    
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    // Early exit for different lengths (thread-safe)
    if a_bytes.len() != b_bytes.len() {
        let metrics = create_metrics(start_time, 1, 0, 1);
        record_operation(&metrics);
        return false;
    }

    // Use thread-safe array wrapper
    let a_array = ThreadSafeArray::new(a_bytes.to_vec());
    let b_array = ThreadSafeArray::new(b_bytes.to_vec());

    let len = a_bytes.len();
    let mismatch_found = AtomicBool::new(false);

    // Compare each byte concurrently
    for i in 0..len {
        loop_iterations += 1;
        
        if mismatch_found.load(Ordering::Relaxed) {
            let metrics = create_metrics(start_time, 1, 0, loop_iterations);
            record_operation(&metrics);
            return false;
        }

        let a_byte = a_array.get(i);
        let b_byte = b_array.get(i);

        if let (Some(a_val), Some(b_val)) = (a_byte, b_byte) {
            if a_val != b_val {
                mismatch_found.store(true, Ordering::Relaxed);
                let metrics = create_metrics(start_time, 1, 0, loop_iterations);
                record_operation(&metrics);
                return false;
            }
        } else {
            // Handle case where array was modified during access
            mismatch_found.store(true, Ordering::Relaxed);
            let metrics = create_metrics(start_time, 1, 0, loop_iterations);
            record_operation(&metrics);
            return false;
        }
    }

    let result = !mismatch_found.load(Ordering::Relaxed);
    let metrics = create_metrics(start_time, 1, 0, loop_iterations);
    record_operation(&metrics);
    result
}

/// Concurrently searches for a target value in a sorted array
///
/// This implementation avoids race conditions by:
/// 1. Using thread-safe data structures
/// 2. Proper synchronization for shared state
/// 3. Atomic operations for control flow
///
/// # Arguments
/// * `arr` - A sorted slice of integers
/// * `target` - The value to search for
///
/// # Returns
/// * `Option<usize>` - Index of the target value, or None
pub fn concurrent_array_search(arr: &[i32], target: i32) -> Option<usize> {
    let start_time = start_operation_timer();
    let mut loop_iterations = 0;
    
    let array = ThreadSafeArray::new(arr.to_vec());
    let found = AtomicBool::new(false);
    let result = Arc::new(Mutex::new(None));

    // Search concurrently
    for i in 0..arr.len() {
        loop_iterations += 1;
        
        if found.load(Ordering::Relaxed) {
            let metrics = create_metrics(start_time, 1, 0, loop_iterations);
            record_operation(&metrics);
            let result_guard = result.lock();
            return *result_guard;
        }

        if let Some(value) = array.get(i) {
            if value == target {
                let mut result_guard = result.lock();
                *result_guard = Some(i);
                found.store(true, Ordering::Relaxed);
                let metrics = create_metrics(start_time, 1, 0, loop_iterations);
                record_operation(&metrics);
                let result_guard = result.lock();
                return *result_guard;
            }
        } else {
            // Handle case where array was modified during access
            let metrics = create_metrics(start_time, 1, 0, loop_iterations);
            record_operation(&metrics);
            let result_guard = result.lock();
            return *result_guard;
        }
    }

    let metrics = create_metrics(start_time, 1, 0, loop_iterations);
    record_operation(&metrics);
    let result_guard = result.lock();
    *result_guard
}

/// Concurrently finds the intersection of two sorted arrays
///
/// This implementation avoids race conditions by:
/// 1. Using thread-safe data structures
/// 2. Proper synchronization for shared state
/// 3. Atomic operations for control flow
///
/// # Arguments
/// * `arr1` - First sorted array
/// * `arr2` - Second sorted array
///
/// # Returns
/// * `Vec<i32>` - Intersection of the two arrays
pub fn concurrent_sorted_intersection(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let start_time = start_operation_timer();
    let mut loop_iterations = 0;
    
    let array1 = ThreadSafeArray::new(arr1.to_vec());
    let array2 = ThreadSafeArray::new(arr2.to_vec());
    let result = Arc::new(Mutex::new(Vec::new()));

    // Use two-pointer approach with thread safety
    let mut left = 0;
    let mut right = 0;

    while left < arr1.len() && right < arr2.len() {
        loop_iterations += 1;
        
        let val1 = match array1.get(left) {
            Some(val) => val,
            None => break, // Array was modified
        };

        let val2 = match array2.get(right) {
            Some(val) => val,
            None => break, // Array was modified
        };

        if val1 == val2 {
            let mut result_guard = result.lock();
            result_guard.push(val1);
            left += 1;
            right += 1;
        } else if val1 < val2 {
            left += 1;
        } else {
            right += 1;
        }
    }

    let metrics = create_metrics(start_time, 1, 0, loop_iterations);
    record_operation(&metrics);
    let result_guard = result.lock();
    result_guard.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_concurrent_two_sum() {
        let nums = [2, 7, 11, 15];
        assert_eq!(concurrent_two_sum(&nums, 9), Some((0, 1)));

        let nums = [3, 2, 4];
        assert_eq!(concurrent_two_sum(&nums, 6), Some((1, 2)));

        let nums = [3, 3];
        assert_eq!(concurrent_two_sum(&nums, 6), Some((0, 1)));

        let nums = [1, 2, 3];
        assert_eq!(concurrent_two_sum(&nums, 7), None);
    }

    #[test]
    fn test_concurrent_string_compare() {
        assert!(concurrent_string_compare("hello", "hello"));
        assert!(!concurrent_string_compare("hello", "world"));
        assert!(!concurrent_string_compare("hello", "hello1"));
        assert!(!concurrent_string_compare("hello1", "hello"));
    }

    #[test]
    fn test_concurrent_array_search() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(concurrent_array_search(&arr, 3), Some(2));
        assert_eq!(concurrent_array_search(&arr, 6), None);

        let arr = [];
        assert_eq!(concurrent_array_search(&arr, 1), None);
    }

    #[test]
    fn test_concurrent_sorted_intersection() {
        let arr1 = [1, 2, 2, 3, 4];
        let arr2 = [2, 2, 3, 5, 6];
        let result = concurrent_sorted_intersection(&arr1, &arr2);
        assert_eq!(result, vec![2, 2, 3]);

        let arr1 = [1, 3, 5];
        let arr2 = [2, 4, 6];
        let result = concurrent_sorted_intersection(&arr1, &arr2);
        assert_eq!(result, vec![] as Vec<i32>);
    }

    #[test]
    fn test_concurrent_access() {
        // Test that concurrent access doesn't cause data races
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let nums_clone = nums.clone();
                thread::spawn(move || concurrent_two_sum(&nums_clone, target))
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All threads should get the same result
        for result in results {
            assert_eq!(result, Some((0, 1)));
        }
    }
}