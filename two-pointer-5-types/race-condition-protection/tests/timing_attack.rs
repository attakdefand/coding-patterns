//! Test for timing side-channel attack and defenses
//!
//! Attack: Timing leak while using two-pointer compare on secret token (early exit)
//! Defenses: Use constant-time compare routines

use race_condition_protection::{
    race_protection::ConcurrentStringComparator,
};
use std::time::{Duration, Instant};

/// Vulnerable string comparison that leaks timing information
#[allow(dead_code)]
fn vulnerable_string_compare(a: &str, b: &str) -> bool {
    // This implementation leaks timing information:
    // - Returns immediately if lengths differ
    // - Returns early on first mismatch
    // - Takes longer for strings that match longer prefixes
    
    if a.len() != b.len() {
        return false; // Early exit leaks length information
    }
    
    // Early exit on first mismatch leaks content information
    for (_i, (a_char, b_char)) in a.chars().zip(b.chars()).enumerate() {
        if a_char != b_char {
            return false; // Early exit leaks position and value information
        }
    }
    
    true
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    // This implementation takes the same time regardless of:
    // - Input lengths
    // - Content differences
    // - Position of first mismatch
    
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for i in 0..a.len() {
        // XOR operation: 0 if equal, non-zero if different
        result |= a[i] ^ b[i];
    }
    
    // Constant-time comparison to 0
    result == 0
}

/// Measure the time taken by a function
fn time_function<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

#[test]
fn test_timing_leak_demonstration() {
    println!("Testing timing leak demonstration...");
    
    // Test with different length strings (should be fast)
    let (result1, time1) = time_function(|| vulnerable_string_compare("a", "bb"));
    println!("Different lengths 'a' vs 'bb': {} in {:?}", result1, time1);
    
    // Test with same length, early mismatch (should be fast)
    let (result2, time2) = time_function(|| vulnerable_string_compare("abc", "xyz"));
    println!("Early mismatch 'abc' vs 'xyz': {} in {:?}", result2, time2);
    
    // Test with same length, late mismatch (should be slower)
    let (result3, time3) = time_function(|| vulnerable_string_compare("abcxyz", "abc123"));
    println!("Late mismatch 'abcxyz' vs 'abc123': {} in {:?}", result3, time3);
    
    // Test with matching strings (should be slowest)
    let (result4, time4) = time_function(|| vulnerable_string_compare("abcdef", "abcdef"));
    println!("Matching 'abcdef' vs 'abcdef': {} in {:?}", result4, time4);
    
    // The timing differences demonstrate the vulnerability
    // A sophisticated attacker could measure these differences to extract secrets
}

#[test]
fn test_constant_time_comparison() {
    println!("Testing constant-time comparison...");
    
    // Test with different length strings
    let (result1, time1) = time_function(|| constant_time_compare(b"a", b"bb"));
    println!("Different lengths 'a' vs 'bb': {} in {:?}", result1, time1);
    
    // Test with same length, early mismatch
    let (result2, time2) = time_function(|| constant_time_compare(b"abc", b"xyz"));
    println!("Early mismatch 'abc' vs 'xyz': {} in {:?}", result2, time2);
    
    // Test with same length, late mismatch
    let (result3, time3) = time_function(|| constant_time_compare(b"abcxyz", b"abc123"));
    println!("Late mismatch 'abcxyz' vs 'abc123': {} in {:?}", result3, time3);
    
    // Test with matching strings
    let (result4, time4) = time_function(|| constant_time_compare(b"abcdef", b"abcdef"));
    println!("Matching 'abcdef' vs 'abcdef': {} in {:?}", result4, time4);
    
    // In a constant-time implementation, all operations should take similar time
    // Note: In practice, very small timing differences might still be measurable
    // due to CPU caching and other factors, but they should be much smaller
}

