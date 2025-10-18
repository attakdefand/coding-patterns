use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::Filter;

#[derive(Serialize, Debug)]
struct PatternInfo {
    id: String,
    name: String,
    category: String,
    description: String,
    complexity: String,
    security_considerations: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ExecuteRequest {
    pattern_id: String,
    input: String,
}

#[derive(Serialize, Debug)]
struct ExecuteResponse {
    result: String,
    execution_time_ms: u128,
    security_checks: Vec<String>,
}

pub async fn start_server() {
    // GET /patterns - Return information about all patterns
    let patterns_route = warp::path("patterns")
        .and(warp::get())
        .map(|| warp::reply::json(&get_patterns_info()));

    // POST /execute - Execute an algorithm
    let execute_route = warp::path("execute")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(execute_algorithm);

    // Serve static files
    let static_files = warp::fs::dir("static");

    // Combine routes
    let routes = patterns_route
        .or(execute_route)
        .or(static_files)
        .with(warp::cors().allow_any_origin());

    println!("Server running on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn get_patterns_info() -> Vec<PatternInfo> {
    let mut patterns = Vec::new();

    // Array Problems
    patterns.push(PatternInfo {
        id: "container_with_most_water".to_string(),
        name: "Container With Most Water".to_string(),
        category: "Array Problems".to_string(),
        description: "Find two lines that together with the x-axis form a container that holds the most water.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Input validation for array length".to_string(),
            "Integer overflow protection for area calculation".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "merge_sorted_arrays".to_string(),
        name: "Merge Sorted Arrays".to_string(),
        category: "Array Problems".to_string(),
        description: "Merge two sorted arrays into one sorted array.".to_string(),
        complexity: "O(m+n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Bounds checking for array access".to_string(),
            "Memory safety for in-place modifications".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "remove_duplicates".to_string(),
        name: "Remove Duplicates".to_string(),
        category: "Array Problems".to_string(),
        description: "Remove duplicates from a sorted array in-place.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Index bounds validation".to_string(),
            "Prevent underflow in pointer arithmetic".to_string(),
        ],
    });

    // Bidirectional Merge
    patterns.push(PatternInfo {
        id: "bidirectional_merge_sorted_arrays".to_string(),
        name: "Bidirectional Merge Sorted Arrays".to_string(),
        category: "Bidirectional Merge".to_string(),
        description: "Merge two sorted arrays from both ends toward the center.".to_string(),
        complexity: "O(m+n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Concurrent access protection".to_string(),
            "Thread safety for shared data".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "intersection_sorted_arrays".to_string(),
        name: "Intersection of Sorted Arrays".to_string(),
        category: "Bidirectional Merge".to_string(),
        description: "Find common elements between two sorted arrays.".to_string(),
        complexity: "O(m+n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Input validation for null arrays".to_string(),
            "Memory bounds checking".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "sorted_squares".to_string(),
        name: "Sorted Squares".to_string(),
        category: "Bidirectional Merge".to_string(),
        description: "Square each element in a sorted array and return the result sorted.".to_string(),
        complexity: "O(n) time, O(n) space".to_string(),
        security_considerations: vec![
            "Integer overflow protection for squaring".to_string(),
            "Memory allocation safety".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "union_sorted_arrays".to_string(),
        name: "Union of Sorted Arrays".to_string(),
        category: "Bidirectional Merge".to_string(),
        description: "Find all unique elements from two sorted arrays.".to_string(),
        complexity: "O(m+n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Duplicate element handling".to_string(),
            "Memory access bounds".to_string(),
        ],
    });

    // Opposite Ends
    patterns.push(PatternInfo {
        id: "is_palindrome".to_string(),
        name: "Is Palindrome".to_string(),
        category: "Opposite Ends".to_string(),
        description: "Check if a string is a palindrome using two pointers from opposite ends.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Unicode character handling".to_string(),
            "String bounds validation".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "reverse_array".to_string(),
        name: "Reverse Array".to_string(),
        category: "Opposite Ends".to_string(),
        description: "Reverse an array in-place using two pointers from opposite ends.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "In-place modification safety".to_string(),
            "Index bounds checking".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "three_sum".to_string(),
        name: "Three Sum".to_string(),
        category: "Opposite Ends".to_string(),
        description: "Find all unique triplets that sum to zero.".to_string(),
        complexity: "O(n²) time, O(1) space".to_string(),
        security_considerations: vec![
            "Duplicate triplet prevention".to_string(),
            "Integer overflow protection".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "opposite_ends_two_sum_sorted".to_string(),
        name: "Two Sum (Sorted Array)".to_string(),
        category: "Opposite Ends".to_string(),
        description: "Find two numbers that add up to a target in a sorted array.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Target validation".to_string(),
            "Index bounds checking".to_string(),
        ],
    });

    // Partitioning
    patterns.push(PatternInfo {
        id: "partition_even_odd".to_string(),
        name: "Partition Even/Odd".to_string(),
        category: "Partitioning".to_string(),
        description: "Partition an array so even numbers come before odd numbers.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Element validation".to_string(),
            "Memory safety for swaps".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "remove_duplicates_sorted".to_string(),
        name: "Remove Duplicates (Sorted)".to_string(),
        category: "Partitioning".to_string(),
        description: "Remove duplicates from a sorted array using partitioning technique.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Index bounds validation".to_string(),
            "Underflow protection".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "remove_element".to_string(),
        name: "Remove Element".to_string(),
        category: "Partitioning".to_string(),
        description: "Remove all instances of a value from an array in-place.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Value validation".to_string(),
            "Memory access bounds".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "sort_colors".to_string(),
        name: "Sort Colors".to_string(),
        category: "Partitioning".to_string(),
        description: "Sort an array of colors (represented by 0, 1, 2) in-place.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Color value validation".to_string(),
            "Index bounds checking".to_string(),
        ],
    });

