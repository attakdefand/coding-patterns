//! Property-based tests for race condition protection

use race_condition_protection::{
    concurrent_array_search, concurrent_sorted_intersection, concurrent_string_compare,
    concurrent_two_sum, ConcurrentTwoPointer,
};
use std::sync::Arc;
use std::thread;

/// Test that concurrent_two_sum produces consistent results under concurrent access
#[test]
fn test_concurrent_two_sum_consistency() {
    let nums = Arc::new([2, 7, 11, 15, 20, 25]);
    let target = 9;

    // Run multiple threads concurrently
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let nums_clone = Arc::clone(&nums);
            thread::spawn(move || concurrent_two_sum(&*nums_clone, target))
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be the same
    for result in results {
        assert_eq!(result, Some((0, 1)));
    }
}

/// Test that concurrent_string_compare produces consistent results
#[test]
fn test_concurrent_string_compare_consistency() {
    let str1 = Arc::new("hello world");
    let str2 = Arc::new("hello world");
    let str3 = Arc::new("different string");

    // Run multiple threads concurrently for equal strings
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let s1_clone = Arc::clone(&str1);
            let s2_clone = Arc::clone(&str2);
            thread::spawn(move || concurrent_string_compare(&*s1_clone, &*s2_clone))
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be true
    for result in results {
        assert!(result);
    }

    // Run multiple threads concurrently for different strings
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let s1_clone = Arc::clone(&str1);
            let s3_clone = Arc::clone(&str3);
            thread::spawn(move || concurrent_string_compare(&*s1_clone, &*s3_clone))
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be false
    for result in results {
        assert!(!result);
    }
}

/// Test that concurrent_array_search produces consistent results
#[test]
fn test_concurrent_array_search_consistency() {
    let arr = Arc::new([1, 3, 5, 7, 9, 11, 13]);
    let target = 7;
    let not_found_target = 8;

    // Run multiple threads concurrently for found target
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let arr_clone = Arc::clone(&arr);
            thread::spawn(move || concurrent_array_search(&*arr_clone, target))
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be the same
    for result in results {
        assert_eq!(result, Some(3));
    }

    // Run multiple threads concurrently for not found target
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let arr_clone = Arc::clone(&arr);
            thread::spawn(move || concurrent_array_search(&*arr_clone, not_found_target))
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be None
    for result in results {
        assert_eq!(result, None);
    }
}

/// Test that concurrent_sorted_intersection produces consistent results
#[test]
fn test_concurrent_sorted_intersection_consistency() {
    let arr1 = Arc::new([1, 2, 2, 3, 4, 5]);
    let arr2 = Arc::new([2, 2, 3, 6, 7]);
    let expected: Vec<i32> = vec![2, 2, 3];

    // Run multiple threads concurrently
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let arr1_clone = Arc::clone(&arr1);
            let arr2_clone = Arc::clone(&arr2);
            thread::spawn(move || concurrent_sorted_intersection(&*arr1_clone, &*arr2_clone))
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be the same
    for result in results {
        assert_eq!(result, expected);
    }
}

/// Test that ConcurrentTwoPointer produces consistent results under concurrent access
#[test]
fn test_concurrent_two_pointer_consistency() {
    let data = vec![2, 7, 11, 15, 20, 25];
    let algo = Arc::new(ConcurrentTwoPointer::new(data));
    let target = 22; // 7 + 15 = 22 or 2 + 20 = 22

    // Run multiple threads concurrently
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let algo_clone = Arc::clone(&algo);
            thread::spawn(move || algo_clone.find_sum(target))
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be valid (multiple valid pairs exist)
    for result in results {
        assert!(result.is_some());
        if let Some((i, j)) = result {
            // Verify indices are valid
            assert!(i < 6 && j < 6 && i != j); // Valid indices for our 6-element array
        }
    }
}

/// Test mathematical properties of concurrent operations
#[test]
fn test_mathematical_properties() {
    // Property: intersection should be commutative
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [3, 4, 5, 6, 7];

    let result1 = concurrent_sorted_intersection(&arr1, &arr2);
    let result2 = concurrent_sorted_intersection(&arr2, &arr1);

    // Results should contain the same elements (though order might differ)
    let mut sorted1 = result1.clone();
    let mut sorted2 = result2.clone();
    sorted1.sort();
    sorted2.sort();

    assert_eq!(sorted1, sorted2);

    // Property: intersection with empty array should be empty
    let empty: Vec<i32> = vec![];
    let result3 = concurrent_sorted_intersection(&arr1, &empty);
    assert_eq!(result3, empty);

    let result4 = concurrent_sorted_intersection(&empty, &arr2);
    assert_eq!(result4, empty);
}

/// Test edge cases for concurrent operations
#[test]
fn test_edge_cases() {
    // Empty arrays
    assert_eq!(concurrent_two_sum(&[], 5), None);
    assert_eq!(concurrent_array_search(&[], 5), None);
    assert_eq!(concurrent_sorted_intersection(&[], &[]), vec![] as Vec<i32>);

    // Single element arrays
    assert_eq!(concurrent_two_sum(&[5], 5), None); // Need two elements
    assert_eq!(concurrent_array_search(&[5], 5), Some(0));
    assert_eq!(
        concurrent_sorted_intersection(&[5], &[]),
        vec![] as Vec<i32>
    );

    // Large arrays
    let large_arr: Vec<i32> = (1..=1000).collect();
    assert_eq!(concurrent_array_search(&large_arr, 500), Some(499));
}
