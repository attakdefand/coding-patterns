//! Test for integer underflow attack and defenses
//!
//! Attack: Provide n=0 to cause n-1 underflow in C code → OOB access
//! Defenses: Always check n>=2 before n-1; use safe accessors

use race_condition_protection::{
    concurrent_two_sum, concurrent_array_search, concurrent_sorted_intersection,
    race_protection::ConcurrentTwoPointer,
};

/// Vulnerable C-style implementation that could cause underflow
#[allow(dead_code)]
fn vulnerable_underflow_example(n: usize) -> Option<usize> {
    // In C code, this could cause underflow:
    // if (n > 0) {
    //     size_t index = n - 1;  // If n is 0, this becomes SIZE_MAX (underflow)
    //     return Some(index);
    // }
    
    // Rust prevents this with checked arithmetic, but let's show what could happen
    if n > 0 {
        let index = n - 1; // This is safe in Rust, but shows the pattern
        Some(index)
    } else {
        None // Defense: check bounds first
    }
}

/// Simulate a vulnerable two-pointer implementation
#[allow(dead_code)]
fn vulnerable_two_pointer(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let len = arr.len();
    
    // Vulnerable: No check for minimum array size
    // In C, if len=0, then right=len-1 would underflow to SIZE_MAX
    let mut left = 0;
    let mut right = len - 1; // DANGEROUS: Could underflow if len=0
    
    while left < right {
        let sum = arr[left] + arr[right];
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

#[test]
fn test_underflow_attack_simulation() {
    println!("Testing integer underflow attack simulation...");
    
    // Test with empty array (n=0)
    let empty_array: Vec<i32> = vec![];
    
    // Our secure implementation should handle this gracefully
    let result = concurrent_two_sum(&empty_array, 0);
    assert_eq!(result, None);
    println!("Empty array handled correctly: {:?}", result);
    
    // Test with single element array (n=1)
    let single_array: Vec<i32> = vec![5];
    
    let result = concurrent_two_sum(&single_array, 5);
    assert_eq!(result, None);
    println!("Single element array handled correctly: {:?}", result);
    
    // Test with normal array
    let normal_array: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    let result = concurrent_two_sum(&normal_array, 5);
    assert_eq!(result, Some((0, 3))); // 1 + 4 = 5
    println!("Normal array handled correctly: {:?}", result);
}

#[test]
fn test_concurrent_pointer_implementation() {
    println!("Testing concurrent two-pointer implementation...");
    
    // Test edge cases
    let empty_data: Vec<i32> = vec![];
    let algo = ConcurrentTwoPointer::new(empty_data);
    let result = algo.find_sum(0);
    assert_eq!(result, None);
    println!("Empty data handled correctly: {:?}", result);
    
    let single_data: Vec<i32> = vec![10];
    let algo = ConcurrentTwoPointer::new(single_data);
    let result = algo.find_sum(10);
    assert_eq!(result, None);
    println!("Single element data handled correctly: {:?}", result);
    
    let normal_data: Vec<i32> = vec![2, 7, 11, 15];
    let algo = ConcurrentTwoPointer::new(normal_data);
    let result = algo.find_sum(9);
    assert_eq!(result, Some((0, 1))); // 2 + 7 = 9
    println!("Normal data handled correctly: {:?}", result);
}

#[test]
fn test_array_search_edge_cases() {
    println!("Testing array search with edge cases...");
    
    // Empty array
    let empty: Vec<i32> = vec![];
    let result = concurrent_array_search(&empty, 5);
    assert_eq!(result, None);
    println!("Empty array search result: {:?}", result);
    
    // Single element
    let single = vec![42];
    let result = concurrent_array_search(&single, 42);
    assert_eq!(result, Some(0));
    println!("Single element search result: {:?}", result);
    
    let result = concurrent_array_search(&single, 99);
    assert_eq!(result, None);
    println!("Single element (not found) search result: {:?}", result);
}

#[test]
fn test_sorted_intersection_edge_cases() {
    println!("Testing sorted intersection with edge cases...");
    
    // Empty arrays
    let empty1: Vec<i32> = vec![];
    let empty2: Vec<i32> = vec![];
    let result = concurrent_sorted_intersection(&empty1, &empty2);
    assert_eq!(result, vec![] as Vec<i32>);
    println!("Empty arrays intersection: {:?}", result);
    
    // One empty array
    let arr1 = vec![1, 2, 3];
    let empty: Vec<i32> = vec![];
    let result = concurrent_sorted_intersection(&arr1, &empty);
    assert_eq!(result, vec![] as Vec<i32>);
    println!("One empty array intersection: {:?}", result);
    
    // Normal case
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![3, 4, 5, 6, 7];
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![3, 4, 5]);
    println!("Normal intersection: {:?}", result);
}

#[test]
fn test_safe_arithmetic_operations() {
    println!("Testing safe arithmetic operations...");
    
    // Test saturating arithmetic
    let a: i32 = i32::MAX;
    let b: i32 = 1;
    
    // Normal addition would overflow, but saturating_add handles it
    let result = a.saturating_add(b);
    assert_eq!(result, i32::MAX);
    println!("Saturating add result: {}", result);
    
    // Test wrapping arithmetic
    let result = a.wrapping_add(b);
    assert_eq!(result, i32::MIN);
    println!("Wrapping add result: {}", result);
    
    // Our implementations use wrapping arithmetic for sum calculations
    // to prevent panics, but this is safe because we're just comparing
}

fn main() {
    // Run the tests
    test_underflow_attack_simulation();
    test_concurrent_pointer_implementation();
    test_array_search_edge_cases();
    test_sorted_intersection_edge_cases();
    test_safe_arithmetic_operations();
    
    println!("Integer underflow attack exploration completed!");
}