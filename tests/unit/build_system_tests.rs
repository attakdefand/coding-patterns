#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_compilation() {
        // Test that the project compiles successfully
        let output = Command::new("cargo")
            .args(&["build"])
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success());
    }

    #[test]
    fn test_artifacts_export() {
        // Test that build artifacts are created
        let output = Command::new("cargo")
            .args(&["build"])
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success());
        
        // Check that the executable exists in the debug directory
        let executable_exists = std::path::Path::new("./two-pointer-project/target/debug/two-pointer-project.exe").exists() ||
            std::path::Path::new("./two-pointer-project/target/debug/two-pointer-project").exists();
        assert!(executable_exists, "Executable not found in debug directory");
    }
}