# Contributing to envsec

Thank you for your interest in contributing to envsec.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/envsec.git`
3. Create a branch: `git checkout -b my-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Run lints: `cargo clippy -- -D warnings && cargo fmt --check`
7. Commit your changes
8. Push to your fork
9. Open a Pull Request

## Development Setup

```bash
# Clone
git clone https://github.com/shawal-mbalire/envsec.git
cd envsec

# Build
cargo build

# Run tests
cargo test

# Run lints
cargo clippy -- -D warnings
cargo fmt --check

# Run locally
cargo run -- --help
cargo run -- init
```

## Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` and fix all warnings
- No emojis in code or output
- Use tabular output (tabled crate) where possible
- Never print secrets to stdout

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new feature
fix: resolve bug
docs: update documentation
refactor: restructure code
test: add or update tests
chore: maintenance tasks
```

## Pull Request Process

1. Update documentation if needed
2. Add tests for new functionality
3. Ensure all tests pass
4. Ensure clippy and fmt pass
5. Keep PRs focused on a single change
6. Reference related issues

## Security

See [SECURITY.md](SECURITY.md) for reporting security vulnerabilities.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
