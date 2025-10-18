//! Web server module for the two-pointer patterns explorer

use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PatternInfo {
    pub name: String,
    pub description: String,
    pub category: String,
    pub example: String,
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
    let mut right = nums.len().saturating_sub(1);
    
    while left < right {
        let sum = nums[left].saturating_add(nums[right]);
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right = right.saturating_sub(1);
        }
    }
    
    None
}

fn get_patterns_info() -> Vec<PatternInfo> {
    vec![
        PatternInfo {
            name: "Two Sum (Sorted Array)".to_string(),
            description: "Find two numbers in a sorted array that add up to a target value".to_string(),
            category: "Opposite Ends".to_string(),
            example: "Array: [2,7,11,15], Target: 9 → Indices: (0,1)".to_string(),
        },
        PatternInfo {
            name: "Reverse Array".to_string(),
            description: "Reverse the elements of an array using two pointers from both ends".to_string(),
            category: "Opposite Ends".to_string(),
            example: "Input: [1,2,3,4,5] → Output: [5,4,3,2,1]".to_string(),
        },
        PatternInfo {
            name: "Palindrome Check".to_string(),
            description: "Check if a string is a palindrome using two pointers".to_string(),
            category: "Opposite Ends".to_string(),
            example: "Input: 'racecar' → Output: true".to_string(),
        },
        PatternInfo {
            name: "Container With Most Water".to_string(),
            description: "Find two lines that form a container with the most water".to_string(),
            category: "Opposite Ends".to_string(),
            example: "Heights: [1,8,6,2,5,4,8,3,7] → Max area: 49".to_string(),
        },
        PatternInfo {
            name: "Three Sum".to_string(),
            description: "Find all unique triplets that sum to zero".to_string(),
            category: "Opposite Ends".to_string(),
            example: "Array: [-1,0,1,2,-1,-4] → Triplets: [[-1,-1,2],[-1,0,1]]".to_string(),
        },
        PatternInfo {
            name: "Remove Duplicates".to_string(),
            description: "Remove duplicates from a sorted array in-place".to_string(),
            category: "Same Direction".to_string(),
            example: "Array: [0,0,1,1,1,2,2,3,3,4] → Length: 5, Array: [0,1,2,3,4]".to_string(),
        },
        PatternInfo {
            name: "Linked List Cycle".to_string(),
            description: "Detect if a linked list has a cycle".to_string(),
            category: "Same Direction".to_string(),
            example: "List with cycle → Output: true".to_string(),
        },
        PatternInfo {
            name: "Middle of Linked List".to_string(),
            description: "Find the middle node of a linked list".to_string(),
            category: "Same Direction".to_string(),
            example: "List: [1,2,3,4,5] → Middle: 3".to_string(),
        },
        PatternInfo {
            name: "Kth Node from End".to_string(),
            description: "Find the kth node from the end of a linked list".to_string(),
            category: "Same Direction".to_string(),
            example: "List: [1,2,3,4,5], k=2 → Node: 4".to_string(),
        },
        PatternInfo {
            name: "Partition Array".to_string(),
            description: "Partition an array around a pivot value".to_string(),
            category: "Partitioning".to_string(),
            example: "Array: [1,4,3,2,5,2], Pivot: 3 → [1,2,2,4,3,5]".to_string(),
        },
        PatternInfo {
            name: "Sort Colors".to_string(),
            description: "Sort an array of colors (0,1,2) in-place".to_string(),
            category: "Partitioning".to_string(),
            example: "Array: [2,0,2,1,1,0] → [0,0,1,1,2,2]".to_string(),
        },
        PatternInfo {
            name: "Remove Element".to_string(),
            description: "Remove all instances of a value from an array".to_string(),
            category: "Partitioning".to_string(),
            example: "Array: [3,2,2,3], Value: 3 → Length: 2, Array: [2,2]".to_string(),
        },
        PatternInfo {
            name: "Max Sum Subarray".to_string(),
            description: "Find the maximum sum of a contiguous subarray".to_string(),
            category: "Window Bounds".to_string(),
            example: "Array: [-2,1,-3,4,-1,2,1,-5,4] → Max sum: 6".to_string(),
        },
        PatternInfo {
            name: "Longest Substring".to_string(),
            description: "Find the longest substring without repeating characters".to_string(),
            category: "Window Bounds".to_string(),
            example: "String: 'abcabcbb' → Length: 3 ('abc')".to_string(),
        },
        PatternInfo {
            name: "Min Window Substring".to_string(),
            description: "Find the minimum window substring that contains all characters".to_string(),
            category: "Window Bounds".to_string(),
            example: "s: 'ADOBECODEBANC', t: 'ABC' → 'BANC'".to_string(),
        },
        PatternInfo {
            name: "Sliding Window Product".to_string(),
            description: "Find a pair of numbers with a specific product in a sorted array".to_string(),
            category: "Sliding Window".to_string(),
            example: "Array: [1,2,3,4,5], Product: 12 → Pair: (2,4)".to_string(),
        },
        PatternInfo {
            name: "Merge Sorted Arrays".to_string(),
            description: "Merge two sorted arrays into one sorted array".to_string(),
            category: "Bidirectional Merge".to_string(),
            example: "Array1: [1,2,3], Array2: [2,5,6] → [1,2,2,3,5,6]".to_string(),
        },
        PatternInfo {
            name: "Intersection of Arrays".to_string(),
            description: "Find the intersection of two sorted arrays".to_string(),
            category: "Bidirectional Merge".to_string(),
            example: "Array1: [1,2,2,3], Array2: [2,2] → [2,2]".to_string(),
        },
        PatternInfo {
            name: "Union of Arrays".to_string(),
            description: "Find the union of two sorted arrays".to_string(),
            category: "Bidirectional Merge".to_string(),
            example: "Array1: [1,2,3], Array2: [2,3,4] → [1,2,3,4]".to_string(),
        },
        PatternInfo {
            name: "Sorted Squares".to_string(),
            description: "Square each element and return sorted result".to_string(),
            category: "Bidirectional Merge".to_string(),
            example: "Array: [-4,-1,0,3,10] → [0,1,9,16,100]".to_string(),
        },
        PatternInfo {
            name: "Container With Most Water".to_string(),
            description: "Find two lines that form a container with the most water".to_string(),
            category: "Array Problems".to_string(),
            example: "Heights: [1,8,6,2,5,4,8,3,7] → Max area: 49".to_string(),
        },
        PatternInfo {
            name: "Merge Sorted Arrays".to_string(),
            description: "Merge two sorted arrays into one sorted array".to_string(),
            category: "Array Problems".to_string(),
            example: "Array1: [1,2,3,0,0,0], Array2: [2,5,6] → [1,2,2,3,5,6]".to_string(),
        },
        PatternInfo {
            name: "Find Triplet Sum".to_string(),
            description: "Find three numbers that sum to a target value".to_string(),
            category: "Sum Problems".to_string(),
            example: "Array: [1,2,3,4,5], Target: 9 → Triplet: (1,3,5)".to_string(),
        },
        PatternInfo {
            name: "Two Sum (Safe)".to_string(),
            description: "Secure implementation of two sum with overflow protection".to_string(),
            category: "Sum Problems".to_string(),
            example: "Array: [2,7,11,15], Target: 9 → Indices: (0,1)".to_string(),
        },
    ]
}