    // Same Direction
    patterns.push(PatternInfo {
        id: "find_middle".to_string(),
        name: "Find Middle Element".to_string(),
        category: "Same Direction".to_string(),
        description: "Find the middle element of a linked list using slow and fast pointers.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Null pointer checks".to_string(),
            "Cycle detection".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "has_cycle".to_string(),
        name: "Detect Cycle".to_string(),
        category: "Same Direction".to_string(),
        description: "Detect if a linked list has a cycle using Floyd's cycle detection algorithm.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Memory safety for node traversal".to_string(),
            "Infinite loop prevention".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "kth_from_end".to_string(),
        name: "Kth Element from End".to_string(),
        category: "Same Direction".to_string(),
        description: "Find the kth element from the end of a linked list.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "K value validation".to_string(),
            "List bounds checking".to_string(),
        ],
    });

    // Sliding Window
    patterns.push(PatternInfo {
        id: "find_pair_with_product".to_string(),
        name: "Find Pair with Product".to_string(),
        category: "Sliding Window".to_string(),
        description: "Find a pair of elements in a sorted array whose product equals a target.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Zero division protection".to_string(),
            "Integer overflow checking".to_string(),
        ],
    });

    // Sum Problems
    patterns.push(PatternInfo {
        id: "find_triplet_sum".to_string(),
        name: "Find Triplet Sum".to_string(),
        category: "Sum Problems".to_string(),
        description: "Find three elements in an array that sum to a target value.".to_string(),
        complexity: "O(n²) time, O(1) space".to_string(),
        security_considerations: vec![
            "Target validation".to_string(),
            "Integer overflow protection".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "two_sum_sorted".to_string(),
        name: "Two Sum (Sorted)".to_string(),
        category: "Sum Problems".to_string(),
        description: "Find two elements in a sorted array that sum to a target.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Target validation".to_string(),
            "Index bounds checking".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "two_sum_sorted_safe".to_string(),
        name: "Two Sum (Sorted) - Safe".to_string(),
        category: "Sum Problems".to_string(),
        description: "Secure implementation of two sum with additional safety checks.".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Timing attack prevention".to_string(),
            "Constant-time comparison for secrets".to_string(),
        ],
    });

    // Window Bounds
    patterns.push(PatternInfo {
        id: "longest_subarray_with_k_distinct".to_string(),
        name: "Longest Subarray with K Distinct".to_string(),
        category: "Window Bounds".to_string(),
        description: "Find the longest subarray with at most K distinct elements.".to_string(),
        complexity: "O(n) time, O(K) space".to_string(),
        security_considerations: vec![
            "Memory allocation limits".to_string(),
            "K value validation".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "longest_substring_without_repeating".to_string(),
        name: "Longest Substring Without Repeating".to_string(),
        category: "Window Bounds".to_string(),
        description: "Find the length of the longest substring without repeating characters.".to_string(),
        complexity: "O(n) time, O(min(m,n)) space".to_string(),
        security_considerations: vec![
            "Character set validation".to_string(),
            "Memory bounds checking".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "max_sum_subarray".to_string(),
        name: "Maximum Sum Subarray".to_string(),
        category: "Window Bounds".to_string(),
        description: "Find the maximum sum of any contiguous subarray (Kadane's algorithm).".to_string(),
        complexity: "O(n) time, O(1) space".to_string(),
        security_considerations: vec![
            "Integer overflow protection".to_string(),
            "Empty array handling".to_string(),
        ],
    });

    patterns.push(PatternInfo {
        id: "min_window_substring".to_string(),
        name: "Minimum Window Substring".to_string(),
        category: "Window Bounds".to_string(),
        description: "Find the minimum window substring that contains all characters of another string.".to_string(),
        complexity: "O(n) time, O(m) space".to_string(),
        security_considerations: vec![
            "Pattern validation".to_string(),
            "Memory allocation safety".to_string(),
        ],
    });

    patterns
}

async fn execute_algorithm(req: ExecuteRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // In a real implementation, this would call the actual two-pointer algorithms
    // For now, we'll simulate execution with a delay and return mock results
    let result = format!("Executed pattern '{}' with input: {}", req.pattern_id, req.input);
    let execution_time_ms = 42; // Mock execution time
    
    let security_checks = match req.pattern_id.as_str() {
        "container_with_most_water" => vec![
            "Input length validated".to_string(),
            "Integer overflow protection applied".to_string(),
        ],
        "three_sum" => vec![
            "Duplicate triplets filtered".to_string(),
            "Integer overflow checks passed".to_string(),
        ],
        _ => vec![
            "Basic input validation passed".to_string(),
            "Memory bounds checked".to_string(),
        ],
    };

    let response = ExecuteResponse {
        result,
        execution_time_ms,
        security_checks,
    };

    Ok(warp::reply::json(&response))
}