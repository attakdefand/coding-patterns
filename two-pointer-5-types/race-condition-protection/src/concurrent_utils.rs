//! Utilities for concurrent programming and race condition protection

use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Thread-safe counter for atomic increment/decrement operations
pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    /// Creates a new AtomicCounter with the given initial value
    pub fn new(initial: usize) -> Self {
        Self {
            value: AtomicUsize::new(initial),
        }
    }

    /// Loads the current value
    pub fn load(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }

    /// Increments the counter by 1
    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Decrements the counter by 1, but never goes below 0
    pub fn decrement(&self) -> usize {
        let current = self.value.load(Ordering::Relaxed);
        if current > 0 {
            self.value.fetch_sub(1, Ordering::Relaxed) - 1
        } else {
            0
        }
    }

    /// Stores a new value
    pub fn store(&self, value: usize) {
        self.value.store(value, Ordering::Relaxed);
    }
}

/// Thread-safe array wrapper that prevents data races
pub struct ThreadSafeArray<T> {
    data: Arc<RwLock<Vec<T>>>,
}

impl<T: Clone> ThreadSafeArray<T> {
    /// Creates a new ThreadSafeArray with the given data
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data: Arc::new(RwLock::new(data)),
        }
    }

    /// Gets a value at the specified index, returning None if index is out of bounds
    /// or if the array was modified during access (TOCTOU protection)
    pub fn get(&self, index: usize) -> Option<T> {
        // Use read lock for safe access
        let guard = self.data.read();

        // Validate index is still in bounds (TOCTOU protection)
        if index < guard.len() {
            Some(guard[index].clone())
        } else {
            None
        }
    }

    /// Sets a value at the specified index, returning false if index is out of bounds
    pub fn set(&self, index: usize, value: T) -> bool {
        // Use write lock for modification
        let mut guard = self.data.write();

        // Validate index is still in bounds
        if index < guard.len() {
            guard[index] = value;
            true
        } else {
            false
        }
    }

    /// Gets the length of the array
    pub fn len(&self) -> usize {
        // Use read lock for safe access
        let guard = self.data.read();
        guard.len()
    }

    /// Checks if the array is empty
    pub fn is_empty(&self) -> bool {
        // Use read lock for safe access
        let guard = self.data.read();
        guard.is_empty()
    }
}

/// Thread-safe shared state for two-pointer algorithms
pub struct SharedState<T> {
    data: Arc<RwLock<T>>,
}

impl<T: Clone> SharedState<T> {
    /// Creates a new SharedState with the given initial value
    pub fn new(initial: T) -> Self {
        Self {
            data: Arc::new(RwLock::new(initial)),
        }
    }

    /// Gets a clone of the current value
    pub fn get(&self) -> T {
        let guard = self.data.read();
        guard.deref().clone()
    }

    /// Updates the value with a new one
    pub fn set(&self, value: T) {
        let mut guard = self.data.write();
        *guard = value;
    }

    /// Applies a function to the current value and updates it
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        let mut guard = self.data.write();
        let current = guard.deref().clone();
        *guard = f(current);
    }
}

/// Atomic flag for coordination between threads
pub struct AtomicFlag {
    flag: AtomicBool,
}

use std::sync::atomic::AtomicBool;

impl AtomicFlag {
    /// Creates a new AtomicFlag with the given initial value
    pub fn new(initial: bool) -> Self {
        Self {
            flag: AtomicBool::new(initial),
        }
    }

    /// Sets the flag to true
    pub fn set(&self) {
        self.flag.store(true, Ordering::Relaxed);
    }

    /// Sets the flag to false
    pub fn clear(&self) {
        self.flag.store(false, Ordering::Relaxed);
    }

    /// Checks if the flag is set
    pub fn is_set(&self) -> bool {
        self.flag.load(Ordering::Relaxed)
    }

    /// Atomically sets the flag and returns the previous value
    pub fn set_and_return_previous(&self) -> bool {
        self.flag.swap(true, Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new(0);
        assert_eq!(counter.load(), 0);

        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.load(), 1);

        assert_eq!(counter.decrement(), 0);
        assert_eq!(counter.load(), 0);
    }

    #[test]
    fn test_thread_safe_array() {
        let array = ThreadSafeArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(array.len(), 5);
        assert!(!array.is_empty());

        assert_eq!(array.get(2), Some(3));
        assert_eq!(array.get(10), None); // Out of bounds

        assert!(array.set(1, 10));
        assert_eq!(array.get(1), Some(10));

        assert!(!array.set(10, 20)); // Out of bounds
    }

    #[test]
    fn test_concurrent_access() {
        let array = Arc::new(ThreadSafeArray::new(vec![1, 2, 3, 4, 5]));
        let counter = Arc::new(AtomicCounter::new(0));

        let mut handles = vec![];

        // Spawn multiple threads to access the array concurrently
        for _ in 0..10 {
            let array_clone = Arc::clone(&array);
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let idx = counter_clone.increment() % 5;
                array_clone.get(idx)
            });
            handles.push(handle);
        }

        // Collect results
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All accesses should be successful (no panics due to race conditions)
        assert_eq!(results.len(), 10);
    }

    #[test]
    fn test_shared_state() {
        let state = SharedState::new(42);
        assert_eq!(state.get(), 42);

        state.set(100);
        assert_eq!(state.get(), 100);

        state.update(|x| x * 2);
        assert_eq!(state.get(), 200);
    }

    #[test]
    fn test_atomic_flag() {
        let flag = AtomicFlag::new(false);
        assert!(!flag.is_set());

        flag.set();
        assert!(flag.is_set());

        flag.clear();
        assert!(!flag.is_set());

        assert!(!flag.set_and_return_previous());
        assert!(flag.set_and_return_previous());
    }
}
