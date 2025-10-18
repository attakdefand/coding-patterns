Write-Host "Running CI/CD Compliance Test Suite"
Write-Host "=================================="

Write-Host ""
Write-Host "1. Running Unit Tests"
Write-Host "------------------"
cargo test --lib

Write-Host ""
Write-Host "2. Running Integration Tests"
Write-Host "------------------------"
cargo test --test "*integration*"

Write-Host ""
Write-Host "3. Running Security Tests"
Write-Host "----------------------"
cargo test --test "*security*"

Write-Host ""
Write-Host "4. Running Performance Tests"
Write-Host "--------------------------"
cargo test --test "*performance*"

Write-Host ""
Write-Host "5. Running E2E Tests"
Write-Host "------------------"
cargo test --test "*e2e*"

Write-Host ""
Write-Host "6. Running Quality Gates Tests"
Write-Host "----------------------------"
cargo fmt -- --check
cargo clippy -- -D warnings

Write-Host ""
Write-Host "All tests completed!"