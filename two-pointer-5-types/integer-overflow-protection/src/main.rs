//! Demonstration of integer overflow protection in two-pointer algorithms

use integer_overflow_protection::{
    container_with_most_water_safe, find_subarray_sum_safe, three_sum_safe, two_sum_safe,
};

fn main() {
    println!("Integer Overflow Protection Demo");
    println!("================================");

    // Demonstrate two_sum_safe
    let nums = vec![2, 7, 11, 15];
    match two_sum_safe(&nums, 9) {
        Some((i, j)) => println!("two_sum_safe: Found indices {} and {} for target 9", i, j),
        None => println!("two_sum_safe: No pair found for target 9"),
    }

    // Demonstrate three_sum_safe
    let nums = vec![-1, 0, 1, 2, -1, -4];
    match three_sum_safe(&nums, 0) {
        Some((i, j, k)) => println!(
            "three_sum_safe: Found indices {}, {}, and {} for target 0",
            i, j, k
        ),
        None => println!("three_sum_safe: No triplet found for target 0"),
    }

    // Demonstrate container_with_most_water_safe
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let max_area = container_with_most_water_safe(&height);
    println!(
        "container_with_most_water_safe: Maximum area is {}",
        max_area
    );

    // Demonstrate find_subarray_sum_safe
    let nums = vec![1, 4, 2, 7, 3];
    match find_subarray_sum_safe(&nums, 6) {
        Some((start, end)) => println!(
            "find_subarray_sum_safe: Found subarray from index {} to {} for target 6",
            start, end
        ),
        None => println!("find_subarray_sum_safe: No subarray found for target 6"),
    }

    // Demonstrate protection against overflow
    println!("\nTesting protection against integer overflow:");

    // Test with large values that could cause overflow
    let nums = vec![i32::MAX, 1, 2];
    // Use a value that doesn't cause compile-time overflow
    let target = i32::MAX.wrapping_add(1); // This will wrap to i32::MIN
    match two_sum_safe(&nums, target) {
        Some((i, j)) => println!(
            "two_sum_safe with overflow risk: Found indices {} and {}",
            i, j
        ),
        None => println!("two_sum_safe with overflow risk: Handled gracefully (no crash)"),
    }

    let height = vec![i32::MAX, i32::MAX];
    let max_area = container_with_most_water_safe(&height);
    println!(
        "container_with_most_water_safe with overflow risk: Result is {} (handled gracefully)",
        max_area
    );

    println!("\nAll operations completed safely with integer overflow protection!");
}
