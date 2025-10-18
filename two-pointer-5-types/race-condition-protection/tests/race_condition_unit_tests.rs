//! Unit tests for race condition protection

use race_condition_protection::{
    concurrent_two_sum,
    concurrent_string_compare,
    concurrent_array_search,
    concurrent_sorted_intersection,
    TwoPointerState,
    ConcurrentTwoPointer,
};
use race_condition_protection::race_protection::ConcurrentStringComparator;

#[test]
fn test_concurrent_two_sum_basic() {
    let nums = [2, 7, 11, 15];
    assert_eq!(concurrent_two_sum(&nums, 9), Some((0, 1)));
    
    let nums = [3, 2, 4];
    assert_eq!(concurrent_two_sum(&nums, 6), Some((1, 2)));
    
    let nums = [3, 3];
    assert_eq!(concurrent_two_sum(&nums, 6), Some((0, 1)));
}

#[test]
fn test_concurrent_two_sum_not_found() {
    let nums = [1, 2, 3];
    assert_eq!(concurrent_two_sum(&nums, 7), None);
    
    let nums = [];
    assert_eq!(concurrent_two_sum(&nums, 5), None);
    
    let nums = [5];
    assert_eq!(concurrent_two_sum(&nums, 5), None);
}

#[test]
fn test_concurrent_string_compare_equal() {
    assert!(concurrent_string_compare("hello", "hello"));
    assert!(concurrent_string_compare("", ""));
    assert!(concurrent_string_compare("a", "a"));
    assert!(concurrent_string_compare("abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz"));
}

#[test]
fn test_concurrent_string_compare_not_equal() {
    assert!(!concurrent_string_compare("hello", "world"));
    assert!(!concurrent_string_compare("hello", "hello1"));
    assert!(!concurrent_string_compare("hello1", "hello"));
    assert!(!concurrent_string_compare("", "a"));
    assert!(!concurrent_string_compare("a", ""));
}

#[test]
fn test_concurrent_array_search_found() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(concurrent_array_search(&arr, 3), Some(2));
    assert_eq!(concurrent_array_search(&arr, 1), Some(0));
    assert_eq!(concurrent_array_search(&arr, 5), Some(4));
}

#[test]
fn test_concurrent_array_search_not_found() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(concurrent_array_search(&arr, 6), None);
    assert_eq!(concurrent_array_search(&arr, 0), None);
    
    let arr = [];
    assert_eq!(concurrent_array_search(&arr, 1), None);
}

/// Test that concurrent_sorted_intersection produces consistent results
#[test]
fn test_concurrent_sorted_intersection_basic() {
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
fn test_concurrent_sorted_intersection_edge_cases() {
    let arr1 = [];
    let arr2 = [1, 2, 3];
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![] as Vec<i32>);
    
    let arr1 = [1, 2, 3];
    let arr2 = [];
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![] as Vec<i32>);
    
    let arr1 = [];
    let arr2 = [];
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![] as Vec<i32>);
}

#[test]
fn test_two_pointer_state() {
    let data = vec![1, 2, 3, 4, 5];
    let state = TwoPointerState::new(data);
    state.init_right_pointer(5);
    
    assert_eq!(state.left(), 0);
    assert_eq!(state.right(), 4);
    assert!(state.is_active());
    
    state.move_left();
    assert_eq!(state.left(), 1);
    
    state.move_right();
    assert_eq!(state.right(), 3);
}

#[test]
fn test_concurrent_two_pointer() {
    let data = vec![2, 7, 11, 15];
    let algo = ConcurrentTwoPointer::new(data);
    
    let result = algo.find_sum(9);
    assert_eq!(result, Some((0, 1)));
    
    let result = algo.find_sum(18);
    assert_eq!(result, Some((1, 2)));
    
    let result = algo.find_sum(100);
    assert_eq!(result, None);
}

#[test]
fn test_concurrent_string_comparator() {
    let comparator = ConcurrentStringComparator::new("hello", "hello");
    assert!(comparator.compare());
    
    let comparator = ConcurrentStringComparator::new("hello", "world");
    assert!(!comparator.compare());
    
    let comparator = ConcurrentStringComparator::new("", "");
    assert!(comparator.compare());
    
    let comparator = ConcurrentStringComparator::new("a", "");
    assert!(!comparator.compare());
}