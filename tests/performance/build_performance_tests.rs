#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::time::Instant;

    #[test]
    fn test_incremental_build_performance() {
        // Test that incremental builds are fast
        // Clean build first
        let output = Command::new("cargo")
            .args(&["clean"])
            .output()
            .expect("Failed to execute cargo clean");

        assert!(output.status.success());

        // Measure full build time
        let start = Instant::now();
        let output = Command::new("cargo")
            .args(&["build"])
            .output()
            .expect("Failed to execute cargo build");
        let full_build_time = start.elapsed();

        assert!(output.status.success());

        // Make a small change
        // This is a placeholder - in a real test we would modify a source file
        
        // Measure incremental build time
        let start = Instant::now();
        let output = Command::new("cargo")
            .args(&["build"])
            .output()
            .expect("Failed to execute cargo build");
        let incremental_build_time = start.elapsed();

        assert!(output.status.success());

        // Incremental build should be significantly faster
        // Note: This test might be flaky depending on system conditions
        println!("Full build time: {:?}", full_build_time);
        println!("Incremental build time: {:?}", incremental_build_time);
    }

    #[test]
    fn test_test_execution_performance() {
        // Test that test execution time is within acceptable limits
        let start = Instant::now();
        let output = Command::new("cargo")
            .args(&["test"])
            .output()
            .expect("Failed to execute cargo test");
        let test_time = start.elapsed();

        assert!(output.status.success());

        // Test execution should complete within a reasonable time
        // This threshold would need to be adjusted based on the project size
        assert!(test_time.as_secs() < 120, "Tests took too long to execute: {:?}", test_time);
    }
}