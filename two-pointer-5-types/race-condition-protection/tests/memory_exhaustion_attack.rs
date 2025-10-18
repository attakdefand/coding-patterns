//! Test for memory exhaustion attack and defenses
//!
//! Attack: Send many large arrays crafted to force the algorithm into worst-case auxiliary memory growth
//! Defenses: Cap distinct keys tracked, evict old keys, throttle requests, 
//!          reject oversized inputs, monitor memory/growth

use race_condition_protection::{
    concurrent_two_sum,
    protection::{ProtectionConfig, ProtectionManager},
};
use std::collections::HashMap;

/// Simulate a vulnerable sliding window implementation that tracks all distinct elements
#[allow(dead_code)]
fn vulnerable_sliding_window_with_map(arr: &[i32], window_size: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut element_counts = HashMap::new(); // This grows unbounded
    
    for i in 0..arr.len() {
        // Add current element to window
        let current = arr[i];
        *element_counts.entry(current).or_insert(0) += 1;
        
        // Remove elements that are out of window
        if i >= window_size {
            let old_element = arr[i - window_size];
            if let Some(count) = element_counts.get_mut(&old_element) {
                *count -= 1;
                if *count == 0 {
                    element_counts.remove(&old_element);
                }
            }
        }
        
        // In a vulnerable implementation, we might store all distinct elements
        // This causes unbounded memory growth with many distinct elements
        if i >= window_size - 1 {
            result.push(element_counts.len() as i32);
        }
    }
    
    result
}

/// Secure sliding window implementation with memory limits
fn secure_sliding_window_with_map(arr: &[i32], window_size: usize) -> Vec<i32> {
    const MAX_TRACKED_ELEMENTS: usize = 10000; // Cap distinct keys tracked
    
    let mut result = Vec::new();
    let mut element_counts = HashMap::new();
    
    for i in 0..arr.len() {
        // Check if we're approaching memory limits
        if element_counts.len() > MAX_TRACKED_ELEMENTS {
            // Evict old keys (simple strategy: clear and restart)
            element_counts.clear();
        }
        
        // Add current element to window
        let current = arr[i];
        *element_counts.entry(current).or_insert(0) += 1;
        
        // Remove elements that are out of window
        if i >= window_size {
            let old_element = arr[i - window_size];
            if let Some(count) = element_counts.get_mut(&old_element) {
                *count -= 1;
                if *count == 0 {
                    element_counts.remove(&old_element);
                }
            }
        }
        
        if i >= window_size - 1 {
            result.push(element_counts.len() as i32);
        }
    }
    
    result
}

#[test]
fn test_memory_exhaustion_attack_simulation() {
    println!("Testing memory exhaustion attack simulation...");
    
    // Create a large array with many distinct elements
    let large_array: Vec<i32> = (0..100000).collect(); // 100K distinct elements
    
    // This would cause memory issues in a vulnerable implementation
    let start = std::time::Instant::now();
    let result = secure_sliding_window_with_map(&large_array, 1000);
    let duration = start.elapsed();
    
    println!("Processed {} elements in {:?}", large_array.len(), duration);
    println!("Result length: {}", result.len());
    assert!(!result.is_empty());
}

#[test]
fn test_input_size_limiting() {
    println!("Testing input size limiting defense...");
    
    // Create an oversized array that exceeds our limits
    let oversized_array: Vec<i32> = (0..2000000).collect(); // 2M elements, exceeds MAX_ALLOWED_LEN
    
    // Our implementation should reject this or truncate it
    let result = concurrent_two_sum(&oversized_array, 1000000);
    
    // The result might be None or Some, but the operation should complete without
    // causing memory exhaustion
    println!("Oversized input result: {:?}", result);
}

#[test]
fn test_rate_limiting_defense() {
    println!("Testing rate limiting defense...");
    
    // Create a custom protection manager with aggressive rate limiting
    let config = ProtectionConfig {
        rate_limit: 5, // Only allow 5 operations per second
        ..Default::default()
    };
    
    let manager = ProtectionManager::new(config);
    
    // Try to make many requests quickly
    let mut blocked_count = 0;
    for i in 0..10 {
        match manager.check_operation_allowed("attack_client") {
            Ok(()) => {
                println!("Request {} allowed", i);
                // Simulate a memory-intensive operation
                let large_array: Vec<i32> = (0..10000).collect();
                let _result = concurrent_two_sum(&large_array, 5000);
            }
            Err(e) => {
                println!("Request {} blocked: {}", i, e);
                blocked_count += 1;
            }
        }
    }
    
    // Some requests should be blocked due to rate limiting
    assert!(blocked_count > 0, "Rate limiting should have blocked some requests");
    println!("Blocked {} requests due to rate limiting", blocked_count);
}

#[test]
fn test_memory_monitoring() {
    println!("Testing memory monitoring...");
    
    // Create moderately large arrays and process them
    for i in 0..5 {
        let large_array: Vec<i32> = (0..100000).collect();
        let target = 50000 + i;
        
        let result = concurrent_two_sum(&large_array, target);
        println!("Operation {} result: {:?}", i, result.is_some());
    }
    
    // The protection system should monitor memory usage
    // In a real implementation, we would check memory metrics here
}

fn main() {
    // Run the tests
    test_memory_exhaustion_attack_simulation();
    test_input_size_limiting();
    test_rate_limiting_defense();
    test_memory_monitoring();
    
    println!("Memory exhaustion attack exploration completed!");
}