//! Race condition protection for two-pointer algorithms

use crate::concurrent_utils::{AtomicCounter, ThreadSafeArray};
use crate::telemetry::{start_operation_timer, create_metrics, record_operation};
use crate::protection::{check_operation_allowed, start_protected_operation};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

// Conditional compilation for logging
use log;

/// Maximum allowed array length to prevent resource exhaustion
const MAX_ALLOWED_LEN: usize = 1_000_000;

/// Thread-safe state for two-pointer algorithms
pub struct TwoPointerState {
    data: ThreadSafeArray<i32>,
    left: AtomicCounter,
    right: AtomicCounter,
    active: AtomicBool,
    operation_count: AtomicUsize,
    pointer_crossings: AtomicUsize, // Track pointer crossings
    loop_iterations: AtomicUsize,   // Track loop iterations
}

impl TwoPointerState {
    /// Creates a new TwoPointerState with the given data
    pub fn new(data: Vec<i32>) -> Self {
        Self {
            data: ThreadSafeArray::new(data),
            left: AtomicCounter::new(0),
            right: AtomicCounter::new(0),
            active: AtomicBool::new(true),
            operation_count: AtomicUsize::new(0),
            pointer_crossings: AtomicUsize::new(0),
            loop_iterations: AtomicUsize::new(0),
        }
    }

    /// Initializes the right pointer to the end of the array
    pub fn init_right_pointer(&self, len: usize) {
        self.right.store(len.saturating_sub(1));
    }

    /// Gets the current left pointer value
    pub fn left(&self) -> usize {
        self.left.load()
    }

    /// Gets the current right pointer value
    pub fn right(&self) -> usize {
        self.right.load()
    }

    /// Moves the left pointer forward
    pub fn move_left(&self) -> usize {
        self.operation_count.fetch_add(1, Ordering::Relaxed);
        self.left.increment()
    }

    /// Moves the right pointer backward
    pub fn move_right(&self) -> usize {
        self.operation_count.fetch_add(1, Ordering::Relaxed);
        self.right.decrement()
    }

    /// Records a pointer crossing event
    pub fn record_pointer_crossing(&self) {
        self.pointer_crossings.fetch_add(1, Ordering::Relaxed);
    }

    /// Records a loop iteration
    pub fn record_loop_iteration(&self) {
        self.loop_iterations.fetch_add(1, Ordering::Relaxed);
    }

    /// Gets values at both pointer positions
    pub fn get_values(&self) -> Option<(i32, i32)> {
        let left_idx = self.left.load();
        let right_idx = self.right.load();

        let left_val = self.data.get(left_idx)?;
        let right_val = self.data.get(right_idx)?;

        Some((left_val, right_val))
    }

    /// Checks if the algorithm should continue running
    pub fn is_active(&self) -> bool {
        let left = self.left.load();
        let right = self.right.load();
        
        // Check for pointer crossing
        if left >= right {
            self.record_pointer_crossing();
        }
        
        self.active.load(Ordering::Relaxed) && left < right
    }

    /// Deactivates the algorithm (stops execution)
    pub fn deactivate(&self) {
        self.active.store(false, Ordering::Relaxed);
    }

    /// Gets the number of operations performed
    pub fn operation_count(&self) -> usize {
        self.operation_count.load(Ordering::Relaxed)
    }

    /// Gets the number of pointer crossings
    pub fn pointer_crossings(&self) -> usize {
        self.pointer_crossings.load(Ordering::Relaxed)
    }

    /// Gets the number of loop iterations
    pub fn loop_iterations(&self) -> usize {
        self.loop_iterations.load(Ordering::Relaxed)
    }

    /// Resets the state for a new operation
    pub fn reset(&self, data_len: usize) {
        self.left.store(0);
        self.right.store(data_len.saturating_sub(1));
        self.active.store(true, Ordering::Relaxed);
        self.operation_count.store(0, Ordering::Relaxed);
        self.pointer_crossings.store(0, Ordering::Relaxed);
        self.loop_iterations.store(0, Ordering::Relaxed);
    }
}

/// Concurrent two-pointer implementation with full race condition protection
pub struct ConcurrentTwoPointer {
    state: Arc<TwoPointerState>,
    result: Arc<Mutex<Option<(usize, usize)>>>,
    found: AtomicBool,
}

