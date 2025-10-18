//! Telemetry module for collecting metrics and detecting anomalies in two-pointer algorithms

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Metrics collected for two-pointer operations
#[derive(Debug, Clone)]
pub struct TwoPointerMetrics {
    /// Total number of operations performed
    pub operation_count: usize,
    /// Duration of the operation
    pub duration: Duration,
    /// Number of pointer crossing events (when left pointer passes right pointer)
    pub pointer_crossings: usize,
    /// Number of iterations in loops
    pub loop_iterations: usize,
    /// Timestamp when the operation started
    pub start_time: Instant,
    /// Timestamp when the operation ended
    pub end_time: Instant,
}

/// Telemetry data collector
pub struct TelemetryCollector {
    /// Total operations performed
    total_operations: AtomicUsize,
    /// Total duration of all operations
    total_duration: AtomicU64, // Stored as nanoseconds
    /// Total pointer crossings
    total_pointer_crossings: AtomicUsize,
    /// Total loop iterations
    total_loop_iterations: AtomicUsize,
    /// Operations that exceeded duration threshold
    long_operations: AtomicUsize,
    /// Operations with unusual pointer crossings
    unusual_pointer_crossings: AtomicUsize,
    /// Operations with excessive loop iterations
    excessive_loop_iterations: AtomicUsize,
}

impl TelemetryCollector {
    /// Creates a new telemetry collector
    pub fn new() -> Self {
        Self {
            total_operations: AtomicUsize::new(0),
            total_duration: AtomicU64::new(0),
            total_pointer_crossings: AtomicUsize::new(0),
            total_loop_iterations: AtomicUsize::new(0),
            long_operations: AtomicUsize::new(0),
            unusual_pointer_crossings: AtomicUsize::new(0),
            excessive_loop_iterations: AtomicUsize::new(0),
        }
    }

