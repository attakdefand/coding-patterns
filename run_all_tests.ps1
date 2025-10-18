Write-Host "Running CI/CD Compliance Test Suite"
Write-Host "=================================="

Write-Host ""
Write-Host "1. Running Unit Tests"
Write-Host "------------------"
Write-Host "Running version control tests..."
Set-Location tests/unit
rustc --test version_control_tests.rs -o version_control_tests.exe
.\version_control_tests.exe
Set-Location ../..

Write-Host "Running build system tests..."
Set-Location tests/unit
rustc --test build_system_tests.rs -o build_system_tests.exe
.\build_system_tests.exe
Set-Location ../..

Write-Host "Running quality gates tests..."
Set-Location tests/unit
rustc --test quality_gates_tests.rs -o quality_gates_tests.exe
.\quality_gates_tests.exe
Set-Location ../..

Write-Host "Running dependency tests..."
Set-Location tests/unit
rustc --test dependency_tests.rs -o dependency_tests.exe
.\dependency_tests.exe
Set-Location ../..

Write-Host ""
Write-Host "2. Running Integration Tests"
Write-Host "------------------------"
Write-Host "Running runner tests..."
Set-Location tests/integration
rustc --test runner_tests.rs -o runner_tests.exe
.\runner_tests.exe
Set-Location ../..

Write-Host "Running test system tests..."
Set-Location tests/integration
rustc --test test_system_tests.rs -o test_system_tests.exe
.\test_system_tests.exe
Set-Location ../..

Write-Host ""
Write-Host "3. Running Security Tests"
Write-Host "----------------------"
Write-Host "Running supply chain tests..."
Set-Location tests/security
rustc --test supply_chain_tests.rs -o supply_chain_tests.exe
.\supply_chain_tests.exe
Set-Location ../..

Write-Host "Running secrets management tests..."
Set-Location tests/security
rustc --test secrets_management_tests.rs -o secrets_management_tests.exe
.\secrets_management_tests.exe
Set-Location ../..

Write-Host ""
Write-Host "4. Running Performance Tests"
Write-Host "--------------------------"
Write-Host "Running build performance tests..."
Set-Location tests/performance
rustc --test build_performance_tests.rs -o build_performance_tests.exe
.\build_performance_tests.exe
Set-Location ../..

Write-Host ""
Write-Host "5. Running E2E Tests"
Write-Host "------------------"
Write-Host "Running deployment tests..."
Set-Location tests/e2e
rustc --test deployment_tests.rs -o deployment_tests.exe
.\deployment_tests.exe
Set-Location ../..

Write-Host "Running trigger tests..."
Set-Location tests/e2e
rustc --test trigger_tests.rs -o trigger_tests.exe
.\trigger_tests.exe
Set-Location ../..

Write-Host ""
Write-Host "6. Running Quality Gates Tests"
Write-Host "----------------------------"
cargo fmt -- --check
cargo clippy -- -D warnings

Write-Host ""
Write-Host "All tests completed!"