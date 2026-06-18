# envsec

Local-first encrypted secret manager. A self-hosted, git-shareable alternative to Doppler.

## Features

- **AES-256-GCM encryption** with Argon2id key derivation
- **Local-first** — all secrets stored in `~/.envsec/`, no cloud dependency
- **Git-shareable** — encrypted vault can be committed to a private repo
- **Clipboard auto-clear** — secrets copied to clipboard are cleared after 2 minutes
- **Session management** — configurable auth expiry (default 2 hours)
- **Project binding** — `.envsec` file in project root maps to vault entries
- **.env import/export** — seamless integration with existing workflows
- **Self-update** — `envsec update` checks and installs latest version
- **Cross-platform** — Linux, macOS, Windows
- **Fast** — Rust binary with minimal overhead

## Installation

### One-liner (Linux/macOS)

```bash
curl -sSf https://raw.githubusercontent.com/shawal-mbalire/envsec/main/install.sh | bash
```

Or with wget:

```bash
wget -qO- https://raw.githubusercontent.com/shawal-mbalire/envsec/main/install.sh | bash
```

### Custom install directory

```bash
curl -sSf https://raw.githubusercontent.com/shawal-mbalire/envsec/main/install.sh | INSTALL_DIR=~/.local/bin bash
```

### From GitHub Releases

Download the latest binary for your platform from [Releases](https://github.com/shawal-mbalire/envsec/releases).

### From Source

```bash
git clone https://github.com/shawal-mbalire/envsec.git
cd envsec
cargo install --path .
```

## Updating

```bash
envsec update
```

This checks GitHub for the latest release and replaces the binary in-place. You will be notified of available updates automatically when running other commands.

## Quick Start

```bash
# Initialize vault with a master passphrase
envsec init

# Bind current directory to a project
envsec use myapp dev

# Set a secret
envsec set DATABASE_URL "postgres://user:pass@localhost/db"

# Set a secret interactively (hidden input)
envsec set API_KEY

# Copy secret to clipboard (auto-clears in 2 minutes)
envsec get DATABASE_URL

# Show masked value
envsec get --show DATABASE_URL

# List all secrets in current project
envsec list

# Run a command with secrets injected
envsec run -- npm start

# Import from .env file
envsec import .env

# Export to .env file (masked by default)
envsec export --file .env
envsec export --file .env --raw  # actual values
```

## Commands

| Command | Description |
|---|---|
| `envsec init` | Create vault, set master passphrase |
| `envsec auth` | Authenticate (starts session, default 2h) |
| `envsec auth --duration 4h` | Authenticate with custom session duration |
| `envsec status` | Show session, project, vault info |
| `envsec set KEY [VALUE]` | Set a secret (interactive if value omitted) |
| `envsec get KEY` | Copy secret to clipboard (2-min auto-clear) |
| `envsec get --show KEY` | Show masked value to stdout |
| `envsec list` | List secrets in current project/env |
| `envsec list --all` | List all projects and environments |
| `envsec rm KEY` | Remove a secret |
| `envsec rename OLD NEW` | Rename a secret |
| `envsec import FILE` | Import a .env file |
| `envsec export` | Export secrets as .env (masked) |
| `envsec export --raw` | Export with actual values |
| `envsec run -- CMD ARGS` | Run command with secrets as env vars |
| `envsec projects` | List all projects |
| `envsec use PROJECT [ENV]` | Switch active project/env |
| `envsec rm-project PROJECT` | Delete a project and all its secrets |
| `envsec update` | Check for updates and install latest version |

## Configuration

`~/.envsec/config.toml`:

```toml
[session]
duration_secs = 7200          # 2 hours

[clipboard]
clear_after_secs = 120        # 2 minutes

[output]
color = true
```

## Security

| Concern | Mitigation |
|---|---|
| Vault at rest | AES-256-GCM, Argon2id KDF (64MB, t=3, p=4) |
| Directory permissions | `~/.envsec/` = 700, `vault.enc` = 600 |
| Passphrase storage | Only Argon2id hash stored, never plaintext |
| Clipboard leak | Forked background process clears after 2 min |
| Stdout leak | Secrets never printed; only masked values |
| Process listing | Secrets injected via `envs()`, not command-line args |

## Sharing Across Machines

1. Initialize on one machine: `envsec init`
2. Copy `~/.envsec/vault.enc` to another machine (via git, rsync, etc.)
3. On the new machine, run `envsec auth` with the same passphrase
4. The `.envsec` project binding file can be committed to your project repo

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

## License

MIT
