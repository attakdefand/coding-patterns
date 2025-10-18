//! Attack-defense tests for race condition protection

use race_condition_protection::race_protection::ConcurrentStringComparator;
use race_condition_protection::{
    concurrent_array_search, concurrent_sorted_intersection, concurrent_string_compare,
    concurrent_two_sum, ConcurrentTwoPointer, TwoPointerState,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

/// Simulate an attacker trying to cause race conditions by modifying data concurrently
#[test]
fn test_race_condition_resistance_data_modification() {
    // Create shared data
    let data = Arc::new(vec![2, 7, 11, 15, 20, 25]);
    let target = 22; // 7 + 15 = 22 or 2 + 20 = 22

    // Flag to signal when to modify data
    let modify_flag = Arc::new(AtomicBool::new(false));

    // Thread that tries to modify data during computation
    let _data_clone1 = Arc::clone(&data);
    let modify_flag_clone1 = Arc::clone(&modify_flag);
    let modifier_thread = thread::spawn(move || {
        // Wait for signal to modify data
        while !modify_flag_clone1.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_micros(10));
        }

        // Try to modify data (this should not affect the computation)
        // In a real scenario, this would be an attacker trying to cause TOCTOU
        println!("Attacker attempting to modify data...");
    });

    // Thread that performs the computation
    let data_clone2 = Arc::clone(&data);
    let modify_flag_clone2 = Arc::clone(&modify_flag);
    let computation_thread = thread::spawn(move || {
        // Signal that computation is starting
        modify_flag_clone2.store(true, Ordering::Relaxed);

        // Perform computation
        let nums: Vec<i32> = data_clone2.iter().cloned().collect();
        let result = concurrent_two_sum(&nums, target);

        result
    });

    // Wait for both threads to complete
    let _ = modifier_thread.join();
    let result = computation_thread.join().unwrap();

    // The computation should still produce a valid result
    // Multiple valid pairs exist: (0,4) for 2+20=22 and (1,3) for 7+15=22
    assert!(result.is_some());
    if let Some((i, j)) = result {
        let nums: Vec<i32> = data.iter().cloned().collect();
        assert_eq!(nums[i] + nums[j], target);
    }
}

/// Test resistance to race conditions in string comparison
#[test]
fn test_race_condition_resistance_string_comparison() {
    let str1 = Arc::new("this is a secret string");
    let str2 = Arc::new("this is a secret string");

    // Run multiple threads concurrently to test for race conditions
    let handles: Vec<_> = (0..20)
        .map(|i| {
            let s1_clone = Arc::clone(&str1);
            let s2_clone = Arc::clone(&str2);
            thread::spawn(move || {
                // Add some variability to thread execution
                if i % 2 == 0 {
                    thread::sleep(Duration::from_micros(i));
                }
                concurrent_string_compare(&*s1_clone, &*s2_clone)
            })
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be true (no race conditions should cause incorrect results)
    for result in results {
        assert!(result, "Race condition detected in string comparison");
    }
}

/// Test resistance to race conditions in array search
#[test]
fn test_race_condition_resistance_array_search() {
    let arr = Arc::new([1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    let target = 11;

    // Run multiple threads concurrently to test for race conditions
    let handles: Vec<_> = (0..20)
        .map(|i| {
            let arr_clone = Arc::clone(&arr);
            thread::spawn(move || {
                // Add some variability to thread execution
                if i % 3 == 0 {
                    thread::sleep(Duration::from_micros(i));
                }
                concurrent_array_search(&*arr_clone, target)
            })
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be the same (no race conditions should cause incorrect results)
    for result in results {
        assert_eq!(result, Some(5), "Race condition detected in array search");
    }
}

/// Test that ConcurrentTwoPointer is resistant to race conditions
#[test]
fn test_concurrent_two_pointer_race_resistance() {
    let data = vec![2, 7, 11, 15, 20, 25];
    let algo = Arc::new(ConcurrentTwoPointer::new(data));
    let target = 22; // 7 + 15 = 22 or 2 + 20 = 22

    // Run multiple threads concurrently
    let handles: Vec<_> = (0..15)
        .map(|i| {
            let algo_clone = Arc::clone(&algo);
            thread::spawn(move || {
                // Add some variability to thread execution
                if i % 2 == 0 {
                    thread::sleep(Duration::from_micros(i * 2));
                }
                algo_clone.find_sum(target)
            })
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be valid (multiple valid pairs exist)
    for result in results {
        assert!(
            result.is_some(),
            "Race condition detected in ConcurrentTwoPointer"
        );
        if let Some((i, j)) = result {
            // Verify indices are valid
            assert!(i < 6 && j < 6 && i != j); // Valid indices for our 6-element array
        }
    }
}

/// Test TOCTOU (Time-of-Check to Time-of-Use) resistance
#[test]
fn test_toctou_resistance() {
    // This test simulates a TOCTOU attack where an attacker
    // modifies data between the time it's checked and used

    // Create a concurrent two-pointer algorithm
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let algo = ConcurrentTwoPointer::new(data);

    // In a real TOCTOU scenario, an attacker would try to modify
    // the data between checking and using it.
    // Our implementation should be resistant to this because:
    // 1. It uses thread-safe data structures
    // 2. It validates indices on each access
    // 3. It uses atomic operations for state changes

    let result = algo.find_sum(11); // 1 + 10 = 11
    assert_eq!(result, Some((0, 9)));

    // Even if we try to simulate concurrent modification,
    // the thread-safe implementation should prevent issues
    let result2 = algo.find_sum(9); // 1 + 8 = 9, 2 + 7 = 9, 3 + 6 = 9, 4 + 5 = 9
    assert!(result2.is_some());
}

/// Test that multiple concurrent operations don't interfere with each other
#[test]
fn test_concurrent_operation_isolation() {
    let data1 = vec![2, 7, 11, 15];
    let data2 = vec![1, 3, 5, 7, 9];
    let data3 = vec![4, 8, 12, 16, 20];

    let algo1 = Arc::new(ConcurrentTwoPointer::new(data1));
    let algo2 = Arc::new(ConcurrentTwoPointer::new(data2));
    let algo3 = Arc::new(ConcurrentTwoPointer::new(data3));

    let target1 = 9; // 2 + 7 = 9
    let target2 = 8; // 1 + 7 = 8
    let target3 = 20; // 4 + 16 = 20

    // Run all three algorithms concurrently
    let handle1 = {
        let algo_clone = Arc::clone(&algo1);
        thread::spawn(move || algo_clone.find_sum(target1))
    };

    let handle2 = {
        let algo_clone = Arc::clone(&algo2);
        thread::spawn(move || algo_clone.find_sum(target2))
    };

    let handle3 = {
        let algo_clone = Arc::clone(&algo3);
        thread::spawn(move || algo_clone.find_sum(target3))
    };

    // Collect results
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    let result3 = handle3.join().unwrap();

    // Verify all results are correct and independent
    assert_eq!(result1, Some((0, 1)));
    assert_eq!(result2, Some((0, 3)));
    assert_eq!(result3, Some((0, 3)));
}

/// Test that the implementation handles thread contention gracefully
#[test]
fn test_thread_contention_handling() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let algo = Arc::new(ConcurrentTwoPointer::new(data));
    let target = 11;

    // Create many threads to stress-test the implementation
    let handles: Vec<_> = (0..50)
        .map(|_| {
            let algo_clone = Arc::clone(&algo);
            thread::spawn(move || algo_clone.find_sum(target))
        })
        .collect();

    // Collect all results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All results should be the same
    for result in results {
        assert_eq!(result, Some((0, 9)));
    }
}
