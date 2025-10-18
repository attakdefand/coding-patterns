//! Example demonstrating telemetry and anomaly detection

use race_condition_protection::{
    concurrent_two_sum, concurrent_string_compare, concurrent_array_search,
    concurrent_sorted_intersection, ConcurrentTwoPointer,
    get_telemetry_collector, check_for_anomalies
};

fn main() {
    println!("Telemetry Demo for Two-Pointer Algorithms");
    println!("========================================");
    
    // Perform some operations
    println!("\n1. Performing concurrent_two_sum operations...");
    let nums = [2, 7, 11, 15, 20, 25, 30];
    for target in [9, 18, 25, 35, 45] {
        let result = concurrent_two_sum(&nums, target);
        println!("   Target {}: {:?}", target, result);
    }
    
    println!("\n2. Performing concurrent_string_compare operations...");
    let strings = vec![
        ("hello", "hello"),
        ("hello", "world"),
        ("rust", "rust"),
        ("concurrent", "concurrent"),
        ("telemetry", "telepathy"),
    ];
    
    for (s1, s2) in strings {
        let result = concurrent_string_compare(s1, s2);
        println!("   \"{}\" == \"{}\": {}", s1, s2, result);
    }
    
    println!("\n3. Performing concurrent_array_search operations...");
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    for target in [1, 5, 10, 15, 20] {
        let result = concurrent_array_search(&arr, target);
        println!("   Searching for {}: {:?}", target, result);
    }
    
    println!("\n4. Performing concurrent_sorted_intersection operations...");
    let arr1 = [1, 2, 3, 4, 5, 6, 7];
    let arr2 = [4, 5, 6, 7, 8, 9, 10];
    let result = concurrent_sorted_intersection(&arr1, &arr2);
    println!("   Intersection of {:?} and {:?}: {:?}", arr1, arr2, result);
    
    println!("\n5. Performing ConcurrentTwoPointer operations...");
    let data = vec![1, 2, 4, 6, 8, 10, 12, 14, 16, 18];
    let algo = ConcurrentTwoPointer::new(data);
    for target in [10, 20, 25, 30] {
        let result = algo.find_sum(target);
        println!("   Finding sum {}: {:?}", target, result);
    }
    
    // Display telemetry statistics
    println!("\n6. Telemetry Statistics:");
    let collector = get_telemetry_collector();
    let stats = collector.get_statistics();
    
    println!("   Total Operations: {}", stats.total_operations);
    println!("   Average Duration: {:?}", stats.average_duration);
    println!("   Total Pointer Crossings: {}", stats.total_pointer_crossings);
    println!("   Total Loop Iterations: {}", stats.total_loop_iterations);
    println!("   Long Operations (>100ms): {}", stats.long_operations);
    println!("   Unusual Pointer Crossings (>5): {}", stats.unusual_pointer_crossings);
    println!("   Excessive Loop Iterations (>10000): {}", stats.excessive_loop_iterations);
    
    // Check for anomalies
    println!("\n7. Anomaly Detection:");
    match check_for_anomalies() {
        Ok(()) => println!("   No anomalies detected - all operations within normal parameters"),
        Err(msg) => println!("   Anomalies detected: {}", msg),
    }
    
    println!("\nDemo completed successfully!");
}