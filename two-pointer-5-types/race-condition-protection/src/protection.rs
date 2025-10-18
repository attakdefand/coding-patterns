//! Runtime protection system for two-pointer operations
//!
//! Provides rate limiting, resource quotas, circuit breakers, and monitoring
//! to protect against algorithmic DoS and resource exhaustion attacks.

// use crate::telemetry::{get_telemetry_collector, TelemetryStatistics};
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use log;

/// Configuration for runtime protections
#[derive(Debug, Clone)]
pub struct ProtectionConfig {
    /// Maximum calls per second per client
    pub rate_limit: u32,
    /// Maximum CPU time per operation (milliseconds)
    pub max_cpu_time_ms: u64,
    /// Maximum memory usage per operation (bytes)
    pub max_memory_bytes: u64,
    /// Maximum iterations per operation
    pub max_iterations: u64,
    /// Circuit breaker failure threshold
    pub failure_threshold: u32,
    /// Circuit breaker timeout (milliseconds)
    pub circuit_breaker_timeout_ms: u64,
    /// Monitoring interval (seconds)
    pub monitoring_interval_secs: u64,
}

impl Default for ProtectionConfig {
    fn default() -> Self {
        Self {
            rate_limit: 100,              // 100 calls per second
            max_cpu_time_ms: 1000,        // 1 second max CPU time
            max_memory_bytes: 10_000_000, // 10MB max memory
            max_iterations: 1_000_000,    // 1M max iterations
            failure_threshold: 5,         // 5 failures trigger circuit breaker
            circuit_breaker_timeout_ms: 30_000, // 30 seconds timeout
            monitoring_interval_secs: 60, // 1 minute monitoring intervals
        }
    }
}

/// Rate limiter for client requests
pub struct RateLimiter {
    /// Client-specific rate limiters
    clients: RwLock<HashMap<String, Arc<ClientRateLimiter>>>,
    /// Default configuration
    config: ProtectionConfig,
}

impl RateLimiter {
    /// Creates a new rate limiter
    pub fn new(config: ProtectionConfig) -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// Checks if a client is allowed to make a request
    pub fn is_allowed(&self, client_id: &str) -> bool {
        let clients = self.clients.read();
        if let Some(client_limiter) = clients.get(client_id) {
            client_limiter.is_allowed()
        } else {
            drop(clients);
            let mut clients = self.clients.write();
            let client_limiter = Arc::new(ClientRateLimiter::new(self.config.rate_limit));
            clients.insert(client_id.to_string(), client_limiter.clone());
            client_limiter.is_allowed()
        }
    }

    /// Gets the current rate limit configuration
    pub fn config(&self) -> &ProtectionConfig {
        &self.config
    }
}

/// Client-specific rate limiter
struct ClientRateLimiter {
    /// Last reset time
    last_reset: Mutex<Instant>,
    /// Current count
    count: AtomicUsize,
    /// Rate limit
    limit: u32,
}

impl ClientRateLimiter {
    /// Creates a new client rate limiter
    fn new(limit: u32) -> Self {
        Self {
            last_reset: Mutex::new(Instant::now()),
            count: AtomicUsize::new(0),
            limit,
        }
    }

