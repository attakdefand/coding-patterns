@echo off
echo Running CI/CD Compliance Test Suite
echo ==================================

echo.
echo 1. Running Unit Tests
echo ------------------
cargo test --lib

echo.
echo 2. Running Integration Tests
echo ------------------------
cargo test --test "*integration*"

echo.
echo 3. Running Security Tests
echo ----------------------
cargo test --test "*security*"

echo.
echo 4. Running Performance Tests
echo --------------------------
cargo test --test "*performance*"

echo.
echo 5. Running E2E Tests
echo ------------------
cargo test --test "*e2e*"

echo.
echo 6. Running Quality Gates Tests
echo ----------------------------
cargo fmt -- --check
cargo clippy -- -D warnings

echo.
echo All tests completed!