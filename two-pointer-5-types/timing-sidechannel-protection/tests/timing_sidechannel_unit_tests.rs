//! Unit tests for timing side-channel protection

use timing_sidechannel_protection::{
    secure_array_search, secure_sorted_intersection, secure_string_compare, secure_two_sum,
};

#[test]
fn test_secure_two_sum_basic() {
    let nums = [2, 7, 11, 15];
    assert_eq!(secure_two_sum(&nums, 9), Some((0, 1)));

    let nums = [3, 2, 4];
    assert_eq!(secure_two_sum(&nums, 6), Some((1, 2)));

    let nums = [3, 3];
    assert_eq!(secure_two_sum(&nums, 6), Some((0, 1)));
}

#[test]
fn test_secure_two_sum_not_found() {
    let nums = [1, 2, 3];
    assert_eq!(secure_two_sum(&nums, 7), None);

    let nums = [];
    assert_eq!(secure_two_sum(&nums, 5), None);

    let nums = [5];
    assert_eq!(secure_two_sum(&nums, 5), None);
}

#[test]
fn test_secure_string_compare_equal() {
    assert!(secure_string_compare("hello", "hello"));
    assert!(secure_string_compare("", ""));
    assert!(secure_string_compare("a", "a"));
    assert!(secure_string_compare(
        "abcdefghijklmnopqrstuvwxyz",
        "abcdefghijklmnopqrstuvwxyz"
    ));
}

#[test]
fn test_secure_string_compare_not_equal() {
    assert!(!secure_string_compare("hello", "world"));
    assert!(!secure_string_compare("hello", "hello1"));
    assert!(!secure_string_compare("hello1", "hello"));
    assert!(!secure_string_compare("", "a"));
    assert!(!secure_string_compare("a", ""));
}

#[test]
fn test_secure_array_search_found() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(secure_array_search(&arr, 3), Some(2));
    assert_eq!(secure_array_search(&arr, 1), Some(0));
    assert_eq!(secure_array_search(&arr, 5), Some(4));
}

#[test]
fn test_secure_array_search_not_found() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(secure_array_search(&arr, 6), None);
    assert_eq!(secure_array_search(&arr, 0), None);

    let arr = [];
    assert_eq!(secure_array_search(&arr, 1), None);
}

#[test]
fn test_secure_sorted_intersection_basic() {
    let arr1 = [1, 2, 2, 3, 4];
    let arr2 = [2, 2, 3, 5, 6];
    let result = secure_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![2, 2, 3]);

    let arr1 = [1, 3, 5];
    let arr2 = [2, 4, 6];
    let result = secure_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![]);
}

#[test]
fn test_secure_sorted_intersection_edge_cases() {
    let arr1 = [];
    let arr2 = [1, 2, 3];
    let result = secure_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![]);

    let arr1 = [1, 2, 3];
    let arr2 = [];
    let result = secure_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![]);

    let arr1 = [];
    let arr2 = [];
    let result = secure_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![]);
}
