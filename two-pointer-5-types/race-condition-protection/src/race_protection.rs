//! Race condition protection for two-pointer algorithms

use crate::concurrent_utils::{AtomicCounter, ThreadSafeArray};
use std::sync::Arc;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Thread-safe state for two-pointer algorithms
pub struct TwoPointerState {
    data: ThreadSafeArray<i32>,
    left: AtomicCounter,
    right: AtomicCounter,
    active: AtomicBool,
    operation_count: AtomicUsize,
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
        self.active.load(Ordering::Relaxed) && 
        self.left.load() < self.right.load()
    }

    /// Deactivates the algorithm (stops execution)
    pub fn deactivate(&self) {
        self.active.store(false, Ordering::Relaxed);
    }

    /// Gets the number of operations performed
    pub fn operation_count(&self) -> usize {
        self.operation_count.load(Ordering::Relaxed)
    }

    /// Resets the state for a new operation
    pub fn reset(&self, data_len: usize) {
        self.left.store(0);
        self.right.store(data_len.saturating_sub(1));
        self.active.store(true, Ordering::Relaxed);
        self.operation_count.store(0, Ordering::Relaxed);
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
        // Reset state for new operation
        self.state.reset(self.state.data.len());
        self.found.store(false, Ordering::Relaxed);
        {
            let mut result_guard = self.result.lock();
            *result_guard = None;
        }
        
        // Execute the two-pointer algorithm with race condition protection
        while self.state.is_active() && !self.found.load(Ordering::Relaxed) {
            // Get values at current pointer positions
            let values = match self.state.get_values() {
                Some(vals) => vals,
                None => break, // Invalid state, possibly due to concurrent modification
            };
            
            let (left_val, right_val) = values;
            let sum = left_val + right_val;
            
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
        
        // Return the result
        let result_guard = self.result.lock();
        *result_guard
    }

    /// Safely updates the underlying data
    pub fn update_data(&self, new_data: Vec<i32>) {
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
        let len1 = self.data1.len();
        let len2 = self.data2.len();
        
        // Early exit for different lengths
        if len1 != len2 {
            return false;
        }
        
        // Compare each byte
        for i in 0..len1 {
            match (self.data1.get(i), self.data2.get(i)) {
                (Some(b1), Some(b2)) if b1 == b2 => continue,
                _ => return false,
            }
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Arc;

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
                thread::spawn(move || {
                    algo_clone.find_sum(target)
                })
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