    /// Checks if the client is allowed to make a request
    fn is_allowed(&self) -> bool {
        let now = Instant::now();
        let mut last_reset = self.last_reset.lock();
        
        // Reset counter if more than 1 second has passed
        if now.duration_since(*last_reset) >= Duration::from_secs(1) {
            *last_reset = now;
            self.count.store(0, Ordering::Relaxed);
        }
        
        // Check if we're under the limit
        let current = self.count.load(Ordering::Relaxed);
        if current < self.limit as usize {
            self.count.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}

/// Resource quota tracker
pub struct ResourceQuota {
    /// CPU time used (nanoseconds)
    cpu_time_ns: AtomicU64,
    /// Memory used (bytes)
    memory_bytes: AtomicU64,
    /// Configuration
    config: ProtectionConfig,
}

impl ResourceQuota {
    /// Creates a new resource quota tracker
    pub fn new(config: ProtectionConfig) -> Self {
        Self {
            cpu_time_ns: AtomicU64::new(0),
            memory_bytes: AtomicU64::new(0),
            config,
        }
    }

    /// Starts tracking CPU time for an operation
    pub fn start_cpu_tracking(&self) -> Instant {
        Instant::now()
    }

    /// Stops tracking CPU time and checks limits
    pub fn stop_cpu_tracking(&self, start: Instant) -> Result<(), String> {
        let duration = start.elapsed();
        let nanos = duration.as_nanos() as u64;
        self.cpu_time_ns.fetch_add(nanos, Ordering::Relaxed);
        
        if nanos > self.config.max_cpu_time_ms * 1_000_000 {
            Err(format!(
                "CPU time limit exceeded: {}ms > {}ms",
                nanos / 1_000_000,
                self.config.max_cpu_time_ms
            ))
        } else {
            Ok(())
        }
    }

    /// Updates memory usage and checks limits
    pub fn update_memory_usage(&self, bytes: u64) -> Result<(), String> {
        self.memory_bytes.store(bytes, Ordering::Relaxed);
        
        if bytes > self.config.max_memory_bytes {
            Err(format!(
                "Memory limit exceeded: {} bytes > {} bytes",
                bytes, self.config.max_memory_bytes
            ))
        } else {
            Ok(())
        }
    }

    /// Checks iteration count limits
    pub fn check_iteration_limit(&self, iterations: u64) -> Result<(), String> {
        if iterations > self.config.max_iterations {
            Err(format!(
                "Iteration limit exceeded: {} > {}",
                iterations, self.config.max_iterations
            ))
        } else {
            Ok(())
        }
    }
}

/// Circuit breaker for failure detection
pub struct CircuitBreaker {
    /// Failure count
    failure_count: AtomicUsize,
    /// Last failure time
    last_failure: Mutex<Instant>,
    /// Whether the circuit is open
    is_open: AtomicBool,
    /// Configuration
    config: ProtectionConfig,
}

impl CircuitBreaker {
    /// Creates a new circuit breaker
    pub fn new(config: ProtectionConfig) -> Self {
        Self {
            failure_count: AtomicUsize::new(0),
            last_failure: Mutex::new(Instant::now()),
            is_open: AtomicBool::new(false),
            config,
        }
    }

    /// Records a successful operation
    pub fn record_success(&self) {
        self.failure_count.store(0, Ordering::Relaxed);
        self.is_open.store(false, Ordering::Relaxed);
    }

    /// Records a failed operation
    pub fn record_failure(&self) {
        let failure_count = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
        *self.last_failure.lock() = Instant::now();
        
        if failure_count >= self.config.failure_threshold as usize {
            self.is_open.store(true, Ordering::Relaxed);
        }
    }

    /// Checks if the circuit is open
    pub fn is_open(&self) -> bool {
        if self.is_open.load(Ordering::Relaxed) {
            // Check if we should close the circuit
            let last_failure = *self.last_failure.lock();
            if last_failure.elapsed() >= Duration::from_millis(self.config.circuit_breaker_timeout_ms) {
                self.is_open.store(false, Ordering::Relaxed);
                self.failure_count.store(0, Ordering::Relaxed);
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}

/// Monitoring system for abnormal behavior detection
pub struct Monitor {
    /// Start time for monitoring interval
    interval_start: Mutex<Instant>,
    /// Total operations in current interval
    operations: AtomicU64,
    /// Operations with high iteration counts
    high_iteration_ops: AtomicU64,
    /// Long-running operations
    long_ops: AtomicU64,
    /// Configuration
    config: ProtectionConfig,
}

impl Monitor {
    /// Creates a new monitor
    pub fn new(config: ProtectionConfig) -> Self {
        Self {
            interval_start: Mutex::new(Instant::now()),
            operations: AtomicU64::new(0),
            high_iteration_ops: AtomicU64::new(0),
            long_ops: AtomicU64::new(0),
            config,
        }
    }

    /// Records an operation for monitoring
    pub fn record_operation(&self, duration: Duration, iterations: u64) {
        self.operations.fetch_add(1, Ordering::Relaxed);
        
        if iterations > self.config.max_iterations / 10 {
            self.high_iteration_ops.fetch_add(1, Ordering::Relaxed);
        }
        
        if duration > Duration::from_millis(self.config.max_cpu_time_ms / 10) {
            self.long_ops.fetch_add(1, Ordering::Relaxed);
        }
        
        // Check if we need to report
        let now = Instant::now();
        let mut interval_start = self.interval_start.lock();
        if now.duration_since(*interval_start) >= Duration::from_secs(self.config.monitoring_interval_secs) {
            self.report_metrics();
            *interval_start = now;
            self.operations.store(0, Ordering::Relaxed);
            self.high_iteration_ops.store(0, Ordering::Relaxed);
            self.long_ops.store(0, Ordering::Relaxed);
        }
    }

    /// Reports monitoring metrics
    fn report_metrics(&self) {
        let total_ops = self.operations.load(Ordering::Relaxed);
        let high_iter_ops = self.high_iteration_ops.load(Ordering::Relaxed);
        let long_ops = self.long_ops.load(Ordering::Relaxed);
        
        if total_ops > 0 {
            let high_iter_rate = (high_iter_ops as f64 / total_ops as f64) * 100.0;
            let long_rate = (long_ops as f64 / total_ops as f64) * 100.0;
            
            if high_iter_rate > 5.0 || long_rate > 5.0 {
                log::warn!(
                    "Abnormal behavior detected: {:.2}% high iteration ops, {:.2}% long ops",
                    high_iter_rate,
                    long_rate
                );
            }
        }
    }
}

/// Global protection manager
pub struct ProtectionManager {
    /// Rate limiter
    rate_limiter: Arc<RateLimiter>,
    /// Resource quota tracker
    resource_quota: Arc<ResourceQuota>,
    /// Circuit breaker
    circuit_breaker: Arc<CircuitBreaker>,
    /// Monitor
    monitor: Arc<Monitor>,
}

impl ProtectionManager {
    /// Creates a new protection manager
    pub fn new(config: ProtectionConfig) -> Self {
        Self {
            rate_limiter: Arc::new(RateLimiter::new(config.clone())),
            resource_quota: Arc::new(ResourceQuota::new(config.clone())),
            circuit_breaker: Arc::new(CircuitBreaker::new(config.clone())),
            monitor: Arc::new(Monitor::new(config)),
        }
    }

    /// Checks if an operation is allowed to proceed
    pub fn check_operation_allowed(&self, client_id: &str) -> Result<(), String> {
        // Check circuit breaker
        if self.circuit_breaker.is_open() {
            return Err("Circuit breaker is open".to_string());
        }
        
        // Check rate limit
        if !self.rate_limiter.is_allowed(client_id) {
            return Err("Rate limit exceeded".to_string());
        }
        
        Ok(())
    }

    /// Starts tracking an operation
    pub fn start_operation(&self, client_id: &str) -> Result<OperationGuard<'_>, String> {
        self.check_operation_allowed(client_id)?;
        let cpu_start = self.resource_quota.start_cpu_tracking();
        Ok(OperationGuard {
            manager: self,
            cpu_start,
            client_id: client_id.to_string(),
        })
    }

    /// Records a successful operation
    pub fn record_success(&self) {
        self.circuit_breaker.record_success();
    }

    /// Records a failed operation
    pub fn record_failure(&self) {
        self.circuit_breaker.record_failure();
    }
}

/// Guard for tracking operation lifecycle
pub struct OperationGuard<'a> {
    /// Protection manager
    manager: &'a ProtectionManager,
    /// CPU tracking start time
    cpu_start: Instant,
    /// Client ID
    #[allow(dead_code)]
    client_id: String,
}

impl<'a> OperationGuard<'a> {
    /// Updates memory usage
    pub fn update_memory_usage(&self, bytes: u64) -> Result<(), String> {
        self.manager.resource_quota.update_memory_usage(bytes)
    }

    /// Checks iteration limit
    pub fn check_iteration_limit(&self, iterations: u64) -> Result<(), String> {
        self.manager.resource_quota.check_iteration_limit(iterations)
    }

    /// Records operation completion
    pub fn complete(self, duration: Duration, iterations: u64) {
        // Stop CPU tracking
        if let Err(e) = self.manager.resource_quota.stop_cpu_tracking(self.cpu_start) {
            log::warn!("CPU limit exceeded: {}", e);
            self.manager.record_failure();
        }
        
        // Record success
        self.manager.record_success();
        
        // Monitor the operation
        self.manager.monitor.record_operation(duration, iterations);
    }
}

/// Global protection manager instance
static PROTECTION_MANAGER: once_cell::sync::Lazy<Arc<ProtectionManager>> = 
    once_cell::sync::Lazy::new(|| Arc::new(ProtectionManager::new(ProtectionConfig::default())));

/// Gets a reference to the global protection manager
pub fn get_protection_manager() -> Arc<ProtectionManager> {
    PROTECTION_MANAGER.clone()
}

/// Checks if an operation is allowed for a client
pub fn check_operation_allowed(client_id: &str) -> Result<(), String> {
    get_protection_manager().check_operation_allowed(client_id)
}

/// Starts tracking a protected operation
pub fn start_protected_operation(client_id: &str) -> Result<OperationGuard<'_>, String> {
    PROTECTION_MANAGER.start_operation(client_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_rate_limiter() {
        let config = ProtectionConfig {
            rate_limit: 5,
            ..Default::default()
        };
        let limiter = RateLimiter::new(config);
        
        // Should allow first 5 requests
        for _ in 0..5 {
            assert!(limiter.is_allowed("test_client"));
        }
        
        // Should reject 6th request
        assert!(!limiter.is_allowed("test_client"));
        
        // Wait for reset
        thread::sleep(Duration::from_secs(1));
        
        // Should allow requests again
        assert!(limiter.is_allowed("test_client"));
    }

    #[test]
    fn test_resource_quota() {
        let config = ProtectionConfig {
            max_cpu_time_ms: 100,
            max_iterations: 1000,
            ..Default::default()
        };
        let quota = ResourceQuota::new(config);
        
        // Should allow normal usage
        quota.update_memory_usage(1000).unwrap();
        quota.check_iteration_limit(500).unwrap();
        
        // Should reject excessive iterations
        assert!(quota.check_iteration_limit(2000).is_err());
    }

    #[test]
    fn test_circuit_breaker() {
        let config = ProtectionConfig {
            failure_threshold: 3,
            circuit_breaker_timeout_ms: 100,
            ..Default::default()
        };
        let breaker = CircuitBreaker::new(config);
        
        // Should not be open initially
        assert!(!breaker.is_open());
        
        // Record failures
        for _ in 0..3 {
            breaker.record_failure();
        }
        
        // Should be open now
        assert!(breaker.is_open());
        
        // Wait for timeout
        thread::sleep(Duration::from_millis(150));
        
        // Should be closed again
        assert!(!breaker.is_open());
    }

    #[test]
    fn test_protection_manager() {
        let config = ProtectionConfig {
            rate_limit: 2,
            ..Default::default()
        };
        let manager = ProtectionManager::new(config);
        
        // Should allow first request
        assert!(manager.check_operation_allowed("test_client").is_ok());
        
        // Should allow second request
        assert!(manager.check_operation_allowed("test_client").is_ok());
        
        // Should reject third request
        assert!(manager.check_operation_allowed("test_client").is_err());
    }
}