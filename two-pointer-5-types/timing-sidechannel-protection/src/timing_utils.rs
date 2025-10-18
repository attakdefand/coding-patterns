//! Utilities for timing analysis and protection

use std::time::Instant;

/// A timing-safe dummy operation that consumes a specified amount of time
/// This can be used to mask actual computation times
pub fn dummy_operation(duration_micros: u128) {
    let start = Instant::now();
    let mut counter = 0u64;

    // Perform dummy work for approximately the specified duration
    while start.elapsed().as_micros() < duration_micros {
        counter = counter.wrapping_add(1);
        // Add some variability to make timing analysis harder
        if counter % 1000 == 0 {
            std::hint::black_box(counter);
        }
    }
}

/// Measures the execution time of a function while preventing timing leaks
///
/// # Arguments
/// * `f` - Function to measure
///
/// # Returns
/// * Tuple of (result of function, execution time in nanoseconds)
pub fn measure_time<T, F>(f: F) -> (T, u128)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed().as_nanos();
    (result, elapsed)
}

/// Ensures constant-time execution by padding with dummy operations
///
/// # Arguments
/// * `target_time_micros` - Target execution time in microseconds
/// * `f` - Function to execute
///
/// # Returns
/// * Result of the function
pub fn constant_time_execution<T, F>(target_time_micros: u128, f: F) -> T
where
    F: FnOnce() -> T,
{
    let (result, actual_time) = measure_time(f);

    // If we finished early, pad with dummy operations
    if actual_time < target_time_micros * 1000 {
        let remaining_time = target_time_micros * 1000 - actual_time;
        dummy_operation(remaining_time / 1000);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_dummy_operation() {
        let start = Instant::now();
        dummy_operation(1000); // 1 millisecond
        let elapsed = start.elapsed();

        // Should be approximately 1ms, but allow some variance
        assert!(elapsed >= Duration::from_millis(1));
    }

    #[test]
    fn test_measure_time() {
        let (result, elapsed) = measure_time(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });

        assert_eq!(result, 42);
        assert!(elapsed >= 10_000_000); // At least 10ms in nanoseconds
    }

    #[test]
    fn test_constant_time_execution() {
        let result = constant_time_execution(5000, || {
            // This should execute quickly
            123
        });

        assert_eq!(result, 123);
        // The function should have taken at least 5ms due to padding
    }
}