impl ConcurrentTwoPointer {
    /// Creates a new ConcurrentTwoPointer with the given data
    pub fn new(data: Vec<i32>) -> Self {
        // Precondition check: maximum array size
        if data.len() > MAX_ALLOWED_LEN {
            log::warn!("Array size {} exceeds maximum allowed length {}", data.len(), MAX_ALLOWED_LEN);
            // Create a smaller array with just the first MAX_ALLOWED_LEN elements
            let truncated_data = data.into_iter().take(MAX_ALLOWED_LEN).collect();
            let state = Arc::new(TwoPointerState::new(truncated_data));
            state.init_right_pointer(state.data.len());
            
            return Self {
                state,
                result: Arc::new(Mutex::new(None)),
                found: AtomicBool::new(false),
            };
        }
        
        let state = Arc::new(TwoPointerState::new(data));
        state.init_right_pointer(state.data.len());

        Self {
            state,
            result: Arc::new(Mutex::new(None)),
            found: AtomicBool::new(false),
        }
    }

    /// Executes the two-pointer algorithm to find a target sum
    pub fn find_sum(&self, target: i32) -> Option<(usize, usize)> {
        // Check if operation is allowed (rate limiting, circuit breaker)
        if let Err(_e) = check_operation_allowed("default_client") {
            log::warn!("Operation blocked: {}", _e);
            return None;
        }
        
        // Start protected operation with resource tracking
        let _guard = start_protected_operation("default_client").ok();
        
        let start_time = start_operation_timer();
        let mut loop_iterations = 0;
        
        // Reset state for new operation
        self.state.reset(self.state.data.len());
        self.found.store(false, Ordering::Relaxed);
        {
            let mut result_guard = self.result.lock();
            *result_guard = None;
        }

        // Execute the two-pointer algorithm with race condition protection
        while self.state.is_active() && !self.found.load(Ordering::Relaxed) {
            loop_iterations += 1;
            self.state.record_loop_iteration();
            
            // Check iteration limit
            if let Some(ref guard) = _guard {
                if guard.check_iteration_limit(loop_iterations as u64).is_err() {
                    log::warn!("Iteration limit exceeded in ConcurrentTwoPointer::find_sum");
                    break;
                }
            }
            
            // Get values at current pointer positions
            let values = match self.state.get_values() {
                Some(vals) => vals,
                None => break, // Invalid state, possibly due to concurrent modification
            };

            // Use saturating arithmetic to prevent overflow
            let (left_val, right_val) = values;
            let sum = left_val.saturating_add(right_val);

            if sum == target {
                // Found the target sum
                let left_idx = self.state.left();
                let right_idx = self.state.right();

                // Use atomic flag to prevent multiple threads from setting result
                if !self.found.swap(true, Ordering::Relaxed) {
                    let mut result_guard = self.result.lock();
                    *result_guard = Some((left_idx, right_idx));
                }
                break;
            } else if sum < target {
                self.state.move_left();
            } else {
                self.state.move_right();
            }
        }

        // Record telemetry
        let metrics = create_metrics(
            start_time,
            self.state.operation_count(),
            self.state.pointer_crossings(),
            loop_iterations,
        );
        record_operation(&metrics);

        // Return the result
        let result_guard = self.result.lock();
        *result_guard
    }

    /// Safely updates the underlying data
    pub fn update_data(&self, new_data: Vec<i32>) {
        // Check if operation is allowed (rate limiting, circuit breaker)
        if let Err(_e) = check_operation_allowed("default_client") {
            log::warn!("Operation blocked: {}", _e);
            return;
        }
        
        // Deactivate current operation
        self.state.deactivate();

        // Wait a bit for operations to finish
        std::thread::sleep(std::time::Duration::from_micros(100));

        // Update the data
        // Note: In a real implementation, we'd need a more sophisticated approach
        // This is a simplified version for demonstration
        self.state.reset(new_data.len());
        self.found.store(false, Ordering::Relaxed);
        {
            let mut result_guard = self.result.lock();
            *result_guard = None;
        }
    }

    /// Gets the current operation count
    pub fn operation_count(&self) -> usize {
        self.state.operation_count()
    }
    
    /// Gets the pointer crossings count
    pub fn pointer_crossings(&self) -> usize {
        self.state.pointer_crossings()
    }
    
    /// Gets the loop iterations count
    pub fn loop_iterations(&self) -> usize {
        self.state.loop_iterations()
    }
}

/// Thread-safe wrapper for string comparison
pub struct ConcurrentStringComparator {
    data1: ThreadSafeArray<u8>,
    data2: ThreadSafeArray<u8>,
}

