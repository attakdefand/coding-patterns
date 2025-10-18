//! Attack-defense tests for timing side-channel protection

use timing_sidechannel_protection::{
    secure_string_compare,
    secure_two_sum,
    secure_array_search,
    secure_sorted_intersection,
};
use std::time::Instant;

/// Simulate an attacker trying to extract information through timing
#[test]
fn test_timing_attack_resistance_string_compare() {
    // Traditional string comparison would leak information through early exits
    // Our secure implementation should resist this
    
    let secret = "secret123";
    let guesses = [
        "aecret123", // Wrong first character
        "sbcret123", // Wrong second character
        "secrft123", // Wrong sixth character
        "secret123", // Correct
    ];
    
    let mut times = Vec::new();
    
    // Measure timing for each guess
    for guess in &guesses {
        let start = Instant::now();
        let result = secure_string_compare(secret, guess);
        let elapsed = start.elapsed().as_nanos();
        times.push((guess, result, elapsed));
    }
    
    // Check that timing differences are minimal
    let min_time = times.iter().map(|(_, _, t)| *t).min().unwrap();
    let max_time = times.iter().map(|(_, _, t)| *t).max().unwrap();
    
    // In a secure implementation, all timings should be similar
    let ratio = (max_time as f64) / (min_time as f64);
    assert!(ratio < 10.0, "Potential timing leak detected: max/min ratio = {}", ratio);
    
    // Verify correct results
    assert!(!times[0].1); // First guess should be false
    assert!(!times[1].1); // Second guess should be false
    assert!(!times[2].1); // Third guess should be false
    assert!(times[3].1);  // Last guess should be true
}

/// Test resistance to timing attacks in two-sum problem
#[test]
fn test_timing_attack_resistance_two_sum() {
    // Create arrays where the solution is at different positions
    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Solution at beginning (1+2=3)
    let arr2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Solution at end (4+6=10)
    
    let start1 = Instant::now();
    let result1 = secure_two_sum(&arr1, 3);
    let time1 = start1.elapsed().as_nanos();
    
    let start2 = Instant::now();
    let result2 = secure_two_sum(&arr2, 10);
    let time2 = start2.elapsed().as_nanos();
    
    // Both should take similar time
    let ratio = (time1.max(time2) as f64) / (time1.min(time2) as f64);
    assert!(ratio < 10.0, "Potential timing leak in two-sum: {} vs {}", time1, time2);
    
    // Verify correct results
    assert_eq!(result1, Some((0, 1)));
    assert_eq!(result2, Some((3, 5)));
}

/// Test that array search doesn't leak timing information
#[test]
fn test_timing_attack_resistance_array_search() {
    let arr = (1..=100).collect::<Vec<i32>>();
    
    // Search for elements at different positions
    let targets = [1, 25, 50, 75, 100];
    let mut times = Vec::new();
    
    for &target in &targets {
        let start = Instant::now();
        let result = secure_array_search(&arr, target);
        let elapsed = start.elapsed().as_nanos();
        times.push((target, result, elapsed));
    }
    
    // All timings should be similar
    let min_time = *times.iter().map(|(_, _, t)| t).min().unwrap();
    let max_time = *times.iter().map(|(_, _, t)| t).max().unwrap();
    
    let ratio = (max_time as f64) / (min_time as f64);
    assert!(ratio < 10.0, "Potential timing leak in array search: max/min ratio = {}", ratio);
    
    // Verify all elements were found (correcting the expected indices)
    // 1 is at index 0, 25 at index 24, 50 at index 49, 75 at index 74, 100 at index 99
    assert_eq!(times[0].1, Some(0));   // 1 is at index 0
    assert_eq!(times[1].1, Some(24));  // 25 is at index 24
    assert_eq!(times[2].1, Some(49));  // 50 is at index 49
    assert_eq!(times[3].1, Some(74));  // 75 is at index 74
    assert_eq!(times[4].1, Some(99));  // 100 is at index 99
}

/// Test that intersection operation doesn't leak timing information
#[test]
fn test_timing_attack_resistance_intersection() {
    // Create arrays with intersections at different positions
    let arr1 = [1, 2, 3, 4, 5];
    let arr2_a = [1, 10, 20, 30, 40]; // Match at beginning
    let arr2_b = [10, 20, 30, 40, 5]; // Match at end
    
    let start1 = Instant::now();
    let result1 = secure_sorted_intersection(&arr1, &arr2_a);
    let time1 = start1.elapsed().as_nanos();
    
    let start2 = Instant::now();
    let result2 = secure_sorted_intersection(&arr1, &arr2_b);
    let time2 = start2.elapsed().as_nanos();
    
    // Both should take similar time
    let ratio = (time1.max(time2) as f64) / (time1.min(time2) as f64);
    assert!(ratio < 10.0, "Potential timing leak in intersection: {} vs {}", time1, time2);
    
    // Verify correct results
    assert_eq!(result1, vec![1]);
    assert_eq!(result2, vec![5]);
}

/// Test that our secure implementations are resistant to statistical timing analysis
#[test]
fn test_statistical_timing_resistance() {
    const ITERATIONS: usize = 100;
    let secret = "secret_key_12345";
    let wrong_guess = "wrong_key_12345";
    let correct_guess = "secret_key_12345";
    
    // Collect timing samples for wrong guess
    let mut wrong_times = Vec::new();
    for _ in 0..ITERATIONS {
        let start = Instant::now();
        secure_string_compare(secret, wrong_guess);
        wrong_times.push(start.elapsed().as_nanos());
    }
    
    // Collect timing samples for correct guess
    let mut correct_times = Vec::new();
    for _ in 0..ITERATIONS {
        let start = Instant::now();
        secure_string_compare(secret, correct_guess);
        correct_times.push(start.elapsed().as_nanos());
    }
    
    // Calculate average times
    let avg_wrong: f64 = wrong_times.iter().sum::<u128>() as f64 / ITERATIONS as f64;
    let avg_correct: f64 = correct_times.iter().sum::<u128>() as f64 / ITERATIONS as f64;
    
    // The difference should be minimal
    let diff_ratio = (avg_wrong.max(avg_correct)) / (avg_wrong.min(avg_correct));
    assert!(diff_ratio < 2.0, "Statistical timing difference detected: wrong={}ns, correct={}ns", avg_wrong, avg_correct);
}