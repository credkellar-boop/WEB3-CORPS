# WEB3-Corps Project Command Automation

# Build the complete architecture in release configuration
build:
    cargo build --release

# Run the localized agent integration test matrices
test:
    cargo test --workspace -- --nocapture

# Trigger active AI Red Team prompt validation models
fuzz-agents:
    cargo test --test security_tests -- --ignored

# Clear environment artifacts and runtime telemetry target stores
clean:
    cargo clean
