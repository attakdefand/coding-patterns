# CI/CD Compliance Test Suite

This directory contains comprehensive tests that validate the project's compliance with the CI/CD components defined in [CORE-COMPONENTS-CI-CD.MD](../CORE-COMPONENTS-CI-CD/CORE-COMPONENTS-CI-CD.MD).

## Test Structure

The tests are organized according to the CI/CD components:

### Unit Tests
- `unit/version_control_tests.rs` - Tests for version control compliance
- `unit/build_system_tests.rs` - Tests for build system functionality
- `unit/quality_gates_tests.rs` - Tests for code quality gates
- `unit/dependency_tests.rs` - Tests for dependency management

### Integration Tests
- `integration/runner_tests.rs` - Tests for CI runners/agents
- `integration/test_system_tests.rs` - Tests for the test system

### Security Tests
- `security/supply_chain_tests.rs` - Tests for supply chain security
- `security/secrets_management_tests.rs` - Tests for secrets management

### Performance Tests
- `performance/build_performance_tests.rs` - Tests for build performance

### End-to-End Tests
- `e2e/deployment_tests.rs` - Tests for artifact deployment
- `e2e/trigger_tests.rs` - Tests for CI/CD triggers

### Compliance Tests
- `ci_cd_compliance_tests.rs` - Comprehensive compliance tests

## Running Tests

To run all tests:

```bash
cargo test
```

To run specific test categories:

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration

# Specific test file
cargo test --test deployment_tests
```