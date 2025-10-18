//! Tests for telemetry and anomaly detection

use race_condition_protection::{
    concurrent_two_sum, concurrent_string_compare, concurrent_array_search, 
    concurrent_sorted_intersection, ConcurrentTwoPointer,
    get_telemetry_collector, check_for_anomalies
};
use std::time::Duration;

/// Test that telemetry collects metrics for concurrent_two_sum
#[test]
fn test_telemetry_concurrent_two_sum() {
    let collector = get_telemetry_collector();
    
    // Reset collector stats
    // Note: In a real implementation, we might want a way to reset the collector
    
    let nums = [2, 7, 11, 15];
    let result = concurrent_two_sum(&nums, 9);
    assert_eq!(result, Some((0, 1)));
    
    // Check that metrics were recorded
    let stats = collector.get_statistics();
    assert!(stats.total_operations >= 1);
    assert!(stats.total_loop_iterations > 0);
    
    // Should not have any anomalies for normal operation
    assert!(check_for_anomalies().is_ok());
}

/// Test that telemetry collects metrics for concurrent_string_compare
#[test]
fn test_telemetry_concurrent_string_compare() {
    let collector = get_telemetry_collector();
    
    let result = concurrent_string_compare("hello", "hello");
    assert!(result);
    
    let result = concurrent_string_compare("hello", "world");
    assert!(!result);
    
    // Check that metrics were recorded
    let stats = collector.get_statistics();
    assert!(stats.total_operations >= 2);
    
    // Should not have any anomalies for normal operation
    assert!(check_for_anomalies().is_ok());
}

/// Test that telemetry collects metrics for concurrent_array_search
#[test]
fn test_telemetry_concurrent_array_search() {
    let collector = get_telemetry_collector();
    
    let arr = [1, 2, 3, 4, 5];
    let result = concurrent_array_search(&arr, 3);
    assert_eq!(result, Some(2));
    
    // Check that metrics were recorded
    let stats = collector.get_statistics();
    assert!(stats.total_operations >= 1);
    
    // Should not have any anomalies for normal operation
    assert!(check_for_anomalies().is_ok());
}

/// Test that telemetry collects metrics for concurrent_sorted_intersection
#[test]
fn test_telemetry_concurrent_sorted_intersection() {
    let collector = get_telemetry_collector();
    
    let arr1 = [1, 2, 2, 3, 4];
    let arr2 = [2, 2, 3, 5, 6];
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    assert_eq!(result, vec![2, 2, 3]);
    
    // Check that metrics were recorded
    let stats = collector.get_statistics();
    assert!(stats.total_operations >= 1);
    
    // Should not have any anomalies for normal operation
    assert!(check_for_anomalies().is_ok());
}

/// Test that telemetry collects metrics for ConcurrentTwoPointer
#[test]
fn test_telemetry_concurrent_two_pointer() {
    let collector = get_telemetry_collector();
    
    let data = vec![2, 7, 11, 15];
    let algo = ConcurrentTwoPointer::new(data);
    
    let result = algo.find_sum(9);
    assert_eq!(result, Some((0, 1)));
    
    // Check that metrics were recorded
    let stats = collector.get_statistics();
    assert!(stats.total_operations >= 1);
    assert!(stats.total_loop_iterations > 0);
    
    // Should not have any anomalies for normal operation
    assert!(check_for_anomalies().is_ok());
}

/// Test anomaly detection with normal operations
#[test]
fn test_anomaly_detection_normal() {
    // Clear any previous anomalies
    let _collector = get_telemetry_collector();
    
    // Perform some normal operations
    let nums = [1, 2, 3, 4, 5];
    let _ = concurrent_two_sum(&nums, 5);
    let _ = concurrent_string_compare("test", "test");
    let _ = concurrent_array_search(&nums, 3);
    
    // Should not detect any anomalies
    assert!(check_for_anomalies().is_ok());
}

/// Test telemetry statistics
#[test]
fn test_telemetry_statistics() {
    let collector = get_telemetry_collector();
    
    // Perform some operations
    let nums = [1, 2, 3, 4, 5];
    for i in 1..=3 {
        let _ = concurrent_two_sum(&nums, i);
    }
    
    let stats = collector.get_statistics();
    assert!(stats.total_operations >= 3);
    assert!(stats.average_duration > Duration::from_nanos(0));
    assert!(stats.total_loop_iterations > 0);
}