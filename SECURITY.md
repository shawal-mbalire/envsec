# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| latest  | Yes                |
| < latest| No (please update) |

## Reporting a Vulnerability

If you discover a security vulnerability in envsec, please report it responsibly.

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, email the maintainer directly or use GitHub's private vulnerability reporting:

1. Go to the [Security tab](https://github.com/shawal-mbalire/envsec/security)
2. Click "Report a vulnerability"
3. Fill in the details

### What to include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response timeline

- Acknowledgment within 48 hours
- Initial assessment within 1 week
- Fix released within 30 days (depending on severity)

## Security Design

envsec follows these security principles:

- **Encryption at rest**: AES-256-GCM with Argon2id key derivation
- **No plaintext secrets**: Values never printed to stdout
- **Clipboard auto-clear**: Secrets cleared from clipboard after 2 minutes
- **Session expiry**: Authentication sessions time out (default 2 hours)
- **File permissions**: Vault directory (700) and file (600) restricted to owner
- **No telemetry**: No data leaves your machine

## Scope

This security policy covers the envsec binary and source code hosted at
github.com/shawal-mbalire/envsec.
