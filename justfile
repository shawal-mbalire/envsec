# envsec monorepo
# Run: just <command>

# Module imports
mod cli "cli/justfile"
mod frontend "frontend/justfile"
mod workers "workers/justfile"

# Default: list all commands
default:
    @just --list

# --- CLI ---

# Build CLI debug binary
cli-build:
    just cli build

# Run CLI tests
cli-test:
    just cli test

# Build CLI release binary
cli-release:
    just cli release

# Lint CLI
cli-lint:
    just cli lint

# Format CLI code
cli-fmt:
    just cli fmt

# Clean CLI
cli-clean:
    just cli clean

# --- Frontend ---

# Install frontend dependencies
fe-install:
    just frontend install

# Start frontend dev server
fe-dev:
    just frontend dev

# Build frontend for production
fe-build:
    just frontend build

# Run frontend tests
fe-test:
    just frontend test

# Clean frontend
fe-clean:
    just frontend clean

# --- Workers ---

# Install workers dependencies
workers-install:
    just workers install

# Start workers dev server (local)
workers-dev:
    just workers dev

# Deploy workers to Cloudflare
workers-deploy:
    just workers deploy

# Tail workers logs
workers-tail:
    just workers tail

# Lint workers
workers-lint:
    just workers lint

# Clean workers
workers-clean:
    just workers clean

# --- Orchestration ---

# Install all dependencies
install:
    just fe-install
    just workers-install

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
    just workers-lint

# Start all dev servers (parallel)
dev:
    just --parallel workers-dev fe-dev

# Clean everything
clean:
    just cli-clean
    just fe-clean
    just workers-clean

# Create a release tag and push (triggers CI binary builds)
publish version:
    git tag v{{version}}
    git push origin v{{version}}
