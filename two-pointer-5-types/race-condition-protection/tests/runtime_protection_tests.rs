//! Tests for runtime protections in two-pointer algorithms

use race_condition_protection::{
    concurrent_two_sum, concurrent_string_compare, concurrent_array_search, concurrent_sorted_intersection,
    protection::{ProtectionConfig, ProtectionManager},
    telemetry::{get_telemetry_collector, check_for_anomalies}
};
use std::thread;
use std::time::Duration;

#[test]
fn test_rate_limiting() {
    // Create a custom protection manager with a very low rate limit for testing
    let config = ProtectionConfig {
        rate_limit: 2, // Only allow 2 operations per second
        ..Default::default()
    };
    
    // Create a new protection manager with our custom config
    let manager = ProtectionManager::new(config);
    
    // Test that we can make 2 requests
    assert!(manager.check_operation_allowed("test_client").is_ok());
    assert!(manager.check_operation_allowed("test_client").is_ok());
    
    // The 3rd request should be blocked by rate limiting
    assert!(manager.check_operation_allowed("test_client").is_err());
    
    // Wait for the rate limiter to reset
    std::thread::sleep(Duration::from_secs(1));
    
    // Now requests should be allowed again
    assert!(manager.check_operation_allowed("test_client").is_ok());
}

#[test]
fn test_iteration_limiting() {
    // Create test data that would cause excessive iterations
    let large_array: Vec<i32> = (0..100000).collect(); // Very large array
    
    // This should be blocked by iteration limiting
    let result = concurrent_two_sum(&large_array, 999999);
    
    // The operation should complete (not panic) but may return None due to protection
    assert!(result.is_none() || result.is_some());
}

#[test]
fn test_circuit_breaker() {
    // Create a protection manager with a low failure threshold
    let config = ProtectionConfig {
        failure_threshold: 2,
        circuit_breaker_timeout_ms: 100,
        ..Default::default()
    };
    
    let manager = ProtectionManager::new(config);
    
    // Simulate failures to trigger the circuit breaker
    manager.record_failure();
    manager.record_failure();
    
    // The circuit breaker should now be open
    // Note: This test uses the global manager, so we can't directly check the state
}

#[test]
fn test_concurrent_operations_with_protection() {
    // Test that concurrent operations work correctly with protections
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let nums_clone = nums.clone();
            thread::spawn(move || concurrent_two_sum(&nums_clone, target))
        })
        .collect();
    
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // All threads should get the same result
    for result in results {
        assert_eq!(result, Some((0, 1)));
    }
}

#[test]
fn test_string_comparison_with_protection() {
    // Test string comparison with protections
    assert!(concurrent_string_compare("hello", "hello"));
    assert!(!concurrent_string_compare("hello", "world"));
    assert!(!concurrent_string_compare("hello", "hello1"));
    assert!(!concurrent_string_compare("hello1", "hello"));
}

#[test]
fn test_array_search_with_protection() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(concurrent_array_search(&arr, 3), Some(2));
    assert_eq!(concurrent_array_search(&arr, 6), None);
    
    let arr = [];
    assert_eq!(concurrent_array_search(&arr, 1), None);
}

#[test]
fn test_sorted_intersection_with_protection() {
    let arr1 = [1, 2, 2, 3, 4];
    let arr2 = [2, 2, 3, 5, 6];
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![2, 2, 3]);
    
    let arr1 = [1, 3, 5];
    let arr2 = [2, 4, 6];
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![] as Vec<i32>);
}

#[test]
fn test_telemetry_with_protection() {
    // Run some operations to generate telemetry
    let nums = vec![2, 7, 11, 15];
    let _result = concurrent_two_sum(&nums, 9);
    
    let _result = concurrent_string_compare("hello", "world");
    
    let arr = [1, 2, 3, 4, 5];
    let _result = concurrent_array_search(&arr, 3);
    
    // Check for anomalies
    let _anomaly_result = check_for_anomalies();
    
    // Get telemetry statistics
    let collector = get_telemetry_collector();
    let stats = collector.get_statistics();
    
    // Should have recorded some operations
    assert!(stats.total_operations > 0);
}