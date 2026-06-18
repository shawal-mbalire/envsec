# envsec monorepo
# Run: just <command>

# Default: list all commands
default:
    @just --list

# --- CLI (Rust) ---

# Build debug binary
cli-build:
    cd cli && cargo build

# Run CLI tests
cli-test:
    cd cli && cargo test

# Build optimized release binary
cli-release:
    cd cli && cargo build --release

# Run CLI lints
cli-lint:
    cd cli && cargo clippy -- -D warnings
    cd cli && cargo fmt --check

# Format CLI code
cli-fmt:
    cd cli && cargo fmt

# --- Frontend (Angular) ---

# Install frontend dependencies
fe-install:
    cd frontend && bun install

# Start frontend dev server
fe-dev:
    cd frontend && bun start

# Build frontend for production
fe-build:
    cd frontend && bun run build:prod

# Run frontend tests
fe-test:
    cd frontend && bun test

# --- Orchestration ---

# Install all dependencies
install:
    just fe-install

# Build everything
build:
    just cli-build
    just fe-build

# Test everything
test:
    just cli-test
    just fe-test

# Lint everything
lint:
    just cli-lint

# Clean everything
clean:
    cd cli && cargo clean
    cd frontend && rm -rf dist node_modules .angular

# Create a release tag and push
publish version:
    git tag v{{version}}
    git push origin v{{version}}