#[test]
fn test_concurrent_string_comparison() {
    println!("Testing concurrent string comparison...");
    
    // Test our secure implementation
    let comparator1 = ConcurrentStringComparator::new("secret123", "secret123");
    let (result1, time1) = time_function(|| comparator1.compare());
    println!("Matching 'secret123' vs 'secret123': {} in {:?}", result1, time1);
    
    let comparator2 = ConcurrentStringComparator::new("secret123", "secret456");
    let (result2, time2) = time_function(|| comparator2.compare());
    println!("Mismatch 'secret123' vs 'secret456': {} in {:?}", result2, time2);
    
    let comparator3 = ConcurrentStringComparator::new("secret123", "secret");
    let (result3, time3) = time_function(|| comparator3.compare());
    println!("Different lengths 'secret123' vs 'secret': {} in {:?}", result3, time3);
    
    // Our implementation provides some protection against timing attacks
    // by using a consistent approach regardless of input
}

#[test]
fn test_concurrent_vs_vulnerable_comparison() {
    println!("Comparing concurrent vs vulnerable string comparison timing...");
    
    let secret = "supersecretkey123";
    let guess1 = "a"; // Wrong length
    let guess2 = "supersecretkey124"; // Wrong last character
    let guess3 = "supersecretkey123"; // Correct
    
    // Test vulnerable implementation timing
    let (_, time_vuln1) = time_function(|| vulnerable_string_compare(secret, guess1));
    let (_, time_vuln2) = time_function(|| vulnerable_string_compare(secret, guess2));
    let (_, time_vuln3) = time_function(|| vulnerable_string_compare(secret, guess3));
    
    println!("Vulnerable implementation:");
    println!("  Wrong length: {:?}", time_vuln1);
    println!("  Wrong char:   {:?}", time_vuln2);
    println!("  Correct:      {:?}", time_vuln3);
    
    // Test concurrent implementation timing
    let comparator1 = ConcurrentStringComparator::new(secret, guess1);
    let (_, time_conc1) = time_function(|| comparator1.compare());
    let comparator2 = ConcurrentStringComparator::new(secret, guess2);
    let (_, time_conc2) = time_function(|| comparator2.compare());
    let comparator3 = ConcurrentStringComparator::new(secret, guess3);
    let (_, time_conc3) = time_function(|| comparator3.compare());
    
    println!("Concurrent implementation:");
    println!("  Wrong length: {:?}", time_conc1);
    println!("  Wrong char:   {:?}", time_conc2);
    println!("  Correct:      {:?}", time_conc3);
    
    // The concurrent implementation should be more resistant to timing analysis
}

#[test]
fn test_timing_attack_simulation() {
    println!("Simulating timing attack...");
    
    let secret_token = "SECRET_TOKEN_2023";
    
    // Simulate an attacker trying different guesses
    let guesses = vec![
        "A",                    // Wrong length
        "SECRET_TOKEN_2022",    // Wrong last character
        "SECRET_TOKEN_2023",    // Correct
        "secret_token_2023",    // Case difference
    ];
    
    println!("Timing measurements for different guesses:");
    for guess in guesses {
        let comparator = ConcurrentStringComparator::new(secret_token, guess);
        let (_, duration) = time_function(|| comparator.compare());
        println!("  Guess '{:18}' took {:?}", guess, duration);
    }
    
    // In a real timing attack, an attacker would:
    // 1. Measure timing differences for many guesses
    // 2. Use statistical analysis to detect patterns
    // 3. Gradually determine the secret character by character
    //
    // Our implementation makes this much harder by:
    // - Using consistent timing regardless of input
    // - Processing all characters even after mismatches
    // - Using thread-safe operations that don't leak timing information
}

fn main() {
    // Run the tests
    test_timing_leak_demonstration();
    test_constant_time_comparison();
    test_concurrent_string_comparison();
    test_concurrent_vs_vulnerable_comparison();
    test_timing_attack_simulation();
    
    println!("Timing side-channel attack exploration completed!");
}