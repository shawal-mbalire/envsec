# envsec

Local-first encrypted secret manager with real-time device sync.

## Monorepo Structure

```
envsec/
├── .sha/                    # SHA stack config
├── .github/workflows/       # CI/CD
├── cli/                     # Rust CLI binary
│   ├── src/
│   ├── Cargo.toml
│   ├── install.sh
│   └── justfile
├── frontend/                # Angular 22 web dashboard
│   ├── src/
│   ├── package.json
│   └── justfile
├── workers/                 # Cloudflare Workers (WebRTC signaling)
│   ├── src/
│   ├── wrangler.toml
│   └── justfile
├── justfile                 # Master orchestrator
└── README.md
```

## Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Bun](https://bun.sh) 1.0+
- [Node](https://nodejs.org/) 20+
- [Just](https://github.com/casey/just) — task runner
- [Wrangler](https://developers.cloudflare.com/workers/wrangler/) — Cloudflare CLI

## Quick Start

```bash
# List all commands
just

# Install all dependencies
just install

# Start dev servers (workers + frontend in parallel)
just dev

# Build everything
just build

# Test everything
just test
```

## Module Commands

### CLI (Rust)

```bash
just cli-build      # Build debug binary
just cli-test       # Run tests
just cli-release    # Build optimized binary
just cli-lint       # Clippy + fmt check
```

### Frontend (Angular 22)

```bash
just fe-install     # Install dependencies
just fe-dev         # Start dev server (:4200)
just fe-build       # Production build
just fe-test        # Run tests
```

### Workers (Cloudflare)

```bash
just workers-install    # Install dependencies
just workers-dev        # Start local dev server (:8787)
just workers-deploy     # Deploy to Cloudflare
just workers-tail       # Tail production logs
just workers-lint       # Type check
```

## Real-Time Device Sync

The `workers/` module provides a WebRTC signaling server for syncing secrets between online devices.

### How it works

1. Devices connect via WebSocket to the signaling server
2. Each room (project) has a Durable Object managing connections
3. Devices are tracked as "online" with presence updates
4. When a secret is updated, other devices are notified
5. WebRTC peer connections handle direct secret transfer

### API

| Endpoint | Method | Description |
|---|---|---|
| `wss://worker/stream?room=PROJECT&device=ID&name=NAME` | WebSocket | Signaling connection |
| `https://worker/api/rooms?room=PROJECT` | GET | List online devices |

### Message Types

| Type | Direction | Description |
|---|---|---|
| `offer` | peer-to-peer | WebRTC offer |
| `answer` | peer-to-peer | WebRTC answer |
| `ice-candidate` | peer-to-peer | ICE candidate |
| `secret-updated` | broadcast | Notify peers of secret change |
| `sync-request` | broadcast | Request state from peers |
| `sync-response` | peer-to-peer | Send state to requesting peer |
| `presence` | server-to-client | Online device list update |

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

## Security

| Concern | Mitigation |
|---|---|
| Vault at rest | AES-256-GCM, Argon2id KDF (64MB, t=3, p=4) |
| Directory permissions | `~/.envsec/` = 700, `vault.enc` = 600 |
| Passphrase storage | Only Argon2id hash stored, never plaintext |
| Clipboard leak | Forked background process clears after 2 min |
| Stdout leak | Secrets never printed; only masked values |
| Sync transport | WebRTC DTLS encrypted, signaling via WSS |

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
