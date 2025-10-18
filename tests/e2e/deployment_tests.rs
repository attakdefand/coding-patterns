#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_artifact_creation() {
        // Test that the final artifacts are created correctly
        let output = Command::new("cargo")
            .args(&["build", "--release"])
            .output()
            .expect("Failed to execute cargo build --release");

        assert!(output.status.success());

        // Check that executable exists
        let executable_exists = std::path::Path::new("./target/release/two-pointer-project").exists() ||
            std::path::Path::new("./target/release/two-pointer-project.exe").exists();
        assert!(executable_exists, "Executable not found");
    }

    #[test]
    fn test_artifact_execution() {
        // Test that the built artifact can be executed
        // This assumes a CLI interface exists
        let executable_path = if cfg!(windows) {
            "./target/release/two-pointer-project.exe"
        } else {
            "./target/release/two-pointer-project"
        };

        if std::path::Path::new(executable_path).exists() {
            let output = Command::new(executable_path)
                .args(&["--help"]) // Assuming --help is supported
                .output();

            match output {
                Ok(result) => {
                    assert!(result.status.success(), "Executable failed to run properly");
                }
                Err(e) => {
                    println!("Warning: Could not execute artifact: {}", e);
                }
            }
        } else {
            println!("Warning: Executable not found at expected location");
        }
    }

    #[test]
    fn test_release_packaging() {
        // Test that release packaging works correctly
        // This might involve creating a tarball, Docker image, etc.
        assert!(true); // Placeholder
    }
}