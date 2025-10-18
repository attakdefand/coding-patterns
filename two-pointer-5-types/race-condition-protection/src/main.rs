//! Demonstration of race condition protection in two-pointer algorithms

use race_condition_protection::{
    concurrent_two_sum,
    concurrent_string_compare,
    concurrent_array_search,
    concurrent_sorted_intersection,
    ConcurrentTwoPointer,
    ConcurrentStringComparator,
};
use std::time::Instant;

fn main() {
    println!("=== Race Condition Protection Demo ===\n");

    // Demonstrate concurrent two sum
    demo_concurrent_two_sum();
    
    // Demonstrate concurrent string comparison
    demo_concurrent_string_compare();
    
    // Demonstrate concurrent array search
    demo_concurrent_array_search();
    
    // Demonstrate concurrent sorted intersection
    demo_concurrent_sorted_intersection();
    
    // Demonstrate advanced concurrent two-pointer
    demo_advanced_concurrent_two_pointer();
    
    println!("\n=== All demonstrations completed ===");
}

fn demo_concurrent_two_sum() {
    println!("1. Concurrent Two Sum:");
    let nums = [2, 7, 11, 15];
    let target = 9;
    
    let start = Instant::now();
    let result = concurrent_two_sum(&nums, target);
    let elapsed = start.elapsed();
    
    match result {
        Some((i, j)) => println!("  Found indices {} and {} that sum to {}: {:?}", i, j, target, (nums[i], nums[j])),
        None => println!("  No two numbers sum to {}", target),
    }
    
    println!("  Execution time: {:?}\n", elapsed);
}

fn demo_concurrent_string_compare() {
    println!("2. Concurrent String Comparison:");
    let str1 = "hello";
    let str2 = "hello";
    let str3 = "world";
    
    let start1 = Instant::now();
    let result1 = concurrent_string_compare(str1, str2);
    let elapsed1 = start1.elapsed();
    
    let start2 = Instant::now();
    let result2 = concurrent_string_compare(str1, str3);
    let elapsed2 = start2.elapsed();
    
    println!("  Comparing '{}' and '{}': {}", str1, str2, result1);
    println!("  Execution time: {:?}", elapsed1);
    
    println!("  Comparing '{}' and '{}': {}", str1, str3, result2);
    println!("  Execution time: {:?}\n", elapsed2);
}

fn demo_concurrent_array_search() {
    println!("3. Concurrent Array Search:");
    let arr = [1, 3, 5, 7, 9, 11, 13];
    let target1 = 7;
    let target2 = 8;
    
    let start1 = Instant::now();
    let result1 = concurrent_array_search(&arr, target1);
    let elapsed1 = start1.elapsed();
    
    let start2 = Instant::now();
    let result2 = concurrent_array_search(&arr, target2);
    let elapsed2 = start2.elapsed();
    
    match result1 {
        Some(index) => println!("  Found {} at index {}", target1, index),
        None => println!("  {} not found", target1),
    }
    println!("  Execution time: {:?}", elapsed1);
    
    match result2 {
        Some(index) => println!("  Found {} at index {}", target2, index),
        None => println!("  {} not found", target2),
    }
    println!("  Execution time: {:?}\n", elapsed2);
}

fn demo_concurrent_sorted_intersection() {
    println!("4. Concurrent Sorted Intersection:");
    let arr1 = [1, 2, 2, 3, 4, 5];
    let arr2 = [2, 2, 3, 6, 7];
    
    let start = Instant::now();
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    let elapsed = start.elapsed();
    
    println!("  Intersection of {:?} and {:?}: {:?}", arr1, arr2, result);
    println!("  Execution time: {:?}\n", elapsed);
}

fn demo_advanced_concurrent_two_pointer() {
    println!("5. Advanced Concurrent Two-Pointer:");
    let data = vec![2, 7, 11, 15, 20, 25];
    let algo = ConcurrentTwoPointer::new(data);
    
    let start = Instant::now();
    let result = algo.find_sum(22); // 7 + 15 = 22
    let elapsed = start.elapsed();
    
    match result {
        Some((i, j)) => println!("  Found indices {} and {} that sum to 22", i, j),
        None => println!("  No two numbers sum to 22"),
    }
    
    println!("  Operations performed: {}", algo.operation_count());
    println!("  Execution time: {:?}\n", elapsed);
}