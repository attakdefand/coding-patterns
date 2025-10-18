//! Demonstration of out-of-bounds protection in two-pointer algorithms

use out_of_bounds_protection::{
    two_sum_sorted,
    is_palindrome,
    reverse_array,
    three_sum,
};

fn main() {
    println!("Out-of-Bounds Protection Demo");
    println!("============================");
    
    // Demonstrate two_sum_sorted
    let nums = vec![2, 7, 11, 15];
    match two_sum_sorted(&nums, 9) {
        Some((i, j)) => println!("two_sum_sorted: Found indices {} and {} for target 9", i, j),
        None => println!("two_sum_sorted: No pair found for target 9"),
    }
    
    // Demonstrate is_palindrome
    let text = "A man, a plan, a canal: Panama";
    if is_palindrome(text) {
        println!("is_palindrome: '{}' is a palindrome", text);
    } else {
        println!("is_palindrome: '{}' is not a palindrome", text);
    }
    
    // Demonstrate reverse_array
    let mut arr = vec![1, 2, 3, 4, 5];
    println!("Original array: {:?}", arr);
    reverse_array(&mut arr);
    println!("Reversed array: {:?}", arr);
    
    // Demonstrate three_sum
    let mut nums = vec![-1, 0, 1, 2, -1, -4];
    nums.sort();
    match three_sum(&nums, 0) {
        Some((i, j, k)) => println!("three_sum: Found indices {}, {}, and {} for target 0", i, j, k),
        None => println!("three_sum: No triplet found for target 0"),
    }
    
    println!("\nAll operations completed safely with out-of-bounds protection!");
}