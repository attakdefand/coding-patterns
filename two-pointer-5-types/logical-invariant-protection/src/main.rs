//! Demonstration of logical invariant protection in two-pointer algorithms

use logical_invariant_protection::{
    container_with_most_water, is_palindrome, three_sum, two_sum_sorted,
};

fn main() {
    println!("Logical Invariant Protection Demo");
    println!("================================");

    // Demonstrate two_sum_sorted with sorted input
    let nums = vec![2, 7, 11, 15];
    match two_sum_sorted(&nums, 9) {
        Some((i, j)) => println!(
            "two_sum_sorted (sorted): Found indices {} and {} for target 9",
            i, j
        ),
        None => println!("two_sum_sorted (sorted): No pair found for target 9"),
    }

    // Demonstrate two_sum_sorted with unsorted input (fallback mechanism)
    let nums = vec![7, 2, 15, 11];
    match two_sum_sorted(&nums, 9) {
        Some((i, j)) => println!(
            "two_sum_sorted (unsorted): Found indices {} and {} for target 9",
            i, j
        ),
        None => println!("two_sum_sorted (unsorted): No pair found for target 9"),
    }

    // Demonstrate is_palindrome
    let text = "A man, a plan, a canal: Panama";
    if is_palindrome(text) {
        println!("is_palindrome: '{}' is a palindrome", text);
    } else {
        println!("is_palindrome: '{}' is not a palindrome", text);
    }

    // Demonstrate three_sum with sorted input
    let mut nums = vec![-1, 0, 1, 2, -1, -4];
    nums.sort();
    match three_sum(&nums, 0) {
        Some((i, j, k)) => println!(
            "three_sum (sorted): Found indices {}, {}, and {} for target 0",
            i, j, k
        ),
        None => println!("three_sum (sorted): No triplet found for target 0"),
    }

    // Demonstrate three_sum with unsorted input (preprocessing)
    let nums = vec![1, -1, -4, 0, 2, -1];
    match three_sum(&nums, 0) {
        Some((i, j, k)) => println!(
            "three_sum (unsorted): Found indices {}, {}, and {} for target 0",
            i, j, k
        ),
        None => println!("three_sum (unsorted): No triplet found for target 0"),
    }

    // Demonstrate container_with_most_water
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let max_area = container_with_most_water(&height);
    println!("container_with_most_water: Maximum area is {}", max_area);

    println!("\nAll operations completed safely with logical invariant protection!");
}