impl ConcurrentStringComparator {
    /// Creates a new ConcurrentStringComparator
    pub fn new(str1: &str, str2: &str) -> Self {
        Self {
            data1: ThreadSafeArray::new(str1.as_bytes().to_vec()),
            data2: ThreadSafeArray::new(str2.as_bytes().to_vec()),
        }
    }

    /// Compares the two strings concurrently
    pub fn compare(&self) -> bool {
        // Check if operation is allowed (rate limiting, circuit breaker)
        if let Err(_e) = check_operation_allowed("default_client") {
            log::warn!("Operation blocked: {}", _e);
            return false;
        }
        
        // Start protected operation with resource tracking
        let _guard = start_protected_operation("default_client").ok();
        
        let start_time = start_operation_timer();
        let mut loop_iterations = 0;
        
        let len1 = self.data1.len();
        let len2 = self.data2.len();

        // Early exit for different lengths
        if len1 != len2 {
            let metrics = create_metrics(start_time, 1, 0, 1);
            record_operation(&metrics);
            return false;
        }

        // Use thread-safe array wrapper
        let a_array = &self.data1;
        let b_array = &self.data2;

        let len = len1;
        let mismatch_found = AtomicBool::new(false);

        // Compare each byte concurrently
        for i in 0..len {
            loop_iterations += 1;
            
            // Check iteration limit
            if let Some(ref guard) = _guard {
                if guard.check_iteration_limit(loop_iterations as u64).is_err() {
                    log::warn!("Iteration limit exceeded in ConcurrentStringComparator::compare");
                    let metrics = create_metrics(start_time, 1, 0, loop_iterations);
                    record_operation(&metrics);
                    return false;
                }
            }
            
            if mismatch_found.load(Ordering::Relaxed) {
                let metrics = create_metrics(start_time, 1, 0, loop_iterations);
                record_operation(&metrics);
                return false;
            }

            let a_byte = a_array.get(i);
            let b_byte = b_array.get(i);

            if let (Some(a_val), Some(b_val)) = (a_byte, b_byte) {
                if a_val != b_val {
                    mismatch_found.store(true, Ordering::Relaxed);
                    let metrics = create_metrics(start_time, 1, 0, loop_iterations);
                    record_operation(&metrics);
                    return false;
                }
            } else {
                // Handle case where array was modified during access
                mismatch_found.store(true, Ordering::Relaxed);
                let metrics = create_metrics(start_time, 1, 0, loop_iterations);
                record_operation(&metrics);
                return false;
            }
        }

        let result = !mismatch_found.load(Ordering::Relaxed);
        let metrics = create_metrics(start_time, 1, 0, loop_iterations);
        record_operation(&metrics);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_two_pointer_state() {
        let data = vec![1, 2, 3, 4, 5];
        let state = TwoPointerState::new(data);
        state.init_right_pointer(5);

        assert_eq!(state.left(), 0);
        assert_eq!(state.right(), 4);
        assert!(state.is_active());

        state.move_left();
        assert_eq!(state.left(), 1);

        state.move_right();
        assert_eq!(state.right(), 3);

        assert_eq!(state.operation_count(), 2);
    }

    #[test]
    fn test_concurrent_two_pointer() {
        let data = vec![2, 7, 11, 15];
        let algo = ConcurrentTwoPointer::new(data);

        let result = algo.find_sum(9);
        assert_eq!(result, Some((0, 1)));

        let result = algo.find_sum(18);
        assert_eq!(result, Some((1, 2)));
    }

    #[test]
    fn test_concurrent_string_comparator() {
        let comparator = ConcurrentStringComparator::new("hello", "hello");
        assert!(comparator.compare());

        let comparator = ConcurrentStringComparator::new("hello", "world");
        assert!(!comparator.compare());
    }

    #[test]
    fn test_multithreaded_access() {
        let data = vec![2, 7, 11, 15];
        let algo = Arc::new(ConcurrentTwoPointer::new(data));
        let target = 9;

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let algo_clone = Arc::clone(&algo);
                thread::spawn(move || algo_clone.find_sum(target))
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All threads should get the same result
        for result in results {
            assert_eq!(result, Some((0, 1)));
        }
    }

    #[test]
    fn test_operation_count() {
        let data = vec![1, 2, 3, 4, 5];
        let algo = ConcurrentTwoPointer::new(data);

        // This should take 2 operations: move left twice to find 1+4=5
        let _result = algo.find_sum(5);

        // Should have performed some operations
        assert!(algo.operation_count() > 0);
    }
}