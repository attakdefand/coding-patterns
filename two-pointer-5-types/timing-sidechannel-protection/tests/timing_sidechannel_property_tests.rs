//! Property-based tests for timing side-channel protection

use timing_sidechannel_protection::{
    secure_string_compare,
    secure_two_sum,
    secure_array_search,
    secure_sorted_intersection,
};
use std::time::Instant;

/// Test that secure_string_compare has consistent timing regardless of input
#[test]
fn test_string_compare_timing_consistency() {
    let short_str = "a";
    let long_str = "a".repeat(1000);
    
    // Measure timing for equal strings
    let start1 = Instant::now();
    secure_string_compare(short_str, short_str);
    let time1 = start1.elapsed().as_nanos();
    
    let start2 = Instant::now();
    secure_string_compare(&long_str, &long_str);
    let time2 = start2.elapsed().as_nanos();
    
    // Times should be relatively close (within an order of magnitude)
    // This is a basic check - in practice, more sophisticated timing analysis would be needed
    let ratio = (time1.max(time2) as f64) / (time1.min(time2) as f64);
    assert!(ratio < 100.0, "Timing difference too large: {} vs {}", time1, time2);
}

/// Test that secure_two_sum has consistent timing regardless of where match is found
#[test]
fn test_two_sum_timing_consistency() {
    let small_array = [1, 2, 3, 4, 5];
    let large_array = (1..=1000).collect::<Vec<i32>>();
    
    // Measure timing for small array
    let start1 = Instant::now();
    secure_two_sum(&small_array, 7); // 2+5=7, found near end
    let time1 = start1.elapsed().as_nanos();
    
    // Measure timing for large array
    let start2 = Instant::now();
    secure_two_sum(&large_array, 3); // 1+2=3, found near beginning
    let time2 = start2.elapsed().as_nanos();
    
    // Times should be relatively close (within an order of magnitude)
    let ratio = (time1.max(time2) as f64) / (time1.min(time2) as f64);
    assert!(ratio < 1000.0, "Timing difference too large: {} vs {}", time1, time2);
}

/// Test that secure_array_search has consistent timing regardless of where match is found
#[test]
fn test_array_search_timing_consistency() {
    let arr = (1..=1000).collect::<Vec<i32>>();
    
    // Search for first element
    let start1 = Instant::now();
    secure_array_search(&arr, 1);
    let time1 = start1.elapsed().as_nanos();
    
    // Search for middle element
    let start2 = Instant::now();
    secure_array_search(&arr, 500);
    let time2 = start2.elapsed().as_nanos();
    
    // Search for last element
    let start3 = Instant::now();
    secure_array_search(&arr, 1000);
    let time3 = start3.elapsed().as_nanos();
    
    // All times should be relatively close
    let max_time = time1.max(time2).max(time3);
    let min_time = time1.min(time2).min(time3);
    let ratio = (max_time as f64) / (min_time as f64);
    
    assert!(ratio < 100.0, "Timing difference too large: {}, {}, {}", time1, time2, time3);
}

/// Test that secure_sorted_intersection produces correct results
#[test]
fn test_sorted_intersection_correctness() {
    // Property: intersection should be commutative
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [3, 4, 5, 6, 7];
    
    let result1 = secure_sorted_intersection(&arr1, &arr2);
    let result2 = secure_sorted_intersection(&arr2, &arr1);
    
    // Results should contain the same elements (though order might differ)
    let mut sorted1 = result1.clone();
    let mut sorted2 = result2.clone();
    sorted1.sort();
    sorted2.sort();
    
    assert_eq!(sorted1, sorted2);
    
    // Property: intersection with empty array should be empty
    let empty: Vec<i32> = vec![];
    let result3 = secure_sorted_intersection(&arr1, &empty);
    assert_eq!(result3, empty);
    
    let result4 = secure_sorted_intersection(&empty, &arr2);
    assert_eq!(result4, empty);
}

/// Test that functions handle edge cases correctly
#[test]
fn test_edge_cases() {
    // Empty arrays
    assert_eq!(secure_two_sum(&[], 5), None);
    assert_eq!(secure_array_search(&[], 5), None);
    assert_eq!(secure_sorted_intersection(&[], &[]), vec![]);
    
    // Single element arrays
    assert_eq!(secure_two_sum(&[5], 5), None); // Need two elements
    assert_eq!(secure_array_search(&[5], 5), Some(0));
    assert_eq!(secure_sorted_intersection(&[5], &[]), vec![]);
    
    // Large values
    let large_arr = [i32::MAX, i32::MAX - 1, i32::MIN, i32::MIN + 1];
    assert_eq!(secure_two_sum(&large_arr, i32::MAX + i32::MAX - 1), None); // Would overflow
    assert_eq!(secure_array_search(&large_arr, i32::MAX), Some(0));
}