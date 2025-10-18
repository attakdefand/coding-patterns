@echo off
echo Running CI/CD Compliance Test Suite
echo ==================================

echo.
echo 1. Running Unit Tests
echo ------------------
echo Running version control tests...
cd tests/unit && rustc --test version_control_tests.rs -o version_control_tests.exe && ./version_control_tests.exe
cd ../..

echo Running build system tests...
cd tests/unit && rustc --test build_system_tests.rs -o build_system_tests.exe && ./build_system_tests.exe
cd ../..

echo Running quality gates tests...
cd tests/unit && rustc --test quality_gates_tests.rs -o quality_gates_tests.exe && ./quality_gates_tests.exe
cd ../..

echo Running dependency tests...
cd tests/unit && rustc --test dependency_tests.rs -o dependency_tests.exe && ./dependency_tests.exe
cd ../..

echo.
echo 2. Running Integration Tests
echo ------------------------
echo Running runner tests...
cd tests/integration && rustc --test runner_tests.rs -o runner_tests.exe && ./runner_tests.exe
cd ../..

echo Running test system tests...
cd tests/integration && rustc --test test_system_tests.rs -o test_system_tests.exe && ./test_system_tests.exe
cd ../..

echo.
echo 3. Running Security Tests
echo ----------------------
echo Running supply chain tests...
cd tests/security && rustc --test supply_chain_tests.rs -o supply_chain_tests.exe && ./supply_chain_tests.exe
cd ../..

echo Running secrets management tests...
cd tests/security && rustc --test secrets_management_tests.rs -o secrets_management_tests.exe && ./secrets_management_tests.exe
cd ../..

echo.
echo 4. Running Performance Tests
echo --------------------------
echo Running build performance tests...
cd tests/performance && rustc --test build_performance_tests.rs -o build_performance_tests.exe && ./build_performance_tests.exe
cd ../..

echo.
echo 5. Running E2E Tests
echo ------------------
echo Running deployment tests...
cd tests/e2e && rustc --test deployment_tests.rs -o deployment_tests.exe && ./deployment_tests.exe
cd ../..

echo Running trigger tests...
cd tests/e2e && rustc --test trigger_tests.rs -o trigger_tests.exe && ./trigger_tests.exe
cd ../..

echo.
echo 6. Running Quality Gates Tests
echo ----------------------------
cargo fmt -- --check
cargo clippy -- -D warnings

echo.
echo All tests completed!