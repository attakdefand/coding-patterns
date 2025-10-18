//! Demonstration of infinite loop protection in two-pointer algorithms

use infinite_loop_protection::{
    two_sum_sorted,
    find_middle,
    longest_substring_without_repeating,
    merge_sorted_arrays,
};
use infinite_loop_protection::two_pointer::same_direction::ListNode;

fn main() {
    println!("Infinite Loop Protection Demo");
    println!("============================");
    
    // Demonstrate two_sum_sorted
    let nums = vec![2, 7, 11, 15];
    match two_sum_sorted(&nums, 9) {
        Some((i, j)) => println!("two_sum_sorted: Found indices {} and {} for target 9", i, j),
        None => println!("two_sum_sorted: No pair found for target 9"),
    }
    
    // Demonstrate find_middle
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(2);
    let mut node3 = ListNode::new(3);
    let mut node4 = ListNode::new(4);
    let node5 = ListNode::new(5);
    
    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));
    
    let binding = Some(Box::new(node1));
    let middle = find_middle(&binding);
    match middle {
        Some(node) => println!("find_middle: Middle node value is {}", node.val),
        None => println!("find_middle: No middle node found"),
    }
    
    // Demonstrate longest_substring_without_repeating
    let text = "abcabcbb";
    let length = longest_substring_without_repeating(text);
    println!("longest_substring_without_repeating: '{}' has longest substring of length {}", 
             text, length);
    
    // Demonstrate merge_sorted_arrays
    let nums1 = vec![1, 2, 4];
    let nums2 = vec![1, 3, 4];
    let merged = merge_sorted_arrays(&nums1, &nums2);
    println!("merge_sorted_arrays: {:?} + {:?} = {:?}", nums1, nums2, merged);
    
    println!("\nAll operations completed safely with infinite loop protection!");
}