    /// Records metrics for a two-pointer operation
    pub fn record_operation(&self, metrics: &TwoPointerMetrics) {
        // Update counters
        self.total_operations.fetch_add(1, Ordering::Relaxed);
        self.total_duration.fetch_add(metrics.duration.as_nanos() as u64, Ordering::Relaxed);
        self.total_pointer_crossings.fetch_add(metrics.pointer_crossings, Ordering::Relaxed);
        self.total_loop_iterations.fetch_add(metrics.loop_iterations, Ordering::Relaxed);

        // Check for anomalies
        if metrics.duration > Duration::from_millis(100) {
            self.long_operations.fetch_add(1, Ordering::Relaxed);
        }

        if metrics.pointer_crossings > 5 {
            self.unusual_pointer_crossings.fetch_add(1, Ordering::Relaxed);
        }

        if metrics.loop_iterations > 10000 {
            self.excessive_loop_iterations.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Gets the current telemetry statistics
    pub fn get_statistics(&self) -> TelemetryStatistics {
        let total_ops = self.total_operations.load(Ordering::Relaxed);
        let total_duration_nanos = self.total_duration.load(Ordering::Relaxed);
        let avg_duration = if total_ops > 0 {
            Duration::from_nanos(total_duration_nanos / total_ops as u64)
        } else {
            Duration::from_nanos(0)
        };

        TelemetryStatistics {
            total_operations: total_ops,
            average_duration: avg_duration,
            total_pointer_crossings: self.total_pointer_crossings.load(Ordering::Relaxed),
            total_loop_iterations: self.total_loop_iterations.load(Ordering::Relaxed),
            long_operations: self.long_operations.load(Ordering::Relaxed),
            unusual_pointer_crossings: self.unusual_pointer_crossings.load(Ordering::Relaxed),
            excessive_loop_iterations: self.excessive_loop_iterations.load(Ordering::Relaxed),
        }
    }

    /// Checks if there are any anomalies detected
    pub fn has_anomalies(&self) -> bool {
        self.long_operations.load(Ordering::Relaxed) > 0 ||
        self.unusual_pointer_crossings.load(Ordering::Relaxed) > 0 ||
        self.excessive_loop_iterations.load(Ordering::Relaxed) > 0
    }

    /// Gets a summary of detected anomalies
    pub fn get_anomaly_summary(&self) -> String {
        let stats = self.get_statistics();
        let mut anomalies = Vec::new();

        if stats.long_operations > 0 {
            anomalies.push(format!("{} long operations (>100ms)", stats.long_operations));
        }

        if stats.unusual_pointer_crossings > 0 {
            anomalies.push(format!("{} operations with unusual pointer crossings (>5)", stats.unusual_pointer_crossings));
        }

        if stats.excessive_loop_iterations > 0 {
            anomalies.push(format!("{} operations with excessive loop iterations (>10000)", stats.excessive_loop_iterations));
        }

        if anomalies.is_empty() {
            "No anomalies detected".to_string()
        } else {
            format!("Anomalies detected: {}", anomalies.join(", "))
        }
    }
}

/// Statistics collected by the telemetry system
#[derive(Debug, Clone)]
pub struct TelemetryStatistics {
    /// Total number of operations
    pub total_operations: usize,
    /// Average duration of operations
    pub average_duration: Duration,
    /// Total pointer crossings
    pub total_pointer_crossings: usize,
    /// Total loop iterations
    pub total_loop_iterations: usize,
    /// Operations that exceeded duration threshold
    pub long_operations: usize,
    /// Operations with unusual pointer crossings
    pub unusual_pointer_crossings: usize,
    /// Operations with excessive loop iterations
    pub excessive_loop_iterations: usize,
}

/// Global telemetry collector instance
static TELEMETRY_COLLECTOR: once_cell::sync::Lazy<Arc<TelemetryCollector>> = 
    once_cell::sync::Lazy::new(|| Arc::new(TelemetryCollector::new()));

/// Gets a reference to the global telemetry collector
pub fn get_telemetry_collector() -> Arc<TelemetryCollector> {
    TELEMETRY_COLLECTOR.clone()
}

/// Starts timing a two-pointer operation
pub fn start_operation_timer() -> Instant {
    Instant::now()
}

/// Creates metrics from timing data
pub fn create_metrics(
    start_time: Instant,
    operation_count: usize,
    pointer_crossings: usize,
    loop_iterations: usize,
) -> TwoPointerMetrics {
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    
    TwoPointerMetrics {
        operation_count,
        duration,
        pointer_crossings,
        loop_iterations,
        start_time,
        end_time,
    }
}

/// Records an operation with the global telemetry collector
pub fn record_operation(metrics: &TwoPointerMetrics) {
    get_telemetry_collector().record_operation(metrics);
}

/// Checks for anomalies and alerts if any are found
pub fn check_for_anomalies() -> Result<(), String> {
    let collector = get_telemetry_collector();
    if collector.has_anomalies() {
        Err(collector.get_anomaly_summary())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_telemetry_collector() {
        let collector = TelemetryCollector::new();
        
        // Create some test metrics
        let start = Instant::now();
        thread::sleep(Duration::from_millis(10));
        let metrics = create_metrics(start, 10, 2, 100);
        
        // Record the operation
        collector.record_operation(&metrics);
        
        // Check statistics
        let stats = collector.get_statistics();
        assert_eq!(stats.total_operations, 1);
        assert_eq!(stats.total_pointer_crossings, 2);
        assert_eq!(stats.total_loop_iterations, 100);
        assert!(stats.average_duration >= Duration::from_millis(10));
    }

    #[test]
    fn test_anomaly_detection() {
        let collector = TelemetryCollector::new();
        
        // Create metrics that should trigger anomalies
        let start = Instant::now();
        let metrics = TwoPointerMetrics {
            operation_count: 1,
            duration: Duration::from_millis(200), // Should trigger long operation
            pointer_crossings: 10, // Should trigger unusual pointer crossings
            loop_iterations: 20000, // Should trigger excessive loop iterations
            start_time: start,
            end_time: start + Duration::from_millis(200),
        };
        
        collector.record_operation(&metrics);
        
        // Check that anomalies are detected
        assert!(collector.has_anomalies());
        let summary = collector.get_anomaly_summary();
        assert!(summary.contains("long operations"));
        assert!(summary.contains("unusual pointer crossings"));
        assert!(summary.contains("excessive loop iterations"));
    }
}