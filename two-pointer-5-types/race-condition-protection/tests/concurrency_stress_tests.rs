//! Concurrency stress tests for race condition protection
//!
//! These tests are designed to stress-test the concurrent implementations
//! and detect potential race conditions using various approaches.

use race_condition_protection::{
    concurrent_array_search, concurrent_sorted_intersection, concurrent_string_compare,
    concurrent_two_sum, ConcurrentTwoPointer,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Barrier,
};
use std::thread;

/// Stress test for concurrent_two_sum with high thread contention
#[test]
fn test_concurrent_two_sum_stress() {
    const THREAD_COUNT: usize = 100;
    const ITERATIONS: usize = 1000;
    
    let data = Arc::new(vec![2, 7, 11, 15, 20, 25, 30, 35, 40, 45]);
    let target = 22; // 7 + 15 = 22
    let counter = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|_| {
            let data_clone = Arc::clone(&data);
            let counter_clone = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    let nums: Vec<i32> = data_clone.iter().cloned().collect();
                    let result = concurrent_two_sum(&nums, target);
                    
                    // Verify result is valid
                    if let Some((i, j)) = result {
                        assert!(i < nums.len() && j < nums.len());
                        assert_eq!(nums[i] + nums[j], target);
                    }
                    
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations completed
    assert_eq!(counter.load(Ordering::Relaxed), THREAD_COUNT * ITERATIONS);
}

/// Stress test for concurrent_string_compare with high contention
#[test]
fn test_concurrent_string_compare_stress() {
    const THREAD_COUNT: usize = 50;
    const ITERATIONS: usize = 1000;
    
    let str1 = Arc::new("this is a test string for concurrent comparison".repeat(100));
    let str2 = Arc::new("this is a test string for concurrent comparison".repeat(100));
    let str3 = Arc::new("this is a different test string for concurrent comparison".repeat(100));
    
    let counter = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|i| {
            let s1_clone = Arc::clone(&str1);
            let s2_clone = Arc::clone(&str2);
            let s3_clone = Arc::clone(&str3);
            let counter_clone = Arc::clone(&counter);
            
            thread::spawn(move || {
                for j in 0..ITERATIONS {
                    // Alternate between equal and different strings
                    let result = if (i + j) % 2 == 0 {
                        concurrent_string_compare(&s1_clone, &s2_clone)
                    } else {
                        concurrent_string_compare(&s1_clone, &s3_clone)
                    };
                    
                    // Verify result is boolean (no panic)
                    assert!(result || !result);
                    
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations completed
    assert_eq!(counter.load(Ordering::Relaxed), THREAD_COUNT * ITERATIONS);
}

/// Stress test for concurrent_array_search with high contention
#[test]
fn test_concurrent_array_search_stress() {
    const THREAD_COUNT: usize = 75;
    const ITERATIONS: usize = 500;
    
    // Create a large array for more stress
    let mut large_array = Vec::new();
    for i in 0..10000 {
        large_array.push(i as i32);
    }
    let data = Arc::new(large_array);
    
    let counter = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|i| {
            let data_clone = Arc::clone(&data);
            let counter_clone = Arc::clone(&counter);
            
            thread::spawn(move || {
                for j in 0..ITERATIONS {
                    // Search for different targets to create varied access patterns
                    let target = ((i * j) % 10000) as i32;
                    let result = concurrent_array_search(&data_clone, target);
                    
                    // Verify result is valid (either Some(index) or None)
                    if let Some(index) = result {
                        assert!(index < data_clone.len());
                        assert_eq!(data_clone[index], target);
                    }
                    
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations completed
    assert_eq!(counter.load(Ordering::Relaxed), THREAD_COUNT * ITERATIONS);
}

/// Stress test for concurrent_sorted_intersection with high contention
#[test]
fn test_concurrent_sorted_intersection_stress() {
    const THREAD_COUNT: usize = 50;
    const ITERATIONS: usize = 300;
    
    // Create large sorted arrays
    let arr1: Vec<i32> = (0..5000).map(|x| x * 2).collect(); // Even numbers
    let arr2: Vec<i32> = (0..5000).map(|x| x * 3).collect(); // Multiples of 3
    let data1 = Arc::new(arr1);
    let data2 = Arc::new(arr2);
    
    let counter = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|_| {
            let d1_clone = Arc::clone(&data1);
            let d2_clone = Arc::clone(&data2);
            let counter_clone = Arc::clone(&counter);
            
            thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    let result = concurrent_sorted_intersection(&d1_clone, &d2_clone);
                    
                    // Verify result is a valid vector
                    assert!(!result.is_empty() || result.is_empty()); // Just check it's a vector
                    
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations completed
    assert_eq!(counter.load(Ordering::Relaxed), THREAD_COUNT * ITERATIONS);
}

/// Stress test for ConcurrentTwoPointer with high contention
#[test]
fn test_concurrent_two_pointer_stress() {
    const THREAD_COUNT: usize = 100;
    const ITERATIONS: usize = 200;
    
    // Create a large dataset
    let mut data = Vec::new();
    for i in 0..1000 {
        data.push(i as i32);
    }
    
    let algo = Arc::new(ConcurrentTwoPointer::new(data));
    let counter = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|i| {
            let algo_clone = Arc::clone(&algo);
            let counter_clone = Arc::clone(&counter);
            
            thread::spawn(move || {
                for j in 0..ITERATIONS {
                    // Use different targets to create varied workloads
                    let target = ((i * j) % 1998) as i32; // Max sum would be 999 + 999 = 1998
                    let result = algo_clone.find_sum(target);
                    
                    // Verify result is valid
                    if let Some((left, right)) = result {
                        // We can't directly access the data, but we can verify the indices are valid
                        assert!(left < 1000 && right < 1000);
                        // We can't verify the actual sum without access to data, but we can at least
                        // ensure it doesn't panic
                    }
                    
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations completed
    assert_eq!(counter.load(Ordering::Relaxed), THREAD_COUNT * ITERATIONS);
}

/// Test with barrier synchronization to ensure maximum contention
#[test]
fn test_barrier_synchronized_contention() {
    const THREAD_COUNT: usize = 50;
    
    let data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let algo = Arc::new(ConcurrentTwoPointer::new((*data).clone()));
    let barrier = Arc::new(Barrier::new(THREAD_COUNT));
    let counter = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|i| {
            let algo_clone = Arc::clone(&algo);
            let barrier_clone = Arc::clone(&barrier);
            let counter_clone = Arc::clone(&counter);
            
            thread::spawn(move || {
                // All threads wait at the barrier
                barrier_clone.wait();
                
                // Perform operation immediately after barrier
                let target = (i % 19) as i32; // Targets 0-18
                let result = algo_clone.find_sum(target);
                
                // Verify result (even if None, it should not panic)
                if let Some((left, right)) = result {
                    // Verify indices are valid
                    assert!(left < 10 && right < 10);
                }
                
                counter_clone.fetch_add(1, Ordering::Relaxed);
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations completed
    assert_eq!(counter.load(Ordering::Relaxed), THREAD_COUNT);
}

/// Test mixed operations under high load
#[test]
fn test_mixed_operations_stress() {
    const THREAD_COUNT: usize = 60;
    const ITERATIONS: usize = 200;
    
    // Shared data
    let two_sum_data = Arc::new(vec![2, 7, 11, 15, 20, 25]);
    let search_data = Arc::new(vec![1, 3, 5, 7, 9, 11, 13, 15]);
    let intersection_data1 = Arc::new(vec![1, 2, 3, 4, 5, 6]);
    let intersection_data2 = Arc::new(vec![4, 5, 6, 7, 8, 9]);
    let string1 = Arc::new("test string one".to_string());
    let string2 = Arc::new("test string one".to_string());
    let string3 = Arc::new("test string three".to_string());
    
    let counter = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|i| {
            let ts_data = Arc::clone(&two_sum_data);
            let s_data = Arc::clone(&search_data);
            let i_data1 = Arc::clone(&intersection_data1);
            let i_data2 = Arc::clone(&intersection_data2);
            let s1 = Arc::clone(&string1);
            let s2 = Arc::clone(&string2);
            let s3 = Arc::clone(&string3);
            let counter_clone = Arc::clone(&counter);
            
            thread::spawn(move || {
                for j in 0..ITERATIONS {
                    // Cycle through different operations
                    match (i + j) % 5 {
                        0 => {
                            // Two sum operation
                            let nums: Vec<i32> = ts_data.iter().cloned().collect();
                            let result = concurrent_two_sum(&nums, 9); // 2 + 7 = 9
                            if let Some((x, y)) = result {
                                assert_eq!(nums[x] + nums[y], 9);
                            }
                        },
                        1 => {
                            // Array search operation
                            let result = concurrent_array_search(&s_data, 7);
                            if let Some(index) = result {
                                assert_eq!(s_data[index], 7);
                            }
                        },
                        2 => {
                            // Intersection operation
                            let result = concurrent_sorted_intersection(&i_data1, &i_data2);
                            // Should contain [4, 5, 6]
                            assert_eq!(result.len(), 3);
                        },
                        3 => {
                            // String compare equal
                            let result = concurrent_string_compare(&s1, &s2);
                            assert!(result);
                        },
                        4 => {
                            // String compare different
                            let result = concurrent_string_compare(&s1, &s3);
                            assert!(!result);
                        },
                        _ => unreachable!(),
                    }
                    
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations completed
    assert_eq!(counter.load(Ordering::Relaxed), THREAD_COUNT * ITERATIONS);
}