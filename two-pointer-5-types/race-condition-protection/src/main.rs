//! Demonstration of race condition protection in two-pointer algorithms

use race_condition_protection::{
    concurrent_array_search, concurrent_sorted_intersection, concurrent_string_compare,
    concurrent_two_sum, ConcurrentTwoPointer,
};
use std::time::Instant;

fn main() {
    println!("Race Condition & TOCTOU Protection for Two-Pointer Algorithms");
    println!("================================================================");
    println!();
    println!("This crate provides implementations of common two-pointer algorithms");
    println!("with protections against race conditions and Time-of-Check to Time-of-Use");
    println!("(TOCTOU) vulnerabilities.");
    println!();
    println!("Features:");
    println!("- Thread-safe data structures using RwLock and Mutex");
    println!("- Atomic operations for shared counters and flags");
    println!("- Proper synchronization primitives to prevent data races");
    println!("- Lock ordering to prevent deadlocks");
    println!("- Clone-on-write patterns for safe concurrent access");
    println!("- Runtime telemetry and anomaly detection");
    println!();
    println!("For usage examples, see the tests and examples directories.");
    println!("To run tests: cargo test");
    println!("To run telemetry demo: cargo run --example telemetry_demo");
    println!();
    
    // Show a brief telemetry summary
    println!("Telemetry System:");
    println!("- Collects metrics on operation duration, pointer crossings, and loop iterations");
    println!("- Detects anomalies like long operations, unusual pointer behavior, and excessive loops");
    println!("- Use `cargo run --example telemetry_demo` for a full demonstration");
}
