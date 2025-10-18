# Code Quality Tools and Configuration

This document explains the code quality tools and configurations used in this project to ensure high-quality, secure, and maintainable Rust code.

## Code Formatting (rustfmt)

We use `rustfmt` to ensure consistent code formatting across the project. The configuration is defined in [rustfmt.toml](rustfmt.toml).

### Key Configuration Options:
- `max_width = 100`: Lines are limited to 100 characters
- `wrap_comments = true`: Comments are automatically wrapped
- `merge_imports = true`: Multiple imports from the same module are merged
- `reorder_imports = true`: Imports are sorted alphabetically

### Running rustfmt:
```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Linting (Clippy)

We use `clippy` for additional linting beyond the default Rust compiler warnings. The configuration is defined in [clippy.toml](clippy.toml).

### Key Configuration Options:
- `allow-unwrap-in-tests = true`: Allows `unwrap()` in test code
- `warn-on-all-wraps = false`: Doesn't warn on all `unwrap()`/`expect()` calls
- `cognitive-complexity-threshold = 25`: Warns on overly complex functions

### Running Clippy:
```bash
# Run clippy with default settings
cargo clippy

# Treat warnings as errors
cargo clippy -- -D warnings
```

## Testing

We have comprehensive testing including unit tests, property-based tests, and security-focused tests.

### Running Tests:
```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test '*'

# Run documentation tests
cargo test --doc
```

## Code Coverage

We use `tarpaulin` to measure code coverage. The configuration is defined in [tarpaulin.toml](tarpaulin.toml).

### Running Coverage:
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin

# Generate HTML and XML reports
cargo tarpaulin --out Html --out Xml
```

## Security Auditing

We use `cargo-audit` to check for security vulnerabilities in dependencies.

### Running Security Audit:
```bash
# Install cargo-audit
cargo install cargo-audit

# Run security audit
cargo audit
```

## Dependency Management

We use several tools to manage dependencies effectively.

### Checking for Unused Dependencies:
```bash
# Install cargo-machete
cargo install cargo-machete

# Check for unused dependencies
cargo machete
```

### Checking for Outdated Dependencies:
```bash
# Install cargo-outdated
cargo install cargo-outdated

# Check for outdated dependencies
cargo outdated
```

## GitHub Actions Workflows

We have several GitHub Actions workflows that automatically run these checks:

1. **Code Quality** ([.github/workflows/code-quality.yml](.github/workflows/code-quality.yml)):
   - Checks code formatting
   - Runs Clippy linting
   - Executes all tests
   - Performs security audits
   - Generates code coverage reports

2. **Continuous Integration** ([.github/workflows/ci.yml](.github/workflows/ci.yml)):
   - Tests on multiple operating systems (Ubuntu, Windows, macOS)
   - Tests with multiple Rust versions (stable, beta)
   - Code coverage reporting
   - Minimum Supported Rust Version (MSRV) testing

3. **Security Audit** ([.github/workflows/security-audit.yml](.github/workflows/security-audit.yml)):
   - Scheduled weekly security audits
   - Dependency review for pull requests
   - Automated vulnerability scanning

4. **Format and Lint** ([.github/workflows/format-lint.yml](.github/workflows/format-lint.yml)):
   - Automatic code formatting for pull requests
   - Clippy linting with warnings as errors

5. **Release** ([.github/workflows/release.yml](.github/workflows/release.yml)):
   - Automated crate publishing to crates.io
   - GitHub release creation

## Best Practices

1. **Always run formatting before committing**:
   ```bash
   cargo fmt
   ```

2. **Run Clippy regularly**:
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Check dependencies regularly**:
   ```bash
   cargo audit
   cargo outdated
   ```

4. **Maintain high test coverage**:
   ```bash
   cargo tarpaulin --out Html
   ```

5. **Follow the configured style guide**:
   All code should pass `cargo fmt -- --check` and `cargo clippy -- -D warnings`

These tools and configurations help ensure that our codebase maintains high quality, security, and consistency.