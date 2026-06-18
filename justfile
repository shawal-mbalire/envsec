# envsec development commands
# Run: just <command>

# Default: build and test
default: test

# Build debug binary
build:
    cargo build

# Run all tests
test:
    cargo test

# Build optimized release binary
release:
    cargo build --release

# Run clippy and fmt checks
lint:
    cargo clippy -- -D warnings
    cargo fmt --check

# Format code
fmt:
    cargo fmt

# Clean build artifacts
clean:
    cargo clean

# Create a release tag and push (triggers CI binary builds)
publish version:
    git tag v{{version}}
    git push origin v{{version}}
