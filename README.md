# envsec

Local-first encrypted secret manager. A self-hosted, git-shareable alternative to Doppler.

## Monorepo Structure

```
envsec/
├── .sha/                    # SHA stack config
├── .github/workflows/       # CI/CD
├── cli/                     # Rust CLI binary
│   ├── src/
│   ├── Cargo.toml
│   └── install.sh
├── frontend/                # Angular 22 web dashboard
│   ├── src/
│   ├── package.json
│   └── angular.json
├── justfile                 # Master task runner
└── README.md
```

## Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Bun](https://bun.sh) 1.0+
- [Just](https://github.com/casey/just) — task runner

## Quick Start

```bash
# List all commands
just

# Install frontend dependencies
just fe-install

# Build CLI
just cli-build

# Run CLI tests
just cli-test

# Start frontend dev server
just fe-dev

# Build everything
just build

# Test everything
just test
```

## CLI Installation

### One-liner (Linux/macOS)

```bash
curl -sSf https://raw.githubusercontent.com/shawal-mbalire/envsec/main/cli/install.sh | bash
```

### From Source

```bash
cd cli
cargo install --path .
```

## CLI Usage

```bash
envsec init                    # Create vault, set passphrase
envsec use myapp dev           # Bind project
envsec set DATABASE_URL "..."  # Set secret
envsec get DATABASE_URL        # Copy to clipboard (2-min clear)
envsec get --show DATABASE_URL # Show masked value
envsec run -- npm start        # Run with secrets injected
envsec update                  # Self-update
```

## Commands

| Command | Action |
|---|---|
| `just` | List all commands |
| `just cli-build` | Build CLI debug binary |
| `just cli-test` | Run CLI tests |
| `just cli-release` | Build optimized CLI binary |
| `just cli-lint` | Run clippy + fmt check |
| `just fe-install` | Install frontend dependencies |
| `just fe-dev` | Start Angular dev server |
| `just fe-build` | Build frontend for production |
| `just fe-test` | Run frontend tests |
| `just build` | Build everything |
| `just test` | Test everything |
| `just lint` | Lint everything |
| `just clean` | Clean all artifacts |
| `just publish v0.1.0` | Tag + push to trigger CI release |

## Security

| Concern | Mitigation |
|---|---|
| Vault at rest | AES-256-GCM, Argon2id KDF (64MB, t=3, p=4) |
| Directory permissions | `~/.envsec/` = 700, `vault.enc` = 600 |
| Passphrase storage | Only Argon2id hash stored, never plaintext |
| Clipboard leak | Forked background process clears after 2 min |
| Stdout leak | Secrets never printed; only masked values |

## Supported Platforms

| Target | OS | Arch |
|---|---|---|
| `x86_64-unknown-linux-gnu` | Linux | x86_64 |
| `aarch64-unknown-linux-gnu` | Linux | ARM64 |
| `x86_64-unknown-linux-musl` | Linux (static) | x86_64 |
| `aarch64-unknown-linux-musl` | Linux (static) | ARM64 |
| `x86_64-apple-darwin` | macOS | Intel |
| `aarch64-apple-darwin` | macOS | Apple Silicon |
| `x86_64-pc-windows-msvc` | Windows | x86_64 |
| `aarch64-pc-windows-msvc` | Windows | ARM64 |

## License

MIT
