//! Demonstration of timing side-channel protection in two-pointer algorithms

use timing_sidechannel_protection::{
    secure_two_sum,
    secure_string_compare,
    secure_array_search,
    secure_sorted_intersection,
    timing_utils::measure_time
};

fn main() {
    println!("=== Timing Side-Channel Protection Demo ===\n");

    // Demonstrate secure two sum
    demo_secure_two_sum();
    
    // Demonstrate secure string comparison
    demo_secure_string_compare();
    
    // Demonstrate secure array search
    demo_secure_array_search();
    
    // Demonstrate secure sorted intersection
    demo_secure_sorted_intersection();
    
    println!("\n=== All demonstrations completed ===");
}

fn demo_secure_two_sum() {
    println!("1. Secure Two Sum:");
    let nums = [2, 7, 11, 15];
    let target = 9;
    
    let (result, elapsed) = measure_time(|| secure_two_sum(&nums, target));
    
    match result {
        Some((i, j)) => println!("  Found indices {} and {} that sum to {}: {:?}", i, j, target, (nums[i], nums[j])),
        None => println!("  No two numbers sum to {}", target),
    }
    
    println!("  Execution time: {} nanoseconds\n", elapsed);
}

fn demo_secure_string_compare() {
    println!("2. Secure String Comparison:");
    let str1 = "hello";
    let str2 = "hello";
    let str3 = "world";
    
    let (result1, elapsed1) = measure_time(|| secure_string_compare(str1, str2));
    let (result2, elapsed2) = measure_time(|| secure_string_compare(str1, str3));
    
    println!("  Comparing '{}' and '{}': {}", str1, str2, result1);
    println!("  Execution time: {} nanoseconds", elapsed1);
    
    println!("  Comparing '{}' and '{}': {}", str1, str3, result2);
    println!("  Execution time: {} nanoseconds\n", elapsed2);
}

fn demo_secure_array_search() {
    println!("3. Secure Array Search:");
    let arr = [1, 3, 5, 7, 9, 11, 13];
    let target1 = 7;
    let target2 = 8;
    
    let (result1, elapsed1) = measure_time(|| secure_array_search(&arr, target1));
    let (result2, elapsed2) = measure_time(|| secure_array_search(&arr, target2));
    
    match result1 {
        Some(index) => println!("  Found {} at index {}", target1, index),
        None => println!("  {} not found", target1),
    }
    println!("  Execution time: {} nanoseconds", elapsed1);
    
    match result2 {
        Some(index) => println!("  Found {} at index {}", target2, index),
        None => println!("  {} not found", target2),
    }
    println!("  Execution time: {} nanoseconds\n", elapsed2);
}

fn demo_secure_sorted_intersection() {
    println!("4. Secure Sorted Intersection:");
    let arr1 = [1, 2, 2, 3, 4, 5];
    let arr2 = [2, 2, 3, 6, 7];
    
    let (result, elapsed) = measure_time(|| secure_sorted_intersection(&arr1, &arr2));
    
    println!("  Intersection of {:?} and {:?}: {:?}", arr1, arr2, result);
    println!("  Execution time: {} nanoseconds\n", elapsed);
}