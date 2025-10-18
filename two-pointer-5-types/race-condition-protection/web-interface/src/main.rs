use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
struct AlgorithmRequest {
    algorithm: String,
    input: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlgorithmResponse {
    result: String,
    execution_time: u128,
}

#[derive(Debug, Deserialize, Serialize)]
struct PatternInfo {
    name: String,
    description: String,
    category: String,
    example: String,
}

async fn execute_algorithm(req: AlgorithmRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let start_time = std::time::Instant::now();
    
    let result = match req.algorithm.as_str() {
        "two_sum_sorted" => {
            // Parse input as comma-separated integers
            let nums: Vec<i32> = req.input
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if nums.len() >= 2 {
                // Simple implementation for demonstration
                format!("Found pair that sums to target: {:?}", find_two_sum(&nums, 9))
            } else {
                "Not enough numbers provided".to_string()
            }
        },
        "reverse_array" => {
            let mut chars: Vec<char> = req.input.chars().collect();
            chars.reverse();
            format!("Reversed: {}", chars.iter().collect::<String>())
        },
        "is_palindrome" => {
            let cleaned: String = req.input.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
            let reversed: String = cleaned.chars().rev().collect();
            format!("Is palindrome: {}", cleaned == reversed)
        },
        _ => format!("Algorithm '{}' not implemented", req.algorithm),
    };
    
    let execution_time = start_time.elapsed().as_millis();
    
    let response = AlgorithmResponse {
        result,
        execution_time,
    };
    
    Ok(warp::reply::json(&response))
}

fn find_two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let sum = nums[left] + nums[right];
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

fn get_patterns_info() -> Vec<PatternInfo> {
    vec![
        PatternInfo {
            name: "Two Sum (Sorted Array)",
            description: "Find two numbers in a sorted array that add up to a target value",
            category: "Opposite Ends",
            example: "Array: [2,7,11,15], Target: 9 → Indices: (0,1)",
        },
        PatternInfo {
            name: "Reverse Array",
            description: "Reverse the elements of an array using two pointers from both ends",
            category: "Opposite Ends",
            example: "Input: [1,2,3,4,5] → Output: [5,4,3,2,1]",
        },
        PatternInfo {
            name: "Palindrome Check",
            description: "Check if a string is a palindrome using two pointers",
            category: "Opposite Ends",
            example: "Input: 'racecar' → Output: true",
        },
        PatternInfo {
            name: "Container With Most Water",
            description: "Find two lines that form a container with the most water",
            category: "Opposite Ends",
            example: "Heights: [1,8,6,2,5,4,8,3,7] → Max area: 49",
        },
        PatternInfo {
            name: "Three Sum",
            description: "Find all unique triplets that sum to zero",
            category: "Opposite Ends",
            example: "Array: [-1,0,1,2,-1,-4] → Triplets: [[-1,-1,2],[-1,0,1]]",
        },
        PatternInfo {
            name: "Remove Duplicates",
            description: "Remove duplicates from a sorted array in-place",
            category: "Same Direction",
            example: "Array: [0,0,1,1,1,2,2,3,3,4] → Length: 5, Array: [0,1,2,3,4]",
        },
        PatternInfo {
            name: "Linked List Cycle",
            description: "Detect if a linked list has a cycle",
            category: "Same Direction",
            example: "List with cycle → Output: true",
        },
        PatternInfo {
            name: "Middle of Linked List",
            description: "Find the middle node of a linked list",
            category: "Same Direction",
            example: "List: [1,2,3,4,5] → Middle: 3",
        },
        PatternInfo {
            name: "Kth Node from End",
            description: "Find the kth node from the end of a linked list",
            category: "Same Direction",
            example: "List: [1,2,3,4,5], k=2 → Node: 4",
        },
        PatternInfo {
            name: "Partition Array",
            description: "Partition an array around a pivot value",
            category: "Partitioning",
            example: "Array: [1,4,3,2,5,2], Pivot: 3 → [1,2,2,4,3,5]",
        },
        PatternInfo {
            name: "Sort Colors",
            description: "Sort an array of colors (0,1,2) in-place",
            category: "Partitioning",
            example: "Array: [2,0,2,1,1,0] → [0,0,1,1,2,2]",
        },
        PatternInfo {
            name: "Remove Element",
            description: "Remove all instances of a value from an array",
            category: "Partitioning",
            example: "Array: [3,2,2,3], Value: 3 → Length: 2, Array: [2,2]",
        },
        PatternInfo {
            name: "Max Sum Subarray",
            description: "Find the maximum sum of a contiguous subarray",
            category: "Window Bounds",
            example: "Array: [-2,1,-3,4,-1,2,1,-5,4] → Max sum: 6",
        },
        PatternInfo {
            name: "Longest Substring",
            description: "Find the longest substring without repeating characters",
            category: "Window Bounds",
            example: "String: 'abcabcbb' → Length: 3 ('abc')",
        },
        PatternInfo {
            name: "Min Window Substring",
            description: "Find the minimum window substring that contains all characters",
            category: "Window Bounds",
            example: "s: 'ADOBECODEBANC', t: 'ABC' → 'BANC'",
        },
        PatternInfo {
            name: "Sliding Window Product",
            description: "Find a pair of numbers with a specific product in a sorted array",
            category: "Sliding Window",
            example: "Array: [1,2,3,4,5], Product: 12 → Pair: (2,4)",
        },
        PatternInfo {
            name: "Merge Sorted Arrays",
            description: "Merge two sorted arrays into one sorted array",
            category: "Bidirectional Merge",
            example: "Array1: [1,2,3], Array2: [2,5,6] → [1,2,2,3,5,6]",
        },
        PatternInfo {
            name: "Intersection of Arrays",
            description: "Find the intersection of two sorted arrays",
            category: "Bidirectional Merge",
            example: "Array1: [1,2,2,3], Array2: [2,2] → [2,2]",
        },
        PatternInfo {
            name: "Union of Arrays",
            description: "Find the union of two sorted arrays",
            category: "Bidirectional Merge",
            example: "Array1: [1,2,3], Array2: [2,3,4] → [1,2,3,4]",
        },
        PatternInfo {
            name: "Sorted Squares",
            description: "Square each element and return sorted result",
            category: "Bidirectional Merge",
            example: "Array: [-4,-1,0,3,10] → [0,1,9,16,100]",
        },
        PatternInfo {
            name: "Container With Most Water",
            description: "Find two lines that form a container with the most water",
            category: "Array Problems",
            example: "Heights: [1,8,6,2,5,4,8,3,7] → Max area: 49",
        },
        PatternInfo {
            name: "Merge Sorted Arrays",
            description: "Merge two sorted arrays into one sorted array",
            category: "Array Problems",
            example: "Array1: [1,2,3,0,0,0], Array2: [2,5,6] → [1,2,2,3,5,6]",
        },
        PatternInfo {
            name: "Find Triplet Sum",
            description: "Find three numbers that sum to a target value",
            category: "Sum Problems",
            example: "Array: [1,2,3,4,5], Target: 9 → Triplet: (1,3,5)",
        },
        PatternInfo {
            name: "Two Sum (Safe)",
            description: "Secure implementation of two sum with overflow protection",
            category: "Sum Problems",
            example: "Array: [2,7,11,15], Target: 9 → Indices: (0,1)",
        },
    ]
}

#[tokio::main]
async fn main() {
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

    // Create static directory and files
    std::fs::create_dir_all("static").unwrap();
    
    // Create index.html
    std::fs::write("static/index.html", include_str!("../static/index.html")).unwrap();
    
    println!("Server running on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}